#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub mod Common;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
#[inline]
pub unsafe fn D2D1ComputeMaximumScaleFactor(matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> f32 {
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1ComputeMaximumScaleFactor ( matrix : *const super::super::super::Foundation::Numerics:: Matrix3x2 ) -> f32 );
    D2D1ComputeMaximumScaleFactor(matrix)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[inline]
pub unsafe fn D2D1ConvertColorSpace(sourcecolorspace: D2D1_COLOR_SPACE, destinationcolorspace: D2D1_COLOR_SPACE, color: *const Common::D2D1_COLOR_F) -> Common::D2D1_COLOR_F {
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1ConvertColorSpace ( sourcecolorspace : D2D1_COLOR_SPACE , destinationcolorspace : D2D1_COLOR_SPACE , color : *const Common:: D2D1_COLOR_F ) -> Common:: D2D1_COLOR_F );
    D2D1ConvertColorSpace(sourcecolorspace, destinationcolorspace, color)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn D2D1CreateDevice<P0>(dxgidevice: P0, creationproperties: ::core::option::Option<*const D2D1_CREATION_PROPERTIES>) -> ::windows::core::Result<ID2D1Device>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
{
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1CreateDevice ( dxgidevice : * mut::core::ffi::c_void , creationproperties : *const D2D1_CREATION_PROPERTIES , d2ddevice : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D2D1CreateDevice(dxgidevice.into().abi(), ::core::mem::transmute(creationproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn D2D1CreateDeviceContext<P0>(dxgisurface: P0, creationproperties: ::core::option::Option<*const D2D1_CREATION_PROPERTIES>) -> ::windows::core::Result<ID2D1DeviceContext>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
{
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1CreateDeviceContext ( dxgisurface : * mut::core::ffi::c_void , creationproperties : *const D2D1_CREATION_PROPERTIES , d2ddevicecontext : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D2D1CreateDeviceContext(dxgisurface.into().abi(), ::core::mem::transmute(creationproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[inline]
pub unsafe fn D2D1CreateFactory<T>(factorytype: D2D1_FACTORY_TYPE, pfactoryoptions: ::core::option::Option<*const D2D1_FACTORY_OPTIONS>) -> ::windows::core::Result<T>
where
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1CreateFactory ( factorytype : D2D1_FACTORY_TYPE , riid : *const :: windows::core::GUID , pfactoryoptions : *const D2D1_FACTORY_OPTIONS , ppifactory : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D2D1CreateFactory(factorytype, &<T as ::windows::core::Interface>::IID, ::core::mem::transmute(pfactoryoptions.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[inline]
pub unsafe fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch(ppoint0: *const Common::D2D_POINT_2F, ppoint1: *const Common::D2D_POINT_2F, ppoint2: *const Common::D2D_POINT_2F, ppoint3: *const Common::D2D_POINT_2F, ppoint4: *const Common::D2D_POINT_2F, ppoint5: *const Common::D2D_POINT_2F, ppoint6: *const Common::D2D_POINT_2F, ppoint7: *const Common::D2D_POINT_2F, ppoint8: *const Common::D2D_POINT_2F, ppoint9: *const Common::D2D_POINT_2F, ppoint10: *const Common::D2D_POINT_2F, ppoint11: *const Common::D2D_POINT_2F, ptensorpoint11: *mut Common::D2D_POINT_2F, ptensorpoint12: *mut Common::D2D_POINT_2F, ptensorpoint21: *mut Common::D2D_POINT_2F, ptensorpoint22: *mut Common::D2D_POINT_2F) {
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch ( ppoint0 : *const Common:: D2D_POINT_2F , ppoint1 : *const Common:: D2D_POINT_2F , ppoint2 : *const Common:: D2D_POINT_2F , ppoint3 : *const Common:: D2D_POINT_2F , ppoint4 : *const Common:: D2D_POINT_2F , ppoint5 : *const Common:: D2D_POINT_2F , ppoint6 : *const Common:: D2D_POINT_2F , ppoint7 : *const Common:: D2D_POINT_2F , ppoint8 : *const Common:: D2D_POINT_2F , ppoint9 : *const Common:: D2D_POINT_2F , ppoint10 : *const Common:: D2D_POINT_2F , ppoint11 : *const Common:: D2D_POINT_2F , ptensorpoint11 : *mut Common:: D2D_POINT_2F , ptensorpoint12 : *mut Common:: D2D_POINT_2F , ptensorpoint21 : *mut Common:: D2D_POINT_2F , ptensorpoint22 : *mut Common:: D2D_POINT_2F ) -> ( ) );
    D2D1GetGradientMeshInteriorPointsFromCoonsPatch(ppoint0, ppoint1, ppoint2, ppoint3, ppoint4, ppoint5, ppoint6, ppoint7, ppoint8, ppoint9, ppoint10, ppoint11, ptensorpoint11, ptensorpoint12, ptensorpoint21, ptensorpoint22)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn D2D1InvertMatrix(matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1InvertMatrix ( matrix : *mut super::super::super::Foundation::Numerics:: Matrix3x2 ) -> super::super::Foundation:: BOOL );
    D2D1InvertMatrix(matrix)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn D2D1IsMatrixInvertible(matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1IsMatrixInvertible ( matrix : *const super::super::super::Foundation::Numerics:: Matrix3x2 ) -> super::super::Foundation:: BOOL );
    D2D1IsMatrixInvertible(matrix)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
#[inline]
pub unsafe fn D2D1MakeRotateMatrix(angle: f32, center: Common::D2D_POINT_2F, matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1MakeRotateMatrix ( angle : f32 , center : Common:: D2D_POINT_2F , matrix : *mut super::super::super::Foundation::Numerics:: Matrix3x2 ) -> ( ) );
    D2D1MakeRotateMatrix(angle, ::core::mem::transmute(center), matrix)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
#[inline]
pub unsafe fn D2D1MakeSkewMatrix(anglex: f32, angley: f32, center: Common::D2D_POINT_2F, matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1MakeSkewMatrix ( anglex : f32 , angley : f32 , center : Common:: D2D_POINT_2F , matrix : *mut super::super::super::Foundation::Numerics:: Matrix3x2 ) -> ( ) );
    D2D1MakeSkewMatrix(anglex, angley, ::core::mem::transmute(center), matrix)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[inline]
pub unsafe fn D2D1SinCos(angle: f32, s: *mut f32, c: *mut f32) {
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1SinCos ( angle : f32 , s : *mut f32 , c : *mut f32 ) -> ( ) );
    D2D1SinCos(angle, s, c)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[inline]
pub unsafe fn D2D1Tan(angle: f32) -> f32 {
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1Tan ( angle : f32 ) -> f32 );
    D2D1Tan(angle)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[inline]
pub unsafe fn D2D1Vec3Length(x: f32, y: f32, z: f32) -> f32 {
    ::windows::core::link ! ( "d2d1.dll""system" fn D2D1Vec3Length ( x : f32 , y : f32 , z : f32 ) -> f32 );
    D2D1Vec3Length(x, y, z)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1AnalysisTransform(::windows::core::IUnknown);
impl ID2D1AnalysisTransform {
    pub unsafe fn ProcessAnalysisResults(&self, analysisdata: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProcessAnalysisResults)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(analysisdata.as_ptr()), analysisdata.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1AnalysisTransform, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1AnalysisTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1AnalysisTransform {}
unsafe impl ::core::marker::Sync for ID2D1AnalysisTransform {}
unsafe impl ::windows::core::Vtable for ID2D1AnalysisTransform {
    type Vtable = ID2D1AnalysisTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1AnalysisTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0359dc30_95e6_4568_9055_27720d130e93);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1AnalysisTransform_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ProcessAnalysisResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, analysisdata: *const u8, analysisdatacount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Bitmap(::windows::core::IUnknown);
impl ID2D1Bitmap {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CopyFromBitmap<P0>(&self, destpoint: ::core::option::Option<*const Common::D2D_POINT_2U>, bitmap: P0, srcrect: ::core::option::Option<*const Common::D2D_RECT_U>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).CopyFromBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(destpoint.unwrap_or(::std::ptr::null())), bitmap.into().abi(), ::core::mem::transmute(srcrect.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CopyFromRenderTarget<P0>(&self, destpoint: ::core::option::Option<*const Common::D2D_POINT_2U>, rendertarget: P0, srcrect: ::core::option::Option<*const Common::D2D_RECT_U>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1RenderTarget>>,
    {
        (::windows::core::Vtable::vtable(self).CopyFromRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(destpoint.unwrap_or(::std::ptr::null())), rendertarget.into().abi(), ::core::mem::transmute(srcrect.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CopyFromMemory(&self, dstrect: ::core::option::Option<*const Common::D2D_RECT_U>, srcdata: *const ::core::ffi::c_void, pitch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CopyFromMemory)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(dstrect.unwrap_or(::std::ptr::null())), srcdata, pitch).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1Bitmap, ::windows::core::IUnknown, ID2D1Resource, ID2D1Image);
impl ::core::clone::Clone for ID2D1Bitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Bitmap {}
unsafe impl ::core::marker::Sync for ID2D1Bitmap {}
unsafe impl ::windows::core::Vtable for ID2D1Bitmap {
    type Vtable = ID2D1Bitmap_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Bitmap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2296057_ea42_4099_983b_539fb6505426);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Bitmap_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPixelSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_U),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPixelSize: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_PIXEL_FORMAT),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    GetPixelFormat: usize,
    pub GetDpi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CopyFromBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destpoint: *const Common::D2D_POINT_2U, bitmap: *mut ::core::ffi::c_void, srcrect: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CopyFromBitmap: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CopyFromRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destpoint: *const Common::D2D_POINT_2U, rendertarget: *mut ::core::ffi::c_void, srcrect: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CopyFromRenderTarget: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CopyFromMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dstrect: *const Common::D2D_RECT_U, srcdata: *const ::core::ffi::c_void, pitch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CopyFromMemory: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Bitmap1(::windows::core::IUnknown);
impl ID2D1Bitmap1 {
    pub unsafe fn GetColorContext(&self) -> ::windows::core::Result<ID2D1ColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetColorContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1ColorContext as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetOptions(&self) -> D2D1_BITMAP_OPTIONS {
        (::windows::core::Vtable::vtable(self).GetOptions)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetSurface(&self) -> ::windows::core::Result<super::Dxgi::IDXGISurface> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSurface)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Map(&self, options: D2D1_MAP_OPTIONS) -> ::windows::core::Result<D2D1_MAPPED_RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Map)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unmap)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1Bitmap1, ::windows::core::IUnknown, ID2D1Resource, ID2D1Image, ID2D1Bitmap);
impl ::core::clone::Clone for ID2D1Bitmap1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Bitmap1 {}
unsafe impl ::core::marker::Sync for ID2D1Bitmap1 {}
unsafe impl ::windows::core::Vtable for ID2D1Bitmap1 {
    type Vtable = ID2D1Bitmap1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Bitmap1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa898a84c_3873_4588_b08b_ebbf978df041);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Bitmap1_Vtbl {
    pub base__: ID2D1Bitmap_Vtbl,
    pub GetColorContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorcontext: *mut *mut ::core::ffi::c_void),
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_BITMAP_OPTIONS,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub GetSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxgisurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    GetSurface: usize,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: D2D1_MAP_OPTIONS, mappedrect: *mut D2D1_MAPPED_RECT) -> ::windows::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1BitmapBrush(::windows::core::IUnknown);
impl ID2D1BitmapBrush {
    pub unsafe fn SetExtendModeX(&self, extendmodex: D2D1_EXTEND_MODE) {
        (::windows::core::Vtable::vtable(self).SetExtendModeX)(::windows::core::Vtable::as_raw(self), extendmodex)
    }
    pub unsafe fn SetExtendModeY(&self, extendmodey: D2D1_EXTEND_MODE) {
        (::windows::core::Vtable::vtable(self).SetExtendModeY)(::windows::core::Vtable::as_raw(self), extendmodey)
    }
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE) {
        (::windows::core::Vtable::vtable(self).SetInterpolationMode)(::windows::core::Vtable::as_raw(self), interpolationmode)
    }
    pub unsafe fn SetBitmap<P0>(&self, bitmap: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).SetBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi())
    }
    pub unsafe fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE {
        (::windows::core::Vtable::vtable(self).GetExtendModeX)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE {
        (::windows::core::Vtable::vtable(self).GetExtendModeY)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetInterpolationMode(&self) -> D2D1_BITMAP_INTERPOLATION_MODE {
        (::windows::core::Vtable::vtable(self).GetInterpolationMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBitmap(&self) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBitmap)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Bitmap as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
::windows::core::interface_hierarchy!(ID2D1BitmapBrush, ::windows::core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ::core::clone::Clone for ID2D1BitmapBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1BitmapBrush {}
unsafe impl ::core::marker::Sync for ID2D1BitmapBrush {}
unsafe impl ::windows::core::Vtable for ID2D1BitmapBrush {
    type Vtable = ID2D1BitmapBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1BitmapBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906aa_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1BitmapBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    pub SetExtendModeX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE),
    pub SetExtendModeY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendmodey: D2D1_EXTEND_MODE),
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE),
    pub SetBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void),
    pub GetExtendModeX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetExtendModeY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_BITMAP_INTERPOLATION_MODE,
    pub GetBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1BitmapBrush1(::windows::core::IUnknown);
impl ID2D1BitmapBrush1 {
    pub unsafe fn SetInterpolationMode1(&self, interpolationmode: D2D1_INTERPOLATION_MODE) {
        (::windows::core::Vtable::vtable(self).SetInterpolationMode1)(::windows::core::Vtable::as_raw(self), interpolationmode)
    }
    pub unsafe fn GetInterpolationMode1(&self) -> D2D1_INTERPOLATION_MODE {
        (::windows::core::Vtable::vtable(self).GetInterpolationMode1)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1BitmapBrush1, ::windows::core::IUnknown, ID2D1Resource, ID2D1Brush, ID2D1BitmapBrush);
impl ::core::clone::Clone for ID2D1BitmapBrush1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1BitmapBrush1 {}
unsafe impl ::core::marker::Sync for ID2D1BitmapBrush1 {}
unsafe impl ::windows::core::Vtable for ID2D1BitmapBrush1 {
    type Vtable = ID2D1BitmapBrush1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1BitmapBrush1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41343a53_e41a_49a2_91cd_21793bbb62e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1BitmapBrush1_Vtbl {
    pub base__: ID2D1BitmapBrush_Vtbl,
    pub SetInterpolationMode1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE),
    pub GetInterpolationMode1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_INTERPOLATION_MODE,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1BitmapRenderTarget(::windows::core::IUnknown);
impl ID2D1BitmapRenderTarget {
    pub unsafe fn GetBitmap(&self) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBitmap)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1BitmapRenderTarget, ::windows::core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ::core::clone::Clone for ID2D1BitmapRenderTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1BitmapRenderTarget {}
unsafe impl ::core::marker::Sync for ID2D1BitmapRenderTarget {}
unsafe impl ::windows::core::Vtable for ID2D1BitmapRenderTarget {
    type Vtable = ID2D1BitmapRenderTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1BitmapRenderTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd90695_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1BitmapRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    pub GetBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1BlendTransform(::windows::core::IUnknown);
impl ID2D1BlendTransform {
    pub unsafe fn SetDescription(&self, description: *const D2D1_BLEND_DESCRIPTION) {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), description)
    }
    pub unsafe fn GetDescription(&self, description: *mut D2D1_BLEND_DESCRIPTION) {
        (::windows::core::Vtable::vtable(self).GetDescription)(::windows::core::Vtable::as_raw(self), description)
    }
}
::windows::core::interface_hierarchy!(ID2D1BlendTransform, ::windows::core::IUnknown, ID2D1TransformNode, ID2D1ConcreteTransform);
impl ::core::clone::Clone for ID2D1BlendTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1BlendTransform {}
unsafe impl ::core::marker::Sync for ID2D1BlendTransform {}
unsafe impl ::windows::core::Vtable for ID2D1BlendTransform {
    type Vtable = ID2D1BlendTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1BlendTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63ac0b32_ba44_450f_8806_7f4ca1ff2f1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1BlendTransform_Vtbl {
    pub base__: ID2D1ConcreteTransform_Vtbl,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *const D2D1_BLEND_DESCRIPTION),
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut D2D1_BLEND_DESCRIPTION),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1BorderTransform(::windows::core::IUnknown);
impl ID2D1BorderTransform {
    pub unsafe fn SetExtendModeX(&self, extendmode: D2D1_EXTEND_MODE) {
        (::windows::core::Vtable::vtable(self).SetExtendModeX)(::windows::core::Vtable::as_raw(self), extendmode)
    }
    pub unsafe fn SetExtendModeY(&self, extendmode: D2D1_EXTEND_MODE) {
        (::windows::core::Vtable::vtable(self).SetExtendModeY)(::windows::core::Vtable::as_raw(self), extendmode)
    }
    pub unsafe fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE {
        (::windows::core::Vtable::vtable(self).GetExtendModeX)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE {
        (::windows::core::Vtable::vtable(self).GetExtendModeY)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1BorderTransform, ::windows::core::IUnknown, ID2D1TransformNode, ID2D1ConcreteTransform);
impl ::core::clone::Clone for ID2D1BorderTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1BorderTransform {}
unsafe impl ::core::marker::Sync for ID2D1BorderTransform {}
unsafe impl ::windows::core::Vtable for ID2D1BorderTransform {
    type Vtable = ID2D1BorderTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1BorderTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4998735c_3a19_473c_9781_656847e3a347);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1BorderTransform_Vtbl {
    pub base__: ID2D1ConcreteTransform_Vtbl,
    pub SetExtendModeX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendmode: D2D1_EXTEND_MODE),
    pub SetExtendModeY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendmode: D2D1_EXTEND_MODE),
    pub GetExtendModeX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetExtendModeY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1BoundsAdjustmentTransform(::windows::core::IUnknown);
impl ID2D1BoundsAdjustmentTransform {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputBounds(&self, outputbounds: *const super::super::Foundation::RECT) {
        (::windows::core::Vtable::vtable(self).SetOutputBounds)(::windows::core::Vtable::as_raw(self), outputbounds)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputBounds(&self) -> super::super::Foundation::RECT {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOutputBounds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
}
::windows::core::interface_hierarchy!(ID2D1BoundsAdjustmentTransform, ::windows::core::IUnknown, ID2D1TransformNode);
impl ::core::clone::Clone for ID2D1BoundsAdjustmentTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1BoundsAdjustmentTransform {}
unsafe impl ::core::marker::Sync for ID2D1BoundsAdjustmentTransform {}
unsafe impl ::windows::core::Vtable for ID2D1BoundsAdjustmentTransform {
    type Vtable = ID2D1BoundsAdjustmentTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1BoundsAdjustmentTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90f732e2_5092_4606_a819_8651970baccd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1BoundsAdjustmentTransform_Vtbl {
    pub base__: ID2D1TransformNode_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutputBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputbounds: *const super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutputBounds: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputbounds: *mut super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputBounds: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Brush(::windows::core::IUnknown);
impl ID2D1Brush {
    pub unsafe fn SetOpacity(&self, opacity: f32) {
        (::windows::core::Vtable::vtable(self).SetOpacity)(::windows::core::Vtable::as_raw(self), opacity)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn GetOpacity(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).GetOpacity)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
}
::windows::core::interface_hierarchy!(ID2D1Brush, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1Brush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Brush {}
unsafe impl ::core::marker::Sync for ID2D1Brush {}
unsafe impl ::windows::core::Vtable for ID2D1Brush {
    type Vtable = ID2D1Brush_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Brush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906a8_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Brush_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32),
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
    pub GetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetTransform: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1ColorContext(::windows::core::IUnknown);
impl ID2D1ColorContext {
    pub unsafe fn GetColorSpace(&self) -> D2D1_COLOR_SPACE {
        (::windows::core::Vtable::vtable(self).GetColorSpace)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetProfileSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetProfileSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetProfile(&self, profile: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetProfile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(profile.as_ptr()), profile.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1ColorContext, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1ColorContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1ColorContext {}
unsafe impl ::core::marker::Sync for ID2D1ColorContext {}
unsafe impl ::windows::core::Vtable for ID2D1ColorContext {
    type Vtable = ID2D1ColorContext_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1ColorContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c4820bb_5771_4518_a581_2fe4dd0ec657);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ColorContext_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE,
    pub GetProfileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut u8, profilesize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1ColorContext1(::windows::core::IUnknown);
impl ID2D1ColorContext1 {
    pub unsafe fn GetColorContextType(&self) -> D2D1_COLOR_CONTEXT_TYPE {
        (::windows::core::Vtable::vtable(self).GetColorContextType)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDXGIColorSpace(&self) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE {
        (::windows::core::Vtable::vtable(self).GetDXGIColorSpace)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSimpleColorProfile(&self, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSimpleColorProfile)(::windows::core::Vtable::as_raw(self), simpleprofile).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1ColorContext1, ::windows::core::IUnknown, ID2D1Resource, ID2D1ColorContext);
impl ::core::clone::Clone for ID2D1ColorContext1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1ColorContext1 {}
unsafe impl ::core::marker::Sync for ID2D1ColorContext1 {}
unsafe impl ::windows::core::Vtable for ID2D1ColorContext1 {
    type Vtable = ID2D1ColorContext1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1ColorContext1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ab42875_c57f_4be9_bd85_9cd78d6f55ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ColorContext1_Vtbl {
    pub base__: ID2D1ColorContext_Vtbl,
    pub GetColorContextType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_CONTEXT_TYPE,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDXGIColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDXGIColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSimpleColorProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSimpleColorProfile: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1CommandList(::windows::core::IUnknown);
impl ID2D1CommandList {
    pub unsafe fn Stream<P0>(&self, sink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1CommandSink>>,
    {
        (::windows::core::Vtable::vtable(self).Stream)(::windows::core::Vtable::as_raw(self), sink.into().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1CommandList, ::windows::core::IUnknown, ID2D1Resource, ID2D1Image);
impl ::core::clone::Clone for ID2D1CommandList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1CommandList {}
unsafe impl ::core::marker::Sync for ID2D1CommandList {}
unsafe impl ::windows::core::Vtable for ID2D1CommandList {
    type Vtable = ID2D1CommandList_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1CommandList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4f34a19_2383_4d76_94f6_ec343657c3dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandList_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1CommandSink(::windows::core::IUnknown);
impl ID2D1CommandSink {
    pub unsafe fn BeginDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).BeginDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EndDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode).ok()
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2).ok()
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTransform)(::windows::core::Vtable::as_raw(self), transform).ok()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, color: ::core::option::Option<*const Common::D2D1_COLOR_F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(color.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters1, layer.into().abi()).ok()
    }
    pub unsafe fn PopAxisAlignedClip(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PopLayer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PopLayer)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1CommandSink, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1CommandSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1CommandSink {}
unsafe impl ::core::marker::Sync for ID2D1CommandSink {}
unsafe impl ::windows::core::Vtable for ID2D1CommandSink {
    type Vtable = ID2D1CommandSink_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1CommandSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54d7898a_a061_40a7_bec7_e465bcba2c4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAntialiasMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::HRESULT,
    pub SetTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag1: u64, tag2: u64) -> ::windows::core::HRESULT,
    pub SetTextAntialiasMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textrenderingparams: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    SetTextRenderingParams: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
    pub SetPrimitiveBlend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::HRESULT,
    pub SetUnitMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unitmode: D2D1_UNIT_MODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Clear: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawGlyphRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut ::core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawLine: usize,
    pub DrawGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawBitmap: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawImage: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdimetafile: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile: usize,
    pub FillMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mesh: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillOpacityMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymask: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillOpacityMask: usize,
    pub FillGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, opacitybrush: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub PushAxisAlignedClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    PushAxisAlignedClip: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub PushLayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    PushLayer: usize,
    pub PopAxisAlignedClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PopLayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1CommandSink1(::windows::core::IUnknown);
impl ID2D1CommandSink1 {
    pub unsafe fn SetPrimitiveBlend1(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPrimitiveBlend1)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1CommandSink1, ::windows::core::IUnknown, ID2D1CommandSink);
impl ::core::clone::Clone for ID2D1CommandSink1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1CommandSink1 {}
unsafe impl ::core::marker::Sync for ID2D1CommandSink1 {}
unsafe impl ::windows::core::Vtable for ID2D1CommandSink1 {
    type Vtable = ID2D1CommandSink1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1CommandSink1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9eb767fd_4269_4467_b8c2_eb30cb305743);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink1_Vtbl {
    pub base__: ID2D1CommandSink_Vtbl,
    pub SetPrimitiveBlend1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1CommandSink2(::windows::core::IUnknown);
impl ID2D1CommandSink2 {
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Ink>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1InkStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawInk)(::windows::core::Vtable::as_raw(self), ink.into().abi(), brush.into().abi(), inkstyle.into().abi()).ok()
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGradientMesh)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile2<P0>(&self, gdimetafile: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGdiMetafile2)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1CommandSink2, ::windows::core::IUnknown, ID2D1CommandSink, ID2D1CommandSink1);
impl ::core::clone::Clone for ID2D1CommandSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1CommandSink2 {}
unsafe impl ::core::marker::Sync for ID2D1CommandSink2 {}
unsafe impl ::windows::core::Vtable for ID2D1CommandSink2 {
    type Vtable = ID2D1CommandSink2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1CommandSink2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bab440e_417e_47df_a2e2_bc0be6a00916);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink2_Vtbl {
    pub base__: ID2D1CommandSink1_Vtbl,
    pub DrawInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ink: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, inkstyle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DrawGradientMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientmesh: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdimetafile: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile2: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1CommandSink3(::windows::core::IUnknown);
impl ID2D1CommandSink3 {
    pub unsafe fn DrawSpriteBatch<P0, P1>(&self, spritebatch: P0, startindex: u32, spritecount: u32, bitmap: P1, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SpriteBatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).DrawSpriteBatch)(::windows::core::Vtable::as_raw(self), spritebatch.into().abi(), startindex, spritecount, bitmap.into().abi(), interpolationmode, spriteoptions).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1CommandSink3, ::windows::core::IUnknown, ID2D1CommandSink, ID2D1CommandSink1, ID2D1CommandSink2);
impl ::core::clone::Clone for ID2D1CommandSink3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1CommandSink3 {}
unsafe impl ::core::marker::Sync for ID2D1CommandSink3 {}
unsafe impl ::windows::core::Vtable for ID2D1CommandSink3 {
    type Vtable = ID2D1CommandSink3_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1CommandSink3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18079135_4cf3_4868_bc8e_06067e6d242d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink3_Vtbl {
    pub base__: ID2D1CommandSink2_Vtbl,
    pub DrawSpriteBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spritebatch: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, bitmap: *mut ::core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1CommandSink4(::windows::core::IUnknown);
impl ID2D1CommandSink4 {
    pub unsafe fn SetPrimitiveBlend2(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPrimitiveBlend2)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1CommandSink4, ::windows::core::IUnknown, ID2D1CommandSink, ID2D1CommandSink1, ID2D1CommandSink2, ID2D1CommandSink3);
impl ::core::clone::Clone for ID2D1CommandSink4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1CommandSink4 {}
unsafe impl ::core::marker::Sync for ID2D1CommandSink4 {}
unsafe impl ::windows::core::Vtable for ID2D1CommandSink4 {
    type Vtable = ID2D1CommandSink4_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1CommandSink4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc78a6519_40d6_4218_b2de_beeeb744bb3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink4_Vtbl {
    pub base__: ID2D1CommandSink3_Vtbl,
    pub SetPrimitiveBlend2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1CommandSink5(::windows::core::IUnknown);
impl ID2D1CommandSink5 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn BlendImage<P0>(&self, image: P0, blendmode: Common::D2D1_BLEND_MODE, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).BlendImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), blendmode, ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1CommandSink5, ::windows::core::IUnknown, ID2D1CommandSink, ID2D1CommandSink1, ID2D1CommandSink2, ID2D1CommandSink3, ID2D1CommandSink4);
impl ::core::clone::Clone for ID2D1CommandSink5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1CommandSink5 {}
unsafe impl ::core::marker::Sync for ID2D1CommandSink5 {}
unsafe impl ::windows::core::Vtable for ID2D1CommandSink5 {
    type Vtable = ID2D1CommandSink5_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1CommandSink5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7047dd26_b1e7_44a7_959a_8349e2144fa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink5_Vtbl {
    pub base__: ID2D1CommandSink4_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub BlendImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    BlendImage: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1ComputeInfo(::windows::core::IUnknown);
impl ID2D1ComputeInfo {
    pub unsafe fn SetComputeShaderConstantBuffer(&self, buffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetComputeShaderConstantBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buffer.as_ptr()), buffer.len() as _).ok()
    }
    pub unsafe fn SetComputeShader(&self, shaderid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetComputeShader)(::windows::core::Vtable::as_raw(self), shaderid).ok()
    }
    pub unsafe fn SetResourceTexture<P0>(&self, textureindex: u32, resourcetexture: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1ResourceTexture>>,
    {
        (::windows::core::Vtable::vtable(self).SetResourceTexture)(::windows::core::Vtable::as_raw(self), textureindex, resourcetexture.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1ComputeInfo, ::windows::core::IUnknown, ID2D1RenderInfo);
impl ::core::clone::Clone for ID2D1ComputeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1ComputeInfo {}
unsafe impl ::core::marker::Sync for ID2D1ComputeInfo {}
unsafe impl ::windows::core::Vtable for ID2D1ComputeInfo {
    type Vtable = ID2D1ComputeInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1ComputeInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5598b14b_9fd7_48b7_9bdb_8f0964eb38bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ComputeInfo_Vtbl {
    pub base__: ID2D1RenderInfo_Vtbl,
    pub SetComputeShaderConstantBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows::core::HRESULT,
    pub SetComputeShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetResourceTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textureindex: u32, resourcetexture: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1ComputeTransform(::windows::core::IUnknown);
impl ID2D1ComputeTransform {
    pub unsafe fn SetComputeInfo<P0>(&self, computeinfo: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1ComputeInfo>>,
    {
        (::windows::core::Vtable::vtable(self).SetComputeInfo)(::windows::core::Vtable::as_raw(self), computeinfo.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CalculateThreadgroups(&self, outputrect: *const super::super::Foundation::RECT, dimensionx: *mut u32, dimensiony: *mut u32, dimensionz: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CalculateThreadgroups)(::windows::core::Vtable::as_raw(self), outputrect, dimensionx, dimensiony, dimensionz).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1ComputeTransform, ::windows::core::IUnknown, ID2D1TransformNode, ID2D1Transform);
impl ::core::clone::Clone for ID2D1ComputeTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1ComputeTransform {}
unsafe impl ::core::marker::Sync for ID2D1ComputeTransform {}
unsafe impl ::windows::core::Vtable for ID2D1ComputeTransform {
    type Vtable = ID2D1ComputeTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1ComputeTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d85573c_01e3_4f7d_bfd9_0d60608bf3c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ComputeTransform_Vtbl {
    pub base__: ID2D1Transform_Vtbl,
    pub SetComputeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, computeinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CalculateThreadgroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputrect: *const super::super::Foundation::RECT, dimensionx: *mut u32, dimensiony: *mut u32, dimensionz: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CalculateThreadgroups: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1ConcreteTransform(::windows::core::IUnknown);
impl ID2D1ConcreteTransform {
    pub unsafe fn SetOutputBuffer(&self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOutputBuffer)(::windows::core::Vtable::as_raw(self), bufferprecision, channeldepth).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCached<P0>(&self, iscached: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetCached)(::windows::core::Vtable::as_raw(self), iscached.into())
    }
}
::windows::core::interface_hierarchy!(ID2D1ConcreteTransform, ::windows::core::IUnknown, ID2D1TransformNode);
impl ::core::clone::Clone for ID2D1ConcreteTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1ConcreteTransform {}
unsafe impl ::core::marker::Sync for ID2D1ConcreteTransform {}
unsafe impl ::windows::core::Vtable for ID2D1ConcreteTransform {
    type Vtable = ID2D1ConcreteTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1ConcreteTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a799d8a_69f7_4e4c_9fed_437ccc6684cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ConcreteTransform_Vtbl {
    pub base__: ID2D1TransformNode_Vtbl,
    pub SetOutputBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iscached: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCached: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DCRenderTarget(::windows::core::IUnknown);
impl ID2D1DCRenderTarget {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn BindDC<P0>(&self, hdc: P0, psubrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).BindDC)(::windows::core::Vtable::as_raw(self), hdc.into(), psubrect).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1DCRenderTarget, ::windows::core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ::core::clone::Clone for ID2D1DCRenderTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DCRenderTarget {}
unsafe impl ::core::marker::Sync for ID2D1DCRenderTarget {}
unsafe impl ::windows::core::Vtable for ID2D1DCRenderTarget {
    type Vtable = ID2D1DCRenderTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DCRenderTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c51bc64_de61_46fd_9899_63a5d8f03950);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DCRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub BindDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, psubrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    BindDC: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Device(::windows::core::IUnknown);
impl ID2D1Device {
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDeviceContext)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
    pub unsafe fn CreatePrintControl<P0, P1>(&self, wicfactory: P0, documenttarget: P1, printcontrolproperties: ::core::option::Option<*const D2D1_PRINT_CONTROL_PROPERTIES>) -> ::windows::core::Result<ID2D1PrintControl>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICImagingFactory>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePrintControl)(::windows::core::Vtable::as_raw(self), wicfactory.into().abi(), documenttarget.into().abi(), ::core::mem::transmute(printcontrolproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumTextureMemory(&self, maximuminbytes: u64) {
        (::windows::core::Vtable::vtable(self).SetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumTextureMemory(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ClearResources(&self, millisecondssinceuse: u32) {
        (::windows::core::Vtable::vtable(self).ClearResources)(::windows::core::Vtable::as_raw(self), millisecondssinceuse)
    }
}
::windows::core::interface_hierarchy!(ID2D1Device, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1Device {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Device {}
unsafe impl ::core::marker::Sync for ID2D1Device {}
unsafe impl ::windows::core::Vtable for ID2D1Device {
    type Vtable = ID2D1Device_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Device {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47dd575d_ac05_4cdd_8049_9b02cd16f44c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
    pub CreatePrintControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wicfactory: *mut ::core::ffi::c_void, documenttarget: *mut ::core::ffi::c_void, printcontrolproperties: *const D2D1_PRINT_CONTROL_PROPERTIES, printcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing")))]
    CreatePrintControl: usize,
    pub SetMaximumTextureMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maximuminbytes: u64),
    pub GetMaximumTextureMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub ClearResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, millisecondssinceuse: u32),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Device1(::windows::core::IUnknown);
impl ID2D1Device1 {
    pub unsafe fn GetRenderingPriority(&self) -> D2D1_RENDERING_PRIORITY {
        (::windows::core::Vtable::vtable(self).GetRenderingPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRenderingPriority(&self, renderingpriority: D2D1_RENDERING_PRIORITY) {
        (::windows::core::Vtable::vtable(self).SetRenderingPriority)(::windows::core::Vtable::as_raw(self), renderingpriority)
    }
    pub unsafe fn CreateDeviceContext2(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDeviceContext2)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Device1, ::windows::core::IUnknown, ID2D1Resource, ID2D1Device);
impl ::core::clone::Clone for ID2D1Device1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Device1 {}
unsafe impl ::core::marker::Sync for ID2D1Device1 {}
unsafe impl ::windows::core::Vtable for ID2D1Device1 {
    type Vtable = ID2D1Device1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Device1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd21768e1_23a4_4823_a14b_7c3eba85d658);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device1_Vtbl {
    pub base__: ID2D1Device_Vtbl,
    pub GetRenderingPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_RENDERING_PRIORITY,
    pub SetRenderingPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingpriority: D2D1_RENDERING_PRIORITY),
    pub CreateDeviceContext2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Device2(::windows::core::IUnknown);
impl ID2D1Device2 {
    pub unsafe fn CreateDeviceContext3(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDeviceContext3)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FlushDeviceContexts<P0>(&self, bitmap: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).FlushDeviceContexts)(::windows::core::Vtable::as_raw(self), bitmap.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetDxgiDevice(&self) -> ::windows::core::Result<super::Dxgi::IDXGIDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDxgiDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Device2, ::windows::core::IUnknown, ID2D1Resource, ID2D1Device, ID2D1Device1);
impl ::core::clone::Clone for ID2D1Device2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Device2 {}
unsafe impl ::core::marker::Sync for ID2D1Device2 {}
unsafe impl ::windows::core::Vtable for ID2D1Device2 {
    type Vtable = ID2D1Device2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Device2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa44472e1_8dfb_4e60_8492_6e2861c9ca8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device2_Vtbl {
    pub base__: ID2D1Device1_Vtbl,
    pub CreateDeviceContext3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FlushDeviceContexts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub GetDxgiDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxgidevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    GetDxgiDevice: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Device3(::windows::core::IUnknown);
impl ID2D1Device3 {
    pub unsafe fn CreateDeviceContext4(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDeviceContext4)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Device3, ::windows::core::IUnknown, ID2D1Resource, ID2D1Device, ID2D1Device1, ID2D1Device2);
impl ::core::clone::Clone for ID2D1Device3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Device3 {}
unsafe impl ::core::marker::Sync for ID2D1Device3 {}
unsafe impl ::windows::core::Vtable for ID2D1Device3 {
    type Vtable = ID2D1Device3_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Device3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x852f2087_802c_4037_ab60_ff2e7ee6fc01);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device3_Vtbl {
    pub base__: ID2D1Device2_Vtbl,
    pub CreateDeviceContext4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext3: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Device4(::windows::core::IUnknown);
impl ID2D1Device4 {
    pub unsafe fn CreateDeviceContext5(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDeviceContext5)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumColorGlyphCacheMemory(&self, maximuminbytes: u64) {
        (::windows::core::Vtable::vtable(self).SetMaximumColorGlyphCacheMemory)(::windows::core::Vtable::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumColorGlyphCacheMemory(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetMaximumColorGlyphCacheMemory)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1Device4, ::windows::core::IUnknown, ID2D1Resource, ID2D1Device, ID2D1Device1, ID2D1Device2, ID2D1Device3);
impl ::core::clone::Clone for ID2D1Device4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Device4 {}
unsafe impl ::core::marker::Sync for ID2D1Device4 {}
unsafe impl ::windows::core::Vtable for ID2D1Device4 {
    type Vtable = ID2D1Device4_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Device4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7bdb159_5683_4a46_bc9c_72dc720b858b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device4_Vtbl {
    pub base__: ID2D1Device3_Vtbl,
    pub CreateDeviceContext5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext4: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMaximumColorGlyphCacheMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maximuminbytes: u64),
    pub GetMaximumColorGlyphCacheMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Device5(::windows::core::IUnknown);
impl ID2D1Device5 {
    pub unsafe fn CreateDeviceContext6(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext5> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDeviceContext6)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Device5, ::windows::core::IUnknown, ID2D1Resource, ID2D1Device, ID2D1Device1, ID2D1Device2, ID2D1Device3, ID2D1Device4);
impl ::core::clone::Clone for ID2D1Device5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Device5 {}
unsafe impl ::core::marker::Sync for ID2D1Device5 {}
unsafe impl ::windows::core::Vtable for ID2D1Device5 {
    type Vtable = ID2D1Device5_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Device5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd55ba0a4_6405_4694_aef5_08ee1a4358b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device5_Vtbl {
    pub base__: ID2D1Device4_Vtbl,
    pub CreateDeviceContext6: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext5: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Device6(::windows::core::IUnknown);
impl ID2D1Device6 {
    pub unsafe fn CreateDeviceContext7(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext6> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDeviceContext7)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Device6, ::windows::core::IUnknown, ID2D1Resource, ID2D1Device, ID2D1Device1, ID2D1Device2, ID2D1Device3, ID2D1Device4, ID2D1Device5);
impl ::core::clone::Clone for ID2D1Device6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Device6 {}
unsafe impl ::core::marker::Sync for ID2D1Device6 {}
unsafe impl ::windows::core::Vtable for ID2D1Device6 {
    type Vtable = ID2D1Device6_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Device6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bfef914_2d75_4bad_be87_e18ddb077b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device6_Vtbl {
    pub base__: ID2D1Device5_Vtbl,
    pub CreateDeviceContext7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext6: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DeviceContext(::windows::core::IUnknown);
impl ID2D1DeviceContext {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap2(&self, size: Common::D2D_SIZE_U, sourcedata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> ::windows::core::Result<ID2D1Bitmap1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBitmap2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(sourcedata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap2<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBitmapFromWicBitmap2)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: ::core::option::Option<&[u8]>) -> ::windows::core::Result<ID2D1ColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateColorContext)(::windows::core::Vtable::as_raw(self), space, ::core::mem::transmute(profile.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateColorContextFromFilename)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICColorContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateColorContextFromWicColorContext)(::windows::core::Vtable::as_raw(self), wiccolorcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmapFromDxgiSurface<P0>(&self, surface: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBitmapFromDxgiSurface)(::windows::core::Vtable::as_raw(self), surface.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEffect(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Effect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEffect)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection2(&self, straightalphagradientstops: &[D2D1_GRADIENT_STOP], preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateGradientStopCollection2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(straightalphagradientstops.as_ptr()), straightalphagradientstops.len() as _, preinterpolationspace, postinterpolationspace, bufferprecision, extendmode, colorinterpolationmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1ImageBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateImageBrush)(::windows::core::Vtable::as_raw(self), image.into().abi(), imagebrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush2<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES1>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBitmapBrush2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList(&self) -> ::windows::core::Result<ID2D1CommandList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCommandList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsDxgiFormatSupported(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsDxgiFormatSupported)(::windows::core::Vtable::as_raw(self), format)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsBufferPrecisionSupported)(::windows::core::Vtable::as_raw(self), bufferprecision)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageLocalBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetImageLocalBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageWorldBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetImageWorldBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetGlyphRunWorldBounds(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGlyphRunWorldBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID2D1Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTarget<P0>(&self, image: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).SetTarget)(::windows::core::Vtable::as_raw(self), image.into().abi())
    }
    pub unsafe fn GetTarget(&self) -> ::windows::core::Result<ID2D1Image> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Image as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
        (::windows::core::Vtable::vtable(self).SetRenderingControls)(::windows::core::Vtable::as_raw(self), renderingcontrols)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRenderingControls(&self) -> D2D1_RENDERING_CONTROLS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRenderingControls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) {
        (::windows::core::Vtable::vtable(self).SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend)
    }
    pub unsafe fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND {
        (::windows::core::Vtable::vtable(self).GetPrimitiveBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) {
        (::windows::core::Vtable::vtable(self).SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode)
    }
    pub unsafe fn GetUnitMode(&self) -> D2D1_UNIT_MODE {
        (::windows::core::Vtable::vtable(self).GetUnitMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun2<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGlyphRun2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap2<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).DrawBitmap2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer2<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).PushLayer2)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn InvalidateEffectInputRectangle<P0>(&self, effect: P0, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).InvalidateEffectInputRectangle)(::windows::core::Vtable::as_raw(self), effect.into().abi(), input, inputrectangle).ok()
    }
    pub unsafe fn GetEffectInvalidRectangleCount<P0>(&self, effect: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEffectInvalidRectangleCount)(::windows::core::Vtable::as_raw(self), effect.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectInvalidRectangles<P0>(&self, effect: P0, rectangles: &mut [Common::D2D_RECT_F]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).GetEffectInvalidRectangles)(::windows::core::Vtable::as_raw(self), effect.into().abi(), ::core::mem::transmute(rectangles.as_ptr()), rectangles.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectRequiredInputRectangles<P0>(&self, rendereffect: P0, renderimagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).GetEffectRequiredInputRectangles)(::windows::core::Vtable::as_raw(self), rendereffect.into().abi(), ::core::mem::transmute(renderimagerectangle.unwrap_or(::std::ptr::null())), inputdescriptions, requiredinputrects, inputcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask2<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).FillOpacityMask2)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
}
::windows::core::interface_hierarchy!(ID2D1DeviceContext, ::windows::core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ::core::clone::Clone for ID2D1DeviceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DeviceContext {}
unsafe impl ::core::marker::Sync for ID2D1DeviceContext {}
unsafe impl ::windows::core::Vtable for ID2D1DeviceContext {
    type Vtable = ID2D1DeviceContext_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DeviceContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8f7fe7a_191c_466d_ad95_975678bda998);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmap2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: Common::D2D_SIZE_U, sourcedata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmap2: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateBitmapFromWicBitmap2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wicbitmapsource: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging")))]
    CreateBitmapFromWicBitmap2: usize,
    pub CreateColorContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateColorContextFromFilename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub CreateColorContextFromWicColorContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wiccolorcontext: *mut ::core::ffi::c_void, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    CreateColorContextFromWicColorContext: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmapFromDxgiSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmapFromDxgiSurface: usize,
    pub CreateEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectid: *const ::windows::core::GUID, effect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGradientStopCollection2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, straightalphagradientstops: *const D2D1_GRADIENT_STOP, straightalphagradientstopscount: u32, preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE, gradientstopcollection1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGradientStopCollection2: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateImageBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateImageBrush: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateBitmapBrush2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES1, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateBitmapBrush2: usize,
    pub CreateCommandList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub IsDxgiFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    IsDxgiFormatSupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsBufferPrecisionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsBufferPrecisionSupported: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetImageLocalBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, localbounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetImageLocalBounds: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetImageWorldBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, worldbounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetImageWorldBounds: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub GetGlyphRunWorldBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    GetGlyphRunWorldBounds: usize,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut *mut ::core::ffi::c_void),
    pub SetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void),
    pub GetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetRenderingControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingcontrols: *const D2D1_RENDERING_CONTROLS),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetRenderingControls: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetRenderingControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingcontrols: *mut D2D1_RENDERING_CONTROLS),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetRenderingControls: usize,
    pub SetPrimitiveBlend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND),
    pub GetPrimitiveBlend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_PRIMITIVE_BLEND,
    pub SetUnitMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unitmode: D2D1_UNIT_MODE),
    pub GetUnitMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_UNIT_MODE,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawGlyphRun2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut ::core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawGlyphRun2: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawImage: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdimetafile: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawBitmap2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawBitmap2: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub PushLayer2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: *mut ::core::ffi::c_void),
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    PushLayer2: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub InvalidateEffectInputRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    InvalidateEffectInputRectangle: usize,
    pub GetEffectInvalidRectangleCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, rectanglecount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEffectInvalidRectangles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, rectangles: *mut Common::D2D_RECT_F, rectanglescount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEffectInvalidRectangles: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEffectRequiredInputRectangles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rendereffect: *mut ::core::ffi::c_void, renderimagerectangle: *const Common::D2D_RECT_F, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEffectRequiredInputRectangles: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillOpacityMask2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymask: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillOpacityMask2: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DeviceContext1(::windows::core::IUnknown);
impl ID2D1DeviceContext1 {
    pub unsafe fn CreateFilledGeometryRealization<P0>(&self, geometry: P0, flatteningtolerance: f32) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFilledGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokedGeometryRealization<P0, P1>(&self, geometry: P0, flatteningtolerance: f32, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStrokedGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, strokewidth, strokestyle.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawGeometryRealization<P0, P1>(&self, geometryrealization: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GeometryRealization>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGeometryRealization)(::windows::core::Vtable::as_raw(self), geometryrealization.into().abi(), brush.into().abi())
    }
}
::windows::core::interface_hierarchy!(ID2D1DeviceContext1, ::windows::core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext);
impl ::core::clone::Clone for ID2D1DeviceContext1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DeviceContext1 {}
unsafe impl ::core::marker::Sync for ID2D1DeviceContext1 {}
unsafe impl ::windows::core::Vtable for ID2D1DeviceContext1 {
    type Vtable = ID2D1DeviceContext1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DeviceContext1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd37f57e4_6908_459f_a199_e72f24f79987);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext1_Vtbl {
    pub base__: ID2D1DeviceContext_Vtbl,
    pub CreateFilledGeometryRealization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, flatteningtolerance: f32, geometryrealization: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStrokedGeometryRealization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, flatteningtolerance: f32, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, geometryrealization: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DrawGeometryRealization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometryrealization: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DeviceContext2(::windows::core::IUnknown);
impl ID2D1DeviceContext2 {
    pub unsafe fn CreateInk(&self, startpoint: *const D2D1_INK_POINT) -> ::windows::core::Result<ID2D1Ink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInk)(::windows::core::Vtable::as_raw(self), startpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateInkStyle(&self, inkstyleproperties: ::core::option::Option<*const D2D1_INK_STYLE_PROPERTIES>) -> ::windows::core::Result<ID2D1InkStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInkStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(inkstyleproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientMesh(&self, patches: &[D2D1_GRADIENT_MESH_PATCH]) -> ::windows::core::Result<ID2D1GradientMesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateGradientMesh)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(patches.as_ptr()), patches.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateImageSourceFromWic<P0>(&self, wicbitmapsource: P0, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE) -> ::windows::core::Result<ID2D1ImageSourceFromWic>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateImageSourceFromWic)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), loadingoptions, alphamode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLookupTable3D(&self, precision: D2D1_BUFFER_PRECISION, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> ::windows::core::Result<ID2D1LookupTable3D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLookupTable3D)(::windows::core::Vtable::as_raw(self), precision, ::core::mem::transmute(extents.as_ptr()), ::core::mem::transmute(data.as_ptr()), data.len() as _, ::core::mem::transmute(strides.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateImageSourceFromDxgi(&self, surfaces: &[super::Dxgi::IDXGISurface], colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS) -> ::windows::core::Result<ID2D1ImageSource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateImageSourceFromDxgi)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(surfaces.as_ptr()), surfaces.len() as _, colorspace, options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientMeshWorldBounds<P0>(&self, gradientmesh: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGradientMeshWorldBounds)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Ink>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1InkStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawInk)(::windows::core::Vtable::as_raw(self), ink.into().abi(), brush.into().abi(), inkstyle.into().abi())
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGradientMesh)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile2<P0>(&self, gdimetafile: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGdiMetafile2)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CreateTransformedImageSource<P0>(&self, imagesource: P0, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) -> ::windows::core::Result<ID2D1TransformedImageSource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1ImageSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTransformedImageSource)(::windows::core::Vtable::as_raw(self), imagesource.into().abi(), properties, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1DeviceContext2, ::windows::core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext, ID2D1DeviceContext1);
impl ::core::clone::Clone for ID2D1DeviceContext2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DeviceContext2 {}
unsafe impl ::core::marker::Sync for ID2D1DeviceContext2 {}
unsafe impl ::windows::core::Vtable for ID2D1DeviceContext2 {
    type Vtable = ID2D1DeviceContext2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DeviceContext2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x394ea6a3_0c34_4321_950b_6ca20f0be6c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext2_Vtbl {
    pub base__: ID2D1DeviceContext1_Vtbl,
    pub CreateInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const D2D1_INK_POINT, ink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateInkStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkstyleproperties: *const D2D1_INK_STYLE_PROPERTIES, inkstyle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateInkStyle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGradientMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, patches: *const D2D1_GRADIENT_MESH_PATCH, patchescount: u32, gradientmesh: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGradientMesh: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateImageSourceFromWic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wicbitmapsource: *mut ::core::ffi::c_void, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE, imagesource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging")))]
    CreateImageSourceFromWic: usize,
    pub CreateLookupTable3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateImageSourceFromDxgi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surfaces: *const *mut ::core::ffi::c_void, surfacecount: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS, imagesource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateImageSourceFromDxgi: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientMeshWorldBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientmesh: *mut ::core::ffi::c_void, pbounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientMeshWorldBounds: usize,
    pub DrawInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ink: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, inkstyle: *mut ::core::ffi::c_void),
    pub DrawGradientMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientmesh: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdimetafile: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile2: usize,
    pub CreateTransformedImageSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagesource: *mut ::core::ffi::c_void, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES, transformedimagesource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DeviceContext3(::windows::core::IUnknown);
impl ID2D1DeviceContext3 {
    pub unsafe fn CreateSpriteBatch(&self) -> ::windows::core::Result<ID2D1SpriteBatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSpriteBatch)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawSpriteBatch<P0, P1>(&self, spritebatch: P0, startindex: u32, spritecount: u32, bitmap: P1, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SpriteBatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).DrawSpriteBatch)(::windows::core::Vtable::as_raw(self), spritebatch.into().abi(), startindex, spritecount, bitmap.into().abi(), interpolationmode, spriteoptions)
    }
}
::windows::core::interface_hierarchy!(ID2D1DeviceContext3, ::windows::core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext, ID2D1DeviceContext1, ID2D1DeviceContext2);
impl ::core::clone::Clone for ID2D1DeviceContext3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DeviceContext3 {}
unsafe impl ::core::marker::Sync for ID2D1DeviceContext3 {}
unsafe impl ::windows::core::Vtable for ID2D1DeviceContext3 {
    type Vtable = ID2D1DeviceContext3_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DeviceContext3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x235a7496_8351_414c_bcd4_6672ab2d8e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext3_Vtbl {
    pub base__: ID2D1DeviceContext2_Vtbl,
    pub CreateSpriteBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spritebatch: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DrawSpriteBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spritebatch: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, bitmap: *mut ::core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DeviceContext4(::windows::core::IUnknown);
impl ID2D1DeviceContext4 {
    pub unsafe fn CreateSvgGlyphStyle(&self) -> ::windows::core::Result<ID2D1SvgGlyphStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSvgGlyphStyle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText2<P0, P1, P2>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, svgglyphstyle: P2, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawText2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout2<P0, P1, P2>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, svgglyphstyle: P2, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawTextLayout2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawColorBitmapGlyphRun(&self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION) {
        (::windows::core::Vtable::vtable(self).DrawColorBitmapGlyphRun)(::windows::core::Vtable::as_raw(self), glyphimageformat, ::core::mem::transmute(baselineorigin), glyphrun, measuringmode, bitmapsnapoption)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawSvgGlyphRun<P0, P1>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: P0, svgglyphstyle: P1, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawSvgGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetColorBitmapGlyphImage<P0, P1>(&self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: Common::D2D_POINT_2F, fontface: P0, fontemsize: f32, glyphindex: u16, issideways: P1, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::core::option::Option<ID2D1Image>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteFontFace>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).GetColorBitmapGlyphImage)(::windows::core::Vtable::as_raw(self), glyphimageformat, ::core::mem::transmute(glyphorigin), fontface.into().abi(), fontemsize, glyphindex, issideways.into(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), dpix, dpiy, glyphtransform, ::core::mem::transmute(glyphimage)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetSvgGlyphImage<P0, P1, P2, P3>(&self, glyphorigin: Common::D2D_POINT_2F, fontface: P0, fontemsize: f32, glyphindex: u16, issideways: P1, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, defaultfillbrush: P2, svgglyphstyle: P3, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::core::option::Option<ID2D1CommandList>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteFontFace>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P3: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).GetSvgGlyphImage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(glyphorigin), fontface.into().abi(), fontemsize, glyphindex, issideways.into(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, glyphtransform, ::core::mem::transmute(glyphimage)).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1DeviceContext4, ::windows::core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext, ID2D1DeviceContext1, ID2D1DeviceContext2, ID2D1DeviceContext3);
impl ::core::clone::Clone for ID2D1DeviceContext4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DeviceContext4 {}
unsafe impl ::core::marker::Sync for ID2D1DeviceContext4 {}
unsafe impl ::windows::core::Vtable for ID2D1DeviceContext4 {
    type Vtable = ID2D1DeviceContext4_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DeviceContext4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c427831_3d90_4476_b647_c4fae349e4db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext4_Vtbl {
    pub base__: ID2D1DeviceContext3_Vtbl,
    pub CreateSvgGlyphStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, svgglyphstyle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawText2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, string: ::windows::core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawText2: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawTextLayout2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: Common::D2D_POINT_2F, textlayout: *mut ::core::ffi::c_void, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawTextLayout2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawColorBitmapGlyphRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawColorBitmapGlyphRun: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawSvgGlyphRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawSvgGlyphRun: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub GetColorBitmapGlyphImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: Common::D2D_POINT_2F, fontface: *mut ::core::ffi::c_void, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    GetColorBitmapGlyphImage: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub GetSvgGlyphImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphorigin: Common::D2D_POINT_2F, fontface: *mut ::core::ffi::c_void, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    GetSvgGlyphImage: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DeviceContext5(::windows::core::IUnknown);
impl ID2D1DeviceContext5 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSvgDocument<P0>(&self, inputxmlstream: P0, viewportsize: Common::D2D_SIZE_F) -> ::windows::core::Result<ID2D1SvgDocument>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSvgDocument)(::windows::core::Vtable::as_raw(self), inputxmlstream.into().abi(), ::core::mem::transmute(viewportsize), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawSvgDocument<P0>(&self, svgdocument: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SvgDocument>>,
    {
        (::windows::core::Vtable::vtable(self).DrawSvgDocument)(::windows::core::Vtable::as_raw(self), svgdocument.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateColorContextFromDxgiColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<ID2D1ColorContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateColorContextFromDxgiColorSpace)(::windows::core::Vtable::as_raw(self), colorspace, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateColorContextFromSimpleColorProfile(&self, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE) -> ::windows::core::Result<ID2D1ColorContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateColorContextFromSimpleColorProfile)(::windows::core::Vtable::as_raw(self), simpleprofile, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1DeviceContext5, ::windows::core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext, ID2D1DeviceContext1, ID2D1DeviceContext2, ID2D1DeviceContext3, ID2D1DeviceContext4);
impl ::core::clone::Clone for ID2D1DeviceContext5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DeviceContext5 {}
unsafe impl ::core::marker::Sync for ID2D1DeviceContext5 {}
unsafe impl ::windows::core::Vtable for ID2D1DeviceContext5 {
    type Vtable = ID2D1DeviceContext5_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DeviceContext5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7836d248_68cc_4df6_b9e8_de991bf62eb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext5_Vtbl {
    pub base__: ID2D1DeviceContext4_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
    pub CreateSvgDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputxmlstream: *mut ::core::ffi::c_void, viewportsize: Common::D2D_SIZE_F, svgdocument: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com")))]
    CreateSvgDocument: usize,
    pub DrawSvgDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, svgdocument: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateColorContextFromDxgiColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateColorContextFromDxgiColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateColorContextFromSimpleColorProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateColorContextFromSimpleColorProfile: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DeviceContext6(::windows::core::IUnknown);
impl ID2D1DeviceContext6 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn BlendImage<P0>(&self, image: P0, blendmode: Common::D2D1_BLEND_MODE, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).BlendImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), blendmode, ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode)
    }
}
::windows::core::interface_hierarchy!(ID2D1DeviceContext6, ::windows::core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext, ID2D1DeviceContext1, ID2D1DeviceContext2, ID2D1DeviceContext3, ID2D1DeviceContext4, ID2D1DeviceContext5);
impl ::core::clone::Clone for ID2D1DeviceContext6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DeviceContext6 {}
unsafe impl ::core::marker::Sync for ID2D1DeviceContext6 {}
unsafe impl ::windows::core::Vtable for ID2D1DeviceContext6 {
    type Vtable = ID2D1DeviceContext6_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DeviceContext6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x985f7e37_4ed0_4a19_98a3_15b0edfde306);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext6_Vtbl {
    pub base__: ID2D1DeviceContext5_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub BlendImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    BlendImage: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DrawInfo(::windows::core::IUnknown);
impl ID2D1DrawInfo {
    pub unsafe fn SetPixelShaderConstantBuffer(&self, buffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPixelShaderConstantBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buffer.as_ptr()), buffer.len() as _).ok()
    }
    pub unsafe fn SetResourceTexture<P0>(&self, textureindex: u32, resourcetexture: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1ResourceTexture>>,
    {
        (::windows::core::Vtable::vtable(self).SetResourceTexture)(::windows::core::Vtable::as_raw(self), textureindex, resourcetexture.into().abi()).ok()
    }
    pub unsafe fn SetVertexShaderConstantBuffer(&self, buffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVertexShaderConstantBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buffer.as_ptr()), buffer.len() as _).ok()
    }
    pub unsafe fn SetPixelShader(&self, shaderid: *const ::windows::core::GUID, pixeloptions: D2D1_PIXEL_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPixelShader)(::windows::core::Vtable::as_raw(self), shaderid, pixeloptions).ok()
    }
    pub unsafe fn SetVertexProcessing<P0>(&self, vertexbuffer: P0, vertexoptions: D2D1_VERTEX_OPTIONS, blenddescription: ::core::option::Option<*const D2D1_BLEND_DESCRIPTION>, vertexrange: ::core::option::Option<*const D2D1_VERTEX_RANGE>, vertexshader: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1VertexBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).SetVertexProcessing)(::windows::core::Vtable::as_raw(self), vertexbuffer.into().abi(), vertexoptions, ::core::mem::transmute(blenddescription.unwrap_or(::std::ptr::null())), ::core::mem::transmute(vertexrange.unwrap_or(::std::ptr::null())), ::core::mem::transmute(vertexshader.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1DrawInfo, ::windows::core::IUnknown, ID2D1RenderInfo);
impl ::core::clone::Clone for ID2D1DrawInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DrawInfo {}
unsafe impl ::core::marker::Sync for ID2D1DrawInfo {}
unsafe impl ::windows::core::Vtable for ID2D1DrawInfo {
    type Vtable = ID2D1DrawInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DrawInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x693ce632_7f2f_45de_93fe_18d88b37aa21);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DrawInfo_Vtbl {
    pub base__: ID2D1RenderInfo_Vtbl,
    pub SetPixelShaderConstantBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows::core::HRESULT,
    pub SetResourceTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textureindex: u32, resourcetexture: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVertexShaderConstantBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows::core::HRESULT,
    pub SetPixelShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID, pixeloptions: D2D1_PIXEL_OPTIONS) -> ::windows::core::HRESULT,
    pub SetVertexProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vertexbuffer: *mut ::core::ffi::c_void, vertexoptions: D2D1_VERTEX_OPTIONS, blenddescription: *const D2D1_BLEND_DESCRIPTION, vertexrange: *const D2D1_VERTEX_RANGE, vertexshader: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DrawTransform(::windows::core::IUnknown);
impl ID2D1DrawTransform {
    pub unsafe fn SetDrawInfo<P0>(&self, drawinfo: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawInfo>>,
    {
        (::windows::core::Vtable::vtable(self).SetDrawInfo)(::windows::core::Vtable::as_raw(self), drawinfo.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1DrawTransform, ::windows::core::IUnknown, ID2D1TransformNode, ID2D1Transform);
impl ::core::clone::Clone for ID2D1DrawTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DrawTransform {}
unsafe impl ::core::marker::Sync for ID2D1DrawTransform {}
unsafe impl ::windows::core::Vtable for ID2D1DrawTransform {
    type Vtable = ID2D1DrawTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DrawTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36bfdcb6_9739_435d_a30d_a653beff6a6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DrawTransform_Vtbl {
    pub base__: ID2D1Transform_Vtbl,
    pub SetDrawInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DrawingStateBlock(::windows::core::IUnknown);
impl ID2D1DrawingStateBlock {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetDescription(&self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION) {
        (::windows::core::Vtable::vtable(self).GetDescription)(::windows::core::Vtable::as_raw(self), statedescription)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetDescription(&self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION) {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), statedescription)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
::windows::core::interface_hierarchy!(ID2D1DrawingStateBlock, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1DrawingStateBlock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DrawingStateBlock {}
unsafe impl ::core::marker::Sync for ID2D1DrawingStateBlock {}
unsafe impl ::windows::core::Vtable for ID2D1DrawingStateBlock {
    type Vtable = ID2D1DrawingStateBlock_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DrawingStateBlock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28506e39_ebf6_46a1_bb47_fd85565ab957);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DrawingStateBlock_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetDescription: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textrenderingparams: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    SetTextRenderingParams: usize,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub GetTextRenderingParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textrenderingparams: *mut *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    GetTextRenderingParams: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1DrawingStateBlock1(::windows::core::IUnknown);
impl ID2D1DrawingStateBlock1 {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetDescription2(&self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1) {
        (::windows::core::Vtable::vtable(self).GetDescription2)(::windows::core::Vtable::as_raw(self), statedescription)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetDescription2(&self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1) {
        (::windows::core::Vtable::vtable(self).SetDescription2)(::windows::core::Vtable::as_raw(self), statedescription)
    }
}
::windows::core::interface_hierarchy!(ID2D1DrawingStateBlock1, ::windows::core::IUnknown, ID2D1Resource, ID2D1DrawingStateBlock);
impl ::core::clone::Clone for ID2D1DrawingStateBlock1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1DrawingStateBlock1 {}
unsafe impl ::core::marker::Sync for ID2D1DrawingStateBlock1 {}
unsafe impl ::windows::core::Vtable for ID2D1DrawingStateBlock1 {
    type Vtable = ID2D1DrawingStateBlock1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1DrawingStateBlock1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x689f1f85_c72e_4e33_8f19_85754efd5ace);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DrawingStateBlock1_Vtbl {
    pub base__: ID2D1DrawingStateBlock_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetDescription2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetDescription2: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDescription2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDescription2: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Effect(::windows::core::IUnknown);
impl ID2D1Effect {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInput<P0, P1>(&self, index: u32, input: P0, invalidate: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetInput)(::windows::core::Vtable::as_raw(self), index, input.into().abi(), invalidate.into())
    }
    pub unsafe fn SetInputCount(&self, inputcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInputCount)(::windows::core::Vtable::as_raw(self), inputcount).ok()
    }
    pub unsafe fn GetInput(&self, index: u32) -> ::windows::core::Result<ID2D1Image> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInput)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr());
        <ID2D1Image as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetOutput(&self) -> ::windows::core::Result<ID2D1Image> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Image as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
::windows::core::interface_hierarchy!(ID2D1Effect, ::windows::core::IUnknown, ID2D1Properties);
impl ::core::clone::Clone for ID2D1Effect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Effect {}
unsafe impl ::core::marker::Sync for ID2D1Effect {}
unsafe impl ::windows::core::Vtable for ID2D1Effect {
    type Vtable = ID2D1Effect_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Effect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28211a43_7d89_476f_8181_2d6159b220ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Effect_Vtbl {
    pub base__: ID2D1Properties_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, invalidate: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInput: usize,
    pub SetInputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputcount: u32) -> ::windows::core::HRESULT,
    pub GetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, input: *mut *mut ::core::ffi::c_void),
    pub GetInputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputimage: *mut *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1EffectContext(::windows::core::IUnknown);
impl ID2D1EffectContext {
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn CreateEffect(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Effect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEffect)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetMaximumSupportedFeatureLevel(&self, featurelevels: &[super::Direct3D::D3D_FEATURE_LEVEL]) -> ::windows::core::Result<super::Direct3D::D3D_FEATURE_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMaximumSupportedFeatureLevel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(featurelevels.as_ptr()), featurelevels.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTransformNodeFromEffect<P0>(&self, effect: P0) -> ::windows::core::Result<ID2D1TransformNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTransformNodeFromEffect)(::windows::core::Vtable::as_raw(self), effect.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBlendTransform(&self, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION) -> ::windows::core::Result<ID2D1BlendTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBlendTransform)(::windows::core::Vtable::as_raw(self), numinputs, blenddescription, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBorderTransform(&self, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1BorderTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBorderTransform)(::windows::core::Vtable::as_raw(self), extendmodex, extendmodey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOffsetTransform(&self, offset: super::super::Foundation::POINT) -> ::windows::core::Result<ID2D1OffsetTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateOffsetTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(offset), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBoundsAdjustmentTransform(&self, outputrectangle: *const super::super::Foundation::RECT) -> ::windows::core::Result<ID2D1BoundsAdjustmentTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBoundsAdjustmentTransform)(::windows::core::Vtable::as_raw(self), outputrectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LoadPixelShader(&self, shaderid: *const ::windows::core::GUID, shaderbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).LoadPixelShader)(::windows::core::Vtable::as_raw(self), shaderid, ::core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len() as _).ok()
    }
    pub unsafe fn LoadVertexShader(&self, resourceid: *const ::windows::core::GUID, shaderbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).LoadVertexShader)(::windows::core::Vtable::as_raw(self), resourceid, ::core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len() as _).ok()
    }
    pub unsafe fn LoadComputeShader(&self, resourceid: *const ::windows::core::GUID, shaderbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).LoadComputeShader)(::windows::core::Vtable::as_raw(self), resourceid, ::core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShaderLoaded(&self, shaderid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsShaderLoaded)(::windows::core::Vtable::as_raw(self), shaderid)
    }
    pub unsafe fn CreateResourceTexture(&self, resourceid: ::core::option::Option<*const ::windows::core::GUID>, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: ::core::option::Option<&[u8]>, strides: ::core::option::Option<*const u32>) -> ::windows::core::Result<ID2D1ResourceTexture> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateResourceTexture)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(resourceid.unwrap_or(::std::ptr::null())), resourcetextureproperties, ::core::mem::transmute(data.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(strides.unwrap_or(::std::ptr::null())), data.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindResourceTexture(&self, resourceid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1ResourceTexture> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindResourceTexture)(::windows::core::Vtable::as_raw(self), resourceid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVertexBuffer(&self, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: ::core::option::Option<*const ::windows::core::GUID>, customvertexbufferproperties: ::core::option::Option<*const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES>) -> ::windows::core::Result<ID2D1VertexBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateVertexBuffer)(::windows::core::Vtable::as_raw(self), vertexbufferproperties, ::core::mem::transmute(resourceid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(customvertexbufferproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindVertexBuffer(&self, resourceid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1VertexBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindVertexBuffer)(::windows::core::Vtable::as_raw(self), resourceid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: ::core::option::Option<&[u8]>) -> ::windows::core::Result<ID2D1ColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateColorContext)(::windows::core::Vtable::as_raw(self), space, ::core::mem::transmute(profile.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateColorContextFromFilename)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICColorContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateColorContextFromWicColorContext)(::windows::core::Vtable::as_raw(self), wiccolorcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D2D1_FEATURE, featuresupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, featuresupportdata, featuresupportdatasize).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsBufferPrecisionSupported)(::windows::core::Vtable::as_raw(self), bufferprecision)
    }
}
::windows::core::interface_hierarchy!(ID2D1EffectContext, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1EffectContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1EffectContext {}
unsafe impl ::core::marker::Sync for ID2D1EffectContext {}
unsafe impl ::windows::core::Vtable for ID2D1EffectContext {
    type Vtable = ID2D1EffectContext_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1EffectContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d9f916b_27dc_4ad7_b4f1_64945340f563);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1EffectContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDpi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32),
    pub CreateEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectid: *const ::windows::core::GUID, effect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetMaximumSupportedFeatureLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, featurelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevelscount: u32, maximumsupportedfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetMaximumSupportedFeatureLevel: usize,
    pub CreateTransformNodeFromEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, transformnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBlendTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION, transform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBorderTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE, transform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateOffsetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: super::super::Foundation::POINT, transform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateOffsetTransform: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateBoundsAdjustmentTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputrectangle: *const super::super::Foundation::RECT, transform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateBoundsAdjustmentTransform: usize,
    pub LoadPixelShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows::core::HRESULT,
    pub LoadVertexShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows::core::HRESULT,
    pub LoadComputeShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsShaderLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsShaderLoaded: usize,
    pub CreateResourceTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: *const u8, strides: *const u32, datasize: u32, resourcetexture: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindResourceTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, resourcetexture: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVertexBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: *const ::windows::core::GUID, customvertexbufferproperties: *const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES, buffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVertexBuffer: usize,
    pub FindVertexBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, buffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateColorContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateColorContextFromFilename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub CreateColorContextFromWicColorContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wiccolorcontext: *mut ::core::ffi::c_void, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    CreateColorContextFromWicColorContext: usize,
    pub CheckFeatureSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: D2D1_FEATURE, featuresupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsBufferPrecisionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsBufferPrecisionSupported: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1EffectContext1(::windows::core::IUnknown);
impl ID2D1EffectContext1 {
    pub unsafe fn CreateLookupTable3D(&self, precision: D2D1_BUFFER_PRECISION, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> ::windows::core::Result<ID2D1LookupTable3D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLookupTable3D)(::windows::core::Vtable::as_raw(self), precision, ::core::mem::transmute(extents.as_ptr()), ::core::mem::transmute(data.as_ptr()), data.len() as _, ::core::mem::transmute(strides.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1EffectContext1, ::windows::core::IUnknown, ID2D1EffectContext);
impl ::core::clone::Clone for ID2D1EffectContext1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1EffectContext1 {}
unsafe impl ::core::marker::Sync for ID2D1EffectContext1 {}
unsafe impl ::windows::core::Vtable for ID2D1EffectContext1 {
    type Vtable = ID2D1EffectContext1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1EffectContext1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84ab595a_fc81_4546_bacd_e8ef4d8abe7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1EffectContext1_Vtbl {
    pub base__: ID2D1EffectContext_Vtbl,
    pub CreateLookupTable3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1EffectContext2(::windows::core::IUnknown);
impl ID2D1EffectContext2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateColorContextFromDxgiColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<ID2D1ColorContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateColorContextFromDxgiColorSpace)(::windows::core::Vtable::as_raw(self), colorspace, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateColorContextFromSimpleColorProfile(&self, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE) -> ::windows::core::Result<ID2D1ColorContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateColorContextFromSimpleColorProfile)(::windows::core::Vtable::as_raw(self), simpleprofile, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1EffectContext2, ::windows::core::IUnknown, ID2D1EffectContext, ID2D1EffectContext1);
impl ::core::clone::Clone for ID2D1EffectContext2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1EffectContext2 {}
unsafe impl ::core::marker::Sync for ID2D1EffectContext2 {}
unsafe impl ::windows::core::Vtable for ID2D1EffectContext2 {
    type Vtable = ID2D1EffectContext2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1EffectContext2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x577ad2a0_9fc7_4dda_8b18_dab810140052);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1EffectContext2_Vtbl {
    pub base__: ID2D1EffectContext1_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateColorContextFromDxgiColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateColorContextFromDxgiColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateColorContextFromSimpleColorProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateColorContextFromSimpleColorProfile: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1EffectImpl(::windows::core::IUnknown);
impl ID2D1EffectImpl {
    pub unsafe fn Initialize<P0, P1>(&self, effectcontext: P0, transformgraph: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1EffectContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1TransformGraph>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), effectcontext.into().abi(), transformgraph.into().abi()).ok()
    }
    pub unsafe fn PrepareForRender(&self, changetype: D2D1_CHANGE_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PrepareForRender)(::windows::core::Vtable::as_raw(self), changetype).ok()
    }
    pub unsafe fn SetGraph<P0>(&self, transformgraph: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TransformGraph>>,
    {
        (::windows::core::Vtable::vtable(self).SetGraph)(::windows::core::Vtable::as_raw(self), transformgraph.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1EffectImpl, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1EffectImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1EffectImpl {}
unsafe impl ::core::marker::Sync for ID2D1EffectImpl {}
unsafe impl ::windows::core::Vtable for ID2D1EffectImpl {
    type Vtable = ID2D1EffectImpl_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1EffectImpl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa248fd3f_3e6c_4e63_9f03_7f68ecc91db9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1EffectImpl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectcontext: *mut ::core::ffi::c_void, transformgraph: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrepareForRender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: D2D1_CHANGE_TYPE) -> ::windows::core::HRESULT,
    pub SetGraph: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transformgraph: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1EllipseGeometry(::windows::core::IUnknown);
impl ID2D1EllipseGeometry {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEllipse(&self) -> D2D1_ELLIPSE {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEllipse)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
}
::windows::core::interface_hierarchy!(ID2D1EllipseGeometry, ::windows::core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ::core::clone::Clone for ID2D1EllipseGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1EllipseGeometry {}
unsafe impl ::core::marker::Sync for ID2D1EllipseGeometry {}
unsafe impl ::windows::core::Vtable for ID2D1EllipseGeometry {
    type Vtable = ID2D1EllipseGeometry_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1EllipseGeometry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906a4_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1EllipseGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEllipse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ellipse: *mut D2D1_ELLIPSE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEllipse: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Factory(::windows::core::IUnknown);
impl ID2D1Factory {
    pub unsafe fn ReloadSystemMetrics(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReloadSystemMetrics)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).GetDesktopDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRectangleGeometry(&self, rectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<ID2D1RectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRectangleGeometry)(::windows::core::Vtable::as_raw(self), rectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> ::windows::core::Result<ID2D1RoundedRectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRoundedRectangleGeometry)(::windows::core::Vtable::as_raw(self), roundedrectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> ::windows::core::Result<ID2D1EllipseGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEllipseGeometry)(::windows::core::Vtable::as_raw(self), ellipse, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGeometryGroup(&self, fillmode: Common::D2D1_FILL_MODE, geometries: &[ID2D1Geometry]) -> ::windows::core::Result<ID2D1GeometryGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateGeometryGroup)(::windows::core::Vtable::as_raw(self), fillmode, ::core::mem::transmute(geometries.as_ptr()), geometries.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateTransformedGeometry<P0>(&self, sourcegeometry: P0, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<ID2D1TransformedGeometry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTransformedGeometry)(::windows::core::Vtable::as_raw(self), sourcegeometry.into().abi(), transform, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry(&self) -> ::windows::core::Result<ID2D1PathGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePathGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStrokeStyle)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDrawingStateBlock)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(&self, target: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateWicBitmapRenderTarget)(::windows::core::Vtable::as_raw(self), target.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1HwndRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateHwndRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, hwndrendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(&self, dxgisurface: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDxgiSurfaceRenderTarget)(::windows::core::Vtable::as_raw(self), dxgisurface.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1DCRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDCRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Factory, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1Factory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Factory {}
unsafe impl ::core::marker::Sync for ID2D1Factory {}
unsafe impl ::windows::core::Vtable for ID2D1Factory {
    type Vtable = ID2D1Factory_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Factory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06152247_6f50_465a_9245_118bfd3b6007);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReloadSystemMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDesktopDpi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateRectangleGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: *const Common::D2D_RECT_F, rectanglegeometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateRectangleGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateRoundedRectangleGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roundedrectangle: *const D2D1_ROUNDED_RECT, roundedrectanglegeometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateRoundedRectangleGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateEllipseGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, ellipsegeometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateEllipseGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGeometryGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillmode: Common::D2D1_FILL_MODE, geometries: *const *mut ::core::ffi::c_void, geometriescount: u32, geometrygroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGeometryGroup: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateTransformedGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcegeometry: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2, transformedgeometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateTransformedGeometry: usize,
    pub CreatePathGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pathgeometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStrokeStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: *const f32, dashescount: u32, strokestyle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub CreateDrawingStateBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION, textrenderingparams: *mut ::core::ffi::c_void, drawingstateblock: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite")))]
    CreateDrawingStateBlock: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateWicBitmapRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging")))]
    CreateWicBitmapRenderTarget: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateHwndRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES, hwndrendertarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateHwndRenderTarget: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDxgiSurfaceRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxgisurface: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDxgiSurfaceRenderTarget: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDCRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, dcrendertarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDCRenderTarget: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Factory1(::windows::core::IUnknown);
impl ID2D1Factory1 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDevice)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle2(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStrokeStyle2)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry2(&self) -> ::windows::core::Result<ID2D1PathGeometry1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePathGeometry2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock2<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION1>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDrawingStateBlock2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGdiMetafile<P0>(&self, metafilestream: P0) -> ::windows::core::Result<ID2D1GdiMetafile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateGdiMetafile)(::windows::core::Vtable::as_raw(self), metafilestream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterEffectFromStream<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterEffectFromStream)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn RegisterEffectFromString<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterEffectFromString)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn UnregisterEffect(&self, classid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterEffect)(::windows::core::Vtable::as_raw(self), classid).ok()
    }
    pub unsafe fn GetRegisteredEffects(&self, effects: ::core::option::Option<&mut [::windows::core::GUID]>, effectsreturned: ::core::option::Option<*mut u32>, effectsregistered: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetRegisteredEffects)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(effects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), effects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(effectsreturned.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(effectsregistered.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetEffectProperties(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Properties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEffectProperties)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Factory1, ::windows::core::IUnknown, ID2D1Factory);
impl ::core::clone::Clone for ID2D1Factory1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Factory1 {}
unsafe impl ::core::marker::Sync for ID2D1Factory1 {}
unsafe impl ::windows::core::Vtable for ID2D1Factory1 {
    type Vtable = ID2D1Factory1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Factory1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb12d362_daee_4b9a_aa1d_14ba401cfa1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory1_Vtbl {
    pub base__: ID2D1Factory_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice: usize,
    pub CreateStrokeStyle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: *const f32, dashescount: u32, strokestyle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePathGeometry2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pathgeometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub CreateDrawingStateBlock2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1, textrenderingparams: *mut ::core::ffi::c_void, drawingstateblock: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite")))]
    CreateDrawingStateBlock2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGdiMetafile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metafilestream: *mut ::core::ffi::c_void, metafile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGdiMetafile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RegisterEffectFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: *const ::windows::core::GUID, propertyxml: *mut ::core::ffi::c_void, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegisterEffectFromStream: usize,
    pub RegisterEffectFromString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: *const ::windows::core::GUID, propertyxml: ::windows::core::PCWSTR, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::HRESULT,
    pub UnregisterEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetRegisteredEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effects: *mut ::windows::core::GUID, effectscount: u32, effectsreturned: *mut u32, effectsregistered: *mut u32) -> ::windows::core::HRESULT,
    pub GetEffectProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectid: *const ::windows::core::GUID, properties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Factory2(::windows::core::IUnknown);
impl ID2D1Factory2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice2<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDevice2)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Factory2, ::windows::core::IUnknown, ID2D1Factory, ID2D1Factory1);
impl ::core::clone::Clone for ID2D1Factory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Factory2 {}
unsafe impl ::core::marker::Sync for ID2D1Factory2 {}
unsafe impl ::windows::core::Vtable for ID2D1Factory2 {
    type Vtable = ID2D1Factory2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Factory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94f81a73_9212_4376_9c58_b16a3a0d3992);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory2_Vtbl {
    pub base__: ID2D1Factory1_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice2: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Factory3(::windows::core::IUnknown);
impl ID2D1Factory3 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice3<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDevice3)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Factory3, ::windows::core::IUnknown, ID2D1Factory, ID2D1Factory1, ID2D1Factory2);
impl ::core::clone::Clone for ID2D1Factory3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Factory3 {}
unsafe impl ::core::marker::Sync for ID2D1Factory3 {}
unsafe impl ::windows::core::Vtable for ID2D1Factory3 {
    type Vtable = ID2D1Factory3_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Factory3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0869759f_4f00_413f_b03e_2bda45404d0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory3_Vtbl {
    pub base__: ID2D1Factory2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice3: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Factory4(::windows::core::IUnknown);
impl ID2D1Factory4 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice4<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDevice4)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Factory4, ::windows::core::IUnknown, ID2D1Factory, ID2D1Factory1, ID2D1Factory2, ID2D1Factory3);
impl ::core::clone::Clone for ID2D1Factory4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Factory4 {}
unsafe impl ::core::marker::Sync for ID2D1Factory4 {}
unsafe impl ::windows::core::Vtable for ID2D1Factory4 {
    type Vtable = ID2D1Factory4_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Factory4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd4ec2d2_0662_4bee_ba8e_6f29f032e096);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory4_Vtbl {
    pub base__: ID2D1Factory3_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice3: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice4: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Factory5(::windows::core::IUnknown);
impl ID2D1Factory5 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice5<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device4>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDevice5)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Factory5, ::windows::core::IUnknown, ID2D1Factory, ID2D1Factory1, ID2D1Factory2, ID2D1Factory3, ID2D1Factory4);
impl ::core::clone::Clone for ID2D1Factory5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Factory5 {}
unsafe impl ::core::marker::Sync for ID2D1Factory5 {}
unsafe impl ::windows::core::Vtable for ID2D1Factory5 {
    type Vtable = ID2D1Factory5_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Factory5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4349994_838e_4b0f_8cab_44997d9eeacc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory5_Vtbl {
    pub base__: ID2D1Factory4_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice4: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice5: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Factory6(::windows::core::IUnknown);
impl ID2D1Factory6 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice6<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device5>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDevice6)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Factory6, ::windows::core::IUnknown, ID2D1Factory, ID2D1Factory1, ID2D1Factory2, ID2D1Factory3, ID2D1Factory4, ID2D1Factory5);
impl ::core::clone::Clone for ID2D1Factory6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Factory6 {}
unsafe impl ::core::marker::Sync for ID2D1Factory6 {}
unsafe impl ::windows::core::Vtable for ID2D1Factory6 {
    type Vtable = ID2D1Factory6_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Factory6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9976f46_f642_44c1_97ca_da32ea2a2635);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory6_Vtbl {
    pub base__: ID2D1Factory5_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice6: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice5: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice6: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Factory7(::windows::core::IUnknown);
impl ID2D1Factory7 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice7<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device6>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDevice7)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Factory7, ::windows::core::IUnknown, ID2D1Factory, ID2D1Factory1, ID2D1Factory2, ID2D1Factory3, ID2D1Factory4, ID2D1Factory5, ID2D1Factory6);
impl ::core::clone::Clone for ID2D1Factory7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Factory7 {}
unsafe impl ::core::marker::Sync for ID2D1Factory7 {}
unsafe impl ::windows::core::Vtable for ID2D1Factory7 {
    type Vtable = ID2D1Factory7_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Factory7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdc2bdd3_b96c_4de6_bdf7_99d4745454de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory7_Vtbl {
    pub base__: ID2D1Factory6_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice6: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice7: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1GdiInteropRenderTarget(::windows::core::IUnknown);
impl ID2D1GdiInteropRenderTarget {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC(&self, mode: D2D1_DC_INITIALIZE_MODE) -> ::windows::core::Result<super::Gdi::HDC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDC)(::windows::core::Vtable::as_raw(self), mode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseDC(&self, update: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReleaseDC)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(update.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1GdiInteropRenderTarget, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1GdiInteropRenderTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1GdiInteropRenderTarget {}
unsafe impl ::core::marker::Sync for ID2D1GdiInteropRenderTarget {}
unsafe impl ::windows::core::Vtable for ID2D1GdiInteropRenderTarget {
    type Vtable = ID2D1GdiInteropRenderTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1GdiInteropRenderTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0db51c3_6f77_4bae_b3d5_e47509b35838);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GdiInteropRenderTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: D2D1_DC_INITIALIZE_MODE, hdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, update: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseDC: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1GdiMetafile(::windows::core::IUnknown);
impl ID2D1GdiMetafile {
    pub unsafe fn Stream<P0>(&self, sink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafileSink>>,
    {
        (::windows::core::Vtable::vtable(self).Stream)(::windows::core::Vtable::as_raw(self), sink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetBounds(&self) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBounds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1GdiMetafile, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1GdiMetafile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1GdiMetafile {}
unsafe impl ::core::marker::Sync for ID2D1GdiMetafile {}
unsafe impl ::windows::core::Vtable for ID2D1GdiMetafile {
    type Vtable = ID2D1GdiMetafile_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1GdiMetafile {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f543dc3_cfc1_4211_864f_cfd91c6f3395);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GdiMetafile_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetBounds: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1GdiMetafile1(::windows::core::IUnknown);
impl ID2D1GdiMetafile1 {
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSourceBounds(&self) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSourceBounds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1GdiMetafile1, ::windows::core::IUnknown, ID2D1Resource, ID2D1GdiMetafile);
impl ::core::clone::Clone for ID2D1GdiMetafile1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1GdiMetafile1 {}
unsafe impl ::core::marker::Sync for ID2D1GdiMetafile1 {}
unsafe impl ::windows::core::Vtable for ID2D1GdiMetafile1 {
    type Vtable = ID2D1GdiMetafile1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1GdiMetafile1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e69f9e8_dd3f_4bf9_95ba_c04f49d788df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GdiMetafile1_Vtbl {
    pub base__: ID2D1GdiMetafile_Vtbl,
    pub GetDpi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSourceBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSourceBounds: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1GdiMetafileSink(::windows::core::IUnknown);
impl ID2D1GdiMetafileSink {
    pub unsafe fn ProcessRecord(&self, recordtype: u32, recorddata: ::core::option::Option<*const ::core::ffi::c_void>, recorddatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProcessRecord)(::windows::core::Vtable::as_raw(self), recordtype, ::core::mem::transmute(recorddata.unwrap_or(::std::ptr::null())), recorddatasize).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1GdiMetafileSink, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1GdiMetafileSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1GdiMetafileSink {}
unsafe impl ::core::marker::Sync for ID2D1GdiMetafileSink {}
unsafe impl ::windows::core::Vtable for ID2D1GdiMetafileSink {
    type Vtable = ID2D1GdiMetafileSink_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1GdiMetafileSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82237326_8111_4f7c_bcf4_b5c1175564fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GdiMetafileSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ProcessRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1GdiMetafileSink1(::windows::core::IUnknown);
impl ID2D1GdiMetafileSink1 {
    pub unsafe fn ProcessRecord2(&self, recordtype: u32, recorddata: ::core::option::Option<*const ::core::ffi::c_void>, recorddatasize: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProcessRecord2)(::windows::core::Vtable::as_raw(self), recordtype, ::core::mem::transmute(recorddata.unwrap_or(::std::ptr::null())), recorddatasize, flags).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1GdiMetafileSink1, ::windows::core::IUnknown, ID2D1GdiMetafileSink);
impl ::core::clone::Clone for ID2D1GdiMetafileSink1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1GdiMetafileSink1 {}
unsafe impl ::core::marker::Sync for ID2D1GdiMetafileSink1 {}
unsafe impl ::windows::core::Vtable for ID2D1GdiMetafileSink1 {
    type Vtable = ID2D1GdiMetafileSink1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1GdiMetafileSink1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd0ecb6b_91e6_411e_8655_395e760f91b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GdiMetafileSink1_Vtbl {
    pub base__: ID2D1GdiMetafileSink_Vtbl,
    pub ProcessRecord2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32, flags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Geometry(::windows::core::IUnknown);
impl ID2D1Geometry {
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetBounds(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetWidenedBounds<P0>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWidenedBounds)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn StrokeContainsPoint<P0>(&self, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StrokeContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn FillContainsPoint(&self, point: Common::D2D_POINT_2F, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FillContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CompareWithGeometry<P0>(&self, inputgeometry: P0, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<D2D1_GEOMETRY_RELATION>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CompareWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Simplify<P0>(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).Simplify)(::windows::core::Vtable::as_raw(self), simplificationoption, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn Tessellate<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, tessellationsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TessellationSink>>,
    {
        (::windows::core::Vtable::vtable(self).Tessellate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, tessellationsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CombineWithGeometry<P0, P1>(&self, inputgeometry: P0, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).CombineWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), combinemode, ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Outline<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).Outline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeArea(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ComputeArea)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeLength(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ComputeLength)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputePointAtLength(&self, length: f32, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, point: ::core::option::Option<*mut Common::D2D_POINT_2F>, unittangentvector: ::core::option::Option<*mut Common::D2D_POINT_2F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ComputePointAtLength)(::windows::core::Vtable::as_raw(self), length, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, ::core::mem::transmute(point.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(unittangentvector.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Widen<P0, P1>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).Widen)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1Geometry, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1Geometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Geometry {}
unsafe impl ::core::marker::Sync for ID2D1Geometry {}
unsafe impl ::windows::core::Vtable for ID2D1Geometry {
    type Vtable = ID2D1Geometry_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Geometry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906a1_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Geometry_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetBounds: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetWidenedBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetWidenedBounds: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub StrokeContainsPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    StrokeContainsPoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub FillContainsPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    FillContainsPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CompareWithGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputgeometry: *mut ::core::ffi::c_void, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, relation: *mut D2D1_GEOMETRY_RELATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CompareWithGeometry: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub Simplify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    Simplify: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Tessellate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, tessellationsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Tessellate: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CombineWithGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputgeometry: *mut ::core::ffi::c_void, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CombineWithGeometry: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub Outline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    Outline: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputeArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, area: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputeArea: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, length: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputeLength: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub ComputePointAtLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: f32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, point: *mut Common::D2D_POINT_2F, unittangentvector: *mut Common::D2D_POINT_2F) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    ComputePointAtLength: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub Widen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    Widen: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1GeometryGroup(::windows::core::IUnknown);
impl ID2D1GeometryGroup {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetFillMode(&self) -> Common::D2D1_FILL_MODE {
        (::windows::core::Vtable::vtable(self).GetFillMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetSourceGeometryCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetSourceGeometryCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetSourceGeometries(&self, geometries: &mut [::core::option::Option<ID2D1Geometry>]) {
        (::windows::core::Vtable::vtable(self).GetSourceGeometries)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(geometries.as_ptr()), geometries.len() as _)
    }
}
::windows::core::interface_hierarchy!(ID2D1GeometryGroup, ::windows::core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ::core::clone::Clone for ID2D1GeometryGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1GeometryGroup {}
unsafe impl ::core::marker::Sync for ID2D1GeometryGroup {}
unsafe impl ::windows::core::Vtable for ID2D1GeometryGroup {
    type Vtable = ID2D1GeometryGroup_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1GeometryGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906a6_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GeometryGroup_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetFillMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> Common::D2D1_FILL_MODE,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetFillMode: usize,
    pub GetSourceGeometryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetSourceGeometries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometries: *mut *mut ::core::ffi::c_void, geometriescount: u32),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1GeometryRealization(::windows::core::IUnknown);
impl ID2D1GeometryRealization {}
::windows::core::interface_hierarchy!(ID2D1GeometryRealization, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1GeometryRealization {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1GeometryRealization {}
unsafe impl ::core::marker::Sync for ID2D1GeometryRealization {}
unsafe impl ::windows::core::Vtable for ID2D1GeometryRealization {
    type Vtable = ID2D1GeometryRealization_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1GeometryRealization {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa16907d7_bc02_4801_99e8_8cf7f485f774);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GeometryRealization_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[repr(transparent)]
pub struct ID2D1GeometrySink(::windows::core::IUnknown);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GeometrySink {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddLine(&self, point: Common::D2D_POINT_2F) {
        (::windows::core::Vtable::vtable(self).AddLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddBezier(&self, bezier: *const Common::D2D1_BEZIER_SEGMENT) {
        (::windows::core::Vtable::vtable(self).AddBezier)(::windows::core::Vtable::as_raw(self), bezier)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddQuadraticBezier(&self, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT) {
        (::windows::core::Vtable::vtable(self).AddQuadraticBezier)(::windows::core::Vtable::as_raw(self), bezier)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddQuadraticBeziers(&self, beziers: &[D2D1_QUADRATIC_BEZIER_SEGMENT]) {
        (::windows::core::Vtable::vtable(self).AddQuadraticBeziers)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(beziers.as_ptr()), beziers.len() as _)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddArc(&self, arc: *const D2D1_ARC_SEGMENT) {
        (::windows::core::Vtable::vtable(self).AddArc)(::windows::core::Vtable::as_raw(self), arc)
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
::windows::core::interface_hierarchy!(ID2D1GeometrySink, ::windows::core::IUnknown, Common::ID2D1SimplifiedGeometrySink);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for ID2D1GeometrySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::core::marker::Send for ID2D1GeometrySink {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::core::marker::Sync for ID2D1GeometrySink {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Vtable for ID2D1GeometrySink {
    type Vtable = ID2D1GeometrySink_Vtbl;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Interface for ID2D1GeometrySink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd9069f_12e2_11dc_9fed_001143a055f9);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GeometrySink_Vtbl {
    pub base__: Common::ID2D1SimplifiedGeometrySink_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddLine: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddBezier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bezier: *const Common::D2D1_BEZIER_SEGMENT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddBezier: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddQuadraticBezier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddQuadraticBezier: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddQuadraticBeziers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT, bezierscount: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddQuadraticBeziers: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddArc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arc: *const D2D1_ARC_SEGMENT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddArc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1GradientMesh(::windows::core::IUnknown);
impl ID2D1GradientMesh {
    pub unsafe fn GetPatchCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetPatchCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPatches(&self, startindex: u32, patches: &mut [D2D1_GRADIENT_MESH_PATCH]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPatches)(::windows::core::Vtable::as_raw(self), startindex, ::core::mem::transmute(patches.as_ptr()), patches.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1GradientMesh, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1GradientMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1GradientMesh {}
unsafe impl ::core::marker::Sync for ID2D1GradientMesh {}
unsafe impl ::windows::core::Vtable for ID2D1GradientMesh {
    type Vtable = ID2D1GradientMesh_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1GradientMesh {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf292e401_c050_4cde_83d7_04962d3b23c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GradientMesh_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetPatchCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPatches: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, patches: *mut D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPatches: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1GradientStopCollection(::windows::core::IUnknown);
impl ID2D1GradientStopCollection {
    pub unsafe fn GetGradientStopCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetGradientStopCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientStops(&self, gradientstops: &mut [D2D1_GRADIENT_STOP]) {
        (::windows::core::Vtable::vtable(self).GetGradientStops)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _)
    }
    pub unsafe fn GetColorInterpolationGamma(&self) -> D2D1_GAMMA {
        (::windows::core::Vtable::vtable(self).GetColorInterpolationGamma)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetExtendMode(&self) -> D2D1_EXTEND_MODE {
        (::windows::core::Vtable::vtable(self).GetExtendMode)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1GradientStopCollection, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1GradientStopCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1GradientStopCollection {}
unsafe impl ::core::marker::Sync for ID2D1GradientStopCollection {}
unsafe impl ::windows::core::Vtable for ID2D1GradientStopCollection {
    type Vtable = ID2D1GradientStopCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1GradientStopCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906a7_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GradientStopCollection_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetGradientStopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientStops: usize,
    pub GetColorInterpolationGamma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_GAMMA,
    pub GetExtendMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1GradientStopCollection1(::windows::core::IUnknown);
impl ID2D1GradientStopCollection1 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientStops1(&self, gradientstops: &mut [D2D1_GRADIENT_STOP]) {
        (::windows::core::Vtable::vtable(self).GetGradientStops1)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _)
    }
    pub unsafe fn GetPreInterpolationSpace(&self) -> D2D1_COLOR_SPACE {
        (::windows::core::Vtable::vtable(self).GetPreInterpolationSpace)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPostInterpolationSpace(&self) -> D2D1_COLOR_SPACE {
        (::windows::core::Vtable::vtable(self).GetPostInterpolationSpace)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBufferPrecision(&self) -> D2D1_BUFFER_PRECISION {
        (::windows::core::Vtable::vtable(self).GetBufferPrecision)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> D2D1_COLOR_INTERPOLATION_MODE {
        (::windows::core::Vtable::vtable(self).GetColorInterpolationMode)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1GradientStopCollection1, ::windows::core::IUnknown, ID2D1Resource, ID2D1GradientStopCollection);
impl ::core::clone::Clone for ID2D1GradientStopCollection1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1GradientStopCollection1 {}
unsafe impl ::core::marker::Sync for ID2D1GradientStopCollection1 {}
unsafe impl ::windows::core::Vtable for ID2D1GradientStopCollection1 {
    type Vtable = ID2D1GradientStopCollection1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1GradientStopCollection1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae1572f4_5dd0_4777_998b_9279472ae63b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GradientStopCollection1_Vtbl {
    pub base__: ID2D1GradientStopCollection_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientStops1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientStops1: usize,
    pub GetPreInterpolationSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE,
    pub GetPostInterpolationSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE,
    pub GetBufferPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_BUFFER_PRECISION,
    pub GetColorInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_INTERPOLATION_MODE,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1HwndRenderTarget(::windows::core::IUnknown);
impl ID2D1HwndRenderTarget {
    pub unsafe fn CheckWindowState(&self) -> D2D1_WINDOW_STATE {
        (::windows::core::Vtable::vtable(self).CheckWindowState)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Resize(&self, pixelsize: *const Common::D2D_SIZE_U) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Resize)(::windows::core::Vtable::as_raw(self), pixelsize).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> super::super::Foundation::HWND {
        (::windows::core::Vtable::vtable(self).GetHwnd)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1HwndRenderTarget, ::windows::core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ::core::clone::Clone for ID2D1HwndRenderTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1HwndRenderTarget {}
unsafe impl ::core::marker::Sync for ID2D1HwndRenderTarget {}
unsafe impl ::windows::core::Vtable for ID2D1HwndRenderTarget {
    type Vtable = ID2D1HwndRenderTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1HwndRenderTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd90698_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1HwndRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    pub CheckWindowState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_WINDOW_STATE,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelsize: *const Common::D2D_SIZE_U) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Resize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HWND,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHwnd: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Image(::windows::core::IUnknown);
impl ID2D1Image {}
::windows::core::interface_hierarchy!(ID2D1Image, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1Image {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Image {}
unsafe impl ::core::marker::Sync for ID2D1Image {}
unsafe impl ::windows::core::Vtable for ID2D1Image {
    type Vtable = ID2D1Image_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Image {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65019f75_8da2_497c_b32c_dfa34e48ede6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Image_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1ImageBrush(::windows::core::IUnknown);
impl ID2D1ImageBrush {
    pub unsafe fn SetImage<P0>(&self, image: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).SetImage)(::windows::core::Vtable::as_raw(self), image.into().abi())
    }
    pub unsafe fn SetExtendModeX(&self, extendmodex: D2D1_EXTEND_MODE) {
        (::windows::core::Vtable::vtable(self).SetExtendModeX)(::windows::core::Vtable::as_raw(self), extendmodex)
    }
    pub unsafe fn SetExtendModeY(&self, extendmodey: D2D1_EXTEND_MODE) {
        (::windows::core::Vtable::vtable(self).SetExtendModeY)(::windows::core::Vtable::as_raw(self), extendmodey)
    }
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: D2D1_INTERPOLATION_MODE) {
        (::windows::core::Vtable::vtable(self).SetInterpolationMode)(::windows::core::Vtable::as_raw(self), interpolationmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetSourceRectangle(&self, sourcerectangle: *const Common::D2D_RECT_F) {
        (::windows::core::Vtable::vtable(self).SetSourceRectangle)(::windows::core::Vtable::as_raw(self), sourcerectangle)
    }
    pub unsafe fn GetImage(&self) -> ::windows::core::Result<ID2D1Image> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetImage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Image as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE {
        (::windows::core::Vtable::vtable(self).GetExtendModeX)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE {
        (::windows::core::Vtable::vtable(self).GetExtendModeY)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetInterpolationMode(&self) -> D2D1_INTERPOLATION_MODE {
        (::windows::core::Vtable::vtable(self).GetInterpolationMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSourceRectangle(&self) -> Common::D2D_RECT_F {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSourceRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
}
::windows::core::interface_hierarchy!(ID2D1ImageBrush, ::windows::core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ::core::clone::Clone for ID2D1ImageBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1ImageBrush {}
unsafe impl ::core::marker::Sync for ID2D1ImageBrush {}
unsafe impl ::windows::core::Vtable for ID2D1ImageBrush {
    type Vtable = ID2D1ImageBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1ImageBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe9e984d_3f95_407c_b5db_cb94d4e8f87c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ImageBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void),
    pub SetExtendModeX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE),
    pub SetExtendModeY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendmodey: D2D1_EXTEND_MODE),
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetSourceRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcerectangle: *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetSourceRectangle: usize,
    pub GetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut *mut ::core::ffi::c_void),
    pub GetExtendModeX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetExtendModeY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_INTERPOLATION_MODE,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSourceRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcerectangle: *mut Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSourceRectangle: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1ImageSource(::windows::core::IUnknown);
impl ID2D1ImageSource {
    pub unsafe fn OfferResources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OfferResources)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryReclaimResources(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TryReclaimResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1ImageSource, ::windows::core::IUnknown, ID2D1Resource, ID2D1Image);
impl ::core::clone::Clone for ID2D1ImageSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1ImageSource {}
unsafe impl ::core::marker::Sync for ID2D1ImageSource {}
unsafe impl ::windows::core::Vtable for ID2D1ImageSource {
    type Vtable = ID2D1ImageSource_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1ImageSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9b664e5_74a1_4378_9ac2_eefc37a3f4d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ImageSource_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    pub OfferResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TryReclaimResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcesdiscarded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TryReclaimResources: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1ImageSourceFromWic(::windows::core::IUnknown);
impl ID2D1ImageSourceFromWic {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EnsureCached(&self, rectangletofill: ::core::option::Option<*const Common::D2D_RECT_U>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EnsureCached)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rectangletofill.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn TrimCache(&self, rectangletopreserve: ::core::option::Option<*const Common::D2D_RECT_U>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).TrimCache)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rectangletopreserve.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<super::Imaging::IWICBitmapSource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::Imaging::IWICBitmapSource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
::windows::core::interface_hierarchy!(ID2D1ImageSourceFromWic, ::windows::core::IUnknown, ID2D1Resource, ID2D1Image, ID2D1ImageSource);
impl ::core::clone::Clone for ID2D1ImageSourceFromWic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1ImageSourceFromWic {}
unsafe impl ::core::marker::Sync for ID2D1ImageSourceFromWic {}
unsafe impl ::windows::core::Vtable for ID2D1ImageSourceFromWic {
    type Vtable = ID2D1ImageSourceFromWic_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1ImageSourceFromWic {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77395441_1c8f_4555_8683_f50dab0fe792);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ImageSourceFromWic_Vtbl {
    pub base__: ID2D1ImageSource_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub EnsureCached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangletofill: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    EnsureCached: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub TrimCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangletopreserve: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    TrimCache: usize,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wicbitmapsource: *mut *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetSource: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Ink(::windows::core::IUnknown);
impl ID2D1Ink {
    pub unsafe fn SetStartPoint(&self, startpoint: *const D2D1_INK_POINT) {
        (::windows::core::Vtable::vtable(self).SetStartPoint)(::windows::core::Vtable::as_raw(self), startpoint)
    }
    pub unsafe fn GetStartPoint(&self) -> D2D1_INK_POINT {
        let mut result__: D2D1_INK_POINT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetStartPoint)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn AddSegments(&self, segments: &[D2D1_INK_BEZIER_SEGMENT]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddSegments)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(segments.as_ptr()), segments.len() as _).ok()
    }
    pub unsafe fn RemoveSegmentsAtEnd(&self, segmentscount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveSegmentsAtEnd)(::windows::core::Vtable::as_raw(self), segmentscount).ok()
    }
    pub unsafe fn SetSegments(&self, startsegment: u32, segments: &[D2D1_INK_BEZIER_SEGMENT]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSegments)(::windows::core::Vtable::as_raw(self), startsegment, ::core::mem::transmute(segments.as_ptr()), segments.len() as _).ok()
    }
    pub unsafe fn SetSegmentAtEnd(&self, segment: *const D2D1_INK_BEZIER_SEGMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSegmentAtEnd)(::windows::core::Vtable::as_raw(self), segment).ok()
    }
    pub unsafe fn GetSegmentCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetSegmentCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetSegments(&self, startsegment: u32, segments: &mut [D2D1_INK_BEZIER_SEGMENT]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSegments)(::windows::core::Vtable::as_raw(self), startsegment, ::core::mem::transmute(segments.as_ptr()), segments.len() as _).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn StreamAsGeometry<P0, P1>(&self, inkstyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1InkStyle>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).StreamAsGeometry)(::windows::core::Vtable::as_raw(self), inkstyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetBounds<P0>(&self, inkstyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1InkStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBounds)(::windows::core::Vtable::as_raw(self), inkstyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Ink, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1Ink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Ink {}
unsafe impl ::core::marker::Sync for ID2D1Ink {}
unsafe impl ::windows::core::Vtable for ID2D1Ink {
    type Vtable = ID2D1Ink_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Ink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb499923b_7029_478f_a8b3_432c7c5f5312);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Ink_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const D2D1_INK_POINT),
    pub GetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D2D1_INK_POINT),
    pub AddSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows::core::HRESULT,
    pub RemoveSegmentsAtEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentscount: u32) -> ::windows::core::HRESULT,
    pub SetSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startsegment: u32, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows::core::HRESULT,
    pub SetSegmentAtEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segment: *const D2D1_INK_BEZIER_SEGMENT) -> ::windows::core::HRESULT,
    pub GetSegmentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startsegment: u32, segments: *mut D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub StreamAsGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkstyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    StreamAsGeometry: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkstyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetBounds: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1InkStyle(::windows::core::IUnknown);
impl ID2D1InkStyle {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetNibTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).SetNibTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetNibTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).GetNibTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetNibShape(&self, nibshape: D2D1_INK_NIB_SHAPE) {
        (::windows::core::Vtable::vtable(self).SetNibShape)(::windows::core::Vtable::as_raw(self), nibshape)
    }
    pub unsafe fn GetNibShape(&self) -> D2D1_INK_NIB_SHAPE {
        (::windows::core::Vtable::vtable(self).GetNibShape)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1InkStyle, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1InkStyle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1InkStyle {}
unsafe impl ::core::marker::Sync for ID2D1InkStyle {}
unsafe impl ::windows::core::Vtable for ID2D1InkStyle {
    type Vtable = ID2D1InkStyle_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1InkStyle {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbae8b344_23fc_4071_8cb5_d05d6f073848);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1InkStyle_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetNibTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetNibTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetNibTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetNibTransform: usize,
    pub SetNibShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nibshape: D2D1_INK_NIB_SHAPE),
    pub GetNibShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_INK_NIB_SHAPE,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Layer(::windows::core::IUnknown);
impl ID2D1Layer {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
::windows::core::interface_hierarchy!(ID2D1Layer, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1Layer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Layer {}
unsafe impl ::core::marker::Sync for ID2D1Layer {}
unsafe impl ::windows::core::Vtable for ID2D1Layer {
    type Vtable = ID2D1Layer_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Layer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd9069b_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Layer_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSize: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1LinearGradientBrush(::windows::core::IUnknown);
impl ID2D1LinearGradientBrush {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetStartPoint(&self, startpoint: Common::D2D_POINT_2F) {
        (::windows::core::Vtable::vtable(self).SetStartPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(startpoint))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetEndPoint(&self, endpoint: Common::D2D_POINT_2F) {
        (::windows::core::Vtable::vtable(self).SetEndPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(endpoint))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetStartPoint(&self) -> Common::D2D_POINT_2F {
        let mut result__: Common::D2D_POINT_2F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetStartPoint)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEndPoint(&self) -> Common::D2D_POINT_2F {
        let mut result__: Common::D2D_POINT_2F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetEndPoint)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetGradientStopCollection(&self) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGradientStopCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1GradientStopCollection as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
::windows::core::interface_hierarchy!(ID2D1LinearGradientBrush, ::windows::core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ::core::clone::Clone for ID2D1LinearGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1LinearGradientBrush {}
unsafe impl ::core::marker::Sync for ID2D1LinearGradientBrush {}
unsafe impl ::windows::core::Vtable for ID2D1LinearGradientBrush {
    type Vtable = ID2D1LinearGradientBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1LinearGradientBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906ab_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1LinearGradientBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetStartPoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetEndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetEndPoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetStartPoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEndPoint: usize,
    pub GetGradientStopCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstopcollection: *mut *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1LookupTable3D(::windows::core::IUnknown);
impl ID2D1LookupTable3D {}
::windows::core::interface_hierarchy!(ID2D1LookupTable3D, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1LookupTable3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1LookupTable3D {}
unsafe impl ::core::marker::Sync for ID2D1LookupTable3D {}
unsafe impl ::windows::core::Vtable for ID2D1LookupTable3D {
    type Vtable = ID2D1LookupTable3D_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1LookupTable3D {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53dd9855_a3b0_4d5b_82e1_26e25c5e5797);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1LookupTable3D_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Mesh(::windows::core::IUnknown);
impl ID2D1Mesh {
    pub unsafe fn Open(&self) -> ::windows::core::Result<ID2D1TessellationSink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Open)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Mesh, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1Mesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Mesh {}
unsafe impl ::core::marker::Sync for ID2D1Mesh {}
unsafe impl ::windows::core::Vtable for ID2D1Mesh {
    type Vtable = ID2D1Mesh_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Mesh {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906c2_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Mesh_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tessellationsink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Multithread(::windows::core::IUnknown);
impl ID2D1Multithread {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMultithreadProtected(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).GetMultithreadProtected)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Enter(&self) {
        (::windows::core::Vtable::vtable(self).Enter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Leave(&self) {
        (::windows::core::Vtable::vtable(self).Leave)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1Multithread, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1Multithread {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Multithread {}
unsafe impl ::core::marker::Sync for ID2D1Multithread {}
unsafe impl ::windows::core::Vtable for ID2D1Multithread {
    type Vtable = ID2D1Multithread_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Multithread {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31e6e7bc_e0ff_4d46_8c64_a0a8c41c15d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Multithread_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMultithreadProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMultithreadProtected: usize,
    pub Enter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Leave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1OffsetTransform(::windows::core::IUnknown);
impl ID2D1OffsetTransform {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOffset(&self, offset: super::super::Foundation::POINT) {
        (::windows::core::Vtable::vtable(self).SetOffset)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(offset))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOffset(&self) -> super::super::Foundation::POINT {
        let mut result__: super::super::Foundation::POINT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetOffset)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
::windows::core::interface_hierarchy!(ID2D1OffsetTransform, ::windows::core::IUnknown, ID2D1TransformNode);
impl ::core::clone::Clone for ID2D1OffsetTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1OffsetTransform {}
unsafe impl ::core::marker::Sync for ID2D1OffsetTransform {}
unsafe impl ::windows::core::Vtable for ID2D1OffsetTransform {
    type Vtable = ID2D1OffsetTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1OffsetTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fe6adea_7643_4f53_bd14_a0ce63f24042);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1OffsetTransform_Vtbl {
    pub base__: ID2D1TransformNode_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: super::super::Foundation::POINT),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOffset: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::POINT),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOffset: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1PathGeometry(::windows::core::IUnknown);
impl ID2D1PathGeometry {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Open(&self) -> ::windows::core::Result<ID2D1GeometrySink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Open)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Stream<P0>(&self, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).Stream)(::windows::core::Vtable::as_raw(self), geometrysink.into().abi()).ok()
    }
    pub unsafe fn GetSegmentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSegmentCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFigureCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFigureCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1PathGeometry, ::windows::core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ::core::clone::Clone for ID2D1PathGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1PathGeometry {}
unsafe impl ::core::marker::Sync for ID2D1PathGeometry {}
unsafe impl ::windows::core::Vtable for ID2D1PathGeometry {
    type Vtable = ID2D1PathGeometry_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1PathGeometry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906a5_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1PathGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometrysink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Open: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometrysink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Stream: usize,
    pub GetSegmentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetFigureCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1PathGeometry1(::windows::core::IUnknown);
impl ID2D1PathGeometry1 {
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputePointAndSegmentAtLength(&self, length: f32, startsegment: u32, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ComputePointAndSegmentAtLength)(::windows::core::Vtable::as_raw(self), length, startsegment, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, pointdescription).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1PathGeometry1, ::windows::core::IUnknown, ID2D1Resource, ID2D1Geometry, ID2D1PathGeometry);
impl ::core::clone::Clone for ID2D1PathGeometry1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1PathGeometry1 {}
unsafe impl ::core::marker::Sync for ID2D1PathGeometry1 {}
unsafe impl ::windows::core::Vtable for ID2D1PathGeometry1 {
    type Vtable = ID2D1PathGeometry1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1PathGeometry1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62baa2d2_ab54_41b7_b872_787e0106a421);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1PathGeometry1_Vtbl {
    pub base__: ID2D1PathGeometry_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub ComputePointAndSegmentAtLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: f32, startsegment: u32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    ComputePointAndSegmentAtLength: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1PrintControl(::windows::core::IUnknown);
impl ID2D1PrintControl {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
    pub unsafe fn AddPage<P0, P1>(&self, commandlist: P0, pagesize: Common::D2D_SIZE_F, pageprintticketstream: P1, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1CommandList>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).AddPage)(::windows::core::Vtable::as_raw(self), commandlist.into().abi(), ::core::mem::transmute(pagesize), pageprintticketstream.into().abi(), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1PrintControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1PrintControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1PrintControl {}
unsafe impl ::core::marker::Sync for ID2D1PrintControl {}
unsafe impl ::windows::core::Vtable for ID2D1PrintControl {
    type Vtable = ID2D1PrintControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1PrintControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c1d867d_c290_41c8_ae7e_34a98702e9a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1PrintControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
    pub AddPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandlist: *mut ::core::ffi::c_void, pagesize: Common::D2D_SIZE_F, pageprintticketstream: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com")))]
    AddPage: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Properties(::windows::core::IUnknown);
impl ID2D1Properties {
    pub unsafe fn GetPropertyCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetPropertyCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPropertyName(&self, index: u32, name: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPropertyName)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute(name.as_ptr()), name.len() as _).ok()
    }
    pub unsafe fn GetPropertyNameLength(&self, index: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).GetPropertyNameLength)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetType(&self, index: u32) -> D2D1_PROPERTY_TYPE {
        (::windows::core::Vtable::vtable(self).GetType)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetPropertyIndex<P0>(&self, name: P0) -> u32
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetPropertyIndex)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn SetValueByName<P0>(&self, name: P0, r#type: D2D1_PROPERTY_TYPE, data: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetValueByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, ::core::mem::transmute(data.as_ptr()), data.len() as _).ok()
    }
    pub unsafe fn SetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), index, r#type, ::core::mem::transmute(data.as_ptr()), data.len() as _).ok()
    }
    pub unsafe fn GetValueByName<P0>(&self, name: P0, r#type: D2D1_PROPERTY_TYPE, data: &mut [u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetValueByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, ::core::mem::transmute(data.as_ptr()), data.len() as _).ok()
    }
    pub unsafe fn GetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetValue)(::windows::core::Vtable::as_raw(self), index, r#type, ::core::mem::transmute(data.as_ptr()), data.len() as _).ok()
    }
    pub unsafe fn GetValueSize(&self, index: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).GetValueSize)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetSubProperties(&self, index: u32) -> ::windows::core::Result<ID2D1Properties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSubProperties)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Properties, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1Properties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Properties {}
unsafe impl ::core::marker::Sync for ID2D1Properties {}
unsafe impl ::windows::core::Vtable for ID2D1Properties {
    type Vtable = ID2D1Properties_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Properties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x483473d7_cd46_4f9d_9d3a_3112aa80159d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Properties_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetPropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, name: ::windows::core::PWSTR, namecount: u32) -> ::windows::core::HRESULT,
    pub GetPropertyNameLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> u32,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> D2D1_PROPERTY_TYPE,
    pub GetPropertyIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> u32,
    pub SetValueByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows::core::HRESULT,
    pub GetValueByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows::core::HRESULT,
    pub GetValueSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> u32,
    pub GetSubProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, subproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1RadialGradientBrush(::windows::core::IUnknown);
impl ID2D1RadialGradientBrush {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetCenter(&self, center: Common::D2D_POINT_2F) {
        (::windows::core::Vtable::vtable(self).SetCenter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(center))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetGradientOriginOffset(&self, gradientoriginoffset: Common::D2D_POINT_2F) {
        (::windows::core::Vtable::vtable(self).SetGradientOriginOffset)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientoriginoffset))
    }
    pub unsafe fn SetRadiusX(&self, radiusx: f32) {
        (::windows::core::Vtable::vtable(self).SetRadiusX)(::windows::core::Vtable::as_raw(self), radiusx)
    }
    pub unsafe fn SetRadiusY(&self, radiusy: f32) {
        (::windows::core::Vtable::vtable(self).SetRadiusY)(::windows::core::Vtable::as_raw(self), radiusy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetCenter(&self) -> Common::D2D_POINT_2F {
        let mut result__: Common::D2D_POINT_2F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetCenter)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientOriginOffset(&self) -> Common::D2D_POINT_2F {
        let mut result__: Common::D2D_POINT_2F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetGradientOriginOffset)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetRadiusX(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).GetRadiusX)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetRadiusY(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).GetRadiusY)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetGradientStopCollection(&self) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGradientStopCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1GradientStopCollection as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
::windows::core::interface_hierarchy!(ID2D1RadialGradientBrush, ::windows::core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ::core::clone::Clone for ID2D1RadialGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1RadialGradientBrush {}
unsafe impl ::core::marker::Sync for ID2D1RadialGradientBrush {}
unsafe impl ::windows::core::Vtable for ID2D1RadialGradientBrush {
    type Vtable = ID2D1RadialGradientBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1RadialGradientBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906ac_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1RadialGradientBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetCenter: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetGradientOriginOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientoriginoffset: Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetGradientOriginOffset: usize,
    pub SetRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiusx: f32),
    pub SetRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiusy: f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetCenter: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientOriginOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientOriginOffset: usize,
    pub GetRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub GetRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub GetGradientStopCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstopcollection: *mut *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1RectangleGeometry(::windows::core::IUnknown);
impl ID2D1RectangleGeometry {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRect(&self) -> Common::D2D_RECT_F {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
}
::windows::core::interface_hierarchy!(ID2D1RectangleGeometry, ::windows::core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ::core::clone::Clone for ID2D1RectangleGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1RectangleGeometry {}
unsafe impl ::core::marker::Sync for ID2D1RectangleGeometry {}
unsafe impl ::windows::core::Vtable for ID2D1RectangleGeometry {
    type Vtable = ID2D1RectangleGeometry_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1RectangleGeometry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906a2_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1RectangleGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *mut Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetRect: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1RenderInfo(::windows::core::IUnknown);
impl ID2D1RenderInfo {
    pub unsafe fn SetInputDescription(&self, inputindex: u32, inputdescription: D2D1_INPUT_DESCRIPTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInputDescription)(::windows::core::Vtable::as_raw(self), inputindex, ::core::mem::transmute(inputdescription)).ok()
    }
    pub unsafe fn SetOutputBuffer(&self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOutputBuffer)(::windows::core::Vtable::as_raw(self), bufferprecision, channeldepth).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCached<P0>(&self, iscached: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetCached)(::windows::core::Vtable::as_raw(self), iscached.into())
    }
    pub unsafe fn SetInstructionCountHint(&self, instructioncount: u32) {
        (::windows::core::Vtable::vtable(self).SetInstructionCountHint)(::windows::core::Vtable::as_raw(self), instructioncount)
    }
}
::windows::core::interface_hierarchy!(ID2D1RenderInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1RenderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1RenderInfo {}
unsafe impl ::core::marker::Sync for ID2D1RenderInfo {}
unsafe impl ::windows::core::Vtable for ID2D1RenderInfo {
    type Vtable = ID2D1RenderInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1RenderInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x519ae1bd_d19a_420d_b849_364f594776b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1RenderInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetInputDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, inputdescription: D2D1_INPUT_DESCRIPTION) -> ::windows::core::HRESULT,
    pub SetOutputBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iscached: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCached: usize,
    pub SetInstructionCountHint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instructioncount: u32),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1RenderTarget(::windows::core::IUnknown);
impl ID2D1RenderTarget {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(srcdata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>) -> ::windows::core::Result<ID2D1Bitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBitmapFromWicBitmap)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CreateSharedBitmap)(::windows::core::Vtable::as_raw(self), riid, data, ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bitmap)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBitmapBrush)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1SolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateGradientStopCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _, colorinterpolationgamma, extendmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1LinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), lineargradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1RadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), radialgradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: ::core::option::Option<*const Common::D2D_SIZE_F>, desiredpixelsize: ::core::option::Option<*const Common::D2D_SIZE_U>, desiredformat: ::core::option::Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows::core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCompatibleRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desiredsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredpixelsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredformat.unwrap_or(::std::ptr::null())), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: ::core::option::Option<*const Common::D2D_SIZE_F>) -> ::windows::core::Result<ID2D1Layer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMesh(&self) -> ::windows::core::Result<ID2D1Mesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateMesh)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).FillRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).FillEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), content, ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).DrawText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).DrawTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).GetAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).GetTextAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (::windows::core::Vtable::vtable(self).SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).GetTags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (::windows::core::Vtable::vtable(self).PopLayer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Flush)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).SaveDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).RestoreDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (::windows::core::Vtable::vtable(self).PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: ::core::option::Option<*const Common::D2D1_COLOR_F>) {
        (::windows::core::Vtable::vtable(self).Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clearcolor.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (::windows::core::Vtable::vtable(self).BeginDraw)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EndDraw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (::windows::core::Vtable::vtable(self).SetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetMaximumBitmapSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsSupported)(::windows::core::Vtable::as_raw(self), rendertargetproperties)
    }
}
::windows::core::interface_hierarchy!(ID2D1RenderTarget, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1RenderTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1RenderTarget {}
unsafe impl ::core::marker::Sync for ID2D1RenderTarget {}
unsafe impl ::windows::core::Vtable for ID2D1RenderTarget {
    type Vtable = ID2D1RenderTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1RenderTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd90694_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1RenderTarget_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: Common::D2D_SIZE_U, srcdata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmap: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateBitmapFromWicBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wicbitmapsource: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging")))]
    CreateBitmapFromWicBitmap: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSharedBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSharedBitmap: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateBitmapBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateBitmapBrush: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateSolidColorBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F, brushproperties: *const D2D1_BRUSH_PROPERTIES, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateSolidColorBrush: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGradientStopCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstops: *const D2D1_GRADIENT_STOP, gradientstopscount: u32, colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE, gradientstopcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGradientStopCollection: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateLinearGradientBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: *mut ::core::ffi::c_void, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateLinearGradientBrush: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateRadialGradientBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: *mut ::core::ffi::c_void, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateRadialGradientBrush: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateCompatibleRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredsize: *const Common::D2D_SIZE_F, desiredpixelsize: *const Common::D2D_SIZE_U, desiredformat: *const Common::D2D1_PIXEL_FORMAT, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS, bitmaprendertarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateCompatibleRenderTarget: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateLayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *const Common::D2D_SIZE_F, layer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateLayer: usize,
    pub CreateMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mesh: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawLine: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawRoundedRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawRoundedRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillRoundedRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillRoundedRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawEllipse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawEllipse: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillEllipse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillEllipse: usize,
    pub DrawGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void),
    pub FillGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, opacitybrush: *mut ::core::ffi::c_void),
    pub FillMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mesh: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillOpacityMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymask: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillOpacityMask: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawBitmap: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, string: ::windows::core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: *mut ::core::ffi::c_void, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawText: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawTextLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: Common::D2D_POINT_2F, textlayout: *mut ::core::ffi::c_void, defaultfillbrush: *mut ::core::ffi::c_void, options: D2D1_DRAW_TEXT_OPTIONS),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawTextLayout: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawGlyphRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: *mut ::core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetTransform: usize,
    pub SetAntialiasMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, antialiasmode: D2D1_ANTIALIAS_MODE),
    pub GetAntialiasMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_ANTIALIAS_MODE,
    pub SetTextAntialiasMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE),
    pub GetTextAntialiasMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_TEXT_ANTIALIAS_MODE,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textrenderingparams: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    SetTextRenderingParams: usize,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub GetTextRenderingParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textrenderingparams: *mut *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    GetTextRenderingParams: usize,
    pub SetTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag1: u64, tag2: u64),
    pub GetTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64),
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub PushLayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: *mut ::core::ffi::c_void),
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    PushLayer: usize,
    pub PopLayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> ::windows::core::HRESULT,
    pub SaveDrawingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingstateblock: *mut ::core::ffi::c_void),
    pub RestoreDrawingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingstateblock: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub PushAxisAlignedClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    PushAxisAlignedClip: usize,
    pub PopAxisAlignedClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clearcolor: *const Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Clear: usize,
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_PIXEL_FORMAT),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    GetPixelFormat: usize,
    pub SetDpi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dpix: f32, dpiy: f32),
    pub GetDpi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPixelSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_U),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPixelSize: usize,
    pub GetMaximumBitmapSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    IsSupported: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Resource(::windows::core::IUnknown);
impl ID2D1Resource {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
::windows::core::interface_hierarchy!(ID2D1Resource, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1Resource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Resource {}
unsafe impl ::core::marker::Sync for ID2D1Resource {}
unsafe impl ::windows::core::Vtable for ID2D1Resource {
    type Vtable = ID2D1Resource_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Resource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd90691_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Resource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1ResourceTexture(::windows::core::IUnknown);
impl ID2D1ResourceTexture {
    pub unsafe fn Update(&self, minimumextents: ::core::option::Option<*const u32>, maximimumextents: ::core::option::Option<*const u32>, strides: ::core::option::Option<*const u32>, dimensions: u32, data: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Update)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(minimumextents.unwrap_or(::std::ptr::null())), ::core::mem::transmute(maximimumextents.unwrap_or(::std::ptr::null())), ::core::mem::transmute(strides.unwrap_or(::std::ptr::null())), dimensions, ::core::mem::transmute(data.as_ptr()), data.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1ResourceTexture, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1ResourceTexture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1ResourceTexture {}
unsafe impl ::core::marker::Sync for ID2D1ResourceTexture {}
unsafe impl ::windows::core::Vtable for ID2D1ResourceTexture {
    type Vtable = ID2D1ResourceTexture_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1ResourceTexture {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x688d15c3_02b0_438d_b13a_d1b44c32c39a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ResourceTexture_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minimumextents: *const u32, maximimumextents: *const u32, strides: *const u32, dimensions: u32, data: *const u8, datacount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1RoundedRectangleGeometry(::windows::core::IUnknown);
impl ID2D1RoundedRectangleGeometry {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRoundedRect(&self, roundedrect: *mut D2D1_ROUNDED_RECT) {
        (::windows::core::Vtable::vtable(self).GetRoundedRect)(::windows::core::Vtable::as_raw(self), roundedrect)
    }
}
::windows::core::interface_hierarchy!(ID2D1RoundedRectangleGeometry, ::windows::core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ::core::clone::Clone for ID2D1RoundedRectangleGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1RoundedRectangleGeometry {}
unsafe impl ::core::marker::Sync for ID2D1RoundedRectangleGeometry {}
unsafe impl ::windows::core::Vtable for ID2D1RoundedRectangleGeometry {
    type Vtable = ID2D1RoundedRectangleGeometry_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1RoundedRectangleGeometry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906a3_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1RoundedRectangleGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetRoundedRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roundedrect: *mut D2D1_ROUNDED_RECT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetRoundedRect: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1SolidColorBrush(::windows::core::IUnknown);
impl ID2D1SolidColorBrush {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetColor(&self, color: *const Common::D2D1_COLOR_F) {
        (::windows::core::Vtable::vtable(self).SetColor)(::windows::core::Vtable::as_raw(self), color)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetColor(&self) -> Common::D2D1_COLOR_F {
        let mut result__: Common::D2D1_COLOR_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetColor)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
::windows::core::interface_hierarchy!(ID2D1SolidColorBrush, ::windows::core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ::core::clone::Clone for ID2D1SolidColorBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1SolidColorBrush {}
unsafe impl ::core::marker::Sync for ID2D1SolidColorBrush {}
unsafe impl ::windows::core::Vtable for ID2D1SolidColorBrush {
    type Vtable = ID2D1SolidColorBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1SolidColorBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906a9_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SolidColorBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetColor: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1SourceTransform(::windows::core::IUnknown);
impl ID2D1SourceTransform {
    pub unsafe fn SetRenderInfo<P0>(&self, renderinfo: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1RenderInfo>>,
    {
        (::windows::core::Vtable::vtable(self).SetRenderInfo)(::windows::core::Vtable::as_raw(self), renderinfo.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Draw<P0>(&self, target: P0, drawrect: *const super::super::Foundation::RECT, targetorigin: Common::D2D_POINT_2U) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap1>>,
    {
        (::windows::core::Vtable::vtable(self).Draw)(::windows::core::Vtable::as_raw(self), target.into().abi(), drawrect, ::core::mem::transmute(targetorigin)).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1SourceTransform, ::windows::core::IUnknown, ID2D1TransformNode, ID2D1Transform);
impl ::core::clone::Clone for ID2D1SourceTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1SourceTransform {}
unsafe impl ::core::marker::Sync for ID2D1SourceTransform {}
unsafe impl ::windows::core::Vtable for ID2D1SourceTransform {
    type Vtable = ID2D1SourceTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1SourceTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb1800dd_0c34_4cf9_be90_31cc0a5653e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SourceTransform_Vtbl {
    pub base__: ID2D1Transform_Vtbl,
    pub SetRenderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, drawrect: *const super::super::Foundation::RECT, targetorigin: Common::D2D_POINT_2U) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    Draw: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1SpriteBatch(::windows::core::IUnknown);
impl ID2D1SpriteBatch {
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn AddSprites(&self, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: ::core::option::Option<*const Common::D2D_RECT_U>, colors: ::core::option::Option<*const Common::D2D1_COLOR_F>, transforms: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddSprites)(::windows::core::Vtable::as_raw(self), spritecount, destinationrectangles, ::core::mem::transmute(sourcerectangles.unwrap_or(::std::ptr::null())), ::core::mem::transmute(colors.unwrap_or(::std::ptr::null())), ::core::mem::transmute(transforms.unwrap_or(::std::ptr::null())), destinationrectanglesstride, sourcerectanglesstride, colorsstride, transformsstride).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn SetSprites(&self, startindex: u32, spritecount: u32, destinationrectangles: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangles: ::core::option::Option<*const Common::D2D_RECT_U>, colors: ::core::option::Option<*const Common::D2D1_COLOR_F>, transforms: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSprites)(::windows::core::Vtable::as_raw(self), startindex, spritecount, ::core::mem::transmute(destinationrectangles.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangles.unwrap_or(::std::ptr::null())), ::core::mem::transmute(colors.unwrap_or(::std::ptr::null())), ::core::mem::transmute(transforms.unwrap_or(::std::ptr::null())), destinationrectanglesstride, sourcerectanglesstride, colorsstride, transformsstride).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetSprites(&self, startindex: u32, spritecount: u32, destinationrectangles: ::core::option::Option<*mut Common::D2D_RECT_F>, sourcerectangles: ::core::option::Option<*mut Common::D2D_RECT_U>, colors: ::core::option::Option<*mut Common::D2D1_COLOR_F>, transforms: ::core::option::Option<*mut super::super::super::Foundation::Numerics::Matrix3x2>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSprites)(::windows::core::Vtable::as_raw(self), startindex, spritecount, ::core::mem::transmute(destinationrectangles.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(sourcerectangles.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(colors.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(transforms.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetSpriteCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetSpriteCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Clear(&self) {
        (::windows::core::Vtable::vtable(self).Clear)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1SpriteBatch, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1SpriteBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1SpriteBatch {}
unsafe impl ::core::marker::Sync for ID2D1SpriteBatch {}
unsafe impl ::windows::core::Vtable for ID2D1SpriteBatch {
    type Vtable = ID2D1SpriteBatch_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1SpriteBatch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dc583bf_3a10_438a_8722_e9765224f1f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SpriteBatch_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub AddSprites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    AddSprites: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub SetSprites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    SetSprites: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetSprites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *mut Common::D2D_RECT_F, sourcerectangles: *mut Common::D2D_RECT_U, colors: *mut Common::D2D1_COLOR_F, transforms: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetSprites: usize,
    pub GetSpriteCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1StrokeStyle(::windows::core::IUnknown);
impl ID2D1StrokeStyle {
    pub unsafe fn GetStartCap(&self) -> D2D1_CAP_STYLE {
        (::windows::core::Vtable::vtable(self).GetStartCap)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetEndCap(&self) -> D2D1_CAP_STYLE {
        (::windows::core::Vtable::vtable(self).GetEndCap)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDashCap(&self) -> D2D1_CAP_STYLE {
        (::windows::core::Vtable::vtable(self).GetDashCap)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMiterLimit(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).GetMiterLimit)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLineJoin(&self) -> D2D1_LINE_JOIN {
        (::windows::core::Vtable::vtable(self).GetLineJoin)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDashOffset(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).GetDashOffset)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDashStyle(&self) -> D2D1_DASH_STYLE {
        (::windows::core::Vtable::vtable(self).GetDashStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDashesCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetDashesCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDashes(&self, dashes: &mut [f32]) {
        (::windows::core::Vtable::vtable(self).GetDashes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(dashes.as_ptr()), dashes.len() as _)
    }
}
::windows::core::interface_hierarchy!(ID2D1StrokeStyle, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1StrokeStyle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1StrokeStyle {}
unsafe impl ::core::marker::Sync for ID2D1StrokeStyle {}
unsafe impl ::windows::core::Vtable for ID2D1StrokeStyle {
    type Vtable = ID2D1StrokeStyle_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1StrokeStyle {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd9069d_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1StrokeStyle_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetStartCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE,
    pub GetEndCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE,
    pub GetDashCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE,
    pub GetMiterLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub GetLineJoin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_LINE_JOIN,
    pub GetDashOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub GetDashStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_DASH_STYLE,
    pub GetDashesCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetDashes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dashes: *mut f32, dashescount: u32),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1StrokeStyle1(::windows::core::IUnknown);
impl ID2D1StrokeStyle1 {
    pub unsafe fn GetStrokeTransformType(&self) -> D2D1_STROKE_TRANSFORM_TYPE {
        (::windows::core::Vtable::vtable(self).GetStrokeTransformType)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1StrokeStyle1, ::windows::core::IUnknown, ID2D1Resource, ID2D1StrokeStyle);
impl ::core::clone::Clone for ID2D1StrokeStyle1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1StrokeStyle1 {}
unsafe impl ::core::marker::Sync for ID2D1StrokeStyle1 {}
unsafe impl ::windows::core::Vtable for ID2D1StrokeStyle1 {
    type Vtable = ID2D1StrokeStyle1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1StrokeStyle1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10a72a66_e91c_43f4_993f_ddf4b82b0b4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1StrokeStyle1_Vtbl {
    pub base__: ID2D1StrokeStyle_Vtbl,
    pub GetStrokeTransformType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_STROKE_TRANSFORM_TYPE,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1SvgAttribute(::windows::core::IUnknown);
impl ID2D1SvgAttribute {
    pub unsafe fn GetElement(&self) -> ::windows::core::Result<ID2D1SvgElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1SvgElement as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ID2D1SvgAttribute> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1SvgAttribute, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1SvgAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1SvgAttribute {}
unsafe impl ::core::marker::Sync for ID2D1SvgAttribute {}
unsafe impl ::windows::core::Vtable for ID2D1SvgAttribute {
    type Vtable = ID2D1SvgAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1SvgAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9cdb0dd_f8c9_4e70_b7c2_301c80292c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgAttribute_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void),
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attribute: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1SvgDocument(::windows::core::IUnknown);
impl ID2D1SvgDocument {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetViewportSize(&self, viewportsize: Common::D2D_SIZE_F) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetViewportSize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewportsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetViewportSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetViewportSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetRoot<P0>(&self, root: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SvgElement>>,
    {
        (::windows::core::Vtable::vtable(self).SetRoot)(::windows::core::Vtable::as_raw(self), root.into().abi()).ok()
    }
    pub unsafe fn GetRoot(&self) -> ::windows::core::Result<ID2D1SvgElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRoot)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1SvgElement as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn FindElementById<P0>(&self, id: P0) -> ::windows::core::Result<ID2D1SvgElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindElementById)(::windows::core::Vtable::as_raw(self), id.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<P0, P1>(&self, outputxmlstream: P0, subtree: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1SvgElement>>,
    {
        (::windows::core::Vtable::vtable(self).Serialize)(::windows::core::Vtable::as_raw(self), outputxmlstream.into().abi(), subtree.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Deserialize<P0>(&self, inputxmlstream: P0) -> ::windows::core::Result<ID2D1SvgElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Deserialize)(::windows::core::Vtable::as_raw(self), inputxmlstream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreatePaint<P0>(&self, painttype: D2D1_SVG_PAINT_TYPE, color: ::core::option::Option<*const Common::D2D1_COLOR_F>, id: P0) -> ::windows::core::Result<ID2D1SvgPaint>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePaint)(::windows::core::Vtable::as_raw(self), painttype, ::core::mem::transmute(color.unwrap_or(::std::ptr::null())), id.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeDashArray(&self, dashes: ::core::option::Option<&[D2D1_SVG_LENGTH]>) -> ::windows::core::Result<ID2D1SvgStrokeDashArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStrokeDashArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreatePointCollection(&self, points: ::core::option::Option<&[Common::D2D_POINT_2F]>) -> ::windows::core::Result<ID2D1SvgPointCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePointCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(points.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), points.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathData(&self, segmentdata: ::core::option::Option<&[f32]>, commands: ::core::option::Option<&[D2D1_SVG_PATH_COMMAND]>) -> ::windows::core::Result<ID2D1SvgPathData> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePathData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(segmentdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), segmentdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(commands.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), commands.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1SvgDocument, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1SvgDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1SvgDocument {}
unsafe impl ::core::marker::Sync for ID2D1SvgDocument {}
unsafe impl ::windows::core::Vtable for ID2D1SvgDocument {
    type Vtable = ID2D1SvgDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1SvgDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86b88e4d_afa4_4d7b_88e4_68a51c4a0aec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgDocument_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetViewportSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewportsize: Common::D2D_SIZE_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetViewportSize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetViewportSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetViewportSize: usize,
    pub SetRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, root: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, root: *mut *mut ::core::ffi::c_void),
    pub FindElementById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::PCWSTR, svgelement: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputxmlstream: *mut ::core::ffi::c_void, subtree: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputxmlstream: *mut ::core::ffi::c_void, subtree: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deserialize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreatePaint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE, color: *const Common::D2D1_COLOR_F, id: ::windows::core::PCWSTR, paint: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreatePaint: usize,
    pub CreateStrokeDashArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, strokedasharray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreatePointCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, points: *const Common::D2D_POINT_2F, pointscount: u32, pointcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreatePointCollection: usize,
    pub CreatePathData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentdata: *const f32, segmentdatacount: u32, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, pathdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1SvgElement(::windows::core::IUnknown);
impl ID2D1SvgElement {
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<ID2D1SvgDocument> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocument)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1SvgDocument as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetTagName(&self, name: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetTagName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(name.as_ptr()), name.len() as _).ok()
    }
    pub unsafe fn GetTagNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetTagNameLength)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTextContent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsTextContent)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<ID2D1SvgElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1SvgElement as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasChildren(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).HasChildren)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFirstChild(&self) -> ::windows::core::Result<ID2D1SvgElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFirstChild)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1SvgElement as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetLastChild(&self) -> ::windows::core::Result<ID2D1SvgElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLastChild)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1SvgElement as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPreviousChild<P0>(&self, referencechild: P0) -> ::windows::core::Result<ID2D1SvgElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SvgElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPreviousChild)(::windows::core::Vtable::as_raw(self), referencechild.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNextChild<P0>(&self, referencechild: P0) -> ::windows::core::Result<ID2D1SvgElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SvgElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNextChild)(::windows::core::Vtable::as_raw(self), referencechild.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertChildBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SvgElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1SvgElement>>,
    {
        (::windows::core::Vtable::vtable(self).InsertChildBefore)(::windows::core::Vtable::as_raw(self), newchild.into().abi(), referencechild.into().abi()).ok()
    }
    pub unsafe fn AppendChild<P0>(&self, newchild: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SvgElement>>,
    {
        (::windows::core::Vtable::vtable(self).AppendChild)(::windows::core::Vtable::as_raw(self), newchild.into().abi()).ok()
    }
    pub unsafe fn ReplaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SvgElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1SvgElement>>,
    {
        (::windows::core::Vtable::vtable(self).ReplaceChild)(::windows::core::Vtable::as_raw(self), newchild.into().abi(), oldchild.into().abi()).ok()
    }
    pub unsafe fn RemoveChild<P0>(&self, oldchild: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SvgElement>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveChild)(::windows::core::Vtable::as_raw(self), oldchild.into().abi()).ok()
    }
    pub unsafe fn CreateChild<P0>(&self, tagname: P0) -> ::windows::core::Result<ID2D1SvgElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateChild)(::windows::core::Vtable::as_raw(self), tagname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAttributeSpecified<P0>(&self, name: P0, inherited: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).IsAttributeSpecified)(::windows::core::Vtable::as_raw(self), name.into().abi(), ::core::mem::transmute(inherited.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GetSpecifiedAttributeCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetSpecifiedAttributeCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecifiedAttributeName(&self, index: u32, name: &mut [u16], inherited: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSpecifiedAttributeName)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute(name.as_ptr()), name.len() as _, ::core::mem::transmute(inherited.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpecifiedAttributeNameLength(&self, index: u32, namelength: *mut u32, inherited: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSpecifiedAttributeNameLength)(::windows::core::Vtable::as_raw(self), index, namelength, ::core::mem::transmute(inherited.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn RemoveAttribute<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveAttribute)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn SetTextValue(&self, name: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTextValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(name.as_ptr()), name.len() as _).ok()
    }
    pub unsafe fn GetTextValue(&self, name: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetTextValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(name.as_ptr()), name.len() as _).ok()
    }
    pub unsafe fn GetTextValueLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetTextValueLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetAttributeValue<P0, P1>(&self, name: P0, value: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1SvgAttribute>>,
    {
        (::windows::core::Vtable::vtable(self).SetAttributeValue)(::windows::core::Vtable::as_raw(self), name.into().abi(), value.into().abi()).ok()
    }
    pub unsafe fn SetAttributeValue2<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAttributeValue2)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, value, valuesizeinbytes).ok()
    }
    pub unsafe fn SetAttributeValue3<P0, P1>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAttributeValue3)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, value.into().abi()).ok()
    }
    pub unsafe fn GetAttributeValue<P0, T>(&self, name: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAttributeValue)(::windows::core::Vtable::as_raw(self), name.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAttributeValue2<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetAttributeValue2)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, value, valuesizeinbytes).ok()
    }
    pub unsafe fn GetAttributeValue3<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: &mut [u16]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetAttributeValue3)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, ::core::mem::transmute(value.as_ptr()), value.len() as _).ok()
    }
    pub unsafe fn GetAttributeValueLength<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAttributeValueLength)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1SvgElement, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1SvgElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1SvgElement {}
unsafe impl ::core::marker::Sync for ID2D1SvgElement {}
unsafe impl ::windows::core::Vtable for ID2D1SvgElement {
    type Vtable = ID2D1SvgElement_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1SvgElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac7b67a6_183e_49c1_a823_0ebe40b0db29);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgElement_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void),
    pub GetTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PWSTR, namecount: u32) -> ::windows::core::HRESULT,
    pub GetTagNameLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTextContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTextContent: usize,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub HasChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasChildren: usize,
    pub GetFirstChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, child: *mut *mut ::core::ffi::c_void),
    pub GetLastChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, child: *mut *mut ::core::ffi::c_void),
    pub GetPreviousChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, previouschild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNextChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, nextchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertChildBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppendChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReplaceChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, oldchild: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldchild: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::windows::core::PCWSTR, newchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAttributeSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, inherited: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAttributeSpecified: usize,
    pub GetSpecifiedAttributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSpecifiedAttributeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, name: ::windows::core::PWSTR, namecount: u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSpecifiedAttributeName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSpecifiedAttributeNameLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, namelength: *mut u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSpecifiedAttributeNameLength: usize,
    pub RemoveAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetTextValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, namecount: u32) -> ::windows::core::HRESULT,
    pub GetTextValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PWSTR, namecount: u32) -> ::windows::core::HRESULT,
    pub GetTextValueLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub SetAttributeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttributeValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows::core::HRESULT,
    pub SetAttributeValue3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetAttributeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAttributeValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows::core::HRESULT,
    pub GetAttributeValue3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: ::windows::core::PWSTR, valuecount: u32) -> ::windows::core::HRESULT,
    pub GetAttributeValueLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, valuelength: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1SvgGlyphStyle(::windows::core::IUnknown);
impl ID2D1SvgGlyphStyle {
    pub unsafe fn SetFill<P0>(&self, brush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).SetFill)(::windows::core::Vtable::as_raw(self), brush.into().abi()).ok()
    }
    pub unsafe fn GetFill(&self) -> ::windows::core::Result<ID2D1Brush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFill)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Brush as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetStroke<P0>(&self, brush: P0, strokewidth: f32, dashes: ::core::option::Option<&[f32]>, dashoffset: f32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).SetStroke)(::windows::core::Vtable::as_raw(self), brush.into().abi(), strokewidth, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), dashoffset).ok()
    }
    pub unsafe fn GetStrokeDashesCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetStrokeDashesCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStroke(&self, brush: ::core::option::Option<*mut ::core::option::Option<ID2D1Brush>>, strokewidth: ::core::option::Option<*mut f32>, dashes: ::core::option::Option<&mut [f32]>, dashoffset: ::core::option::Option<*mut f32>) {
        (::windows::core::Vtable::vtable(self).GetStroke)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(brush.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(strokewidth.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(dashoffset.unwrap_or(::std::ptr::null_mut())))
    }
}
::windows::core::interface_hierarchy!(ID2D1SvgGlyphStyle, ::windows::core::IUnknown, ID2D1Resource);
impl ::core::clone::Clone for ID2D1SvgGlyphStyle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1SvgGlyphStyle {}
unsafe impl ::core::marker::Sync for ID2D1SvgGlyphStyle {}
unsafe impl ::windows::core::Vtable for ID2D1SvgGlyphStyle {
    type Vtable = ID2D1SvgGlyphStyle_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1SvgGlyphStyle {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf671749_d241_4db8_8e41_dcc2e5c1a438);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgGlyphStyle_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub SetFill: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFill: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void),
    pub SetStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, strokewidth: f32, dashes: *const f32, dashescount: u32, dashoffset: f32) -> ::windows::core::HRESULT,
    pub GetStrokeDashesCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void, strokewidth: *mut f32, dashes: *mut f32, dashescount: u32, dashoffset: *mut f32),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1SvgPaint(::windows::core::IUnknown);
impl ID2D1SvgPaint {
    pub unsafe fn SetPaintType(&self, painttype: D2D1_SVG_PAINT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPaintType)(::windows::core::Vtable::as_raw(self), painttype).ok()
    }
    pub unsafe fn GetPaintType(&self) -> D2D1_SVG_PAINT_TYPE {
        (::windows::core::Vtable::vtable(self).GetPaintType)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetColor(&self, color: *const Common::D2D1_COLOR_F) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetColor)(::windows::core::Vtable::as_raw(self), color).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetColor(&self) -> Common::D2D1_COLOR_F {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetId<P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetId)(::windows::core::Vtable::as_raw(self), id.into().abi()).ok()
    }
    pub unsafe fn GetId(&self, id: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetId)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(id.as_ptr()), id.len() as _).ok()
    }
    pub unsafe fn GetIdLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetIdLength)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1SvgPaint, ::windows::core::IUnknown, ID2D1Resource, ID2D1SvgAttribute);
impl ::core::clone::Clone for ID2D1SvgPaint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1SvgPaint {}
unsafe impl ::core::marker::Sync for ID2D1SvgPaint {}
unsafe impl ::windows::core::Vtable for ID2D1SvgPaint {
    type Vtable = ID2D1SvgPaint_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1SvgPaint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd59bab0a_68a2_455b_a5dc_9eb2854e2490);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgPaint_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub SetPaintType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE) -> ::windows::core::HRESULT,
    pub GetPaintType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D2D1_SVG_PAINT_TYPE,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *mut Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetColor: usize,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::PWSTR, idcount: u32) -> ::windows::core::HRESULT,
    pub GetIdLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1SvgPathData(::windows::core::IUnknown);
impl ID2D1SvgPathData {
    pub unsafe fn RemoveSegmentDataAtEnd(&self, datacount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveSegmentDataAtEnd)(::windows::core::Vtable::as_raw(self), datacount).ok()
    }
    pub unsafe fn UpdateSegmentData(&self, data: &[f32], startindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateSegmentData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(data.as_ptr()), data.len() as _, startindex).ok()
    }
    pub unsafe fn GetSegmentData(&self, data: &mut [f32], startindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSegmentData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(data.as_ptr()), data.len() as _, startindex).ok()
    }
    pub unsafe fn GetSegmentDataCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetSegmentDataCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn RemoveCommandsAtEnd(&self, commandscount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveCommandsAtEnd)(::windows::core::Vtable::as_raw(self), commandscount).ok()
    }
    pub unsafe fn UpdateCommands(&self, commands: &[D2D1_SVG_PATH_COMMAND], startindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateCommands)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(commands.as_ptr()), commands.len() as _, startindex).ok()
    }
    pub unsafe fn GetCommands(&self, commands: &mut [D2D1_SVG_PATH_COMMAND], startindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCommands)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(commands.as_ptr()), commands.len() as _, startindex).ok()
    }
    pub unsafe fn GetCommandsCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetCommandsCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreatePathGeometry(&self, fillmode: Common::D2D1_FILL_MODE) -> ::windows::core::Result<ID2D1PathGeometry1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePathGeometry)(::windows::core::Vtable::as_raw(self), fillmode, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1SvgPathData, ::windows::core::IUnknown, ID2D1Resource, ID2D1SvgAttribute);
impl ::core::clone::Clone for ID2D1SvgPathData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1SvgPathData {}
unsafe impl ::core::marker::Sync for ID2D1SvgPathData {}
unsafe impl ::windows::core::Vtable for ID2D1SvgPathData {
    type Vtable = ID2D1SvgPathData_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1SvgPathData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc095e4f4_bb98_43d6_9745_4d1b84ec9888);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgPathData_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub RemoveSegmentDataAtEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datacount: u32) -> ::windows::core::HRESULT,
    pub UpdateSegmentData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *const f32, datacount: u32, startindex: u32) -> ::windows::core::HRESULT,
    pub GetSegmentData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut f32, datacount: u32, startindex: u32) -> ::windows::core::HRESULT,
    pub GetSegmentDataCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub RemoveCommandsAtEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandscount: u32) -> ::windows::core::HRESULT,
    pub UpdateCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows::core::HRESULT,
    pub GetCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commands: *mut D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows::core::HRESULT,
    pub GetCommandsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreatePathGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillmode: Common::D2D1_FILL_MODE, pathgeometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreatePathGeometry: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1SvgPointCollection(::windows::core::IUnknown);
impl ID2D1SvgPointCollection {
    pub unsafe fn RemovePointsAtEnd(&self, pointscount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemovePointsAtEnd)(::windows::core::Vtable::as_raw(self), pointscount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn UpdatePoints(&self, points: &[Common::D2D_POINT_2F], startindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdatePoints)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(points.as_ptr()), points.len() as _, startindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPoints(&self, points: &mut [Common::D2D_POINT_2F], startindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPoints)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(points.as_ptr()), points.len() as _, startindex).ok()
    }
    pub unsafe fn GetPointsCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetPointsCount)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1SvgPointCollection, ::windows::core::IUnknown, ID2D1Resource, ID2D1SvgAttribute);
impl ::core::clone::Clone for ID2D1SvgPointCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1SvgPointCollection {}
unsafe impl ::core::marker::Sync for ID2D1SvgPointCollection {}
unsafe impl ::windows::core::Vtable for ID2D1SvgPointCollection {
    type Vtable = ID2D1SvgPointCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1SvgPointCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9dbe4c0d_3572_4dd9_9825_5530813bb712);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgPointCollection_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub RemovePointsAtEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointscount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub UpdatePoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, points: *const Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    UpdatePoints: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, points: *mut Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPoints: usize,
    pub GetPointsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1SvgStrokeDashArray(::windows::core::IUnknown);
impl ID2D1SvgStrokeDashArray {
    pub unsafe fn RemoveDashesAtEnd(&self, dashescount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveDashesAtEnd)(::windows::core::Vtable::as_raw(self), dashescount).ok()
    }
    pub unsafe fn UpdateDashes(&self, dashes: &[D2D1_SVG_LENGTH], startindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateDashes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(dashes.as_ptr()), dashes.len() as _, startindex).ok()
    }
    pub unsafe fn UpdateDashes2(&self, dashes: &[f32], startindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateDashes2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(dashes.as_ptr()), dashes.len() as _, startindex).ok()
    }
    pub unsafe fn GetDashes(&self, dashes: &mut [D2D1_SVG_LENGTH], startindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDashes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(dashes.as_ptr()), dashes.len() as _, startindex).ok()
    }
    pub unsafe fn GetDashes2(&self, dashes: &mut [f32], startindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDashes2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(dashes.as_ptr()), dashes.len() as _, startindex).ok()
    }
    pub unsafe fn GetDashesCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetDashesCount)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1SvgStrokeDashArray, ::windows::core::IUnknown, ID2D1Resource, ID2D1SvgAttribute);
impl ::core::clone::Clone for ID2D1SvgStrokeDashArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1SvgStrokeDashArray {}
unsafe impl ::core::marker::Sync for ID2D1SvgStrokeDashArray {}
unsafe impl ::windows::core::Vtable for ID2D1SvgStrokeDashArray {
    type Vtable = ID2D1SvgStrokeDashArray_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1SvgStrokeDashArray {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1c0ca52_92a3_4f00_b4ce_f35691efd9d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgStrokeDashArray_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub RemoveDashesAtEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dashescount: u32) -> ::windows::core::HRESULT,
    pub UpdateDashes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT,
    pub UpdateDashes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dashes: *const f32, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT,
    pub GetDashes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dashes: *mut D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT,
    pub GetDashes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dashes: *mut f32, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT,
    pub GetDashesCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1TessellationSink(::windows::core::IUnknown);
impl ID2D1TessellationSink {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddTriangles(&self, triangles: &[D2D1_TRIANGLE]) {
        (::windows::core::Vtable::vtable(self).AddTriangles)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(triangles.as_ptr()), triangles.len() as _)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1TessellationSink, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1TessellationSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1TessellationSink {}
unsafe impl ::core::marker::Sync for ID2D1TessellationSink {}
unsafe impl ::windows::core::Vtable for ID2D1TessellationSink {
    type Vtable = ID2D1TessellationSink_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1TessellationSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906c1_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1TessellationSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddTriangles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triangles: *const D2D1_TRIANGLE, trianglescount: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddTriangles: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1Transform(::windows::core::IUnknown);
impl ID2D1Transform {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapOutputRectToInputRects(&self, outputrect: *const super::super::Foundation::RECT, inputrects: &mut [super::super::Foundation::RECT]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MapOutputRectToInputRects)(::windows::core::Vtable::as_raw(self), outputrect, ::core::mem::transmute(inputrects.as_ptr()), inputrects.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapInputRectsToOutputRect(&self, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MapInputRectsToOutputRect)(::windows::core::Vtable::as_raw(self), inputrects, inputopaquesubrects, inputrectcount, outputrect, outputopaquesubrect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapInvalidRect(&self, inputindex: u32, invalidinputrect: super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MapInvalidRect)(::windows::core::Vtable::as_raw(self), inputindex, ::core::mem::transmute(invalidinputrect), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID2D1Transform, ::windows::core::IUnknown, ID2D1TransformNode);
impl ::core::clone::Clone for ID2D1Transform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1Transform {}
unsafe impl ::core::marker::Sync for ID2D1Transform {}
unsafe impl ::windows::core::Vtable for ID2D1Transform {
    type Vtable = ID2D1Transform_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1Transform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef1a287d_342a_4f76_8fdb_da0d6ea9f92b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Transform_Vtbl {
    pub base__: ID2D1TransformNode_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MapOutputRectToInputRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputrect: *const super::super::Foundation::RECT, inputrects: *mut super::super::Foundation::RECT, inputrectscount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MapOutputRectToInputRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MapInputRectsToOutputRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MapInputRectsToOutputRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MapInvalidRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, invalidinputrect: super::super::Foundation::RECT, invalidoutputrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MapInvalidRect: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1TransformGraph(::windows::core::IUnknown);
impl ID2D1TransformGraph {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetSingleTransformNode<P0>(&self, node: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TransformNode>>,
    {
        (::windows::core::Vtable::vtable(self).SetSingleTransformNode)(::windows::core::Vtable::as_raw(self), node.into().abi()).ok()
    }
    pub unsafe fn AddNode<P0>(&self, node: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TransformNode>>,
    {
        (::windows::core::Vtable::vtable(self).AddNode)(::windows::core::Vtable::as_raw(self), node.into().abi()).ok()
    }
    pub unsafe fn RemoveNode<P0>(&self, node: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TransformNode>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveNode)(::windows::core::Vtable::as_raw(self), node.into().abi()).ok()
    }
    pub unsafe fn SetOutputNode<P0>(&self, node: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TransformNode>>,
    {
        (::windows::core::Vtable::vtable(self).SetOutputNode)(::windows::core::Vtable::as_raw(self), node.into().abi()).ok()
    }
    pub unsafe fn ConnectNode<P0, P1>(&self, fromnode: P0, tonode: P1, tonodeinputindex: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TransformNode>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1TransformNode>>,
    {
        (::windows::core::Vtable::vtable(self).ConnectNode)(::windows::core::Vtable::as_raw(self), fromnode.into().abi(), tonode.into().abi(), tonodeinputindex).ok()
    }
    pub unsafe fn ConnectToEffectInput<P0>(&self, toeffectinputindex: u32, node: P0, tonodeinputindex: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TransformNode>>,
    {
        (::windows::core::Vtable::vtable(self).ConnectToEffectInput)(::windows::core::Vtable::as_raw(self), toeffectinputindex, node.into().abi(), tonodeinputindex).ok()
    }
    pub unsafe fn Clear(&self) {
        (::windows::core::Vtable::vtable(self).Clear)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetPassthroughGraph(&self, effectinputindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPassthroughGraph)(::windows::core::Vtable::as_raw(self), effectinputindex).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1TransformGraph, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1TransformGraph {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1TransformGraph {}
unsafe impl ::core::marker::Sync for ID2D1TransformGraph {}
unsafe impl ::windows::core::Vtable for ID2D1TransformGraph {
    type Vtable = ID2D1TransformGraph_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1TransformGraph {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13d29038_c3e6_4034_9081_13b53a417992);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1TransformGraph_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub SetSingleTransformNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOutputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConnectNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromnode: *mut ::core::ffi::c_void, tonode: *mut ::core::ffi::c_void, tonodeinputindex: u32) -> ::windows::core::HRESULT,
    pub ConnectToEffectInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, toeffectinputindex: u32, node: *mut ::core::ffi::c_void, tonodeinputindex: u32) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub SetPassthroughGraph: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectinputindex: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1TransformNode(::windows::core::IUnknown);
impl ID2D1TransformNode {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID2D1TransformNode, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1TransformNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1TransformNode {}
unsafe impl ::core::marker::Sync for ID2D1TransformNode {}
unsafe impl ::windows::core::Vtable for ID2D1TransformNode {
    type Vtable = ID2D1TransformNode_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1TransformNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2efe1e7_729f_4102_949f_505fa21bf666);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1TransformNode_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1TransformedGeometry(::windows::core::IUnknown);
impl ID2D1TransformedGeometry {
    pub unsafe fn GetSourceGeometry(&self) -> ::windows::core::Result<ID2D1Geometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSourceGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Geometry as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
}
::windows::core::interface_hierarchy!(ID2D1TransformedGeometry, ::windows::core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ::core::clone::Clone for ID2D1TransformedGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1TransformedGeometry {}
unsafe impl ::core::marker::Sync for ID2D1TransformedGeometry {}
unsafe impl ::windows::core::Vtable for ID2D1TransformedGeometry {
    type Vtable = ID2D1TransformedGeometry_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1TransformedGeometry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd906bb_12e2_11dc_9fed_001143a055f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1TransformedGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    pub GetSourceGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcegeometry: *mut *mut ::core::ffi::c_void),
    #[cfg(feature = "Foundation_Numerics")]
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetTransform: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1TransformedImageSource(::windows::core::IUnknown);
impl ID2D1TransformedImageSource {
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<ID2D1ImageSource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1ImageSource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetProperties(&self, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) {
        (::windows::core::Vtable::vtable(self).GetProperties)(::windows::core::Vtable::as_raw(self), properties)
    }
}
::windows::core::interface_hierarchy!(ID2D1TransformedImageSource, ::windows::core::IUnknown, ID2D1Resource, ID2D1Image);
impl ::core::clone::Clone for ID2D1TransformedImageSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1TransformedImageSource {}
unsafe impl ::core::marker::Sync for ID2D1TransformedImageSource {}
unsafe impl ::windows::core::Vtable for ID2D1TransformedImageSource {
    type Vtable = ID2D1TransformedImageSource_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1TransformedImageSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f1f79e5_2796_416c_8f55_700f911445e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1TransformedImageSource_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagesource: *mut *mut ::core::ffi::c_void),
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct ID2D1VertexBuffer(::windows::core::IUnknown);
impl ID2D1VertexBuffer {
    pub unsafe fn Map(&self, data: *mut *mut u8, buffersize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Map)(::windows::core::Vtable::as_raw(self), data, buffersize).ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unmap)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ID2D1VertexBuffer, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID2D1VertexBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID2D1VertexBuffer {}
unsafe impl ::core::marker::Sync for ID2D1VertexBuffer {}
unsafe impl ::windows::core::Vtable for ID2D1VertexBuffer {
    type Vtable = ID2D1VertexBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for ID2D1VertexBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b8b1336_00a5_4668_92b7_ced5d8bf9b7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1VertexBuffer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut *mut u8, buffersize: u32) -> ::windows::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D12DAffineTransform: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aa97485_6354_4cfc_908c_e4a74f62c96c);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D13DPerspectiveTransform: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2844d0b_3d86_46e7_85ba_526c9240f3fb);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D13DTransform: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8467b04_ec61_4b8a_b5de_d4d73debea5a);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1AlphaMask: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc80ecff0_3fd5_4f05_8328_c5d1724b4f0a);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1ArithmeticComposite: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc151437_049a_4784_a24a_f1c4daf20987);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Atlas: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x913e2be4_fdcf_4fe2_a5f0_2454f14ff408);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1BitmapSource: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fb6c24d_c6dd_4231_9404_50f4d5c3252d);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Blend: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c5b77b_13f8_4cdd_ad20_c890547ac65d);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Border: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a2d49c0_4acf_43c7_8c6a_7c4a27874d27);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Brightness: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cea8d1e_77b0_4986_b3b9_2f0c0eae7887);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1ChromaKey: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74c01f5b_2a0d_408c_88e2_c7a3c7197742);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1ColorManagement: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a28524c_fdd6_4aa4_ae8f_837eb8267b37);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1ColorMatrix: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x921f03d6_641c_47df_852d_b4bb6153ae11);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Composite: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48fc9f51_f6ac_48f1_8b58_3b28ac46f76d);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Contrast: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb648a78a_0ed5_4f80_a94a_8e825aca6b77);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1ConvolveMatrix: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x407f8c08_5533_4331_a341_23cc3877843e);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Crop: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe23f7110_0e9a_4324_af47_6a2c0c46f35b);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1CrossFade: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12f575e8_4db1_485f_9a84_03a07dd3829f);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1DirectionalBlur: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x174319a6_58e9_49b2_bb63_caf2c811a3db);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1DiscreteTransfer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90866fcd_488e_454b_af06_e5041b66c36c);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1DisplacementMap: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedc48364_0417_4111_9450_43845fa9f890);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1DistantDiffuse: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e7efd62_a32d_46d4_a83c_5278889ac954);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1DistantSpecular: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x428c1ee5_77b8_4450_8ab5_72219c21abda);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1DpiCompensation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c26c5c7_34e0_46fc_9cfd_e5823706e228);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1EdgeDetection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeff583ca_cb07_4aa9_ac5d_2cc44c76460f);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Emboss: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1c5eb2b_0348_43f0_8107_4957cacba2ae);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Exposure: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb56c8cfa_f634_41ee_bee0_ffa617106004);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Flood: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61c23c20_ae69_4d8e_94cf_50078df638f2);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1GammaTransfer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x409444c4_c419_41a0_b0c1_8cd0c0a18e42);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1GaussianBlur: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1feb6d69_2fe6_4ac9_8c58_1d7f93e7a6a5);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Grayscale: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36dde0eb_3725_42e0_836d_52fb20aee644);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1HdrToneMap: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b0b748d_4610_4486_a90c_999d9a2e2b11);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1HighlightsShadows: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcadc8384_323f_4c7e_a361_2e2b24df6ee4);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Histogram: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x881db7d0_f7ee_4d4d_a6d2_4697acc66ee8);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1HueRotation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f4458ec_4b32_491b_9e85_bd73f44d3eb6);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1HueToRgb: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b78a6bd_0141_4def_8a52_6356ee0cbdd5);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Invert: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0c3784d_cb39_4e84_b6fd_6b72f0810263);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1LinearTransfer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad47c8fd_63ef_4acc_9b51_67979c036c06);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1LookupTable3D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x349e0eda_0088_4a79_9ca3_c7e300202020);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1LuminanceToAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41251ab7_0beb_46f8_9da7_59e93fcce5de);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Morphology: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeae6c40d_626a_4c2d_bfcb_391001abe202);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Opacity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x811d79a4_de28_4454_8094_c64685f8bd4c);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1OpacityMetadata: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c53006a_4450_4199_aa5b_ad1656fece5e);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1PointDiffuse: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9e303c3_c08c_4f91_8b7b_38656bc48c20);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1PointSpecular: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09c3ca26_3ae2_4f09_9ebc_ed3865d53f22);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Posterize: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2188945e_33a3_4366_b7bc_086bd02d0884);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Premultiply: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06eab419_deed_4018_80d2_3e1d471adeb2);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1RgbToHue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23f3e5ec_91e8_4d3d_ad0a_afadc1004aa1);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Saturation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cb2d9cf_327d_459f_a0ce_40c0b2086bf7);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Scale: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9daf9369_3846_4d0e_a44e_0c607934a5d7);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Sepia: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a1af410_5f1d_4dbe_84df_915da79b7153);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Shadow: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc67ea361_1863_4e69_89db_695d3e9a5b6b);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Sharpen: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9b887cb_c5ff_4dc5_9779_273dcf417c7d);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1SpotDiffuse: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x818a1105_7932_44f4_aa86_08ae7b2f2c93);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1SpotSpecular: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedae421e_7654_4a37_9db8_71acc1beb3c1);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Straighten: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4da47b12_79a3_4fb0_8237_bbc3b2a4de08);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1TableTransfer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bf818c3_5e43_48cb_b631_868396d6a1d4);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1TemperatureTint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89176087_8af9_4a08_aeb1_895f38db1766);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Tile: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0784138_3b76_4bc5_b13b_0fa2ad02659f);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Tint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36312b17_f7dd_4014_915d_ffca768cf211);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Turbulence: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf2bb6ae_889a_4ad7_ba29_a2fd732c9fc9);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1UnPremultiply: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb9ac489_ad8d_41ed_9999_bb6347d110f7);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1Vignette: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc00c40be_5e67_4ca3_95b4_f4b02c115135);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1WhiteLevelAdjustment: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44a1cadb_6cdd_4818_8ff4_26c1cfe95bdb);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const CLSID_D2D1YCbCr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99503cc1_66c7_45c9_a875_8ad8a7914401);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCENE_REFERRED_SDR_WHITE_LEVEL: f32 = 80f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const FACILITY_D2D: u32 = 2201u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_2DAFFINETRANSFORM_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_INTERPOLATION_MODE: D2D1_2DAFFINETRANSFORM_PROP = D2D1_2DAFFINETRANSFORM_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_BORDER_MODE: D2D1_2DAFFINETRANSFORM_PROP = D2D1_2DAFFINETRANSFORM_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_TRANSFORM_MATRIX: D2D1_2DAFFINETRANSFORM_PROP = D2D1_2DAFFINETRANSFORM_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_SHARPNESS: D2D1_2DAFFINETRANSFORM_PROP = D2D1_2DAFFINETRANSFORM_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_FORCE_DWORD: D2D1_2DAFFINETRANSFORM_PROP = D2D1_2DAFFINETRANSFORM_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_2DAFFINETRANSFORM_PROP {}
impl ::core::clone::Clone for D2D1_2DAFFINETRANSFORM_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_2DAFFINETRANSFORM_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_FORCE_DWORD: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE {}
impl ::core::clone::Clone for D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_3DPERSPECTIVETRANSFORM_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_INTERPOLATION_MODE: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_BORDER_MODE: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_DEPTH: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_PERSPECTIVE_ORIGIN: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_LOCAL_OFFSET: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_GLOBAL_OFFSET: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION_ORIGIN: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_FORCE_DWORD: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_3DPERSPECTIVETRANSFORM_PROP {}
impl ::core::clone::Clone for D2D1_3DPERSPECTIVETRANSFORM_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_3DPERSPECTIVETRANSFORM_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_3DTRANSFORM_INTERPOLATION_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = D2D1_3DTRANSFORM_INTERPOLATION_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = D2D1_3DTRANSFORM_INTERPOLATION_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_3DTRANSFORM_INTERPOLATION_MODE = D2D1_3DTRANSFORM_INTERPOLATION_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = D2D1_3DTRANSFORM_INTERPOLATION_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_3DTRANSFORM_INTERPOLATION_MODE = D2D1_3DTRANSFORM_INTERPOLATION_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_FORCE_DWORD: D2D1_3DTRANSFORM_INTERPOLATION_MODE = D2D1_3DTRANSFORM_INTERPOLATION_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_3DTRANSFORM_INTERPOLATION_MODE {}
impl ::core::clone::Clone for D2D1_3DTRANSFORM_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_3DTRANSFORM_INTERPOLATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_3DTRANSFORM_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_INTERPOLATION_MODE: D2D1_3DTRANSFORM_PROP = D2D1_3DTRANSFORM_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_BORDER_MODE: D2D1_3DTRANSFORM_PROP = D2D1_3DTRANSFORM_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_TRANSFORM_MATRIX: D2D1_3DTRANSFORM_PROP = D2D1_3DTRANSFORM_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_FORCE_DWORD: D2D1_3DTRANSFORM_PROP = D2D1_3DTRANSFORM_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_3DTRANSFORM_PROP {}
impl ::core::clone::Clone for D2D1_3DTRANSFORM_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_3DTRANSFORM_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_ANTIALIAS_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ANTIALIAS_MODE_PER_PRIMITIVE: D2D1_ANTIALIAS_MODE = D2D1_ANTIALIAS_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ANTIALIAS_MODE_ALIASED: D2D1_ANTIALIAS_MODE = D2D1_ANTIALIAS_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ANTIALIAS_MODE_FORCE_DWORD: D2D1_ANTIALIAS_MODE = D2D1_ANTIALIAS_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_ANTIALIAS_MODE {}
impl ::core::clone::Clone for D2D1_ANTIALIAS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_ANTIALIAS_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_ARC_SIZE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARC_SIZE_SMALL: D2D1_ARC_SIZE = D2D1_ARC_SIZE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARC_SIZE_LARGE: D2D1_ARC_SIZE = D2D1_ARC_SIZE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARC_SIZE_FORCE_DWORD: D2D1_ARC_SIZE = D2D1_ARC_SIZE(4294967295u32);
impl ::core::marker::Copy for D2D1_ARC_SIZE {}
impl ::core::clone::Clone for D2D1_ARC_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_ARC_SIZE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_ARITHMETICCOMPOSITE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARITHMETICCOMPOSITE_PROP_COEFFICIENTS: D2D1_ARITHMETICCOMPOSITE_PROP = D2D1_ARITHMETICCOMPOSITE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARITHMETICCOMPOSITE_PROP_CLAMP_OUTPUT: D2D1_ARITHMETICCOMPOSITE_PROP = D2D1_ARITHMETICCOMPOSITE_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARITHMETICCOMPOSITE_PROP_FORCE_DWORD: D2D1_ARITHMETICCOMPOSITE_PROP = D2D1_ARITHMETICCOMPOSITE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_ARITHMETICCOMPOSITE_PROP {}
impl ::core::clone::Clone for D2D1_ARITHMETICCOMPOSITE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_ARITHMETICCOMPOSITE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_ATLAS_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ATLAS_PROP_INPUT_RECT: D2D1_ATLAS_PROP = D2D1_ATLAS_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ATLAS_PROP_INPUT_PADDING_RECT: D2D1_ATLAS_PROP = D2D1_ATLAS_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ATLAS_PROP_FORCE_DWORD: D2D1_ATLAS_PROP = D2D1_ATLAS_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_ATLAS_PROP {}
impl ::core::clone::Clone for D2D1_ATLAS_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_ATLAS_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BITMAPSOURCE_ALPHA_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ALPHA_MODE_PREMULTIPLIED: D2D1_BITMAPSOURCE_ALPHA_MODE = D2D1_BITMAPSOURCE_ALPHA_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ALPHA_MODE_STRAIGHT: D2D1_BITMAPSOURCE_ALPHA_MODE = D2D1_BITMAPSOURCE_ALPHA_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ALPHA_MODE_FORCE_DWORD: D2D1_BITMAPSOURCE_ALPHA_MODE = D2D1_BITMAPSOURCE_ALPHA_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_BITMAPSOURCE_ALPHA_MODE {}
impl ::core::clone::Clone for D2D1_BITMAPSOURCE_ALPHA_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BITMAPSOURCE_ALPHA_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BITMAPSOURCE_INTERPOLATION_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = D2D1_BITMAPSOURCE_INTERPOLATION_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_LINEAR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = D2D1_BITMAPSOURCE_INTERPOLATION_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_CUBIC: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = D2D1_BITMAPSOURCE_INTERPOLATION_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_FANT: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = D2D1_BITMAPSOURCE_INTERPOLATION_MODE(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_MIPMAP_LINEAR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = D2D1_BITMAPSOURCE_INTERPOLATION_MODE(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_FORCE_DWORD: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = D2D1_BITMAPSOURCE_INTERPOLATION_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_BITMAPSOURCE_INTERPOLATION_MODE {}
impl ::core::clone::Clone for D2D1_BITMAPSOURCE_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BITMAPSOURCE_INTERPOLATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BITMAPSOURCE_ORIENTATION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_DEFAULT: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_FORCE_DWORD: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(4294967295u32);
impl ::core::marker::Copy for D2D1_BITMAPSOURCE_ORIENTATION {}
impl ::core::clone::Clone for D2D1_BITMAPSOURCE_ORIENTATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BITMAPSOURCE_ORIENTATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BITMAPSOURCE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_WIC_BITMAP_SOURCE: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_SCALE: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_INTERPOLATION_MODE: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_ENABLE_DPI_CORRECTION: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_ALPHA_MODE: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_ORIENTATION: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_FORCE_DWORD: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_BITMAPSOURCE_PROP {}
impl ::core::clone::Clone for D2D1_BITMAPSOURCE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BITMAPSOURCE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BITMAP_INTERPOLATION_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_BITMAP_INTERPOLATION_MODE = D2D1_BITMAP_INTERPOLATION_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_INTERPOLATION_MODE_LINEAR: D2D1_BITMAP_INTERPOLATION_MODE = D2D1_BITMAP_INTERPOLATION_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_INTERPOLATION_MODE_FORCE_DWORD: D2D1_BITMAP_INTERPOLATION_MODE = D2D1_BITMAP_INTERPOLATION_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_BITMAP_INTERPOLATION_MODE {}
impl ::core::clone::Clone for D2D1_BITMAP_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BITMAP_INTERPOLATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BITMAP_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_NONE: D2D1_BITMAP_OPTIONS = D2D1_BITMAP_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_TARGET: D2D1_BITMAP_OPTIONS = D2D1_BITMAP_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_CANNOT_DRAW: D2D1_BITMAP_OPTIONS = D2D1_BITMAP_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_CPU_READ: D2D1_BITMAP_OPTIONS = D2D1_BITMAP_OPTIONS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE: D2D1_BITMAP_OPTIONS = D2D1_BITMAP_OPTIONS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_FORCE_DWORD: D2D1_BITMAP_OPTIONS = D2D1_BITMAP_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_BITMAP_OPTIONS {}
impl ::core::clone::Clone for D2D1_BITMAP_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BITMAP_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BLEND(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_ZERO: D2D1_BLEND = D2D1_BLEND(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_ONE: D2D1_BLEND = D2D1_BLEND(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_SRC_COLOR: D2D1_BLEND = D2D1_BLEND(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_SRC_COLOR: D2D1_BLEND = D2D1_BLEND(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_SRC_ALPHA: D2D1_BLEND = D2D1_BLEND(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_SRC_ALPHA: D2D1_BLEND = D2D1_BLEND(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_DEST_ALPHA: D2D1_BLEND = D2D1_BLEND(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_DEST_ALPHA: D2D1_BLEND = D2D1_BLEND(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_DEST_COLOR: D2D1_BLEND = D2D1_BLEND(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_DEST_COLOR: D2D1_BLEND = D2D1_BLEND(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_SRC_ALPHA_SAT: D2D1_BLEND = D2D1_BLEND(11u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_BLEND_FACTOR: D2D1_BLEND = D2D1_BLEND(14u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_BLEND_FACTOR: D2D1_BLEND = D2D1_BLEND(15u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_FORCE_DWORD: D2D1_BLEND = D2D1_BLEND(4294967295u32);
impl ::core::marker::Copy for D2D1_BLEND {}
impl ::core::clone::Clone for D2D1_BLEND {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BLEND {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BLEND_OPERATION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_ADD: D2D1_BLEND_OPERATION = D2D1_BLEND_OPERATION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_SUBTRACT: D2D1_BLEND_OPERATION = D2D1_BLEND_OPERATION(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_REV_SUBTRACT: D2D1_BLEND_OPERATION = D2D1_BLEND_OPERATION(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_MIN: D2D1_BLEND_OPERATION = D2D1_BLEND_OPERATION(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_MAX: D2D1_BLEND_OPERATION = D2D1_BLEND_OPERATION(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_FORCE_DWORD: D2D1_BLEND_OPERATION = D2D1_BLEND_OPERATION(4294967295u32);
impl ::core::marker::Copy for D2D1_BLEND_OPERATION {}
impl ::core::clone::Clone for D2D1_BLEND_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BLEND_OPERATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BLEND_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_PROP_MODE: D2D1_BLEND_PROP = D2D1_BLEND_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_PROP_FORCE_DWORD: D2D1_BLEND_PROP = D2D1_BLEND_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_BLEND_PROP {}
impl ::core::clone::Clone for D2D1_BLEND_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BLEND_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BORDER_EDGE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_CLAMP: D2D1_BORDER_EDGE_MODE = D2D1_BORDER_EDGE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_WRAP: D2D1_BORDER_EDGE_MODE = D2D1_BORDER_EDGE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_MIRROR: D2D1_BORDER_EDGE_MODE = D2D1_BORDER_EDGE_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_FORCE_DWORD: D2D1_BORDER_EDGE_MODE = D2D1_BORDER_EDGE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_BORDER_EDGE_MODE {}
impl ::core::clone::Clone for D2D1_BORDER_EDGE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BORDER_EDGE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BORDER_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_PROP_EDGE_MODE_X: D2D1_BORDER_PROP = D2D1_BORDER_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_PROP_EDGE_MODE_Y: D2D1_BORDER_PROP = D2D1_BORDER_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_PROP_FORCE_DWORD: D2D1_BORDER_PROP = D2D1_BORDER_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_BORDER_PROP {}
impl ::core::clone::Clone for D2D1_BORDER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BORDER_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BRIGHTNESS_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BRIGHTNESS_PROP_WHITE_POINT: D2D1_BRIGHTNESS_PROP = D2D1_BRIGHTNESS_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BRIGHTNESS_PROP_BLACK_POINT: D2D1_BRIGHTNESS_PROP = D2D1_BRIGHTNESS_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BRIGHTNESS_PROP_FORCE_DWORD: D2D1_BRIGHTNESS_PROP = D2D1_BRIGHTNESS_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_BRIGHTNESS_PROP {}
impl ::core::clone::Clone for D2D1_BRIGHTNESS_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BRIGHTNESS_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_BUFFER_PRECISION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_UNKNOWN: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_8BPC_UNORM: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_8BPC_UNORM_SRGB: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_16BPC_UNORM: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_16BPC_FLOAT: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_32BPC_FLOAT: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_FORCE_DWORD: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(4294967295u32);
impl ::core::marker::Copy for D2D1_BUFFER_PRECISION {}
impl ::core::clone::Clone for D2D1_BUFFER_PRECISION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BUFFER_PRECISION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_CAP_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_FLAT: D2D1_CAP_STYLE = D2D1_CAP_STYLE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_SQUARE: D2D1_CAP_STYLE = D2D1_CAP_STYLE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_ROUND: D2D1_CAP_STYLE = D2D1_CAP_STYLE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_TRIANGLE: D2D1_CAP_STYLE = D2D1_CAP_STYLE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_FORCE_DWORD: D2D1_CAP_STYLE = D2D1_CAP_STYLE(4294967295u32);
impl ::core::marker::Copy for D2D1_CAP_STYLE {}
impl ::core::clone::Clone for D2D1_CAP_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_CAP_STYLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_CHANGE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_NONE: D2D1_CHANGE_TYPE = D2D1_CHANGE_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_PROPERTIES: D2D1_CHANGE_TYPE = D2D1_CHANGE_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_CONTEXT: D2D1_CHANGE_TYPE = D2D1_CHANGE_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_GRAPH: D2D1_CHANGE_TYPE = D2D1_CHANGE_TYPE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_FORCE_DWORD: D2D1_CHANGE_TYPE = D2D1_CHANGE_TYPE(4294967295u32);
impl ::core::marker::Copy for D2D1_CHANGE_TYPE {}
impl ::core::clone::Clone for D2D1_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_CHANGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_CHANNEL_DEPTH(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_DEFAULT: D2D1_CHANNEL_DEPTH = D2D1_CHANNEL_DEPTH(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_1: D2D1_CHANNEL_DEPTH = D2D1_CHANNEL_DEPTH(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_4: D2D1_CHANNEL_DEPTH = D2D1_CHANNEL_DEPTH(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_FORCE_DWORD: D2D1_CHANNEL_DEPTH = D2D1_CHANNEL_DEPTH(4294967295u32);
impl ::core::marker::Copy for D2D1_CHANNEL_DEPTH {}
impl ::core::clone::Clone for D2D1_CHANNEL_DEPTH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_CHANNEL_DEPTH {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_CHANNEL_SELECTOR(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_R: D2D1_CHANNEL_SELECTOR = D2D1_CHANNEL_SELECTOR(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_G: D2D1_CHANNEL_SELECTOR = D2D1_CHANNEL_SELECTOR(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_B: D2D1_CHANNEL_SELECTOR = D2D1_CHANNEL_SELECTOR(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_A: D2D1_CHANNEL_SELECTOR = D2D1_CHANNEL_SELECTOR(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_FORCE_DWORD: D2D1_CHANNEL_SELECTOR = D2D1_CHANNEL_SELECTOR(4294967295u32);
impl ::core::marker::Copy for D2D1_CHANNEL_SELECTOR {}
impl ::core::clone::Clone for D2D1_CHANNEL_SELECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_CHANNEL_SELECTOR {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_CHROMAKEY_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_COLOR: D2D1_CHROMAKEY_PROP = D2D1_CHROMAKEY_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_TOLERANCE: D2D1_CHROMAKEY_PROP = D2D1_CHROMAKEY_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_INVERT_ALPHA: D2D1_CHROMAKEY_PROP = D2D1_CHROMAKEY_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_FEATHER: D2D1_CHROMAKEY_PROP = D2D1_CHROMAKEY_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_FORCE_DWORD: D2D1_CHROMAKEY_PROP = D2D1_CHROMAKEY_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_CHROMAKEY_PROP {}
impl ::core::clone::Clone for D2D1_CHROMAKEY_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_CHROMAKEY_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COLORMANAGEMENT_ALPHA_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_ALPHA_MODE_PREMULTIPLIED: D2D1_COLORMANAGEMENT_ALPHA_MODE = D2D1_COLORMANAGEMENT_ALPHA_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_ALPHA_MODE_STRAIGHT: D2D1_COLORMANAGEMENT_ALPHA_MODE = D2D1_COLORMANAGEMENT_ALPHA_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_ALPHA_MODE_FORCE_DWORD: D2D1_COLORMANAGEMENT_ALPHA_MODE = D2D1_COLORMANAGEMENT_ALPHA_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_COLORMANAGEMENT_ALPHA_MODE {}
impl ::core::clone::Clone for D2D1_COLORMANAGEMENT_ALPHA_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COLORMANAGEMENT_ALPHA_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COLORMANAGEMENT_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_SOURCE_COLOR_CONTEXT: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_SOURCE_RENDERING_INTENT: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_DESTINATION_COLOR_CONTEXT: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_DESTINATION_RENDERING_INTENT: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_ALPHA_MODE: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_QUALITY: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_FORCE_DWORD: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_COLORMANAGEMENT_PROP {}
impl ::core::clone::Clone for D2D1_COLORMANAGEMENT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COLORMANAGEMENT_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COLORMANAGEMENT_QUALITY(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_PROOF: D2D1_COLORMANAGEMENT_QUALITY = D2D1_COLORMANAGEMENT_QUALITY(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_NORMAL: D2D1_COLORMANAGEMENT_QUALITY = D2D1_COLORMANAGEMENT_QUALITY(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_BEST: D2D1_COLORMANAGEMENT_QUALITY = D2D1_COLORMANAGEMENT_QUALITY(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_FORCE_DWORD: D2D1_COLORMANAGEMENT_QUALITY = D2D1_COLORMANAGEMENT_QUALITY(4294967295u32);
impl ::core::marker::Copy for D2D1_COLORMANAGEMENT_QUALITY {}
impl ::core::clone::Clone for D2D1_COLORMANAGEMENT_QUALITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COLORMANAGEMENT_QUALITY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COLORMANAGEMENT_RENDERING_INTENT(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_PERCEPTUAL: D2D1_COLORMANAGEMENT_RENDERING_INTENT = D2D1_COLORMANAGEMENT_RENDERING_INTENT(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_RELATIVE_COLORIMETRIC: D2D1_COLORMANAGEMENT_RENDERING_INTENT = D2D1_COLORMANAGEMENT_RENDERING_INTENT(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_SATURATION: D2D1_COLORMANAGEMENT_RENDERING_INTENT = D2D1_COLORMANAGEMENT_RENDERING_INTENT(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_ABSOLUTE_COLORIMETRIC: D2D1_COLORMANAGEMENT_RENDERING_INTENT = D2D1_COLORMANAGEMENT_RENDERING_INTENT(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_FORCE_DWORD: D2D1_COLORMANAGEMENT_RENDERING_INTENT = D2D1_COLORMANAGEMENT_RENDERING_INTENT(4294967295u32);
impl ::core::marker::Copy for D2D1_COLORMANAGEMENT_RENDERING_INTENT {}
impl ::core::clone::Clone for D2D1_COLORMANAGEMENT_RENDERING_INTENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COLORMANAGEMENT_RENDERING_INTENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COLORMATRIX_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_COLOR_MATRIX: D2D1_COLORMATRIX_PROP = D2D1_COLORMATRIX_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_ALPHA_MODE: D2D1_COLORMATRIX_PROP = D2D1_COLORMATRIX_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_CLAMP_OUTPUT: D2D1_COLORMATRIX_PROP = D2D1_COLORMATRIX_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_FORCE_DWORD: D2D1_COLORMATRIX_PROP = D2D1_COLORMATRIX_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_COLORMATRIX_PROP {}
impl ::core::clone::Clone for D2D1_COLORMATRIX_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COLORMATRIX_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DEFAULT: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DISABLE: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_FORCE_DWORD: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION(4294967295u32);
impl ::core::marker::Copy for D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION {}
impl ::core::clone::Clone for D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COLOR_CONTEXT_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_ICC: D2D1_COLOR_CONTEXT_TYPE = D2D1_COLOR_CONTEXT_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_SIMPLE: D2D1_COLOR_CONTEXT_TYPE = D2D1_COLOR_CONTEXT_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_DXGI: D2D1_COLOR_CONTEXT_TYPE = D2D1_COLOR_CONTEXT_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_FORCE_DWORD: D2D1_COLOR_CONTEXT_TYPE = D2D1_COLOR_CONTEXT_TYPE(4294967295u32);
impl ::core::marker::Copy for D2D1_COLOR_CONTEXT_TYPE {}
impl ::core::clone::Clone for D2D1_COLOR_CONTEXT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COLOR_CONTEXT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COLOR_INTERPOLATION_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_INTERPOLATION_MODE_STRAIGHT: D2D1_COLOR_INTERPOLATION_MODE = D2D1_COLOR_INTERPOLATION_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED: D2D1_COLOR_INTERPOLATION_MODE = D2D1_COLOR_INTERPOLATION_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_INTERPOLATION_MODE_FORCE_DWORD: D2D1_COLOR_INTERPOLATION_MODE = D2D1_COLOR_INTERPOLATION_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_COLOR_INTERPOLATION_MODE {}
impl ::core::clone::Clone for D2D1_COLOR_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COLOR_INTERPOLATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COLOR_SPACE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_CUSTOM: D2D1_COLOR_SPACE = D2D1_COLOR_SPACE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_SRGB: D2D1_COLOR_SPACE = D2D1_COLOR_SPACE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_SCRGB: D2D1_COLOR_SPACE = D2D1_COLOR_SPACE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_FORCE_DWORD: D2D1_COLOR_SPACE = D2D1_COLOR_SPACE(4294967295u32);
impl ::core::marker::Copy for D2D1_COLOR_SPACE {}
impl ::core::clone::Clone for D2D1_COLOR_SPACE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COLOR_SPACE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COMBINE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_UNION: D2D1_COMBINE_MODE = D2D1_COMBINE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_INTERSECT: D2D1_COMBINE_MODE = D2D1_COMBINE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_XOR: D2D1_COMBINE_MODE = D2D1_COMBINE_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_EXCLUDE: D2D1_COMBINE_MODE = D2D1_COMBINE_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_FORCE_DWORD: D2D1_COMBINE_MODE = D2D1_COMBINE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_COMBINE_MODE {}
impl ::core::clone::Clone for D2D1_COMBINE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COMBINE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_GDI_COMPATIBLE: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_FORCE_DWORD: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {}
impl ::core::clone::Clone for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_COMPOSITE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPOSITE_PROP_MODE: D2D1_COMPOSITE_PROP = D2D1_COMPOSITE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPOSITE_PROP_FORCE_DWORD: D2D1_COMPOSITE_PROP = D2D1_COMPOSITE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_COMPOSITE_PROP {}
impl ::core::clone::Clone for D2D1_COMPOSITE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_COMPOSITE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_CONTRAST_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONTRAST_PROP_CONTRAST: D2D1_CONTRAST_PROP = D2D1_CONTRAST_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONTRAST_PROP_CLAMP_INPUT: D2D1_CONTRAST_PROP = D2D1_CONTRAST_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONTRAST_PROP_FORCE_DWORD: D2D1_CONTRAST_PROP = D2D1_CONTRAST_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_CONTRAST_PROP {}
impl ::core::clone::Clone for D2D1_CONTRAST_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_CONTRAST_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_CONVOLVEMATRIX_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_UNIT_LENGTH: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_SCALE_MODE: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_X: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_Y: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_MATRIX: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_DIVISOR: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_BIAS: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_OFFSET: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_PRESERVE_ALPHA: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_BORDER_MODE: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_CLAMP_OUTPUT: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_FORCE_DWORD: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_CONVOLVEMATRIX_PROP {}
impl ::core::clone::Clone for D2D1_CONVOLVEMATRIX_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_CONVOLVEMATRIX_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_CONVOLVEMATRIX_SCALE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_LINEAR: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_CUBIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_ANISOTROPIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_FORCE_DWORD: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_CONVOLVEMATRIX_SCALE_MODE {}
impl ::core::clone::Clone for D2D1_CONVOLVEMATRIX_SCALE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_CONVOLVEMATRIX_SCALE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_CROP_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROP_PROP_RECT: D2D1_CROP_PROP = D2D1_CROP_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROP_PROP_BORDER_MODE: D2D1_CROP_PROP = D2D1_CROP_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROP_PROP_FORCE_DWORD: D2D1_CROP_PROP = D2D1_CROP_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_CROP_PROP {}
impl ::core::clone::Clone for D2D1_CROP_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_CROP_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_CROSSFADE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROSSFADE_PROP_WEIGHT: D2D1_CROSSFADE_PROP = D2D1_CROSSFADE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROSSFADE_PROP_FORCE_DWORD: D2D1_CROSSFADE_PROP = D2D1_CROSSFADE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_CROSSFADE_PROP {}
impl ::core::clone::Clone for D2D1_CROSSFADE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_CROSSFADE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DASH_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_SOLID: D2D1_DASH_STYLE = D2D1_DASH_STYLE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DASH: D2D1_DASH_STYLE = D2D1_DASH_STYLE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DOT: D2D1_DASH_STYLE = D2D1_DASH_STYLE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DASH_DOT: D2D1_DASH_STYLE = D2D1_DASH_STYLE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DASH_DOT_DOT: D2D1_DASH_STYLE = D2D1_DASH_STYLE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_CUSTOM: D2D1_DASH_STYLE = D2D1_DASH_STYLE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_FORCE_DWORD: D2D1_DASH_STYLE = D2D1_DASH_STYLE(4294967295u32);
impl ::core::marker::Copy for D2D1_DASH_STYLE {}
impl ::core::clone::Clone for D2D1_DASH_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DASH_STYLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DC_INITIALIZE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DC_INITIALIZE_MODE_COPY: D2D1_DC_INITIALIZE_MODE = D2D1_DC_INITIALIZE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DC_INITIALIZE_MODE_CLEAR: D2D1_DC_INITIALIZE_MODE = D2D1_DC_INITIALIZE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DC_INITIALIZE_MODE_FORCE_DWORD: D2D1_DC_INITIALIZE_MODE = D2D1_DC_INITIALIZE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_DC_INITIALIZE_MODE {}
impl ::core::clone::Clone for D2D1_DC_INITIALIZE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DC_INITIALIZE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DEBUG_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_NONE: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_ERROR: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_WARNING: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_INFORMATION: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_FORCE_DWORD: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(4294967295u32);
impl ::core::marker::Copy for D2D1_DEBUG_LEVEL {}
impl ::core::clone::Clone for D2D1_DEBUG_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DEBUG_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DEVICE_CONTEXT_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEVICE_CONTEXT_OPTIONS_NONE: D2D1_DEVICE_CONTEXT_OPTIONS = D2D1_DEVICE_CONTEXT_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS: D2D1_DEVICE_CONTEXT_OPTIONS = D2D1_DEVICE_CONTEXT_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEVICE_CONTEXT_OPTIONS_FORCE_DWORD: D2D1_DEVICE_CONTEXT_OPTIONS = D2D1_DEVICE_CONTEXT_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_DEVICE_CONTEXT_OPTIONS {}
impl ::core::clone::Clone for D2D1_DEVICE_CONTEXT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DEVICE_CONTEXT_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DIRECTIONALBLUR_OPTIMIZATION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_SPEED: D2D1_DIRECTIONALBLUR_OPTIMIZATION = D2D1_DIRECTIONALBLUR_OPTIMIZATION(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_BALANCED: D2D1_DIRECTIONALBLUR_OPTIMIZATION = D2D1_DIRECTIONALBLUR_OPTIMIZATION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_QUALITY: D2D1_DIRECTIONALBLUR_OPTIMIZATION = D2D1_DIRECTIONALBLUR_OPTIMIZATION(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_FORCE_DWORD: D2D1_DIRECTIONALBLUR_OPTIMIZATION = D2D1_DIRECTIONALBLUR_OPTIMIZATION(4294967295u32);
impl ::core::marker::Copy for D2D1_DIRECTIONALBLUR_OPTIMIZATION {}
impl ::core::clone::Clone for D2D1_DIRECTIONALBLUR_OPTIMIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DIRECTIONALBLUR_OPTIMIZATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DIRECTIONALBLUR_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_STANDARD_DEVIATION: D2D1_DIRECTIONALBLUR_PROP = D2D1_DIRECTIONALBLUR_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_ANGLE: D2D1_DIRECTIONALBLUR_PROP = D2D1_DIRECTIONALBLUR_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_OPTIMIZATION: D2D1_DIRECTIONALBLUR_PROP = D2D1_DIRECTIONALBLUR_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_BORDER_MODE: D2D1_DIRECTIONALBLUR_PROP = D2D1_DIRECTIONALBLUR_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_FORCE_DWORD: D2D1_DIRECTIONALBLUR_PROP = D2D1_DIRECTIONALBLUR_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_DIRECTIONALBLUR_PROP {}
impl ::core::clone::Clone for D2D1_DIRECTIONALBLUR_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DIRECTIONALBLUR_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DISCRETETRANSFER_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_RED_TABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_RED_DISABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_GREEN_TABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_GREEN_DISABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_BLUE_TABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_BLUE_DISABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_ALPHA_TABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_ALPHA_DISABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_CLAMP_OUTPUT: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_FORCE_DWORD: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_DISCRETETRANSFER_PROP {}
impl ::core::clone::Clone for D2D1_DISCRETETRANSFER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DISCRETETRANSFER_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DISPLACEMENTMAP_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_SCALE: D2D1_DISPLACEMENTMAP_PROP = D2D1_DISPLACEMENTMAP_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_X_CHANNEL_SELECT: D2D1_DISPLACEMENTMAP_PROP = D2D1_DISPLACEMENTMAP_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_Y_CHANNEL_SELECT: D2D1_DISPLACEMENTMAP_PROP = D2D1_DISPLACEMENTMAP_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_FORCE_DWORD: D2D1_DISPLACEMENTMAP_PROP = D2D1_DISPLACEMENTMAP_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_DISPLACEMENTMAP_PROP {}
impl ::core::clone::Clone for D2D1_DISPLACEMENTMAP_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DISPLACEMENTMAP_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DISTANTDIFFUSE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_AZIMUTH: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_ELEVATION: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_SURFACE_SCALE: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_COLOR: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_SCALE_MODE: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_FORCE_DWORD: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_DISTANTDIFFUSE_PROP {}
impl ::core::clone::Clone for D2D1_DISTANTDIFFUSE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DISTANTDIFFUSE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DISTANTDIFFUSE_SCALE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_LINEAR: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_CUBIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_FORCE_DWORD: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_DISTANTDIFFUSE_SCALE_MODE {}
impl ::core::clone::Clone for D2D1_DISTANTDIFFUSE_SCALE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DISTANTDIFFUSE_SCALE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DISTANTSPECULAR_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_AZIMUTH: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_ELEVATION: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SURFACE_SCALE: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_COLOR: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SCALE_MODE: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_FORCE_DWORD: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_DISTANTSPECULAR_PROP {}
impl ::core::clone::Clone for D2D1_DISTANTSPECULAR_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DISTANTSPECULAR_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DISTANTSPECULAR_SCALE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_LINEAR: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_CUBIC: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_FORCE_DWORD: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_DISTANTSPECULAR_SCALE_MODE {}
impl ::core::clone::Clone for D2D1_DISTANTSPECULAR_SCALE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DISTANTSPECULAR_SCALE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DPICOMPENSATION_INTERPOLATION_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_LINEAR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_CUBIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_ANISOTROPIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_FORCE_DWORD: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_DPICOMPENSATION_INTERPOLATION_MODE {}
impl ::core::clone::Clone for D2D1_DPICOMPENSATION_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DPICOMPENSATION_INTERPOLATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DPICOMPENSATION_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_INTERPOLATION_MODE: D2D1_DPICOMPENSATION_PROP = D2D1_DPICOMPENSATION_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_BORDER_MODE: D2D1_DPICOMPENSATION_PROP = D2D1_DPICOMPENSATION_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_INPUT_DPI: D2D1_DPICOMPENSATION_PROP = D2D1_DPICOMPENSATION_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_FORCE_DWORD: D2D1_DPICOMPENSATION_PROP = D2D1_DPICOMPENSATION_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_DPICOMPENSATION_PROP {}
impl ::core::clone::Clone for D2D1_DPICOMPENSATION_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DPICOMPENSATION_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_DRAW_TEXT_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_NO_SNAP: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_CLIP: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_FORCE_DWORD: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_DRAW_TEXT_OPTIONS {}
impl ::core::clone::Clone for D2D1_DRAW_TEXT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_DRAW_TEXT_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_EDGEDETECTION_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_MODE_SOBEL: D2D1_EDGEDETECTION_MODE = D2D1_EDGEDETECTION_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_MODE_PREWITT: D2D1_EDGEDETECTION_MODE = D2D1_EDGEDETECTION_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_MODE_FORCE_DWORD: D2D1_EDGEDETECTION_MODE = D2D1_EDGEDETECTION_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_EDGEDETECTION_MODE {}
impl ::core::clone::Clone for D2D1_EDGEDETECTION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_EDGEDETECTION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_EDGEDETECTION_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_STRENGTH: D2D1_EDGEDETECTION_PROP = D2D1_EDGEDETECTION_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_BLUR_RADIUS: D2D1_EDGEDETECTION_PROP = D2D1_EDGEDETECTION_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_MODE: D2D1_EDGEDETECTION_PROP = D2D1_EDGEDETECTION_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_OVERLAY_EDGES: D2D1_EDGEDETECTION_PROP = D2D1_EDGEDETECTION_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_ALPHA_MODE: D2D1_EDGEDETECTION_PROP = D2D1_EDGEDETECTION_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_FORCE_DWORD: D2D1_EDGEDETECTION_PROP = D2D1_EDGEDETECTION_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_EDGEDETECTION_PROP {}
impl ::core::clone::Clone for D2D1_EDGEDETECTION_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_EDGEDETECTION_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_EMBOSS_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EMBOSS_PROP_HEIGHT: D2D1_EMBOSS_PROP = D2D1_EMBOSS_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EMBOSS_PROP_DIRECTION: D2D1_EMBOSS_PROP = D2D1_EMBOSS_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EMBOSS_PROP_FORCE_DWORD: D2D1_EMBOSS_PROP = D2D1_EMBOSS_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_EMBOSS_PROP {}
impl ::core::clone::Clone for D2D1_EMBOSS_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_EMBOSS_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_EXPOSURE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXPOSURE_PROP_EXPOSURE_VALUE: D2D1_EXPOSURE_PROP = D2D1_EXPOSURE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXPOSURE_PROP_FORCE_DWORD: D2D1_EXPOSURE_PROP = D2D1_EXPOSURE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_EXPOSURE_PROP {}
impl ::core::clone::Clone for D2D1_EXPOSURE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_EXPOSURE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_EXTEND_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_CLAMP: D2D1_EXTEND_MODE = D2D1_EXTEND_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_WRAP: D2D1_EXTEND_MODE = D2D1_EXTEND_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_MIRROR: D2D1_EXTEND_MODE = D2D1_EXTEND_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_FORCE_DWORD: D2D1_EXTEND_MODE = D2D1_EXTEND_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_EXTEND_MODE {}
impl ::core::clone::Clone for D2D1_EXTEND_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_EXTEND_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_FACTORY_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FACTORY_TYPE_SINGLE_THREADED: D2D1_FACTORY_TYPE = D2D1_FACTORY_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FACTORY_TYPE_MULTI_THREADED: D2D1_FACTORY_TYPE = D2D1_FACTORY_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FACTORY_TYPE_FORCE_DWORD: D2D1_FACTORY_TYPE = D2D1_FACTORY_TYPE(4294967295u32);
impl ::core::marker::Copy for D2D1_FACTORY_TYPE {}
impl ::core::clone::Clone for D2D1_FACTORY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_FACTORY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_FEATURE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_DOUBLES: D2D1_FEATURE = D2D1_FEATURE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_D3D10_X_HARDWARE_OPTIONS: D2D1_FEATURE = D2D1_FEATURE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_FORCE_DWORD: D2D1_FEATURE = D2D1_FEATURE(4294967295u32);
impl ::core::marker::Copy for D2D1_FEATURE {}
impl ::core::clone::Clone for D2D1_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_FEATURE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_FEATURE_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_DEFAULT: D2D1_FEATURE_LEVEL = D2D1_FEATURE_LEVEL(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_9: D2D1_FEATURE_LEVEL = D2D1_FEATURE_LEVEL(37120u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_10: D2D1_FEATURE_LEVEL = D2D1_FEATURE_LEVEL(40960u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_FORCE_DWORD: D2D1_FEATURE_LEVEL = D2D1_FEATURE_LEVEL(4294967295u32);
impl ::core::marker::Copy for D2D1_FEATURE_LEVEL {}
impl ::core::clone::Clone for D2D1_FEATURE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_FEATURE_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_FILTER(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_MIP_POINT: D2D1_FILTER = D2D1_FILTER(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_POINT_MIP_LINEAR: D2D1_FILTER = D2D1_FILTER(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D2D1_FILTER = D2D1_FILTER(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_POINT_MAG_MIP_LINEAR: D2D1_FILTER = D2D1_FILTER(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_LINEAR_MAG_MIP_POINT: D2D1_FILTER = D2D1_FILTER(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D2D1_FILTER = D2D1_FILTER(17u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_LINEAR_MIP_POINT: D2D1_FILTER = D2D1_FILTER(20u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_MIP_LINEAR: D2D1_FILTER = D2D1_FILTER(21u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_ANISOTROPIC: D2D1_FILTER = D2D1_FILTER(85u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_FORCE_DWORD: D2D1_FILTER = D2D1_FILTER(4294967295u32);
impl ::core::marker::Copy for D2D1_FILTER {}
impl ::core::clone::Clone for D2D1_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_FILTER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_FLOOD_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FLOOD_PROP_COLOR: D2D1_FLOOD_PROP = D2D1_FLOOD_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FLOOD_PROP_FORCE_DWORD: D2D1_FLOOD_PROP = D2D1_FLOOD_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_FLOOD_PROP {}
impl ::core::clone::Clone for D2D1_FLOOD_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_FLOOD_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_GAMMA(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA_2_2: D2D1_GAMMA = D2D1_GAMMA(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA_1_0: D2D1_GAMMA = D2D1_GAMMA(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA_FORCE_DWORD: D2D1_GAMMA = D2D1_GAMMA(4294967295u32);
impl ::core::marker::Copy for D2D1_GAMMA {}
impl ::core::clone::Clone for D2D1_GAMMA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_GAMMA {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_GAMMA1(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_G22: D2D1_GAMMA1 = D2D1_GAMMA1(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_G10: D2D1_GAMMA1 = D2D1_GAMMA1(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_G2084: D2D1_GAMMA1 = D2D1_GAMMA1(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_FORCE_DWORD: D2D1_GAMMA1 = D2D1_GAMMA1(4294967295u32);
impl ::core::marker::Copy for D2D1_GAMMA1 {}
impl ::core::clone::Clone for D2D1_GAMMA1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_GAMMA1 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_GAMMATRANSFER_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_EXPONENT: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_OFFSET: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_DISABLE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_EXPONENT: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_OFFSET: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_DISABLE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_EXPONENT: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_OFFSET: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_DISABLE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(11u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(12u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_EXPONENT: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(13u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_OFFSET: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(14u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_DISABLE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(15u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_CLAMP_OUTPUT: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_FORCE_DWORD: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_GAMMATRANSFER_PROP {}
impl ::core::clone::Clone for D2D1_GAMMATRANSFER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_GAMMATRANSFER_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_GAUSSIANBLUR_OPTIMIZATION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_SPEED: D2D1_GAUSSIANBLUR_OPTIMIZATION = D2D1_GAUSSIANBLUR_OPTIMIZATION(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_BALANCED: D2D1_GAUSSIANBLUR_OPTIMIZATION = D2D1_GAUSSIANBLUR_OPTIMIZATION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_QUALITY: D2D1_GAUSSIANBLUR_OPTIMIZATION = D2D1_GAUSSIANBLUR_OPTIMIZATION(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_FORCE_DWORD: D2D1_GAUSSIANBLUR_OPTIMIZATION = D2D1_GAUSSIANBLUR_OPTIMIZATION(4294967295u32);
impl ::core::marker::Copy for D2D1_GAUSSIANBLUR_OPTIMIZATION {}
impl ::core::clone::Clone for D2D1_GAUSSIANBLUR_OPTIMIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_GAUSSIANBLUR_OPTIMIZATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_GAUSSIANBLUR_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_STANDARD_DEVIATION: D2D1_GAUSSIANBLUR_PROP = D2D1_GAUSSIANBLUR_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_OPTIMIZATION: D2D1_GAUSSIANBLUR_PROP = D2D1_GAUSSIANBLUR_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_BORDER_MODE: D2D1_GAUSSIANBLUR_PROP = D2D1_GAUSSIANBLUR_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_FORCE_DWORD: D2D1_GAUSSIANBLUR_PROP = D2D1_GAUSSIANBLUR_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_GAUSSIANBLUR_PROP {}
impl ::core::clone::Clone for D2D1_GAUSSIANBLUR_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_GAUSSIANBLUR_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_GEOMETRY_RELATION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_UNKNOWN: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_DISJOINT: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_IS_CONTAINED: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_CONTAINS: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_OVERLAP: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_FORCE_DWORD: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION(4294967295u32);
impl ::core::marker::Copy for D2D1_GEOMETRY_RELATION {}
impl ::core::clone::Clone for D2D1_GEOMETRY_RELATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_GEOMETRY_RELATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_GEOMETRY_SIMPLIFICATION_OPTION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_CUBICS_AND_LINES: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = D2D1_GEOMETRY_SIMPLIFICATION_OPTION(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_LINES: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = D2D1_GEOMETRY_SIMPLIFICATION_OPTION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_FORCE_DWORD: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = D2D1_GEOMETRY_SIMPLIFICATION_OPTION(4294967295u32);
impl ::core::marker::Copy for D2D1_GEOMETRY_SIMPLIFICATION_OPTION {}
impl ::core::clone::Clone for D2D1_GEOMETRY_SIMPLIFICATION_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_GEOMETRY_SIMPLIFICATION_OPTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_HDRTONEMAP_DISPLAY_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_DISPLAY_MODE_SDR: D2D1_HDRTONEMAP_DISPLAY_MODE = D2D1_HDRTONEMAP_DISPLAY_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_DISPLAY_MODE_HDR: D2D1_HDRTONEMAP_DISPLAY_MODE = D2D1_HDRTONEMAP_DISPLAY_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_DISPLAY_MODE_FORCE_DWORD: D2D1_HDRTONEMAP_DISPLAY_MODE = D2D1_HDRTONEMAP_DISPLAY_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_HDRTONEMAP_DISPLAY_MODE {}
impl ::core::clone::Clone for D2D1_HDRTONEMAP_DISPLAY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_HDRTONEMAP_DISPLAY_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_HDRTONEMAP_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_INPUT_MAX_LUMINANCE: D2D1_HDRTONEMAP_PROP = D2D1_HDRTONEMAP_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_OUTPUT_MAX_LUMINANCE: D2D1_HDRTONEMAP_PROP = D2D1_HDRTONEMAP_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_DISPLAY_MODE: D2D1_HDRTONEMAP_PROP = D2D1_HDRTONEMAP_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_FORCE_DWORD: D2D1_HDRTONEMAP_PROP = D2D1_HDRTONEMAP_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_HDRTONEMAP_PROP {}
impl ::core::clone::Clone for D2D1_HDRTONEMAP_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_HDRTONEMAP_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_LINEAR: D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_SRGB: D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_FORCE_DWORD: D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA(4294967295u32);
impl ::core::marker::Copy for D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA {}
impl ::core::clone::Clone for D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_HIGHLIGHTSANDSHADOWS_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_HIGHLIGHTS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = D2D1_HIGHLIGHTSANDSHADOWS_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_SHADOWS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = D2D1_HIGHLIGHTSANDSHADOWS_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_CLARITY: D2D1_HIGHLIGHTSANDSHADOWS_PROP = D2D1_HIGHLIGHTSANDSHADOWS_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_INPUT_GAMMA: D2D1_HIGHLIGHTSANDSHADOWS_PROP = D2D1_HIGHLIGHTSANDSHADOWS_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_MASK_BLUR_RADIUS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = D2D1_HIGHLIGHTSANDSHADOWS_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_FORCE_DWORD: D2D1_HIGHLIGHTSANDSHADOWS_PROP = D2D1_HIGHLIGHTSANDSHADOWS_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_HIGHLIGHTSANDSHADOWS_PROP {}
impl ::core::clone::Clone for D2D1_HIGHLIGHTSANDSHADOWS_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_HIGHLIGHTSANDSHADOWS_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_HISTOGRAM_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_NUM_BINS: D2D1_HISTOGRAM_PROP = D2D1_HISTOGRAM_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_CHANNEL_SELECT: D2D1_HISTOGRAM_PROP = D2D1_HISTOGRAM_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_HISTOGRAM_OUTPUT: D2D1_HISTOGRAM_PROP = D2D1_HISTOGRAM_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_FORCE_DWORD: D2D1_HISTOGRAM_PROP = D2D1_HISTOGRAM_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_HISTOGRAM_PROP {}
impl ::core::clone::Clone for D2D1_HISTOGRAM_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_HISTOGRAM_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_HUEROTATION_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUEROTATION_PROP_ANGLE: D2D1_HUEROTATION_PROP = D2D1_HUEROTATION_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUEROTATION_PROP_FORCE_DWORD: D2D1_HUEROTATION_PROP = D2D1_HUEROTATION_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_HUEROTATION_PROP {}
impl ::core::clone::Clone for D2D1_HUEROTATION_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_HUEROTATION_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_HUETORGB_INPUT_COLOR_SPACE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_VALUE: D2D1_HUETORGB_INPUT_COLOR_SPACE = D2D1_HUETORGB_INPUT_COLOR_SPACE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS: D2D1_HUETORGB_INPUT_COLOR_SPACE = D2D1_HUETORGB_INPUT_COLOR_SPACE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_INPUT_COLOR_SPACE_FORCE_DWORD: D2D1_HUETORGB_INPUT_COLOR_SPACE = D2D1_HUETORGB_INPUT_COLOR_SPACE(4294967295u32);
impl ::core::marker::Copy for D2D1_HUETORGB_INPUT_COLOR_SPACE {}
impl ::core::clone::Clone for D2D1_HUETORGB_INPUT_COLOR_SPACE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_HUETORGB_INPUT_COLOR_SPACE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_HUETORGB_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_PROP_INPUT_COLOR_SPACE: D2D1_HUETORGB_PROP = D2D1_HUETORGB_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_PROP_FORCE_DWORD: D2D1_HUETORGB_PROP = D2D1_HUETORGB_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_HUETORGB_PROP {}
impl ::core::clone::Clone for D2D1_HUETORGB_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_HUETORGB_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_NONE: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_LOW_QUALITY_PRIMARY_CONVERSION: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_FORCE_DWORD: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {}
impl ::core::clone::Clone for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_IMAGE_SOURCE_LOADING_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_NONE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = D2D1_IMAGE_SOURCE_LOADING_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_RELEASE_SOURCE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = D2D1_IMAGE_SOURCE_LOADING_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_CACHE_ON_DEMAND: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = D2D1_IMAGE_SOURCE_LOADING_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_FORCE_DWORD: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = D2D1_IMAGE_SOURCE_LOADING_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {}
impl ::core::clone::Clone for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_INK_NIB_SHAPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INK_NIB_SHAPE_ROUND: D2D1_INK_NIB_SHAPE = D2D1_INK_NIB_SHAPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INK_NIB_SHAPE_SQUARE: D2D1_INK_NIB_SHAPE = D2D1_INK_NIB_SHAPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INK_NIB_SHAPE_FORCE_DWORD: D2D1_INK_NIB_SHAPE = D2D1_INK_NIB_SHAPE(4294967295u32);
impl ::core::marker::Copy for D2D1_INK_NIB_SHAPE {}
impl ::core::clone::Clone for D2D1_INK_NIB_SHAPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_INK_NIB_SHAPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_INTERPOLATION_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_LINEAR: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_CUBIC: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_ANISOTROPIC: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_FORCE_DWORD: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_INTERPOLATION_MODE {}
impl ::core::clone::Clone for D2D1_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_INTERPOLATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_INTERPOLATION_MODE_DEFINITION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_NEAREST_NEIGHBOR: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_CUBIC: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MULTI_SAMPLE_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_ANISOTROPIC: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_HIGH_QUALITY_CUBIC: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_FANT: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MIPMAP_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(7i32);
impl ::core::marker::Copy for D2D1_INTERPOLATION_MODE_DEFINITION {}
impl ::core::clone::Clone for D2D1_INTERPOLATION_MODE_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_INTERPOLATION_MODE_DEFINITION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_LAYER_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS_NONE: D2D1_LAYER_OPTIONS = D2D1_LAYER_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS_INITIALIZE_FOR_CLEARTYPE: D2D1_LAYER_OPTIONS = D2D1_LAYER_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS_FORCE_DWORD: D2D1_LAYER_OPTIONS = D2D1_LAYER_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_LAYER_OPTIONS {}
impl ::core::clone::Clone for D2D1_LAYER_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_LAYER_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_LAYER_OPTIONS1(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_NONE: D2D1_LAYER_OPTIONS1 = D2D1_LAYER_OPTIONS1(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_INITIALIZE_FROM_BACKGROUND: D2D1_LAYER_OPTIONS1 = D2D1_LAYER_OPTIONS1(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_IGNORE_ALPHA: D2D1_LAYER_OPTIONS1 = D2D1_LAYER_OPTIONS1(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_FORCE_DWORD: D2D1_LAYER_OPTIONS1 = D2D1_LAYER_OPTIONS1(4294967295u32);
impl ::core::marker::Copy for D2D1_LAYER_OPTIONS1 {}
impl ::core::clone::Clone for D2D1_LAYER_OPTIONS1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_LAYER_OPTIONS1 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_LINEARTRANSFER_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_RED_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_RED_SLOPE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_RED_DISABLE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_GREEN_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_GREEN_SLOPE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_GREEN_DISABLE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_BLUE_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_BLUE_SLOPE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_BLUE_DISABLE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_SLOPE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_DISABLE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(11u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_CLAMP_OUTPUT: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(12u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_FORCE_DWORD: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_LINEARTRANSFER_PROP {}
impl ::core::clone::Clone for D2D1_LINEARTRANSFER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_LINEARTRANSFER_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_LINE_JOIN(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_MITER: D2D1_LINE_JOIN = D2D1_LINE_JOIN(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_BEVEL: D2D1_LINE_JOIN = D2D1_LINE_JOIN(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_ROUND: D2D1_LINE_JOIN = D2D1_LINE_JOIN(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_MITER_OR_BEVEL: D2D1_LINE_JOIN = D2D1_LINE_JOIN(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_FORCE_DWORD: D2D1_LINE_JOIN = D2D1_LINE_JOIN(4294967295u32);
impl ::core::marker::Copy for D2D1_LINE_JOIN {}
impl ::core::clone::Clone for D2D1_LINE_JOIN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_LINE_JOIN {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_LOOKUPTABLE3D_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LOOKUPTABLE3D_PROP_LUT: D2D1_LOOKUPTABLE3D_PROP = D2D1_LOOKUPTABLE3D_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LOOKUPTABLE3D_PROP_ALPHA_MODE: D2D1_LOOKUPTABLE3D_PROP = D2D1_LOOKUPTABLE3D_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LOOKUPTABLE3D_PROP_FORCE_DWORD: D2D1_LOOKUPTABLE3D_PROP = D2D1_LOOKUPTABLE3D_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_LOOKUPTABLE3D_PROP {}
impl ::core::clone::Clone for D2D1_LOOKUPTABLE3D_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_LOOKUPTABLE3D_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_MAP_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_NONE: D2D1_MAP_OPTIONS = D2D1_MAP_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_READ: D2D1_MAP_OPTIONS = D2D1_MAP_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_WRITE: D2D1_MAP_OPTIONS = D2D1_MAP_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_DISCARD: D2D1_MAP_OPTIONS = D2D1_MAP_OPTIONS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_FORCE_DWORD: D2D1_MAP_OPTIONS = D2D1_MAP_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_MAP_OPTIONS {}
impl ::core::clone::Clone for D2D1_MAP_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_MAP_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_MORPHOLOGY_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_MODE_ERODE: D2D1_MORPHOLOGY_MODE = D2D1_MORPHOLOGY_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_MODE_DILATE: D2D1_MORPHOLOGY_MODE = D2D1_MORPHOLOGY_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_MODE_FORCE_DWORD: D2D1_MORPHOLOGY_MODE = D2D1_MORPHOLOGY_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_MORPHOLOGY_MODE {}
impl ::core::clone::Clone for D2D1_MORPHOLOGY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_MORPHOLOGY_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_MORPHOLOGY_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_MODE: D2D1_MORPHOLOGY_PROP = D2D1_MORPHOLOGY_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_WIDTH: D2D1_MORPHOLOGY_PROP = D2D1_MORPHOLOGY_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_HEIGHT: D2D1_MORPHOLOGY_PROP = D2D1_MORPHOLOGY_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_FORCE_DWORD: D2D1_MORPHOLOGY_PROP = D2D1_MORPHOLOGY_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_MORPHOLOGY_PROP {}
impl ::core::clone::Clone for D2D1_MORPHOLOGY_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_MORPHOLOGY_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_OPACITYMETADATA_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITYMETADATA_PROP_INPUT_OPAQUE_RECT: D2D1_OPACITYMETADATA_PROP = D2D1_OPACITYMETADATA_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITYMETADATA_PROP_FORCE_DWORD: D2D1_OPACITYMETADATA_PROP = D2D1_OPACITYMETADATA_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_OPACITYMETADATA_PROP {}
impl ::core::clone::Clone for D2D1_OPACITYMETADATA_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_OPACITYMETADATA_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_OPACITY_MASK_CONTENT(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_GRAPHICS: D2D1_OPACITY_MASK_CONTENT = D2D1_OPACITY_MASK_CONTENT(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_TEXT_NATURAL: D2D1_OPACITY_MASK_CONTENT = D2D1_OPACITY_MASK_CONTENT(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_TEXT_GDI_COMPATIBLE: D2D1_OPACITY_MASK_CONTENT = D2D1_OPACITY_MASK_CONTENT(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_FORCE_DWORD: D2D1_OPACITY_MASK_CONTENT = D2D1_OPACITY_MASK_CONTENT(4294967295u32);
impl ::core::marker::Copy for D2D1_OPACITY_MASK_CONTENT {}
impl ::core::clone::Clone for D2D1_OPACITY_MASK_CONTENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_OPACITY_MASK_CONTENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_OPACITY_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_PROP_OPACITY: D2D1_OPACITY_PROP = D2D1_OPACITY_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_PROP_FORCE_DWORD: D2D1_OPACITY_PROP = D2D1_OPACITY_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_OPACITY_PROP {}
impl ::core::clone::Clone for D2D1_OPACITY_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_OPACITY_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_ORIENTATION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_DEFAULT: D2D1_ORIENTATION = D2D1_ORIENTATION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_FLIP_HORIZONTAL: D2D1_ORIENTATION = D2D1_ORIENTATION(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180: D2D1_ORIENTATION = D2D1_ORIENTATION(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL: D2D1_ORIENTATION = D2D1_ORIENTATION(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL: D2D1_ORIENTATION = D2D1_ORIENTATION(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270: D2D1_ORIENTATION = D2D1_ORIENTATION(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL: D2D1_ORIENTATION = D2D1_ORIENTATION(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90: D2D1_ORIENTATION = D2D1_ORIENTATION(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_FORCE_DWORD: D2D1_ORIENTATION = D2D1_ORIENTATION(4294967295u32);
impl ::core::marker::Copy for D2D1_ORIENTATION {}
impl ::core::clone::Clone for D2D1_ORIENTATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_ORIENTATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_PATCH_EDGE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_ALIASED: D2D1_PATCH_EDGE_MODE = D2D1_PATCH_EDGE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_ANTIALIASED: D2D1_PATCH_EDGE_MODE = D2D1_PATCH_EDGE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_ALIASED_INFLATED: D2D1_PATCH_EDGE_MODE = D2D1_PATCH_EDGE_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_FORCE_DWORD: D2D1_PATCH_EDGE_MODE = D2D1_PATCH_EDGE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_PATCH_EDGE_MODE {}
impl ::core::clone::Clone for D2D1_PATCH_EDGE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_PATCH_EDGE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_PIXEL_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PIXEL_OPTIONS_NONE: D2D1_PIXEL_OPTIONS = D2D1_PIXEL_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PIXEL_OPTIONS_TRIVIAL_SAMPLING: D2D1_PIXEL_OPTIONS = D2D1_PIXEL_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PIXEL_OPTIONS_FORCE_DWORD: D2D1_PIXEL_OPTIONS = D2D1_PIXEL_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_PIXEL_OPTIONS {}
impl ::core::clone::Clone for D2D1_PIXEL_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_PIXEL_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_POINTDIFFUSE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_LIGHT_POSITION: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_SURFACE_SCALE: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_COLOR: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_SCALE_MODE: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_FORCE_DWORD: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_POINTDIFFUSE_PROP {}
impl ::core::clone::Clone for D2D1_POINTDIFFUSE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_POINTDIFFUSE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_POINTDIFFUSE_SCALE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_LINEAR: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_CUBIC: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_FORCE_DWORD: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_POINTDIFFUSE_SCALE_MODE {}
impl ::core::clone::Clone for D2D1_POINTDIFFUSE_SCALE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_POINTDIFFUSE_SCALE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_POINTSPECULAR_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_LIGHT_POSITION: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SURFACE_SCALE: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_COLOR: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SCALE_MODE: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_FORCE_DWORD: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_POINTSPECULAR_PROP {}
impl ::core::clone::Clone for D2D1_POINTSPECULAR_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_POINTSPECULAR_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_POINTSPECULAR_SCALE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_LINEAR: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_CUBIC: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_FORCE_DWORD: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_POINTSPECULAR_SCALE_MODE {}
impl ::core::clone::Clone for D2D1_POINTSPECULAR_SCALE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_POINTSPECULAR_SCALE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_POSTERIZE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_RED_VALUE_COUNT: D2D1_POSTERIZE_PROP = D2D1_POSTERIZE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_GREEN_VALUE_COUNT: D2D1_POSTERIZE_PROP = D2D1_POSTERIZE_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_BLUE_VALUE_COUNT: D2D1_POSTERIZE_PROP = D2D1_POSTERIZE_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_FORCE_DWORD: D2D1_POSTERIZE_PROP = D2D1_POSTERIZE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_POSTERIZE_PROP {}
impl ::core::clone::Clone for D2D1_POSTERIZE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_POSTERIZE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_PRESENT_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_NONE: D2D1_PRESENT_OPTIONS = D2D1_PRESENT_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS: D2D1_PRESENT_OPTIONS = D2D1_PRESENT_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_IMMEDIATELY: D2D1_PRESENT_OPTIONS = D2D1_PRESENT_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_FORCE_DWORD: D2D1_PRESENT_OPTIONS = D2D1_PRESENT_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_PRESENT_OPTIONS {}
impl ::core::clone::Clone for D2D1_PRESENT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_PRESENT_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_PRIMITIVE_BLEND(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_SOURCE_OVER: D2D1_PRIMITIVE_BLEND = D2D1_PRIMITIVE_BLEND(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_COPY: D2D1_PRIMITIVE_BLEND = D2D1_PRIMITIVE_BLEND(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_MIN: D2D1_PRIMITIVE_BLEND = D2D1_PRIMITIVE_BLEND(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_ADD: D2D1_PRIMITIVE_BLEND = D2D1_PRIMITIVE_BLEND(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_MAX: D2D1_PRIMITIVE_BLEND = D2D1_PRIMITIVE_BLEND(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_FORCE_DWORD: D2D1_PRIMITIVE_BLEND = D2D1_PRIMITIVE_BLEND(4294967295u32);
impl ::core::marker::Copy for D2D1_PRIMITIVE_BLEND {}
impl ::core::clone::Clone for D2D1_PRIMITIVE_BLEND {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_PRIMITIVE_BLEND {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_PRINT_FONT_SUBSET_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_DEFAULT: D2D1_PRINT_FONT_SUBSET_MODE = D2D1_PRINT_FONT_SUBSET_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_EACHPAGE: D2D1_PRINT_FONT_SUBSET_MODE = D2D1_PRINT_FONT_SUBSET_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_NONE: D2D1_PRINT_FONT_SUBSET_MODE = D2D1_PRINT_FONT_SUBSET_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_FORCE_DWORD: D2D1_PRINT_FONT_SUBSET_MODE = D2D1_PRINT_FONT_SUBSET_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_PRINT_FONT_SUBSET_MODE {}
impl ::core::clone::Clone for D2D1_PRINT_FONT_SUBSET_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_PRINT_FONT_SUBSET_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_PROPERTY(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_CLSID: D2D1_PROPERTY = D2D1_PROPERTY(2147483648u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_DISPLAYNAME: D2D1_PROPERTY = D2D1_PROPERTY(2147483649u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_AUTHOR: D2D1_PROPERTY = D2D1_PROPERTY(2147483650u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_CATEGORY: D2D1_PROPERTY = D2D1_PROPERTY(2147483651u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_DESCRIPTION: D2D1_PROPERTY = D2D1_PROPERTY(2147483652u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_INPUTS: D2D1_PROPERTY = D2D1_PROPERTY(2147483653u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_CACHED: D2D1_PROPERTY = D2D1_PROPERTY(2147483654u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_PRECISION: D2D1_PROPERTY = D2D1_PROPERTY(2147483655u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_MIN_INPUTS: D2D1_PROPERTY = D2D1_PROPERTY(2147483656u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_MAX_INPUTS: D2D1_PROPERTY = D2D1_PROPERTY(2147483657u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_FORCE_DWORD: D2D1_PROPERTY = D2D1_PROPERTY(4294967295u32);
impl ::core::marker::Copy for D2D1_PROPERTY {}
impl ::core::clone::Clone for D2D1_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_PROPERTY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_PROPERTY_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_UNKNOWN: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_STRING: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_BOOL: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_UINT32: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_INT32: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_FLOAT: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_VECTOR2: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_VECTOR3: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_VECTOR4: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_BLOB: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_IUNKNOWN: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_ENUM: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(11u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_ARRAY: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(12u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_CLSID: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(13u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_3X2: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(14u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_4X3: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(15u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_4X4: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_5X4: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(17u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_COLOR_CONTEXT: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(18u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_FORCE_DWORD: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(4294967295u32);
impl ::core::marker::Copy for D2D1_PROPERTY_TYPE {}
impl ::core::clone::Clone for D2D1_PROPERTY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_PROPERTY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_RENDERING_PRIORITY(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDERING_PRIORITY_NORMAL: D2D1_RENDERING_PRIORITY = D2D1_RENDERING_PRIORITY(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDERING_PRIORITY_LOW: D2D1_RENDERING_PRIORITY = D2D1_RENDERING_PRIORITY(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDERING_PRIORITY_FORCE_DWORD: D2D1_RENDERING_PRIORITY = D2D1_RENDERING_PRIORITY(4294967295u32);
impl ::core::marker::Copy for D2D1_RENDERING_PRIORITY {}
impl ::core::clone::Clone for D2D1_RENDERING_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_RENDERING_PRIORITY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_RENDER_TARGET_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_DEFAULT: D2D1_RENDER_TARGET_TYPE = D2D1_RENDER_TARGET_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_SOFTWARE: D2D1_RENDER_TARGET_TYPE = D2D1_RENDER_TARGET_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_HARDWARE: D2D1_RENDER_TARGET_TYPE = D2D1_RENDER_TARGET_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_FORCE_DWORD: D2D1_RENDER_TARGET_TYPE = D2D1_RENDER_TARGET_TYPE(4294967295u32);
impl ::core::marker::Copy for D2D1_RENDER_TARGET_TYPE {}
impl ::core::clone::Clone for D2D1_RENDER_TARGET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_RENDER_TARGET_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_RENDER_TARGET_USAGE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_NONE: D2D1_RENDER_TARGET_USAGE = D2D1_RENDER_TARGET_USAGE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_FORCE_BITMAP_REMOTING: D2D1_RENDER_TARGET_USAGE = D2D1_RENDER_TARGET_USAGE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE: D2D1_RENDER_TARGET_USAGE = D2D1_RENDER_TARGET_USAGE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_FORCE_DWORD: D2D1_RENDER_TARGET_USAGE = D2D1_RENDER_TARGET_USAGE(4294967295u32);
impl ::core::marker::Copy for D2D1_RENDER_TARGET_USAGE {}
impl ::core::clone::Clone for D2D1_RENDER_TARGET_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_RENDER_TARGET_USAGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_VALUE: D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS: D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_FORCE_DWORD: D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE(4294967295u32);
impl ::core::marker::Copy for D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE {}
impl ::core::clone::Clone for D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_RGBTOHUE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_PROP_OUTPUT_COLOR_SPACE: D2D1_RGBTOHUE_PROP = D2D1_RGBTOHUE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_PROP_FORCE_DWORD: D2D1_RGBTOHUE_PROP = D2D1_RGBTOHUE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_RGBTOHUE_PROP {}
impl ::core::clone::Clone for D2D1_RGBTOHUE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_RGBTOHUE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SATURATION_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SATURATION_PROP_SATURATION: D2D1_SATURATION_PROP = D2D1_SATURATION_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SATURATION_PROP_FORCE_DWORD: D2D1_SATURATION_PROP = D2D1_SATURATION_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_SATURATION_PROP {}
impl ::core::clone::Clone for D2D1_SATURATION_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SATURATION_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SCALE_INTERPOLATION_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_LINEAR: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_CUBIC: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_ANISOTROPIC: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_FORCE_DWORD: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_SCALE_INTERPOLATION_MODE {}
impl ::core::clone::Clone for D2D1_SCALE_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SCALE_INTERPOLATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SCALE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_SCALE: D2D1_SCALE_PROP = D2D1_SCALE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_CENTER_POINT: D2D1_SCALE_PROP = D2D1_SCALE_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_INTERPOLATION_MODE: D2D1_SCALE_PROP = D2D1_SCALE_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_BORDER_MODE: D2D1_SCALE_PROP = D2D1_SCALE_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_SHARPNESS: D2D1_SCALE_PROP = D2D1_SCALE_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_FORCE_DWORD: D2D1_SCALE_PROP = D2D1_SCALE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_SCALE_PROP {}
impl ::core::clone::Clone for D2D1_SCALE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SCALE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SEPIA_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SEPIA_PROP_INTENSITY: D2D1_SEPIA_PROP = D2D1_SEPIA_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SEPIA_PROP_ALPHA_MODE: D2D1_SEPIA_PROP = D2D1_SEPIA_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SEPIA_PROP_FORCE_DWORD: D2D1_SEPIA_PROP = D2D1_SEPIA_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_SEPIA_PROP {}
impl ::core::clone::Clone for D2D1_SEPIA_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SEPIA_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SHADOW_OPTIMIZATION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_SPEED: D2D1_SHADOW_OPTIMIZATION = D2D1_SHADOW_OPTIMIZATION(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_BALANCED: D2D1_SHADOW_OPTIMIZATION = D2D1_SHADOW_OPTIMIZATION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_QUALITY: D2D1_SHADOW_OPTIMIZATION = D2D1_SHADOW_OPTIMIZATION(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_FORCE_DWORD: D2D1_SHADOW_OPTIMIZATION = D2D1_SHADOW_OPTIMIZATION(4294967295u32);
impl ::core::marker::Copy for D2D1_SHADOW_OPTIMIZATION {}
impl ::core::clone::Clone for D2D1_SHADOW_OPTIMIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SHADOW_OPTIMIZATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SHADOW_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_BLUR_STANDARD_DEVIATION: D2D1_SHADOW_PROP = D2D1_SHADOW_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_COLOR: D2D1_SHADOW_PROP = D2D1_SHADOW_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_OPTIMIZATION: D2D1_SHADOW_PROP = D2D1_SHADOW_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_FORCE_DWORD: D2D1_SHADOW_PROP = D2D1_SHADOW_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_SHADOW_PROP {}
impl ::core::clone::Clone for D2D1_SHADOW_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SHADOW_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SHARPEN_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHARPEN_PROP_SHARPNESS: D2D1_SHARPEN_PROP = D2D1_SHARPEN_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHARPEN_PROP_THRESHOLD: D2D1_SHARPEN_PROP = D2D1_SHARPEN_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHARPEN_PROP_FORCE_DWORD: D2D1_SHARPEN_PROP = D2D1_SHARPEN_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_SHARPEN_PROP {}
impl ::core::clone::Clone for D2D1_SHARPEN_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SHARPEN_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SPOTDIFFUSE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_LIGHT_POSITION: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_POINTS_AT: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_FOCUS: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_LIMITING_CONE_ANGLE: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_SURFACE_SCALE: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_COLOR: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_SCALE_MODE: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_FORCE_DWORD: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_SPOTDIFFUSE_PROP {}
impl ::core::clone::Clone for D2D1_SPOTDIFFUSE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SPOTDIFFUSE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SPOTDIFFUSE_SCALE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_LINEAR: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_CUBIC: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_FORCE_DWORD: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_SPOTDIFFUSE_SCALE_MODE {}
impl ::core::clone::Clone for D2D1_SPOTDIFFUSE_SCALE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SPOTDIFFUSE_SCALE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SPOTSPECULAR_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_LIGHT_POSITION: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_POINTS_AT: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_FOCUS: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_LIMITING_CONE_ANGLE: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SURFACE_SCALE: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_COLOR: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SCALE_MODE: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_FORCE_DWORD: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_SPOTSPECULAR_PROP {}
impl ::core::clone::Clone for D2D1_SPOTSPECULAR_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SPOTSPECULAR_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SPOTSPECULAR_SCALE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_LINEAR: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_CUBIC: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_FORCE_DWORD: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_SPOTSPECULAR_SCALE_MODE {}
impl ::core::clone::Clone for D2D1_SPOTSPECULAR_SCALE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SPOTSPECULAR_SCALE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SPRITE_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPRITE_OPTIONS_NONE: D2D1_SPRITE_OPTIONS = D2D1_SPRITE_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPRITE_OPTIONS_CLAMP_TO_SOURCE_RECTANGLE: D2D1_SPRITE_OPTIONS = D2D1_SPRITE_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPRITE_OPTIONS_FORCE_DWORD: D2D1_SPRITE_OPTIONS = D2D1_SPRITE_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_SPRITE_OPTIONS {}
impl ::core::clone::Clone for D2D1_SPRITE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SPRITE_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_STRAIGHTEN_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_ANGLE: D2D1_STRAIGHTEN_PROP = D2D1_STRAIGHTEN_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_MAINTAIN_SIZE: D2D1_STRAIGHTEN_PROP = D2D1_STRAIGHTEN_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_SCALE_MODE: D2D1_STRAIGHTEN_PROP = D2D1_STRAIGHTEN_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_FORCE_DWORD: D2D1_STRAIGHTEN_PROP = D2D1_STRAIGHTEN_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_STRAIGHTEN_PROP {}
impl ::core::clone::Clone for D2D1_STRAIGHTEN_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_STRAIGHTEN_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_STRAIGHTEN_SCALE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_STRAIGHTEN_SCALE_MODE = D2D1_STRAIGHTEN_SCALE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_LINEAR: D2D1_STRAIGHTEN_SCALE_MODE = D2D1_STRAIGHTEN_SCALE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_CUBIC: D2D1_STRAIGHTEN_SCALE_MODE = D2D1_STRAIGHTEN_SCALE_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_STRAIGHTEN_SCALE_MODE = D2D1_STRAIGHTEN_SCALE_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_ANISOTROPIC: D2D1_STRAIGHTEN_SCALE_MODE = D2D1_STRAIGHTEN_SCALE_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_FORCE_DWORD: D2D1_STRAIGHTEN_SCALE_MODE = D2D1_STRAIGHTEN_SCALE_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_STRAIGHTEN_SCALE_MODE {}
impl ::core::clone::Clone for D2D1_STRAIGHTEN_SCALE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_STRAIGHTEN_SCALE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_STROKE_TRANSFORM_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_NORMAL: D2D1_STROKE_TRANSFORM_TYPE = D2D1_STROKE_TRANSFORM_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_FIXED: D2D1_STROKE_TRANSFORM_TYPE = D2D1_STROKE_TRANSFORM_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_HAIRLINE: D2D1_STROKE_TRANSFORM_TYPE = D2D1_STROKE_TRANSFORM_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_FORCE_DWORD: D2D1_STROKE_TRANSFORM_TYPE = D2D1_STROKE_TRANSFORM_TYPE(4294967295u32);
impl ::core::marker::Copy for D2D1_STROKE_TRANSFORM_TYPE {}
impl ::core::clone::Clone for D2D1_STROKE_TRANSFORM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_STROKE_TRANSFORM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SUBPROPERTY(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_DISPLAYNAME: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(2147483648u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_ISREADONLY: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(2147483649u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_MIN: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(2147483650u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_MAX: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(2147483651u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_DEFAULT: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(2147483652u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_FIELDS: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(2147483653u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_INDEX: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(2147483654u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_FORCE_DWORD: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(4294967295u32);
impl ::core::marker::Copy for D2D1_SUBPROPERTY {}
impl ::core::clone::Clone for D2D1_SUBPROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SUBPROPERTY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_ASPECT_ALIGN(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_NONE: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MIN: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MIN: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MIN: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MID: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MID: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MID: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MAX: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MAX: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MAX: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_FORCE_DWORD: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_ASPECT_ALIGN {}
impl ::core::clone::Clone for D2D1_SVG_ASPECT_ALIGN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_ASPECT_ALIGN {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_ASPECT_SCALING(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_SCALING_MEET: D2D1_SVG_ASPECT_SCALING = D2D1_SVG_ASPECT_SCALING(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_SCALING_SLICE: D2D1_SVG_ASPECT_SCALING = D2D1_SVG_ASPECT_SCALING(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_SCALING_FORCE_DWORD: D2D1_SVG_ASPECT_SCALING = D2D1_SVG_ASPECT_SCALING(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_ASPECT_SCALING {}
impl ::core::clone::Clone for D2D1_SVG_ASPECT_SCALING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_ASPECT_SCALING {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_ATTRIBUTE_POD_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FLOAT: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_COLOR: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FILL_MODE: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_DISPLAY: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_OVERFLOW: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_CAP: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_JOIN: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_VISIBILITY: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_MATRIX: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_UNIT_TYPE: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_EXTEND_MODE: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_PRESERVE_ASPECT_RATIO: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(11u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_VIEWBOX: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(12u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LENGTH: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(13u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FORCE_DWORD: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_ATTRIBUTE_POD_TYPE {}
impl ::core::clone::Clone for D2D1_SVG_ATTRIBUTE_POD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_ATTRIBUTE_POD_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_ATTRIBUTE_STRING_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_SVG: D2D1_SVG_ATTRIBUTE_STRING_TYPE = D2D1_SVG_ATTRIBUTE_STRING_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_ID: D2D1_SVG_ATTRIBUTE_STRING_TYPE = D2D1_SVG_ATTRIBUTE_STRING_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_FORCE_DWORD: D2D1_SVG_ATTRIBUTE_STRING_TYPE = D2D1_SVG_ATTRIBUTE_STRING_TYPE(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_ATTRIBUTE_STRING_TYPE {}
impl ::core::clone::Clone for D2D1_SVG_ATTRIBUTE_STRING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_ATTRIBUTE_STRING_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_DISPLAY(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_DISPLAY_INLINE: D2D1_SVG_DISPLAY = D2D1_SVG_DISPLAY(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_DISPLAY_NONE: D2D1_SVG_DISPLAY = D2D1_SVG_DISPLAY(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_DISPLAY_FORCE_DWORD: D2D1_SVG_DISPLAY = D2D1_SVG_DISPLAY(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_DISPLAY {}
impl ::core::clone::Clone for D2D1_SVG_DISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_DISPLAY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_LENGTH_UNITS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LENGTH_UNITS_NUMBER: D2D1_SVG_LENGTH_UNITS = D2D1_SVG_LENGTH_UNITS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LENGTH_UNITS_PERCENTAGE: D2D1_SVG_LENGTH_UNITS = D2D1_SVG_LENGTH_UNITS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LENGTH_UNITS_FORCE_DWORD: D2D1_SVG_LENGTH_UNITS = D2D1_SVG_LENGTH_UNITS(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_LENGTH_UNITS {}
impl ::core::clone::Clone for D2D1_SVG_LENGTH_UNITS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_LENGTH_UNITS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_LINE_CAP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_BUTT: D2D1_SVG_LINE_CAP = D2D1_SVG_LINE_CAP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_SQUARE: D2D1_SVG_LINE_CAP = D2D1_SVG_LINE_CAP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_ROUND: D2D1_SVG_LINE_CAP = D2D1_SVG_LINE_CAP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_FORCE_DWORD: D2D1_SVG_LINE_CAP = D2D1_SVG_LINE_CAP(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_LINE_CAP {}
impl ::core::clone::Clone for D2D1_SVG_LINE_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_LINE_CAP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_LINE_JOIN(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_BEVEL: D2D1_SVG_LINE_JOIN = D2D1_SVG_LINE_JOIN(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_MITER: D2D1_SVG_LINE_JOIN = D2D1_SVG_LINE_JOIN(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_ROUND: D2D1_SVG_LINE_JOIN = D2D1_SVG_LINE_JOIN(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_FORCE_DWORD: D2D1_SVG_LINE_JOIN = D2D1_SVG_LINE_JOIN(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_LINE_JOIN {}
impl ::core::clone::Clone for D2D1_SVG_LINE_JOIN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_LINE_JOIN {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_OVERFLOW(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_OVERFLOW_VISIBLE: D2D1_SVG_OVERFLOW = D2D1_SVG_OVERFLOW(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_OVERFLOW_HIDDEN: D2D1_SVG_OVERFLOW = D2D1_SVG_OVERFLOW(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_OVERFLOW_FORCE_DWORD: D2D1_SVG_OVERFLOW = D2D1_SVG_OVERFLOW(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_OVERFLOW {}
impl ::core::clone::Clone for D2D1_SVG_OVERFLOW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_OVERFLOW {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_PAINT_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_NONE: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_COLOR: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_CURRENT_COLOR: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI_NONE: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI_COLOR: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI_CURRENT_COLOR: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_FORCE_DWORD: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_PAINT_TYPE {}
impl ::core::clone::Clone for D2D1_SVG_PAINT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_PAINT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_PATH_COMMAND(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CLOSE_PATH: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_MOVE_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_MOVE_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_LINE_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_LINE_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_ARC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_ARC_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_HORIZONTAL_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(11u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_HORIZONTAL_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(12u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_VERTICAL_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(13u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_VERTICAL_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(14u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(15u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(17u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(18u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_FORCE_DWORD: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_PATH_COMMAND {}
impl ::core::clone::Clone for D2D1_SVG_PATH_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_PATH_COMMAND {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_UNIT_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_UNIT_TYPE_USER_SPACE_ON_USE: D2D1_SVG_UNIT_TYPE = D2D1_SVG_UNIT_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_UNIT_TYPE_OBJECT_BOUNDING_BOX: D2D1_SVG_UNIT_TYPE = D2D1_SVG_UNIT_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_UNIT_TYPE_FORCE_DWORD: D2D1_SVG_UNIT_TYPE = D2D1_SVG_UNIT_TYPE(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_UNIT_TYPE {}
impl ::core::clone::Clone for D2D1_SVG_UNIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_UNIT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SVG_VISIBILITY(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_VISIBILITY_VISIBLE: D2D1_SVG_VISIBILITY = D2D1_SVG_VISIBILITY(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_VISIBILITY_HIDDEN: D2D1_SVG_VISIBILITY = D2D1_SVG_VISIBILITY(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_VISIBILITY_FORCE_DWORD: D2D1_SVG_VISIBILITY = D2D1_SVG_VISIBILITY(4294967295u32);
impl ::core::marker::Copy for D2D1_SVG_VISIBILITY {}
impl ::core::clone::Clone for D2D1_SVG_VISIBILITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_VISIBILITY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_SWEEP_DIRECTION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE: D2D1_SWEEP_DIRECTION = D2D1_SWEEP_DIRECTION(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SWEEP_DIRECTION_CLOCKWISE: D2D1_SWEEP_DIRECTION = D2D1_SWEEP_DIRECTION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SWEEP_DIRECTION_FORCE_DWORD: D2D1_SWEEP_DIRECTION = D2D1_SWEEP_DIRECTION(4294967295u32);
impl ::core::marker::Copy for D2D1_SWEEP_DIRECTION {}
impl ::core::clone::Clone for D2D1_SWEEP_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SWEEP_DIRECTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_TABLETRANSFER_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_RED_TABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_RED_DISABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_GREEN_TABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_GREEN_DISABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_BLUE_TABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_BLUE_DISABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_ALPHA_TABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_ALPHA_DISABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_CLAMP_OUTPUT: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_FORCE_DWORD: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_TABLETRANSFER_PROP {}
impl ::core::clone::Clone for D2D1_TABLETRANSFER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_TABLETRANSFER_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_TEMPERATUREANDTINT_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEMPERATUREANDTINT_PROP_TEMPERATURE: D2D1_TEMPERATUREANDTINT_PROP = D2D1_TEMPERATUREANDTINT_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEMPERATUREANDTINT_PROP_TINT: D2D1_TEMPERATUREANDTINT_PROP = D2D1_TEMPERATUREANDTINT_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEMPERATUREANDTINT_PROP_FORCE_DWORD: D2D1_TEMPERATUREANDTINT_PROP = D2D1_TEMPERATUREANDTINT_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_TEMPERATUREANDTINT_PROP {}
impl ::core::clone::Clone for D2D1_TEMPERATUREANDTINT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_TEMPERATUREANDTINT_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_TEXT_ANTIALIAS_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_DEFAULT: D2D1_TEXT_ANTIALIAS_MODE = D2D1_TEXT_ANTIALIAS_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE: D2D1_TEXT_ANTIALIAS_MODE = D2D1_TEXT_ANTIALIAS_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE: D2D1_TEXT_ANTIALIAS_MODE = D2D1_TEXT_ANTIALIAS_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_ALIASED: D2D1_TEXT_ANTIALIAS_MODE = D2D1_TEXT_ANTIALIAS_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_FORCE_DWORD: D2D1_TEXT_ANTIALIAS_MODE = D2D1_TEXT_ANTIALIAS_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_TEXT_ANTIALIAS_MODE {}
impl ::core::clone::Clone for D2D1_TEXT_ANTIALIAS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_TEXT_ANTIALIAS_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_THREADING_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_THREADING_MODE_SINGLE_THREADED: D2D1_THREADING_MODE = D2D1_THREADING_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_THREADING_MODE_MULTI_THREADED: D2D1_THREADING_MODE = D2D1_THREADING_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_THREADING_MODE_FORCE_DWORD: D2D1_THREADING_MODE = D2D1_THREADING_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_THREADING_MODE {}
impl ::core::clone::Clone for D2D1_THREADING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_THREADING_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_TILE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TILE_PROP_RECT: D2D1_TILE_PROP = D2D1_TILE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TILE_PROP_FORCE_DWORD: D2D1_TILE_PROP = D2D1_TILE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_TILE_PROP {}
impl ::core::clone::Clone for D2D1_TILE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_TILE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_TINT_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TINT_PROP_COLOR: D2D1_TINT_PROP = D2D1_TINT_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TINT_PROP_CLAMP_OUTPUT: D2D1_TINT_PROP = D2D1_TINT_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TINT_PROP_FORCE_DWORD: D2D1_TINT_PROP = D2D1_TINT_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_TINT_PROP {}
impl ::core::clone::Clone for D2D1_TINT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_TINT_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_NONE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_DISABLE_DPI_SCALE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_FORCE_DWORD: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {}
impl ::core::clone::Clone for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_TURBULENCE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_OFFSET: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_SIZE: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_BASE_FREQUENCY: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_NUM_OCTAVES: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_SEED: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_NOISE: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_STITCHABLE: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_FORCE_DWORD: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_TURBULENCE_PROP {}
impl ::core::clone::Clone for D2D1_TURBULENCE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_TURBULENCE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_UNIT_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_UNIT_MODE_DIPS: D2D1_UNIT_MODE = D2D1_UNIT_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_UNIT_MODE_PIXELS: D2D1_UNIT_MODE = D2D1_UNIT_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_UNIT_MODE_FORCE_DWORD: D2D1_UNIT_MODE = D2D1_UNIT_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_UNIT_MODE {}
impl ::core::clone::Clone for D2D1_UNIT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_UNIT_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_VERTEX_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_NONE: D2D1_VERTEX_OPTIONS = D2D1_VERTEX_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_DO_NOT_CLEAR: D2D1_VERTEX_OPTIONS = D2D1_VERTEX_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_USE_DEPTH_BUFFER: D2D1_VERTEX_OPTIONS = D2D1_VERTEX_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_ASSUME_NO_OVERLAP: D2D1_VERTEX_OPTIONS = D2D1_VERTEX_OPTIONS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_FORCE_DWORD: D2D1_VERTEX_OPTIONS = D2D1_VERTEX_OPTIONS(4294967295u32);
impl ::core::marker::Copy for D2D1_VERTEX_OPTIONS {}
impl ::core::clone::Clone for D2D1_VERTEX_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_VERTEX_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_VERTEX_USAGE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_USAGE_STATIC: D2D1_VERTEX_USAGE = D2D1_VERTEX_USAGE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_USAGE_DYNAMIC: D2D1_VERTEX_USAGE = D2D1_VERTEX_USAGE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_USAGE_FORCE_DWORD: D2D1_VERTEX_USAGE = D2D1_VERTEX_USAGE(4294967295u32);
impl ::core::marker::Copy for D2D1_VERTEX_USAGE {}
impl ::core::clone::Clone for D2D1_VERTEX_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_VERTEX_USAGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_VIGNETTE_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_COLOR: D2D1_VIGNETTE_PROP = D2D1_VIGNETTE_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_TRANSITION_SIZE: D2D1_VIGNETTE_PROP = D2D1_VIGNETTE_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_STRENGTH: D2D1_VIGNETTE_PROP = D2D1_VIGNETTE_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_FORCE_DWORD: D2D1_VIGNETTE_PROP = D2D1_VIGNETTE_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_VIGNETTE_PROP {}
impl ::core::clone::Clone for D2D1_VIGNETTE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_VIGNETTE_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_WHITELEVELADJUSTMENT_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WHITELEVELADJUSTMENT_PROP_INPUT_WHITE_LEVEL: D2D1_WHITELEVELADJUSTMENT_PROP = D2D1_WHITELEVELADJUSTMENT_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WHITELEVELADJUSTMENT_PROP_OUTPUT_WHITE_LEVEL: D2D1_WHITELEVELADJUSTMENT_PROP = D2D1_WHITELEVELADJUSTMENT_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WHITELEVELADJUSTMENT_PROP_FORCE_DWORD: D2D1_WHITELEVELADJUSTMENT_PROP = D2D1_WHITELEVELADJUSTMENT_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_WHITELEVELADJUSTMENT_PROP {}
impl ::core::clone::Clone for D2D1_WHITELEVELADJUSTMENT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_WHITELEVELADJUSTMENT_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_WINDOW_STATE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WINDOW_STATE_NONE: D2D1_WINDOW_STATE = D2D1_WINDOW_STATE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WINDOW_STATE_OCCLUDED: D2D1_WINDOW_STATE = D2D1_WINDOW_STATE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WINDOW_STATE_FORCE_DWORD: D2D1_WINDOW_STATE = D2D1_WINDOW_STATE(4294967295u32);
impl ::core::marker::Copy for D2D1_WINDOW_STATE {}
impl ::core::clone::Clone for D2D1_WINDOW_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_WINDOW_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_YCBCR_CHROMA_SUBSAMPLING(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_AUTO: D2D1_YCBCR_CHROMA_SUBSAMPLING = D2D1_YCBCR_CHROMA_SUBSAMPLING(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_420: D2D1_YCBCR_CHROMA_SUBSAMPLING = D2D1_YCBCR_CHROMA_SUBSAMPLING(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_422: D2D1_YCBCR_CHROMA_SUBSAMPLING = D2D1_YCBCR_CHROMA_SUBSAMPLING(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_444: D2D1_YCBCR_CHROMA_SUBSAMPLING = D2D1_YCBCR_CHROMA_SUBSAMPLING(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_440: D2D1_YCBCR_CHROMA_SUBSAMPLING = D2D1_YCBCR_CHROMA_SUBSAMPLING(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_FORCE_DWORD: D2D1_YCBCR_CHROMA_SUBSAMPLING = D2D1_YCBCR_CHROMA_SUBSAMPLING(4294967295u32);
impl ::core::marker::Copy for D2D1_YCBCR_CHROMA_SUBSAMPLING {}
impl ::core::clone::Clone for D2D1_YCBCR_CHROMA_SUBSAMPLING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_YCBCR_CHROMA_SUBSAMPLING {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_YCBCR_INTERPOLATION_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_LINEAR: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_CUBIC: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_ANISOTROPIC: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_FORCE_DWORD: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(4294967295u32);
impl ::core::marker::Copy for D2D1_YCBCR_INTERPOLATION_MODE {}
impl ::core::clone::Clone for D2D1_YCBCR_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_YCBCR_INTERPOLATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D2D1_YCBCR_PROP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_CHROMA_SUBSAMPLING: D2D1_YCBCR_PROP = D2D1_YCBCR_PROP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_TRANSFORM_MATRIX: D2D1_YCBCR_PROP = D2D1_YCBCR_PROP(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_INTERPOLATION_MODE: D2D1_YCBCR_PROP = D2D1_YCBCR_PROP(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_FORCE_DWORD: D2D1_YCBCR_PROP = D2D1_YCBCR_PROP(4294967295u32);
impl ::core::marker::Copy for D2D1_YCBCR_PROP {}
impl ::core::clone::Clone for D2D1_YCBCR_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_YCBCR_PROP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ARC_SEGMENT {
    pub point: Common::D2D_POINT_2F,
    pub size: Common::D2D_SIZE_F,
    pub rotationAngle: f32,
    pub sweepDirection: D2D1_SWEEP_DIRECTION,
    pub arcSize: D2D1_ARC_SIZE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_ARC_SEGMENT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_ARC_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_ARC_SEGMENT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
}
impl ::core::marker::Copy for D2D1_BITMAP_BRUSH_PROPERTIES {}
impl ::core::clone::Clone for D2D1_BITMAP_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BITMAP_BRUSH_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES1 {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
impl ::core::marker::Copy for D2D1_BITMAP_BRUSH_PROPERTIES1 {}
impl ::core::clone::Clone for D2D1_BITMAP_BRUSH_PROPERTIES1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BITMAP_BRUSH_PROPERTIES1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_BITMAP_PROPERTIES {
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D2D1_BITMAP_PROPERTIES {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D2D1_BITMAP_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D2D1_BITMAP_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_BITMAP_PROPERTIES1 {
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub bitmapOptions: D2D1_BITMAP_OPTIONS,
    pub colorContext: ::windows::core::ManuallyDrop<ID2D1ColorContext>,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D2D1_BITMAP_PROPERTIES1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D2D1_BITMAP_PROPERTIES1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_BLEND_DESCRIPTION {
    pub sourceBlend: D2D1_BLEND,
    pub destinationBlend: D2D1_BLEND,
    pub blendOperation: D2D1_BLEND_OPERATION,
    pub sourceBlendAlpha: D2D1_BLEND,
    pub destinationBlendAlpha: D2D1_BLEND,
    pub blendOperationAlpha: D2D1_BLEND_OPERATION,
    pub blendFactor: [f32; 4],
}
impl ::core::marker::Copy for D2D1_BLEND_DESCRIPTION {}
impl ::core::clone::Clone for D2D1_BLEND_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_BLEND_DESCRIPTION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_BRUSH_PROPERTIES {
    pub opacity: f32,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_BRUSH_PROPERTIES {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for D2D1_BRUSH_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_CREATION_PROPERTIES {
    pub threadingMode: D2D1_THREADING_MODE,
    pub debugLevel: D2D1_DEBUG_LEVEL,
    pub options: D2D1_DEVICE_CONTEXT_OPTIONS,
}
impl ::core::marker::Copy for D2D1_CREATION_PROPERTIES {}
impl ::core::clone::Clone for D2D1_CREATION_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_CREATION_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    pub shaderBufferWithInputSignature: *const u8,
    pub shaderBufferSize: u32,
    pub inputElements: *const D2D1_INPUT_ELEMENT_DESC,
    pub elementCount: u32,
    pub stride: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_DRAWING_STATE_DESCRIPTION {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: u64,
    pub tag2: u64,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_DRAWING_STATE_DESCRIPTION {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_DRAWING_STATE_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for D2D1_DRAWING_STATE_DESCRIPTION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_DRAWING_STATE_DESCRIPTION1 {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: u64,
    pub tag2: u64,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub primitiveBlend: D2D1_PRIMITIVE_BLEND,
    pub unitMode: D2D1_UNIT_MODE,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_DRAWING_STATE_DESCRIPTION1 {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_DRAWING_STATE_DESCRIPTION1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for D2D1_DRAWING_STATE_DESCRIPTION1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_EFFECT_INPUT_DESCRIPTION {
    pub effect: ::windows::core::ManuallyDrop<ID2D1Effect>,
    pub inputIndex: u32,
    pub inputRectangle: Common::D2D_RECT_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_EFFECT_INPUT_DESCRIPTION {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_EFFECT_INPUT_DESCRIPTION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ELLIPSE {
    pub point: Common::D2D_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_ELLIPSE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_ELLIPSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_ELLIPSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_FACTORY_OPTIONS {
    pub debugLevel: D2D1_DEBUG_LEVEL,
}
impl ::core::marker::Copy for D2D1_FACTORY_OPTIONS {}
impl ::core::clone::Clone for D2D1_FACTORY_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_FACTORY_OPTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    pub computeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_FEATURE_DATA_DOUBLES {
    pub doublePrecisionFloatShaderOps: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D2D1_FEATURE_DATA_DOUBLES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D2D1_FEATURE_DATA_DOUBLES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D2D1_FEATURE_DATA_DOUBLES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_GRADIENT_MESH_PATCH {
    pub point00: Common::D2D_POINT_2F,
    pub point01: Common::D2D_POINT_2F,
    pub point02: Common::D2D_POINT_2F,
    pub point03: Common::D2D_POINT_2F,
    pub point10: Common::D2D_POINT_2F,
    pub point11: Common::D2D_POINT_2F,
    pub point12: Common::D2D_POINT_2F,
    pub point13: Common::D2D_POINT_2F,
    pub point20: Common::D2D_POINT_2F,
    pub point21: Common::D2D_POINT_2F,
    pub point22: Common::D2D_POINT_2F,
    pub point23: Common::D2D_POINT_2F,
    pub point30: Common::D2D_POINT_2F,
    pub point31: Common::D2D_POINT_2F,
    pub point32: Common::D2D_POINT_2F,
    pub point33: Common::D2D_POINT_2F,
    pub color00: Common::D2D1_COLOR_F,
    pub color03: Common::D2D1_COLOR_F,
    pub color30: Common::D2D1_COLOR_F,
    pub color33: Common::D2D1_COLOR_F,
    pub topEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub leftEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub bottomEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub rightEdgeMode: D2D1_PATCH_EDGE_MODE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_GRADIENT_MESH_PATCH {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_GRADIENT_MESH_PATCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_GRADIENT_MESH_PATCH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: Common::D2D1_COLOR_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_GRADIENT_STOP {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_GRADIENT_STOP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_GRADIENT_STOP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
    pub hwnd: super::super::Foundation::HWND,
    pub pixelSize: Common::D2D_SIZE_U,
    pub presentOptions: D2D1_PRESENT_OPTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for D2D1_HWND_RENDER_TARGET_PROPERTIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for D2D1_HWND_RENDER_TARGET_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
unsafe impl ::windows::core::Abi for D2D1_HWND_RENDER_TARGET_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_IMAGE_BRUSH_PROPERTIES {
    pub sourceRectangle: Common::D2D_RECT_F,
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_IMAGE_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_IMAGE_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_IMAGE_BRUSH_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_INK_BEZIER_SEGMENT {
    pub point1: D2D1_INK_POINT,
    pub point2: D2D1_INK_POINT,
    pub point3: D2D1_INK_POINT,
}
impl ::core::marker::Copy for D2D1_INK_BEZIER_SEGMENT {}
impl ::core::clone::Clone for D2D1_INK_BEZIER_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_INK_BEZIER_SEGMENT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_INK_POINT {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl ::core::marker::Copy for D2D1_INK_POINT {}
impl ::core::clone::Clone for D2D1_INK_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_INK_POINT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_INK_STYLE_PROPERTIES {
    pub nibShape: D2D1_INK_NIB_SHAPE,
    pub nibTransform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_INK_STYLE_PROPERTIES {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_INK_STYLE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for D2D1_INK_STYLE_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_INPUT_DESCRIPTION {
    pub filter: D2D1_FILTER,
    pub levelOfDetailCount: u32,
}
impl ::core::marker::Copy for D2D1_INPUT_DESCRIPTION {}
impl ::core::clone::Clone for D2D1_INPUT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_INPUT_DESCRIPTION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D2D1_INPUT_ELEMENT_DESC {
    pub semanticName: ::windows::core::PCSTR,
    pub semanticIndex: u32,
    pub format: super::Dxgi::Common::DXGI_FORMAT,
    pub inputSlot: u32,
    pub alignedByteOffset: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D2D1_INPUT_ELEMENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D2D1_INPUT_ELEMENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D2D1_INPUT_ELEMENT_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_LAYER_PARAMETERS {
    pub contentBounds: Common::D2D_RECT_F,
    pub geometricMask: ::windows::core::ManuallyDrop<ID2D1Geometry>,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: ::windows::core::ManuallyDrop<ID2D1Brush>,
    pub layerOptions: D2D1_LAYER_OPTIONS,
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for D2D1_LAYER_PARAMETERS {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
unsafe impl ::windows::core::Abi for D2D1_LAYER_PARAMETERS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_LAYER_PARAMETERS1 {
    pub contentBounds: Common::D2D_RECT_F,
    pub geometricMask: ::windows::core::ManuallyDrop<ID2D1Geometry>,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: ::windows::core::ManuallyDrop<ID2D1Brush>,
    pub layerOptions: D2D1_LAYER_OPTIONS1,
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for D2D1_LAYER_PARAMETERS1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
unsafe impl ::windows::core::Abi for D2D1_LAYER_PARAMETERS1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    pub startPoint: Common::D2D_POINT_2F,
    pub endPoint: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_MAPPED_RECT {
    pub pitch: u32,
    pub bits: *mut u8,
}
impl ::core::marker::Copy for D2D1_MAPPED_RECT {}
impl ::core::clone::Clone for D2D1_MAPPED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_MAPPED_RECT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_POINT_DESCRIPTION {
    pub point: Common::D2D_POINT_2F,
    pub unitTangentVector: Common::D2D_POINT_2F,
    pub endSegment: u32,
    pub endFigure: u32,
    pub lengthToEndSegment: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_POINT_DESCRIPTION {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_POINT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_POINT_DESCRIPTION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_PRINT_CONTROL_PROPERTIES {
    pub fontSubset: D2D1_PRINT_FONT_SUBSET_MODE,
    pub rasterDPI: f32,
    pub colorSpace: D2D1_COLOR_SPACE,
}
impl ::core::marker::Copy for D2D1_PRINT_CONTROL_PROPERTIES {}
impl ::core::clone::Clone for D2D1_PRINT_CONTROL_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_PRINT_CONTROL_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_PROPERTY_BINDING {
    pub propertyName: ::windows::core::PCWSTR,
    pub setFunction: PD2D1_PROPERTY_SET_FUNCTION,
    pub getFunction: PD2D1_PROPERTY_GET_FUNCTION,
}
impl ::core::marker::Copy for D2D1_PROPERTY_BINDING {}
impl ::core::clone::Clone for D2D1_PROPERTY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_PROPERTY_BINDING {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_QUADRATIC_BEZIER_SEGMENT {
    pub point1: Common::D2D_POINT_2F,
    pub point2: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_QUADRATIC_BEZIER_SEGMENT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_QUADRATIC_BEZIER_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_QUADRATIC_BEZIER_SEGMENT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    pub center: Common::D2D_POINT_2F,
    pub gradientOriginOffset: Common::D2D_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_RENDERING_CONTROLS {
    pub bufferPrecision: D2D1_BUFFER_PRECISION,
    pub tileSize: Common::D2D_SIZE_U,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_RENDERING_CONTROLS {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_RENDERING_CONTROLS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_RENDERING_CONTROLS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_RENDER_TARGET_PROPERTIES {
    pub r#type: D2D1_RENDER_TARGET_TYPE,
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub usage: D2D1_RENDER_TARGET_USAGE,
    pub minLevel: D2D1_FEATURE_LEVEL,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D2D1_RENDER_TARGET_PROPERTIES {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D2D1_RENDER_TARGET_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D2D1_RENDER_TARGET_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_RESOURCE_TEXTURE_PROPERTIES {
    pub extents: *const u32,
    pub dimensions: u32,
    pub bufferPrecision: D2D1_BUFFER_PRECISION,
    pub channelDepth: D2D1_CHANNEL_DEPTH,
    pub filter: D2D1_FILTER,
    pub extendModes: *const D2D1_EXTEND_MODE,
}
impl ::core::marker::Copy for D2D1_RESOURCE_TEXTURE_PROPERTIES {}
impl ::core::clone::Clone for D2D1_RESOURCE_TEXTURE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_RESOURCE_TEXTURE_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ROUNDED_RECT {
    pub rect: Common::D2D_RECT_F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_ROUNDED_RECT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_ROUNDED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_ROUNDED_RECT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_SIMPLE_COLOR_PROFILE {
    pub redPrimary: Common::D2D_POINT_2F,
    pub greenPrimary: Common::D2D_POINT_2F,
    pub bluePrimary: Common::D2D_POINT_2F,
    pub whitePointXZ: Common::D2D_POINT_2F,
    pub gamma: D2D1_GAMMA1,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_SIMPLE_COLOR_PROFILE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_SIMPLE_COLOR_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_SIMPLE_COLOR_PROFILE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_STROKE_STYLE_PROPERTIES {
    pub startCap: D2D1_CAP_STYLE,
    pub endCap: D2D1_CAP_STYLE,
    pub dashCap: D2D1_CAP_STYLE,
    pub lineJoin: D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: D2D1_DASH_STYLE,
    pub dashOffset: f32,
}
impl ::core::marker::Copy for D2D1_STROKE_STYLE_PROPERTIES {}
impl ::core::clone::Clone for D2D1_STROKE_STYLE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_STROKE_STYLE_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_STROKE_STYLE_PROPERTIES1 {
    pub startCap: D2D1_CAP_STYLE,
    pub endCap: D2D1_CAP_STYLE,
    pub dashCap: D2D1_CAP_STYLE,
    pub lineJoin: D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: D2D1_DASH_STYLE,
    pub dashOffset: f32,
    pub transformType: D2D1_STROKE_TRANSFORM_TYPE,
}
impl ::core::marker::Copy for D2D1_STROKE_STYLE_PROPERTIES1 {}
impl ::core::clone::Clone for D2D1_STROKE_STYLE_PROPERTIES1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_STROKE_STYLE_PROPERTIES1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_SVG_LENGTH {
    pub value: f32,
    pub units: D2D1_SVG_LENGTH_UNITS,
}
impl ::core::marker::Copy for D2D1_SVG_LENGTH {}
impl ::core::clone::Clone for D2D1_SVG_LENGTH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_LENGTH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_SVG_PRESERVE_ASPECT_RATIO {
    pub defer: super::super::Foundation::BOOL,
    pub align: D2D1_SVG_ASPECT_ALIGN,
    pub meetOrSlice: D2D1_SVG_ASPECT_SCALING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D2D1_SVG_PRESERVE_ASPECT_RATIO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D2D1_SVG_PRESERVE_ASPECT_RATIO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D2D1_SVG_PRESERVE_ASPECT_RATIO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_SVG_VIEWBOX {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for D2D1_SVG_VIEWBOX {}
impl ::core::clone::Clone for D2D1_SVG_VIEWBOX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_SVG_VIEWBOX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    pub orientation: D2D1_ORIENTATION,
    pub scaleX: f32,
    pub scaleY: f32,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
    pub options: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS,
}
impl ::core::marker::Copy for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {}
impl ::core::clone::Clone for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_TRIANGLE {
    pub point1: Common::D2D_POINT_2F,
    pub point2: Common::D2D_POINT_2F,
    pub point3: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_TRIANGLE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_TRIANGLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl ::windows::core::Abi for D2D1_TRIANGLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_VERTEX_BUFFER_PROPERTIES {
    pub inputCount: u32,
    pub usage: D2D1_VERTEX_USAGE,
    pub data: *const u8,
    pub byteWidth: u32,
}
impl ::core::marker::Copy for D2D1_VERTEX_BUFFER_PROPERTIES {}
impl ::core::clone::Clone for D2D1_VERTEX_BUFFER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_VERTEX_BUFFER_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_VERTEX_RANGE {
    pub startVertex: u32,
    pub vertexCount: u32,
}
impl ::core::marker::Copy for D2D1_VERTEX_RANGE {}
impl ::core::clone::Clone for D2D1_VERTEX_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D2D1_VERTEX_RANGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type PD2D1_EFFECT_FACTORY = ::core::option::Option<unsafe extern "system" fn(effectimpl: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type PD2D1_PROPERTY_GET_FUNCTION = ::core::option::Option<unsafe extern "system" fn(effect: ::core::option::Option<::windows::core::IUnknown>, data: *mut u8, datasize: u32, actualsize: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type PD2D1_PROPERTY_SET_FUNCTION = ::core::option::Option<unsafe extern "system" fn(effect: ::core::option::Option<::windows::core::IUnknown>, data: *const u8, datasize: u32) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
