#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffect(::windows::core::IUnknown);
impl IMILBitmapEffect {
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<P0>(&self, uiindex: u32, pcontext: P0) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectRenderContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOutput)(::windows::core::Vtable::as_raw(self), uiindex, pcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetParentEffect(&self) -> ::windows::core::Result<IMILBitmapEffectGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetParentEffect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn SetInputSource<P0>(&self, uiindex: u32, pbitmapsource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Imaging::IWICBitmapSource>>,
    {
        (::windows::core::Vtable::vtable(self).SetInputSource)(::windows::core::Vtable::as_raw(self), uiindex, pbitmapsource.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffect, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffect {
    type Vtable = IMILBitmapEffect_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a6ff321_c944_4a1b_9944_9954af301258);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffect_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: *mut ::core::ffi::c_void, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub GetParentEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparenteffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub SetInputSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pbitmapsource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    SetInputSource: usize,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnections(::windows::core::IUnknown);
impl IMILBitmapEffectConnections {
    pub unsafe fn GetInputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInputConnector)(::windows::core::Vtable::as_raw(self), uiindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOutputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOutputConnector)(::windows::core::Vtable::as_raw(self), uiindex, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectConnections, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectConnections {
    type Vtable = IMILBitmapEffectConnections_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnections {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2b5d861_9b1a_4374_89b0_dec4874d6a81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnections_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnectionsInfo(::windows::core::IUnknown);
impl IMILBitmapEffectConnectionsInfo {
    pub unsafe fn GetNumberInputs(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNumberInputs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNumberOutputs(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNumberOutputs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInputConnectorInfo(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInputConnectorInfo)(::windows::core::Vtable::as_raw(self), uiindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOutputConnectorInfo(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOutputConnectorInfo)(::windows::core::Vtable::as_raw(self), uiindex, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectConnectionsInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectConnectionsInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectConnectionsInfo {
    type Vtable = IMILBitmapEffectConnectionsInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnectionsInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x476b538a_c765_4237_ba4a_d6a880ff0cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectionsInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNumberInputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinuminputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetNumberOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumoutputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputConnectorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputConnectorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnector(::windows::core::IUnknown);
impl IMILBitmapEffectConnector {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsConnected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBitmapEffect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectConnector, ::windows::core::IUnknown, IMILBitmapEffectConnectorInfo);
impl ::core::clone::Clone for IMILBitmapEffectConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectConnector {
    type Vtable = IMILBitmapEffectConnector_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf59567b3_76c1_4d47_ba1e_79f955e350ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnector_Vtbl {
    pub base__: IMILBitmapEffectConnectorInfo_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnected: usize,
    pub GetBitmapEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnectorInfo(::windows::core::IUnknown);
impl IMILBitmapEffectConnectorInfo {
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIndex)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOptimalFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNumberFormats)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFormat)(::windows::core::Vtable::as_raw(self), ulindex, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectConnectorInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectConnectorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectConnectorInfo {
    type Vtable = IMILBitmapEffectConnectorInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnectorInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf66d2e4b_b46b_42fc_859e_3da0ecdb3c43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectorInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiindex: *mut u32) -> ::windows::core::HRESULT,
    pub GetOptimalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetNumberFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulnumberformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectEvents(::windows::core::IUnknown);
impl IMILBitmapEffectEvents {
    pub unsafe fn PropertyChange<P0>(&self, peffect: P0, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffect>>,
    {
        (::windows::core::Vtable::vtable(self).PropertyChange)(::windows::core::Vtable::as_raw(self), peffect.into().abi(), ::core::mem::transmute_copy(bstrpropertyname)).ok()
    }
    pub unsafe fn DirtyRegion<P0>(&self, peffect: P0, prect: *const MilRectD) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffect>>,
    {
        (::windows::core::Vtable::vtable(self).DirtyRegion)(::windows::core::Vtable::as_raw(self), peffect.into().abi(), prect).ok()
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectEvents, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectEvents {
    type Vtable = IMILBitmapEffectEvents_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e880dd8_f8ce_457b_8199_d60bb3d7ef98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub PropertyChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, bstrpropertyname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DirtyRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectFactory(::windows::core::IUnknown);
impl IMILBitmapEffectFactory {
    pub unsafe fn CreateEffect(&self, pguideffect: *const ::windows::core::GUID) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEffect)(::windows::core::Vtable::as_raw(self), pguideffect, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateContext(&self) -> ::windows::core::Result<IMILBitmapEffectRenderContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEffectOuter(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEffectOuter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectFactory {
    type Vtable = IMILBitmapEffectFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a9df34_a403_4ec7_b07e_bc0682370845);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguideffect: *const ::windows::core::GUID, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateEffectOuter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectGroup(::windows::core::IUnknown);
impl IMILBitmapEffectGroup {
    pub unsafe fn GetInteriorInputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInteriorInputConnector)(::windows::core::Vtable::as_raw(self), uiindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInteriorOutputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInteriorOutputConnector)(::windows::core::Vtable::as_raw(self), uiindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Add<P0>(&self, peffect: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffect>>,
    {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), peffect.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectGroup, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectGroup {
    type Vtable = IMILBitmapEffectGroup_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f952360_698a_4ac6_81a1_bcfdf08eb8e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInteriorInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInteriorOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectGroupImpl(::windows::core::IUnknown);
impl IMILBitmapEffectGroupImpl {
    pub unsafe fn Preprocess<P0>(&self, pcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Vtable::vtable(self).Preprocess)(::windows::core::Vtable::as_raw(self), pcontext.into().abi()).ok()
    }
    pub unsafe fn GetNumberChildren(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNumberChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetChildren(&self) -> ::windows::core::Result<IMILBitmapEffects> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectGroupImpl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectGroupImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectGroupImpl {
    type Vtable = IMILBitmapEffectGroupImpl_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectGroupImpl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78fed518_1cfc_4807_8b85_6b6e51398f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroupImpl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Preprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumberChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumberchildren: *mut u32) -> ::windows::core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchildren: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectImpl(::windows::core::IUnknown);
impl IMILBitmapEffectImpl {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInPlaceModificationAllowed<P0>(&self, poutputconnector: P0) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectOutputConnector>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsInPlaceModificationAllowed)(::windows::core::Vtable::as_raw(self), poutputconnector.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentEffect<P0>(&self, pparenteffect: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectGroup>>,
    {
        (::windows::core::Vtable::vtable(self).SetParentEffect)(::windows::core::Vtable::as_raw(self), pparenteffect.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputSource(&self, uiindex: u32) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInputSource)(::windows::core::Vtable::as_raw(self), uiindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInputSourceBounds(&self, uiindex: u32, prect: *mut MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetInputSourceBounds)(::windows::core::Vtable::as_raw(self), uiindex, prect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn GetInputBitmapSource<P0>(&self, uiindex: u32, prendercontext: P0, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Vtable::vtable(self).GetInputBitmapSource)(::windows::core::Vtable::as_raw(self), uiindex, prendercontext.into().abi(), pfmodifyinplace, ::core::mem::transmute(ppbitmapsource)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn GetOutputBitmapSource<P0>(&self, uiindex: u32, prendercontext: P0, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Vtable::vtable(self).GetOutputBitmapSource)(::windows::core::Vtable::as_raw(self), uiindex, prendercontext.into().abi(), pfmodifyinplace, ::core::mem::transmute(ppbitmapsource)).ok()
    }
    pub unsafe fn Initialize<P0>(&self, pinner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), pinner.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectImpl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectImpl {
    type Vtable = IMILBitmapEffectImpl_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectImpl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc2468f2_9936_47be_b4af_06b5df5dbcbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectImpl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInPlaceModificationAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutputconnector: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInPlaceModificationAllowed: usize,
    pub SetParentEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparenteffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputSource: usize,
    pub GetInputSourceBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub GetInputBitmapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))]
    GetInputBitmapSource: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub GetOutputBitmapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))]
    GetOutputBitmapSource: usize,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectInputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectInputConnector {
    pub unsafe fn ConnectTo<P0>(&self, pconnector: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectOutputConnector>>,
    {
        (::windows::core::Vtable::vtable(self).ConnectTo)(::windows::core::Vtable::as_raw(self), pconnector.into().abi()).ok()
    }
    pub unsafe fn GetConnection(&self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetConnection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectInputConnector, ::windows::core::IUnknown, IMILBitmapEffectConnectorInfo, IMILBitmapEffectConnector);
impl ::core::clone::Clone for IMILBitmapEffectInputConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectInputConnector {
    type Vtable = IMILBitmapEffectInputConnector_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectInputConnector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b4ecaa_7a3c_45e7_8573_f4b81b60dd6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInputConnector_Vtbl {
    pub base__: IMILBitmapEffectConnector_Vtbl,
    pub ConnectTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectInteriorInputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectInteriorInputConnector {
    pub unsafe fn GetInputConnector(&self) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInputConnector)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectInteriorInputConnector, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectInteriorInputConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectInteriorInputConnector {
    type Vtable = IMILBitmapEffectInteriorInputConnector_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectInteriorInputConnector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20287e9e_86a2_4e15_953d_eb1438a5b842);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorInputConnector_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectInteriorOutputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectInteriorOutputConnector {
    pub unsafe fn GetOutputConnector(&self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOutputConnector)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectInteriorOutputConnector, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectInteriorOutputConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectInteriorOutputConnector {
    type Vtable = IMILBitmapEffectInteriorOutputConnector_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectInteriorOutputConnector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00bbb6dc_acc9_4bfc_b344_8bee383dfefa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorOutputConnector_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutputconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectOutputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectOutputConnector {
    pub unsafe fn GetNumberConnections(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNumberConnections)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetConnection(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetConnection)(::windows::core::Vtable::as_raw(self), uiindex, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectOutputConnector, ::windows::core::IUnknown, IMILBitmapEffectConnectorInfo, IMILBitmapEffectConnector);
impl ::core::clone::Clone for IMILBitmapEffectOutputConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectOutputConnector {
    type Vtable = IMILBitmapEffectOutputConnector_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectOutputConnector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92957aad_841b_4866_82ec_8752468b07fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnector_Vtbl {
    pub base__: IMILBitmapEffectConnector_Vtbl,
    pub GetNumberConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumberconnections: *mut u32) -> ::windows::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectOutputConnectorImpl(::windows::core::IUnknown);
impl IMILBitmapEffectOutputConnectorImpl {
    pub unsafe fn AddBackLink<P0>(&self, pconnection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectInputConnector>>,
    {
        (::windows::core::Vtable::vtable(self).AddBackLink)(::windows::core::Vtable::as_raw(self), pconnection.into().abi()).ok()
    }
    pub unsafe fn RemoveBackLink<P0>(&self, pconnection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectInputConnector>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveBackLink)(::windows::core::Vtable::as_raw(self), pconnection.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectOutputConnectorImpl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectOutputConnectorImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectOutputConnectorImpl {
    type Vtable = IMILBitmapEffectOutputConnectorImpl_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectOutputConnectorImpl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21fae777_8b39_4bfa_9f2d_f3941ed36913);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnectorImpl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddBackLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveBackLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectPrimitive(::windows::core::IUnknown);
impl IMILBitmapEffectPrimitive {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn GetOutput<P0>(&self, uiindex: u32, pcontext: P0, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Vtable::vtable(self).GetOutput)(::windows::core::Vtable::as_raw(self), uiindex, pcontext.into().abi(), pfmodifyinplace, ::core::mem::transmute(ppbitmapsource)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransformPoint<P0, P1>(&self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: P0, pcontext: P1, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Vtable::vtable(self).TransformPoint)(::windows::core::Vtable::as_raw(self), uiindex, p, fforwardtransform.into(), pcontext.into().abi(), pfpointtransformed).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransformRect<P0, P1>(&self, uiindex: u32, p: *mut MilRectD, fforwardtransform: P0, pcontext: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Vtable::vtable(self).TransformRect)(::windows::core::Vtable::as_raw(self), uiindex, p, fforwardtransform.into(), pcontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasAffineTransform(&self, uiindex: u32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HasAffineTransform)(::windows::core::Vtable::as_raw(self), uiindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasInverseTransform(&self, uiindex: u32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HasInverseTransform)(::windows::core::Vtable::as_raw(self), uiindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub unsafe fn GetAffineMatrix(&self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetAffineMatrix)(::windows::core::Vtable::as_raw(self), uiindex, pmatrix).ok()
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectPrimitive, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectPrimitive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectPrimitive {
    type Vtable = IMILBitmapEffectPrimitive_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectPrimitive {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67e31025_3091_4dfc_98d6_dd494551461d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitive_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))]
    GetOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransformPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut ::core::ffi::c_void, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransformPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransformRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransformRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HasAffineTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pfaffine: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasAffineTransform: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HasInverseTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pfhasinverse: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasInverseTransform: usize,
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub GetAffineMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dwm"))]
    GetAffineMatrix: usize,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectPrimitiveImpl(::windows::core::IUnknown);
impl IMILBitmapEffectPrimitiveImpl {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDirty(&self, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).IsDirty)(::windows::core::Vtable::as_raw(self), uioutputindex, pfdirty)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVolatile(&self, uioutputindex: u32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsVolatile)(::windows::core::Vtable::as_raw(self), uioutputindex, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectPrimitiveImpl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectPrimitiveImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectPrimitiveImpl {
    type Vtable = IMILBitmapEffectPrimitiveImpl_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectPrimitiveImpl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce41e00b_efa6_44e7_b007_dd042e3ae126);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitiveImpl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDirty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVolatile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVolatile: usize,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectRenderContext(::windows::core::IUnknown);
impl IMILBitmapEffectRenderContext {
    pub unsafe fn SetOutputPixelFormat(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOutputPixelFormat)(::windows::core::Vtable::as_raw(self), format).ok()
    }
    pub unsafe fn GetOutputPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOutputPixelFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseSoftwareRenderer<P0>(&self, fsoftware: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetUseSoftwareRenderer)(::windows::core::Vtable::as_raw(self), fsoftware.into()).ok()
    }
    pub unsafe fn SetInitialTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialTransform)(::windows::core::Vtable::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetFinalTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetFinalTransform)(::windows::core::Vtable::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn SetOutputDPI(&self, dbldpix: f64, dbldpiy: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOutputDPI)(::windows::core::Vtable::as_raw(self), dbldpix, dbldpiy).ok()
    }
    pub unsafe fn GetOutputDPI(&self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetOutputDPI)(::windows::core::Vtable::as_raw(self), pdbldpix, pdbldpiy).ok()
    }
    pub unsafe fn SetRegionOfInterest(&self, prect: *const MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRegionOfInterest)(::windows::core::Vtable::as_raw(self), prect).ok()
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectRenderContext, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectRenderContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectRenderContext {
    type Vtable = IMILBitmapEffectRenderContext_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectRenderContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12a2ec7e_2d33_44b2_b334_1abb7846e390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetOutputPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetOutputPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseSoftwareRenderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsoftware: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseSoftwareRenderer: usize,
    pub SetInitialTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT,
    pub GetFinalTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT,
    pub SetOutputDPI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dbldpix: f64, dbldpiy: f64) -> ::windows::core::HRESULT,
    pub GetOutputDPI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::HRESULT,
    pub SetRegionOfInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectRenderContextImpl(::windows::core::IUnknown);
impl IMILBitmapEffectRenderContextImpl {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUseSoftwareRenderer(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUseSoftwareRenderer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetTransform)(::windows::core::Vtable::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn UpdateTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateTransform)(::windows::core::Vtable::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetOutputBounds(&self, prect: *mut MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetOutputBounds)(::windows::core::Vtable::as_raw(self), prect).ok()
    }
    pub unsafe fn UpdateOutputBounds(&self, prect: *const MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateOutputBounds)(::windows::core::Vtable::as_raw(self), prect).ok()
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffectRenderContextImpl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffectRenderContextImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffectRenderContextImpl {
    type Vtable = IMILBitmapEffectRenderContextImpl_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectRenderContextImpl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d25accb_797d_4fd2_b128_dffeff84fcc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContextImpl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUseSoftwareRenderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsoftware: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUseSoftwareRenderer: usize,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT,
    pub UpdateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT,
    pub GetOutputBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *mut MilRectD) -> ::windows::core::HRESULT,
    pub UpdateOutputBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffects(::windows::core::IUnknown);
impl IMILBitmapEffects {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<IMILBitmapEffectGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Item(&self, uindex: u32) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Item)(::windows::core::Vtable::as_raw(self), uindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMILBitmapEffects, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMILBitmapEffects {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMILBitmapEffects {
    type Vtable = IMILBitmapEffects_Vtbl;
}
unsafe impl ::windows::core::Interface for IMILBitmapEffects {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51ac3dce_67c5_448b_9180_ad3eabddd5dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffects_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectBevel: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd361dbe_6c9b_4de0_8290_f6400c2737ed);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectBlur: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa924df87_225d_4373_8f5b_b90ec85ae3de);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectDropShadow: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x459a3fbe_d8ac_4692_874b_7a265715aa16);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectEmboss: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd299846_824f_47ec_a007_12aa767f2816);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectGroup: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac9c1a9a_7e18_4f64_ac7e_47cf7f051e95);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectOuterGlow: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2161bdd_7eb6_4725_9c0b_8a2a1b4f0667);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub struct MILMatrixF {
    pub _11: f64,
    pub _12: f64,
    pub _13: f64,
    pub _14: f64,
    pub _21: f64,
    pub _22: f64,
    pub _23: f64,
    pub _24: f64,
    pub _31: f64,
    pub _32: f64,
    pub _33: f64,
    pub _34: f64,
    pub _41: f64,
    pub _42: f64,
    pub _43: f64,
    pub _44: f64,
}
impl ::core::marker::Copy for MILMatrixF {}
impl ::core::clone::Clone for MILMatrixF {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MILMatrixF {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub struct MilPoint2D {
    pub X: f64,
    pub Y: f64,
}
impl ::core::marker::Copy for MilPoint2D {}
impl ::core::clone::Clone for MilPoint2D {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MilPoint2D {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub struct MilRectD {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl ::core::marker::Copy for MilRectD {}
impl ::core::clone::Clone for MilRectD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MilRectD {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
