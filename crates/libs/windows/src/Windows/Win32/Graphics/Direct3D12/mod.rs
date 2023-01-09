#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D12CreateDevice<P0, T>(padapter: P0, minimumfeaturelevel: super::Direct3D::D3D_FEATURE_LEVEL, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "d3d12.dll""system" fn D3D12CreateDevice ( padapter : * mut::core::ffi::c_void , minimumfeaturelevel : super::Direct3D:: D3D_FEATURE_LEVEL , riid : *const :: windows::core::GUID , ppdevice : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    D3D12CreateDevice(padapter.into().abi(), minimumfeaturelevel, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[inline]
pub unsafe fn D3D12CreateRootSignatureDeserializer(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows::core::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "d3d12.dll""system" fn D3D12CreateRootSignatureDeserializer ( psrcdata : *const ::core::ffi::c_void , srcdatasizeinbytes : usize , prootsignaturedeserializerinterface : *const :: windows::core::GUID , pprootsignaturedeserializer : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    D3D12CreateRootSignatureDeserializer(psrcdata, srcdatasizeinbytes, prootsignaturedeserializerinterface, pprootsignaturedeserializer).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[inline]
pub unsafe fn D3D12CreateVersionedRootSignatureDeserializer(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows::core::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "d3d12.dll""system" fn D3D12CreateVersionedRootSignatureDeserializer ( psrcdata : *const ::core::ffi::c_void , srcdatasizeinbytes : usize , prootsignaturedeserializerinterface : *const :: windows::core::GUID , pprootsignaturedeserializer : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    D3D12CreateVersionedRootSignatureDeserializer(psrcdata, srcdatasizeinbytes, prootsignaturedeserializerinterface, pprootsignaturedeserializer).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[inline]
pub unsafe fn D3D12EnableExperimentalFeatures(numfeatures: u32, piids: *const ::windows::core::GUID, pconfigurationstructs: ::core::option::Option<*const ::core::ffi::c_void>, pconfigurationstructsizes: ::core::option::Option<*const u32>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "d3d12.dll""system" fn D3D12EnableExperimentalFeatures ( numfeatures : u32 , piids : *const :: windows::core::GUID , pconfigurationstructs : *const ::core::ffi::c_void , pconfigurationstructsizes : *const u32 ) -> :: windows::core::HRESULT );
    D3D12EnableExperimentalFeatures(numfeatures, piids, ::core::mem::transmute(pconfigurationstructs.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pconfigurationstructsizes.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[inline]
pub unsafe fn D3D12GetDebugInterface<T>(result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
where
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "d3d12.dll""system" fn D3D12GetDebugInterface ( riid : *const :: windows::core::GUID , ppvdebug : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    D3D12GetDebugInterface(&<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[inline]
pub unsafe fn D3D12GetInterface<T>(rclsid: *const ::windows::core::GUID, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
where
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "d3d12.dll""system" fn D3D12GetInterface ( rclsid : *const :: windows::core::GUID , riid : *const :: windows::core::GUID , ppvdebug : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    D3D12GetInterface(rclsid, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D12SerializeRootSignature(prootsignature: *const D3D12_ROOT_SIGNATURE_DESC, version: D3D_ROOT_SIGNATURE_VERSION, ppblob: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrorblob: ::core::option::Option<*mut ::core::option::Option<super::Direct3D::ID3DBlob>>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "d3d12.dll""system" fn D3D12SerializeRootSignature ( prootsignature : *const D3D12_ROOT_SIGNATURE_DESC , version : D3D_ROOT_SIGNATURE_VERSION , ppblob : *mut * mut::core::ffi::c_void , pperrorblob : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    D3D12SerializeRootSignature(prootsignature, version, ::core::mem::transmute(ppblob), ::core::mem::transmute(pperrorblob.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D12SerializeVersionedRootSignature(prootsignature: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppblob: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrorblob: ::core::option::Option<*mut ::core::option::Option<super::Direct3D::ID3DBlob>>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "d3d12.dll""system" fn D3D12SerializeVersionedRootSignature ( prootsignature : *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC , ppblob : *mut * mut::core::ffi::c_void , pperrorblob : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    D3D12SerializeVersionedRootSignature(prootsignature, ::core::mem::transmute(ppblob), ::core::mem::transmute(pperrorblob.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12CommandAllocator(::windows::core::IUnknown);
impl ID3D12CommandAllocator {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12CommandAllocator, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ::core::clone::Clone for ID3D12CommandAllocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12CommandAllocator {}
unsafe impl ::core::marker::Sync for ID3D12CommandAllocator {}
unsafe impl ::windows::core::Vtable for ID3D12CommandAllocator {
    type Vtable = ID3D12CommandAllocator_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12CommandAllocator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6102dee4_af59_4b09_b999_b44d73f09b24);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12CommandAllocator_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12CommandList(::windows::core::IUnknown);
impl ID3D12CommandList {
    pub unsafe fn GetType(&self) -> D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).GetType)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12CommandList, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ::core::clone::Clone for ID3D12CommandList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12CommandList {}
unsafe impl ::core::marker::Sync for ID3D12CommandList {}
unsafe impl ::windows::core::Vtable for ID3D12CommandList {
    type Vtable = ID3D12CommandList_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12CommandList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7116d91c_e7e4_47ce_b8c6_ec8168f437e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12CommandList_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D3D12_COMMAND_LIST_TYPE,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12CommandQueue(::windows::core::IUnknown);
impl ID3D12CommandQueue {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateTileMappings<P0, P1>(&self, presource: P0, numresourceregions: u32, presourceregionstartcoordinates: ::core::option::Option<*const D3D12_TILED_RESOURCE_COORDINATE>, presourceregionsizes: ::core::option::Option<*const D3D12_TILE_REGION_SIZE>, pheap: P1, numranges: u32, prangeflags: ::core::option::Option<*const D3D12_TILE_RANGE_FLAGS>, pheaprangestartoffsets: ::core::option::Option<*const u32>, prangetilecounts: ::core::option::Option<*const u32>, flags: D3D12_TILE_MAPPING_FLAGS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
    {
        (::windows::core::Vtable::vtable(self).UpdateTileMappings)(
            ::windows::core::Vtable::as_raw(self),
            presource.into().abi(),
            numresourceregions,
            ::core::mem::transmute(presourceregionstartcoordinates.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(presourceregionsizes.unwrap_or(::std::ptr::null())),
            pheap.into().abi(),
            numranges,
            ::core::mem::transmute(prangeflags.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pheaprangestartoffsets.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(prangetilecounts.unwrap_or(::std::ptr::null())),
            flags,
        )
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTileMappings<P0, P1>(&self, pdstresource: P0, pdstregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, psrcresource: P1, psrcregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, pregionsize: *const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).CopyTileMappings)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), pdstregionstartcoordinate, psrcresource.into().abi(), psrcregionstartcoordinate, pregionsize, flags)
    }
    pub unsafe fn ExecuteCommandLists(&self, ppcommandlists: &[ID3D12CommandList]) {
        (::windows::core::Vtable::vtable(self).ExecuteCommandLists)(::windows::core::Vtable::as_raw(self), ppcommandlists.len() as _, ::core::mem::transmute(ppcommandlists.as_ptr()))
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Signal<P0>(&self, pfence: P0, value: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Fence>>,
    {
        (::windows::core::Vtable::vtable(self).Signal)(::windows::core::Vtable::as_raw(self), pfence.into().abi(), value).ok()
    }
    pub unsafe fn Wait<P0>(&self, pfence: P0, value: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Fence>>,
    {
        (::windows::core::Vtable::vtable(self).Wait)(::windows::core::Vtable::as_raw(self), pfence.into().abi(), value).ok()
    }
    pub unsafe fn GetTimestampFrequency(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTimestampFrequency)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetClockCalibration(&self, pgputimestamp: *mut u64, pcputimestamp: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetClockCalibration)(::windows::core::Vtable::as_raw(self), pgputimestamp, pcputimestamp).ok()
    }
    pub unsafe fn GetDesc(&self) -> D3D12_COMMAND_QUEUE_DESC {
        let mut result__: D3D12_COMMAND_QUEUE_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
::windows::core::interface_hierarchy!(ID3D12CommandQueue, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ::core::clone::Clone for ID3D12CommandQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12CommandQueue {}
unsafe impl ::core::marker::Sync for ID3D12CommandQueue {}
unsafe impl ::windows::core::Vtable for ID3D12CommandQueue {
    type Vtable = ID3D12CommandQueue_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12CommandQueue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ec870a6_5d7e_4c22_8cfc_5baae07616ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12CommandQueue_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateTileMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, numresourceregions: u32, presourceregionstartcoordinates: *const D3D12_TILED_RESOURCE_COORDINATE, presourceregionsizes: *const D3D12_TILE_REGION_SIZE, pheap: *mut ::core::ffi::c_void, numranges: u32, prangeflags: *const D3D12_TILE_RANGE_FLAGS, pheaprangestartoffsets: *const u32, prangetilecounts: *const u32, flags: D3D12_TILE_MAPPING_FLAGS),
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateTileMappings: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTileMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, pdstregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, psrcresource: *mut ::core::ffi::c_void, psrcregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, pregionsize: *const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS),
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTileMappings: usize,
    pub ExecuteCommandLists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numcommandlists: u32, ppcommandlists: *const *mut ::core::ffi::c_void),
    pub SetMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32),
    pub BeginEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32),
    pub EndEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Signal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfence: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfence: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub GetTimestampFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrequency: *mut u64) -> ::windows::core::HRESULT,
    pub GetClockCalibration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgputimestamp: *mut u64, pcputimestamp: *mut u64) -> ::windows::core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_COMMAND_QUEUE_DESC),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12CommandSignature(::windows::core::IUnknown);
impl ID3D12CommandSignature {}
::windows::core::interface_hierarchy!(ID3D12CommandSignature, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ::core::clone::Clone for ID3D12CommandSignature {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12CommandSignature {}
unsafe impl ::core::marker::Sync for ID3D12CommandSignature {}
unsafe impl ::windows::core::Vtable for ID3D12CommandSignature {
    type Vtable = ID3D12CommandSignature_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12CommandSignature {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc36a797c_ec80_4f0a_8985_a7b2475082d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12CommandSignature_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Debug(::windows::core::IUnknown);
impl ID3D12Debug {
    pub unsafe fn EnableDebugLayer(&self) {
        (::windows::core::Vtable::vtable(self).EnableDebugLayer)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12Debug, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12Debug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Debug {}
unsafe impl ::core::marker::Sync for ID3D12Debug {}
unsafe impl ::windows::core::Vtable for ID3D12Debug {
    type Vtable = ID3D12Debug_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Debug {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x344488b7_6846_474b_b989_f027448245e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Debug_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnableDebugLayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Debug1(::windows::core::IUnknown);
impl ID3D12Debug1 {
    pub unsafe fn EnableDebugLayer(&self) {
        (::windows::core::Vtable::vtable(self).EnableDebugLayer)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableGPUBasedValidation<P0>(&self, enable: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetEnableGPUBasedValidation)(::windows::core::Vtable::as_raw(self), enable.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableSynchronizedCommandQueueValidation<P0>(&self, enable: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetEnableSynchronizedCommandQueueValidation)(::windows::core::Vtable::as_raw(self), enable.into())
    }
}
::windows::core::interface_hierarchy!(ID3D12Debug1, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12Debug1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Debug1 {}
unsafe impl ::core::marker::Sync for ID3D12Debug1 {}
unsafe impl ::windows::core::Vtable for ID3D12Debug1 {
    type Vtable = ID3D12Debug1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Debug1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaffaa4ca_63fe_4d8e_b8ad_159000af4304);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Debug1_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnableDebugLayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableGPUBasedValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableGPUBasedValidation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableSynchronizedCommandQueueValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableSynchronizedCommandQueueValidation: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Debug2(::windows::core::IUnknown);
impl ID3D12Debug2 {
    pub unsafe fn SetGPUBasedValidationFlags(&self, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
        (::windows::core::Vtable::vtable(self).SetGPUBasedValidationFlags)(::windows::core::Vtable::as_raw(self), flags)
    }
}
::windows::core::interface_hierarchy!(ID3D12Debug2, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12Debug2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Debug2 {}
unsafe impl ::core::marker::Sync for ID3D12Debug2 {}
unsafe impl ::windows::core::Vtable for ID3D12Debug2 {
    type Vtable = ID3D12Debug2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Debug2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93a665c4_a3b2_4e5d_b692_a26ae14e3374);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Debug2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetGPUBasedValidationFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Debug3(::windows::core::IUnknown);
impl ID3D12Debug3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableGPUBasedValidation<P0>(&self, enable: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetEnableGPUBasedValidation)(::windows::core::Vtable::as_raw(self), enable.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableSynchronizedCommandQueueValidation<P0>(&self, enable: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetEnableSynchronizedCommandQueueValidation)(::windows::core::Vtable::as_raw(self), enable.into())
    }
    pub unsafe fn SetGPUBasedValidationFlags(&self, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
        (::windows::core::Vtable::vtable(self).SetGPUBasedValidationFlags)(::windows::core::Vtable::as_raw(self), flags)
    }
}
::windows::core::interface_hierarchy!(ID3D12Debug3, ::windows::core::IUnknown, ID3D12Debug);
impl ::core::clone::Clone for ID3D12Debug3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Debug3 {}
unsafe impl ::core::marker::Sync for ID3D12Debug3 {}
unsafe impl ::windows::core::Vtable for ID3D12Debug3 {
    type Vtable = ID3D12Debug3_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Debug3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cf4e58f_f671_4ff1_a542_3686e3d153d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Debug3_Vtbl {
    pub base__: ID3D12Debug_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableGPUBasedValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableGPUBasedValidation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableSynchronizedCommandQueueValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableSynchronizedCommandQueueValidation: usize,
    pub SetGPUBasedValidationFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Debug4(::windows::core::IUnknown);
impl ID3D12Debug4 {
    pub unsafe fn DisableDebugLayer(&self) {
        (::windows::core::Vtable::vtable(self).DisableDebugLayer)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12Debug4, ::windows::core::IUnknown, ID3D12Debug, ID3D12Debug3);
impl ::core::clone::Clone for ID3D12Debug4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Debug4 {}
unsafe impl ::core::marker::Sync for ID3D12Debug4 {}
unsafe impl ::windows::core::Vtable for ID3D12Debug4 {
    type Vtable = ID3D12Debug4_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Debug4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x014b816e_9ec5_4a2f_a845_ffbe441ce13a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Debug4_Vtbl {
    pub base__: ID3D12Debug3_Vtbl,
    pub DisableDebugLayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Debug5(::windows::core::IUnknown);
impl ID3D12Debug5 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableAutoName<P0>(&self, enable: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetEnableAutoName)(::windows::core::Vtable::as_raw(self), enable.into())
    }
}
::windows::core::interface_hierarchy!(ID3D12Debug5, ::windows::core::IUnknown, ID3D12Debug, ID3D12Debug3, ID3D12Debug4);
impl ::core::clone::Clone for ID3D12Debug5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Debug5 {}
unsafe impl ::core::marker::Sync for ID3D12Debug5 {}
unsafe impl ::windows::core::Vtable for ID3D12Debug5 {
    type Vtable = ID3D12Debug5_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Debug5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x548d6b12_09fa_40e0_9069_5dcd589a52c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Debug5_Vtbl {
    pub base__: ID3D12Debug4_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableAutoName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableAutoName: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DebugCommandList(::windows::core::IUnknown);
impl ID3D12DebugCommandList {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssertResourceState<P0>(&self, presource: P0, subresource: u32, state: u32) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).AssertResourceState)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource, state)
    }
    pub unsafe fn SetFeatureMask(&self, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFeatureMask)(::windows::core::Vtable::as_raw(self), mask).ok()
    }
    pub unsafe fn GetFeatureMask(&self) -> D3D12_DEBUG_FEATURE {
        (::windows::core::Vtable::vtable(self).GetFeatureMask)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12DebugCommandList, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12DebugCommandList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DebugCommandList {}
unsafe impl ::core::marker::Sync for ID3D12DebugCommandList {}
unsafe impl ::windows::core::Vtable for ID3D12DebugCommandList {
    type Vtable = ID3D12DebugCommandList_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DebugCommandList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09e0bf36_54ac_484f_8847_4baeeab6053f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DebugCommandList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AssertResourceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, state: u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    AssertResourceState: usize,
    pub SetFeatureMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::HRESULT,
    pub GetFeatureMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D3D12_DEBUG_FEATURE,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DebugCommandList1(::windows::core::IUnknown);
impl ID3D12DebugCommandList1 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssertResourceState<P0>(&self, presource: P0, subresource: u32, state: u32) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).AssertResourceState)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource, state)
    }
    pub unsafe fn SetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDebugParameter)(::windows::core::Vtable::as_raw(self), r#type, pdata, datasize).ok()
    }
    pub unsafe fn GetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDebugParameter)(::windows::core::Vtable::as_raw(self), r#type, pdata, datasize).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12DebugCommandList1, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12DebugCommandList1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DebugCommandList1 {}
unsafe impl ::core::marker::Sync for ID3D12DebugCommandList1 {}
unsafe impl ::windows::core::Vtable for ID3D12DebugCommandList1 {
    type Vtable = ID3D12DebugCommandList1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DebugCommandList1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x102ca951_311b_4b01_b11f_ecb83e061b37);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DebugCommandList1_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AssertResourceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, state: u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    AssertResourceState: usize,
    pub SetDebugParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT,
    pub GetDebugParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DebugCommandList2(::windows::core::IUnknown);
impl ID3D12DebugCommandList2 {
    pub unsafe fn SetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDebugParameter)(::windows::core::Vtable::as_raw(self), r#type, pdata, datasize).ok()
    }
    pub unsafe fn GetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDebugParameter)(::windows::core::Vtable::as_raw(self), r#type, pdata, datasize).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12DebugCommandList2, ::windows::core::IUnknown, ID3D12DebugCommandList);
impl ::core::clone::Clone for ID3D12DebugCommandList2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DebugCommandList2 {}
unsafe impl ::core::marker::Sync for ID3D12DebugCommandList2 {}
unsafe impl ::windows::core::Vtable for ID3D12DebugCommandList2 {
    type Vtable = ID3D12DebugCommandList2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DebugCommandList2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaeb575cf_4e06_48be_ba3b_c450fc96652e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DebugCommandList2_Vtbl {
    pub base__: ID3D12DebugCommandList_Vtbl,
    pub SetDebugParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT,
    pub GetDebugParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DebugCommandQueue(::windows::core::IUnknown);
impl ID3D12DebugCommandQueue {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssertResourceState<P0>(&self, presource: P0, subresource: u32, state: u32) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).AssertResourceState)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource, state)
    }
}
::windows::core::interface_hierarchy!(ID3D12DebugCommandQueue, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12DebugCommandQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DebugCommandQueue {}
unsafe impl ::core::marker::Sync for ID3D12DebugCommandQueue {}
unsafe impl ::windows::core::Vtable for ID3D12DebugCommandQueue {
    type Vtable = ID3D12DebugCommandQueue_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DebugCommandQueue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09e0bf36_54ac_484f_8847_4baeeab6053a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DebugCommandQueue_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AssertResourceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, state: u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    AssertResourceState: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DebugDevice(::windows::core::IUnknown);
impl ID3D12DebugDevice {
    pub unsafe fn SetFeatureMask(&self, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFeatureMask)(::windows::core::Vtable::as_raw(self), mask).ok()
    }
    pub unsafe fn GetFeatureMask(&self) -> D3D12_DEBUG_FEATURE {
        (::windows::core::Vtable::vtable(self).GetFeatureMask)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ReportLiveDeviceObjects(&self, flags: D3D12_RLDO_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportLiveDeviceObjects)(::windows::core::Vtable::as_raw(self), flags).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12DebugDevice, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12DebugDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DebugDevice {}
unsafe impl ::core::marker::Sync for ID3D12DebugDevice {}
unsafe impl ::windows::core::Vtable for ID3D12DebugDevice {
    type Vtable = ID3D12DebugDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DebugDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3febd6dd_4973_4787_8194_e45f9e28923e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DebugDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetFeatureMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::HRESULT,
    pub GetFeatureMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D3D12_DEBUG_FEATURE,
    pub ReportLiveDeviceObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DebugDevice1(::windows::core::IUnknown);
impl ID3D12DebugDevice1 {
    pub unsafe fn SetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDebugParameter)(::windows::core::Vtable::as_raw(self), r#type, pdata, datasize).ok()
    }
    pub unsafe fn GetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDebugParameter)(::windows::core::Vtable::as_raw(self), r#type, pdata, datasize).ok()
    }
    pub unsafe fn ReportLiveDeviceObjects(&self, flags: D3D12_RLDO_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportLiveDeviceObjects)(::windows::core::Vtable::as_raw(self), flags).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12DebugDevice1, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12DebugDevice1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DebugDevice1 {}
unsafe impl ::core::marker::Sync for ID3D12DebugDevice1 {}
unsafe impl ::windows::core::Vtable for ID3D12DebugDevice1 {
    type Vtable = ID3D12DebugDevice1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DebugDevice1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b71770_d099_4a65_a698_3dee10020f88);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DebugDevice1_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetDebugParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT,
    pub GetDebugParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT,
    pub ReportLiveDeviceObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DebugDevice2(::windows::core::IUnknown);
impl ID3D12DebugDevice2 {
    pub unsafe fn SetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDebugParameter)(::windows::core::Vtable::as_raw(self), r#type, pdata, datasize).ok()
    }
    pub unsafe fn GetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDebugParameter)(::windows::core::Vtable::as_raw(self), r#type, pdata, datasize).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12DebugDevice2, ::windows::core::IUnknown, ID3D12DebugDevice);
impl ::core::clone::Clone for ID3D12DebugDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DebugDevice2 {}
unsafe impl ::core::marker::Sync for ID3D12DebugDevice2 {}
unsafe impl ::windows::core::Vtable for ID3D12DebugDevice2 {
    type Vtable = ID3D12DebugDevice2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DebugDevice2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60eccbc1_378d_4df1_894c_f8ac5ce4d7dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DebugDevice2_Vtbl {
    pub base__: ID3D12DebugDevice_Vtbl,
    pub SetDebugParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT,
    pub GetDebugParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut ::core::ffi::c_void, datasize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DescriptorHeap(::windows::core::IUnknown);
impl ID3D12DescriptorHeap {
    pub unsafe fn GetDesc(&self) -> D3D12_DESCRIPTOR_HEAP_DESC {
        let mut result__: D3D12_DESCRIPTOR_HEAP_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetCPUDescriptorHandleForHeapStart(&self) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        let mut result__: D3D12_CPU_DESCRIPTOR_HANDLE = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetCPUDescriptorHandleForHeapStart)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetGPUDescriptorHandleForHeapStart(&self) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        let mut result__: D3D12_GPU_DESCRIPTOR_HANDLE = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetGPUDescriptorHandleForHeapStart)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
::windows::core::interface_hierarchy!(ID3D12DescriptorHeap, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ::core::clone::Clone for ID3D12DescriptorHeap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DescriptorHeap {}
unsafe impl ::core::marker::Sync for ID3D12DescriptorHeap {}
unsafe impl ::windows::core::Vtable for ID3D12DescriptorHeap {
    type Vtable = ID3D12DescriptorHeap_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DescriptorHeap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8efb471d_616c_4f49_90f7_127bb763fa51);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DescriptorHeap_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_DESCRIPTOR_HEAP_DESC),
    pub GetCPUDescriptorHandleForHeapStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_CPU_DESCRIPTOR_HANDLE),
    pub GetGPUDescriptorHandleForHeapStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_GPU_DESCRIPTOR_HANDLE),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Device(::windows::core::IUnknown);
impl ID3D12Device {
    pub unsafe fn GetNodeCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetNodeCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateCommandQueue<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCommandQueue)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandAllocator<T>(&self, r#type: D3D12_COMMAND_LIST_TYPE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCommandAllocator)(::windows::core::Vtable::as_raw(self), r#type, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateGraphicsPipelineState<T>(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateGraphicsPipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateComputePipelineState<T>(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateComputePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList<P0, P1, T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: P0, pinitialstate: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCommandList)(::windows::core::Vtable::as_raw(self), nodemask, r#type, pcommandallocator.into().abi(), pinitialstate.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateDescriptorHeap<T>(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDescriptorHeap)(::windows::core::Vtable::as_raw(self), pdescriptorheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
        (::windows::core::Vtable::vtable(self).GetDescriptorHandleIncrementSize)(::windows::core::Vtable::as_raw(self), descriptorheaptype)
    }
    pub unsafe fn CreateRootSignature<T>(&self, nodemask: u32, pblobwithrootsignature: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRootSignature)(::windows::core::Vtable::as_raw(self), nodemask, ::core::mem::transmute(pblobwithrootsignature.as_ptr()), pblobwithrootsignature.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantBufferView(&self, pdesc: ::core::option::Option<*const D3D12_CONSTANT_BUFFER_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).CreateConstantBufferView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_SHADER_RESOURCE_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0, P1>(&self, presource: P0, pcounterresource: P1, pdesc: ::core::option::Option<*const D3D12_UNORDERED_ACCESS_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pcounterresource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_RENDER_TARGET_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_DEPTH_STENCIL_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).CreateSampler)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: ::core::option::Option<*const u32>, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: ::core::option::Option<*const u32>, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).CopyDescriptors)(::windows::core::Vtable::as_raw(self), numdestdescriptorranges, pdestdescriptorrangestarts, ::core::mem::transmute(pdestdescriptorrangesizes.unwrap_or(::std::ptr::null())), numsrcdescriptorranges, psrcdescriptorrangestarts, ::core::mem::transmute(psrcdescriptorrangesizes.unwrap_or(::std::ptr::null())), descriptorheapstype)
    }
    pub unsafe fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).CopyDescriptorsSimple)(::windows::core::Vtable::as_raw(self), numdescriptors, ::core::mem::transmute(destdescriptorrangestart), ::core::mem::transmute(srcdescriptorrangestart), descriptorheapstype)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo(&self, visiblemask: u32, presourcedescs: &[D3D12_RESOURCE_DESC]) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetResourceAllocationInfo)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, presourcedescs.len() as _, ::core::mem::transmute(presourcedescs.as_ptr()));
        result__
    }
    pub unsafe fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
        let mut result__: D3D12_HEAP_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetCustomHeapProperties)(::windows::core::Vtable::as_raw(self), &mut result__, nodemask, heaptype);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource<T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreateCommittedResource)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap<T>(&self, pdesc: *const D3D12_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreateHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreatePlacedResource)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource<T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreateReservedResource)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0, P1>(&self, pobject: P0, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, access: u32, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSharedHandle)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), access, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<P0, T>(&self, nthandle: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).OpenSharedHandle)(::windows::core::Vtable::as_raw(self), nthandle.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandleByName<P0>(&self, name: P0, access: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenSharedHandleByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), access, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MakeResident)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn Evict(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Evict)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFence)(::windows::core::Vtable::as_raw(self), initialvalue, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).GetCopyableFootprints)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CreateQueryHeap<T>(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreateQueryHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStablePowerState<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetStablePowerState)(::windows::core::Vtable::as_raw(self), enable.into()).ok()
    }
    pub unsafe fn CreateCommandSignature<P0, T>(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreateCommandSignature)(::windows::core::Vtable::as_raw(self), pdesc, prootsignature.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D12_PACKED_MIP_INFO>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D12_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterLuid(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetAdapterLuid)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
::windows::core::interface_hierarchy!(ID3D12Device, ::windows::core::IUnknown, ID3D12Object);
impl ::core::clone::Clone for ID3D12Device {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Device {}
unsafe impl ::core::marker::Sync for ID3D12Device {}
unsafe impl ::windows::core::Vtable for ID3D12Device {
    type Vtable = ID3D12Device_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Device {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x189819f1_1db6_4b57_be54_1821339b85f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Device_Vtbl {
    pub base__: ID3D12Object_Vtbl,
    pub GetNodeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub CreateCommandQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, riid: *const ::windows::core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCommandAllocator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: D3D12_COMMAND_LIST_TYPE, riid: *const ::windows::core::GUID, ppcommandallocator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateGraphicsPipelineState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateGraphicsPipelineState: usize,
    pub CreateComputePipelineState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCommandList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: *mut ::core::ffi::c_void, pinitialstate: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CheckFeatureSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT,
    pub CreateDescriptorHeap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDescriptorHandleIncrementSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32,
    pub CreateRootSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodemask: u32, pblobwithrootsignature: *const ::core::ffi::c_void, bloblengthinbytes: usize, riid: *const ::windows::core::GUID, ppvrootsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateConstantBufferView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_CONSTANT_BUFFER_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateShaderResourceView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D12_SHADER_RESOURCE_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateShaderResourceView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateUnorderedAccessView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pcounterresource: *mut ::core::ffi::c_void, pdesc: *const D3D12_UNORDERED_ACCESS_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateUnorderedAccessView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateRenderTargetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D12_RENDER_TARGET_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateRenderTargetView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateDepthStencilView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D12_DEPTH_STENCIL_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateDepthStencilView: usize,
    pub CreateSampler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE),
    pub CopyDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: *const u32, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: *const u32, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE),
    pub CopyDescriptorsSimple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetResourceAllocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetResourceAllocationInfo: usize,
    pub GetCustomHeapProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_HEAP_PROPERTIES, nodemask: u32, heaptype: D3D12_HEAP_TYPE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateCommittedResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateCommittedResource: usize,
    pub CreateHeap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreatePlacedResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheap: *mut ::core::ffi::c_void, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreatePlacedResource: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateReservedResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateReservedResource: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub CreateSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::windows::core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    CreateSharedHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nthandle: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenSharedHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenSharedHandleByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, access: u32, pnthandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenSharedHandleByName: usize,
    pub MakeResident: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Evict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: u64, flags: D3D12_FENCE_FLAGS, riid: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceRemovedReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetCopyableFootprints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetCopyableFootprints: usize,
    pub CreateQueryHeap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_QUERY_HEAP_DESC, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStablePowerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStablePowerState: usize,
    pub CreateCommandSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvcommandsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetResourceTiling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptiledresource: *mut ::core::ffi::c_void, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D12_PACKED_MIP_INFO, pstandardtileshapefornonpackedmips: *mut D3D12_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING),
    #[cfg(feature = "Win32_Foundation")]
    pub GetAdapterLuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAdapterLuid: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Device1(::windows::core::IUnknown);
impl ID3D12Device1 {
    pub unsafe fn CreatePipelineLibrary<T>(&self, plibraryblob: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePipelineLibrary)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plibraryblob.as_ptr()), plibraryblob.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventOnMultipleFenceCompletion<P0>(&self, ppfences: *const ::core::option::Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).SetEventOnMultipleFenceCompletion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppfences), pfencevalues, numfences, flags, hevent.into()).ok()
    }
    pub unsafe fn SetResidencyPriority(&self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetResidencyPriority)(::windows::core::Vtable::as_raw(self), numobjects, ::core::mem::transmute(ppobjects), ppriorities).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12Device1, ::windows::core::IUnknown, ID3D12Object, ID3D12Device);
impl ::core::clone::Clone for ID3D12Device1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Device1 {}
unsafe impl ::core::marker::Sync for ID3D12Device1 {}
unsafe impl ::windows::core::Vtable for ID3D12Device1 {
    type Vtable = ID3D12Device1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Device1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77acce80_638e_4e65_8895_c1f23386863e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Device1_Vtbl {
    pub base__: ID3D12Device_Vtbl,
    pub CreatePipelineLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plibraryblob: *const ::core::ffi::c_void, bloblength: usize, riid: *const ::windows::core::GUID, pppipelinelibrary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventOnMultipleFenceCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfences: *const *mut ::core::ffi::c_void, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventOnMultipleFenceCompletion: usize,
    pub SetResidencyPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numobjects: u32, ppobjects: *const *mut ::core::ffi::c_void, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Device2(::windows::core::IUnknown);
impl ID3D12Device2 {
    pub unsafe fn CreatePipelineState<T>(&self, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID3D12Device2, ::windows::core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1);
impl ::core::clone::Clone for ID3D12Device2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Device2 {}
unsafe impl ::core::marker::Sync for ID3D12Device2 {}
unsafe impl ::windows::core::Vtable for ID3D12Device2 {
    type Vtable = ID3D12Device2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Device2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30baa41e_b15b_475c_a0bb_1af5c5b64328);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Device2_Vtbl {
    pub base__: ID3D12Device1_Vtbl,
    pub CreatePipelineState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Device3(::windows::core::IUnknown);
impl ID3D12Device3 {
    pub unsafe fn OpenExistingHeapFromAddress<T>(&self, paddress: *const ::core::ffi::c_void) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenExistingHeapFromAddress)(::windows::core::Vtable::as_raw(self), paddress, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenExistingHeapFromFileMapping<P0, T>(&self, hfilemapping: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenExistingHeapFromFileMapping)(::windows::core::Vtable::as_raw(self), hfilemapping.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnqueueMakeResident<P0>(&self, flags: D3D12_RESIDENCY_FLAGS, ppobjects: &[ID3D12Pageable], pfencetosignal: P0, fencevaluetosignal: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Fence>>,
    {
        (::windows::core::Vtable::vtable(self).EnqueueMakeResident)(::windows::core::Vtable::as_raw(self), flags, ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr()), pfencetosignal.into().abi(), fencevaluetosignal).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12Device3, ::windows::core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2);
impl ::core::clone::Clone for ID3D12Device3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Device3 {}
unsafe impl ::core::marker::Sync for ID3D12Device3 {}
unsafe impl ::windows::core::Vtable for ID3D12Device3 {
    type Vtable = ID3D12Device3_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Device3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81dadc15_2bad_4392_93c5_101345c4aa98);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Device3_Vtbl {
    pub base__: ID3D12Device2_Vtbl,
    pub OpenExistingHeapFromAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenExistingHeapFromFileMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hfilemapping: super::super::Foundation::HANDLE, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenExistingHeapFromFileMapping: usize,
    pub EnqueueMakeResident: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: D3D12_RESIDENCY_FLAGS, numobjects: u32, ppobjects: *const *mut ::core::ffi::c_void, pfencetosignal: *mut ::core::ffi::c_void, fencevaluetosignal: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Device4(::windows::core::IUnknown);
impl ID3D12Device4 {
    pub unsafe fn CreateCommandList1<T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCommandList1)(::windows::core::Vtable::as_raw(self), nodemask, r#type, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateProtectedResourceSession<T>(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource1<P0, T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreateCommittedResource1)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap1<P0, T>(&self, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreateHeap1)(::windows::core::Vtable::as_raw(self), pdesc, pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource1<P0, T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreateReservedResource1)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo1(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: ::core::option::Option<*mut D3D12_RESOURCE_ALLOCATION_INFO1>) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetResourceAllocationInfo1)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, numresourcedescs, presourcedescs, ::core::mem::transmute(presourceallocationinfo1.unwrap_or(::std::ptr::null_mut())));
        result__
    }
}
::windows::core::interface_hierarchy!(ID3D12Device4, ::windows::core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3);
impl ::core::clone::Clone for ID3D12Device4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Device4 {}
unsafe impl ::core::marker::Sync for ID3D12Device4 {}
unsafe impl ::windows::core::Vtable for ID3D12Device4 {
    type Vtable = ID3D12Device4_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Device4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe865df17_a9ee_46f9_a463_3098315aa2e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Device4_Vtbl {
    pub base__: ID3D12Device3_Vtbl,
    pub CreateCommandList1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS, riid: *const ::windows::core::GUID, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateProtectedResourceSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateCommittedResource1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut ::core::ffi::c_void, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateCommittedResource1: usize,
    pub CreateHeap1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvheap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateReservedResource1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateReservedResource1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetResourceAllocationInfo1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetResourceAllocationInfo1: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Device5(::windows::core::IUnknown);
impl ID3D12Device5 {
    pub unsafe fn CreateLifetimeTracker<P0, T>(&self, powner: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12LifetimeOwner>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLifetimeTracker)(::windows::core::Vtable::as_raw(self), powner.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveDevice(&self) {
        (::windows::core::Vtable::vtable(self).RemoveDevice)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EnumerateMetaCommands(&self, pnummetacommands: *mut u32, pdescs: ::core::option::Option<*mut D3D12_META_COMMAND_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EnumerateMetaCommands)(::windows::core::Vtable::as_raw(self), pnummetacommands, ::core::mem::transmute(pdescs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EnumerateMetaCommandParameters(&self, commandid: *const ::windows::core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: ::core::option::Option<*mut u32>, pparametercount: *mut u32, pparameterdescs: ::core::option::Option<*mut D3D12_META_COMMAND_PARAMETER_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EnumerateMetaCommandParameters)(::windows::core::Vtable::as_raw(self), commandid, stage, ::core::mem::transmute(ptotalstructuresizeinbytes.unwrap_or(::std::ptr::null_mut())), pparametercount, ::core::mem::transmute(pparameterdescs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateMetaCommand<T>(&self, commandid: *const ::windows::core::GUID, nodemask: u32, pcreationparametersdata: ::core::option::Option<*const ::core::ffi::c_void>, creationparametersdatasizeinbytes: usize) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateMetaCommand)(::windows::core::Vtable::as_raw(self), commandid, nodemask, ::core::mem::transmute(pcreationparametersdata.unwrap_or(::std::ptr::null())), creationparametersdatasizeinbytes, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStateObject<T>(&self, pdesc: *const D3D12_STATE_OBJECT_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStateObject)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRaytracingAccelerationStructurePrebuildInfo(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) {
        (::windows::core::Vtable::vtable(self).GetRaytracingAccelerationStructurePrebuildInfo)(::windows::core::Vtable::as_raw(self), pdesc, pinfo)
    }
    pub unsafe fn CheckDriverMatchingIdentifier(&self, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
        (::windows::core::Vtable::vtable(self).CheckDriverMatchingIdentifier)(::windows::core::Vtable::as_raw(self), serializeddatatype, pidentifiertocheck)
    }
}
::windows::core::interface_hierarchy!(ID3D12Device5, ::windows::core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4);
impl ::core::clone::Clone for ID3D12Device5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Device5 {}
unsafe impl ::core::marker::Sync for ID3D12Device5 {}
unsafe impl ::windows::core::Vtable for ID3D12Device5 {
    type Vtable = ID3D12Device5_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Device5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b4f173b_2fea_4b80_8f58_4307191ab95d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Device5_Vtbl {
    pub base__: ID3D12Device4_Vtbl,
    pub CreateLifetimeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powner: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvtracker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub EnumerateMetaCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnummetacommands: *mut u32, pdescs: *mut D3D12_META_COMMAND_DESC) -> ::windows::core::HRESULT,
    pub EnumerateMetaCommandParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: *const ::windows::core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: *mut u32, pparametercount: *mut u32, pparameterdescs: *mut D3D12_META_COMMAND_PARAMETER_DESC) -> ::windows::core::HRESULT,
    pub CreateMetaCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: *const ::windows::core::GUID, nodemask: u32, pcreationparametersdata: *const ::core::ffi::c_void, creationparametersdatasizeinbytes: usize, riid: *const ::windows::core::GUID, ppmetacommand: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStateObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_STATE_OBJECT_DESC, riid: *const ::windows::core::GUID, ppstateobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetRaytracingAccelerationStructurePrebuildInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetRaytracingAccelerationStructurePrebuildInfo: usize,
    pub CheckDriverMatchingIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Device6(::windows::core::IUnknown);
impl ID3D12Device6 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackgroundProcessingMode<P0>(&self, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: P0, pbfurthermeasurementsdesired: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).SetBackgroundProcessingMode)(::windows::core::Vtable::as_raw(self), mode, measurementsaction, heventtosignaluponcompletion.into(), ::core::mem::transmute(pbfurthermeasurementsdesired.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12Device6, ::windows::core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5);
impl ::core::clone::Clone for ID3D12Device6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Device6 {}
unsafe impl ::core::marker::Sync for ID3D12Device6 {}
unsafe impl ::windows::core::Vtable for ID3D12Device6 {
    type Vtable = ID3D12Device6_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Device6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc70b221b_40e4_4a17_89af_025a0727a6dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Device6_Vtbl {
    pub base__: ID3D12Device5_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBackgroundProcessingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: super::super::Foundation::HANDLE, pbfurthermeasurementsdesired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBackgroundProcessingMode: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Device7(::windows::core::IUnknown);
impl ID3D12Device7 {
    pub unsafe fn AddToStateObject<P0, T>(&self, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12StateObject>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddToStateObject)(::windows::core::Vtable::as_raw(self), paddition, pstateobjecttogrowfrom.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateProtectedResourceSession1<T>(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateProtectedResourceSession1)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID3D12Device7, ::windows::core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5, ID3D12Device6);
impl ::core::clone::Clone for ID3D12Device7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Device7 {}
unsafe impl ::core::marker::Sync for ID3D12Device7 {}
unsafe impl ::windows::core::Vtable for ID3D12Device7 {
    type Vtable = ID3D12Device7_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Device7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c014b53_68a1_4b9b_8bd1_dd6046b9358b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Device7_Vtbl {
    pub base__: ID3D12Device6_Vtbl,
    pub AddToStateObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppnewstateobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateProtectedResourceSession1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Device8(::windows::core::IUnknown);
impl ID3D12Device8 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo2(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: ::core::option::Option<*mut D3D12_RESOURCE_ALLOCATION_INFO1>) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetResourceAllocationInfo2)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, numresourcedescs, presourcedescs, ::core::mem::transmute(presourceallocationinfo1.unwrap_or(::std::ptr::null_mut())));
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource2<P0, T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreateCommittedResource2)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource1<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreatePlacedResource1)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateSamplerFeedbackUnorderedAccessView<P0, P1>(&self, ptargetedresource: P0, pfeedbackresource: P1, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).CreateSamplerFeedbackUnorderedAccessView)(::windows::core::Vtable::as_raw(self), ptargetedresource.into().abi(), pfeedbackresource.into().abi(), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints1(&self, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).GetCopyableFootprints1)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
}
::windows::core::interface_hierarchy!(ID3D12Device8, ::windows::core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5, ID3D12Device6, ID3D12Device7);
impl ::core::clone::Clone for ID3D12Device8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Device8 {}
unsafe impl ::core::marker::Sync for ID3D12Device8 {}
unsafe impl ::windows::core::Vtable for ID3D12Device8 {
    type Vtable = ID3D12Device8_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Device8 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9218e6bb_f944_4f7e_a75c_b1b2c7b701f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Device8_Vtbl {
    pub base__: ID3D12Device7_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetResourceAllocationInfo2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetResourceAllocationInfo2: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateCommittedResource2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut ::core::ffi::c_void, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateCommittedResource2: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreatePlacedResource1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheap: *mut ::core::ffi::c_void, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreatePlacedResource1: usize,
    pub CreateSamplerFeedbackUnorderedAccessView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetedresource: *mut ::core::ffi::c_void, pfeedbackresource: *mut ::core::ffi::c_void, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetCopyableFootprints1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetCopyableFootprints1: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Device9(::windows::core::IUnknown);
impl ID3D12Device9 {
    pub unsafe fn CreateShaderCacheSession<T>(&self, pdesc: *const D3D12_SHADER_CACHE_SESSION_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).CreateShaderCacheSession)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn ShaderCacheControl(&self, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ShaderCacheControl)(::windows::core::Vtable::as_raw(self), kinds, control).ok()
    }
    pub unsafe fn CreateCommandQueue1<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC, creatorid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCommandQueue1)(::windows::core::Vtable::as_raw(self), pdesc, creatorid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID3D12Device9, ::windows::core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5, ID3D12Device6, ID3D12Device7, ID3D12Device8);
impl ::core::clone::Clone for ID3D12Device9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Device9 {}
unsafe impl ::core::marker::Sync for ID3D12Device9 {}
unsafe impl ::windows::core::Vtable for ID3D12Device9 {
    type Vtable = ID3D12Device9_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Device9 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c80e962_f032_4f60_bc9e_ebc2cfa1d83c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Device9_Vtbl {
    pub base__: ID3D12Device8_Vtbl,
    pub CreateShaderCacheSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_SHADER_CACHE_SESSION_DESC, riid: *const ::windows::core::GUID, ppvsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShaderCacheControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> ::windows::core::HRESULT,
    pub CreateCommandQueue1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, creatorid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppcommandqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DeviceChild(::windows::core::IUnknown);
impl ID3D12DeviceChild {
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12DeviceChild, ::windows::core::IUnknown, ID3D12Object);
impl ::core::clone::Clone for ID3D12DeviceChild {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DeviceChild {}
unsafe impl ::core::marker::Sync for ID3D12DeviceChild {}
unsafe impl ::windows::core::Vtable for ID3D12DeviceChild {
    type Vtable = ID3D12DeviceChild_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DeviceChild {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905db94b_a00c_4140_9df5_2b64ca9ea357);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DeviceChild_Vtbl {
    pub base__: ID3D12Object_Vtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DeviceRemovedExtendedData(::windows::core::IUnknown);
impl ID3D12DeviceRemovedExtendedData {
    pub unsafe fn GetAutoBreadcrumbsOutput(&self) -> ::windows::core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAutoBreadcrumbsOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPageFaultAllocationOutput(&self) -> ::windows::core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPageFaultAllocationOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID3D12DeviceRemovedExtendedData, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12DeviceRemovedExtendedData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DeviceRemovedExtendedData {}
unsafe impl ::core::marker::Sync for ID3D12DeviceRemovedExtendedData {}
unsafe impl ::windows::core::Vtable for ID3D12DeviceRemovedExtendedData {
    type Vtable = ID3D12DeviceRemovedExtendedData_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DeviceRemovedExtendedData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98931d33_5ae8_4791_aa3c_1a73a2934e71);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DeviceRemovedExtendedData_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAutoBreadcrumbsOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT) -> ::windows::core::HRESULT,
    pub GetPageFaultAllocationOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DeviceRemovedExtendedData1(::windows::core::IUnknown);
impl ID3D12DeviceRemovedExtendedData1 {
    pub unsafe fn GetAutoBreadcrumbsOutput1(&self) -> ::windows::core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAutoBreadcrumbsOutput1)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPageFaultAllocationOutput1(&self) -> ::windows::core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPageFaultAllocationOutput1)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID3D12DeviceRemovedExtendedData1, ::windows::core::IUnknown, ID3D12DeviceRemovedExtendedData);
impl ::core::clone::Clone for ID3D12DeviceRemovedExtendedData1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DeviceRemovedExtendedData1 {}
unsafe impl ::core::marker::Sync for ID3D12DeviceRemovedExtendedData1 {}
unsafe impl ::windows::core::Vtable for ID3D12DeviceRemovedExtendedData1 {
    type Vtable = ID3D12DeviceRemovedExtendedData1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DeviceRemovedExtendedData1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9727a022_cf1d_4dda_9eba_effa653fc506);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DeviceRemovedExtendedData1_Vtbl {
    pub base__: ID3D12DeviceRemovedExtendedData_Vtbl,
    pub GetAutoBreadcrumbsOutput1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1) -> ::windows::core::HRESULT,
    pub GetPageFaultAllocationOutput1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT1) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DeviceRemovedExtendedData2(::windows::core::IUnknown);
impl ID3D12DeviceRemovedExtendedData2 {
    pub unsafe fn GetPageFaultAllocationOutput2(&self, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT2) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPageFaultAllocationOutput2)(::windows::core::Vtable::as_raw(self), poutput).ok()
    }
    pub unsafe fn GetDeviceState(&self) -> D3D12_DRED_DEVICE_STATE {
        (::windows::core::Vtable::vtable(self).GetDeviceState)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12DeviceRemovedExtendedData2, ::windows::core::IUnknown, ID3D12DeviceRemovedExtendedData, ID3D12DeviceRemovedExtendedData1);
impl ::core::clone::Clone for ID3D12DeviceRemovedExtendedData2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DeviceRemovedExtendedData2 {}
unsafe impl ::core::marker::Sync for ID3D12DeviceRemovedExtendedData2 {}
unsafe impl ::windows::core::Vtable for ID3D12DeviceRemovedExtendedData2 {
    type Vtable = ID3D12DeviceRemovedExtendedData2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DeviceRemovedExtendedData2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67fc5816_e4ca_4915_bf18_42541272da54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DeviceRemovedExtendedData2_Vtbl {
    pub base__: ID3D12DeviceRemovedExtendedData1_Vtbl,
    pub GetPageFaultAllocationOutput2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT2) -> ::windows::core::HRESULT,
    pub GetDeviceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D3D12_DRED_DEVICE_STATE,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DeviceRemovedExtendedDataSettings(::windows::core::IUnknown);
impl ID3D12DeviceRemovedExtendedDataSettings {
    pub unsafe fn SetAutoBreadcrumbsEnablement(&self, enablement: D3D12_DRED_ENABLEMENT) {
        (::windows::core::Vtable::vtable(self).SetAutoBreadcrumbsEnablement)(::windows::core::Vtable::as_raw(self), enablement)
    }
    pub unsafe fn SetPageFaultEnablement(&self, enablement: D3D12_DRED_ENABLEMENT) {
        (::windows::core::Vtable::vtable(self).SetPageFaultEnablement)(::windows::core::Vtable::as_raw(self), enablement)
    }
    pub unsafe fn SetWatsonDumpEnablement(&self, enablement: D3D12_DRED_ENABLEMENT) {
        (::windows::core::Vtable::vtable(self).SetWatsonDumpEnablement)(::windows::core::Vtable::as_raw(self), enablement)
    }
}
::windows::core::interface_hierarchy!(ID3D12DeviceRemovedExtendedDataSettings, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12DeviceRemovedExtendedDataSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DeviceRemovedExtendedDataSettings {}
unsafe impl ::core::marker::Sync for ID3D12DeviceRemovedExtendedDataSettings {}
unsafe impl ::windows::core::Vtable for ID3D12DeviceRemovedExtendedDataSettings {
    type Vtable = ID3D12DeviceRemovedExtendedDataSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DeviceRemovedExtendedDataSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82bc481c_6b9b_4030_aedb_7ee3d1df1e63);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DeviceRemovedExtendedDataSettings_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetAutoBreadcrumbsEnablement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT),
    pub SetPageFaultEnablement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT),
    pub SetWatsonDumpEnablement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12DeviceRemovedExtendedDataSettings1(::windows::core::IUnknown);
impl ID3D12DeviceRemovedExtendedDataSettings1 {
    pub unsafe fn SetBreadcrumbContextEnablement(&self, enablement: D3D12_DRED_ENABLEMENT) {
        (::windows::core::Vtable::vtable(self).SetBreadcrumbContextEnablement)(::windows::core::Vtable::as_raw(self), enablement)
    }
}
::windows::core::interface_hierarchy!(ID3D12DeviceRemovedExtendedDataSettings1, ::windows::core::IUnknown, ID3D12DeviceRemovedExtendedDataSettings);
impl ::core::clone::Clone for ID3D12DeviceRemovedExtendedDataSettings1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12DeviceRemovedExtendedDataSettings1 {}
unsafe impl ::core::marker::Sync for ID3D12DeviceRemovedExtendedDataSettings1 {}
unsafe impl ::windows::core::Vtable for ID3D12DeviceRemovedExtendedDataSettings1 {
    type Vtable = ID3D12DeviceRemovedExtendedDataSettings1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12DeviceRemovedExtendedDataSettings1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbd5ae51_3317_4f0a_adf9_1d7cedcaae0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DeviceRemovedExtendedDataSettings1_Vtbl {
    pub base__: ID3D12DeviceRemovedExtendedDataSettings_Vtbl,
    pub SetBreadcrumbContextEnablement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Fence(::windows::core::IUnknown);
impl ID3D12Fence {
    pub unsafe fn GetCompletedValue(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetCompletedValue)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventOnCompletion<P0>(&self, value: u64, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).SetEventOnCompletion)(::windows::core::Vtable::as_raw(self), value, hevent.into()).ok()
    }
    pub unsafe fn Signal(&self, value: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Signal)(::windows::core::Vtable::as_raw(self), value).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12Fence, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ::core::clone::Clone for ID3D12Fence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Fence {}
unsafe impl ::core::marker::Sync for ID3D12Fence {}
unsafe impl ::windows::core::Vtable for ID3D12Fence {
    type Vtable = ID3D12Fence_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Fence {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a753dcf_c4d8_4b91_adf6_be5a60d95a76);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Fence_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub GetCompletedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventOnCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventOnCompletion: usize,
    pub Signal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Fence1(::windows::core::IUnknown);
impl ID3D12Fence1 {
    pub unsafe fn GetCreationFlags(&self) -> D3D12_FENCE_FLAGS {
        (::windows::core::Vtable::vtable(self).GetCreationFlags)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12Fence1, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable, ID3D12Fence);
impl ::core::clone::Clone for ID3D12Fence1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Fence1 {}
unsafe impl ::core::marker::Sync for ID3D12Fence1 {}
unsafe impl ::windows::core::Vtable for ID3D12Fence1 {
    type Vtable = ID3D12Fence1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Fence1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x433685fe_e22b_4ca0_a8db_b5b4f4dd0e4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Fence1_Vtbl {
    pub base__: ID3D12Fence_Vtbl,
    pub GetCreationFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D3D12_FENCE_FLAGS,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12FunctionParameterReflection(::std::ptr::NonNull<::std::ffi::c_void>);
impl ID3D12FunctionParameterReflection {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_PARAMETER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
}
impl ::core::clone::Clone for ID3D12FunctionParameterReflection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12FunctionParameterReflection {}
unsafe impl ::core::marker::Sync for ID3D12FunctionParameterReflection {}
unsafe impl ::windows::core::Vtable for ID3D12FunctionParameterReflection {
    type Vtable = ID3D12FunctionParameterReflection_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12FunctionParameterReflection_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12FunctionReflection(::std::ptr::NonNull<::std::ffi::c_void>);
impl ID3D12FunctionReflection {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_FUNCTION_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetConstantBufferByIndex(&self, bufferindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
        (::windows::core::Vtable::vtable(self).GetConstantBufferByIndex)(::windows::core::Vtable::as_raw(self), bufferindex)
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetConstantBufferByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetResourceBindingDesc)(::windows::core::Vtable::as_raw(self), resourceindex, pdesc).ok()
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D12ShaderReflectionVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetVariableByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetResourceBindingDescByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), pdesc).ok()
    }
    pub unsafe fn GetFunctionParameter(&self, parameterindex: i32) -> ::core::option::Option<ID3D12FunctionParameterReflection> {
        (::windows::core::Vtable::vtable(self).GetFunctionParameter)(::windows::core::Vtable::as_raw(self), parameterindex)
    }
}
impl ::core::clone::Clone for ID3D12FunctionReflection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12FunctionReflection {}
unsafe impl ::core::marker::Sync for ID3D12FunctionReflection {}
unsafe impl ::windows::core::Vtable for ID3D12FunctionReflection {
    type Vtable = ID3D12FunctionReflection_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12FunctionReflection_Vtbl {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_FUNCTION_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bufferindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDescByName: usize,
    pub GetFunctionParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: i32) -> ::core::option::Option<ID3D12FunctionParameterReflection>,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList(::windows::core::IUnknown);
impl ID3D12GraphicsCommandList {
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset<P0, P1>(&self, pallocator: P0, pinitialstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi(), pinitialstate.into().abi()).ok()
    }
    pub unsafe fn ClearState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).ClearState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).Dispatch)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn CopyBufferRegion<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, numbytes: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).CopyBufferRegion)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, numbytes)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CopyTextureRegion(&self, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: ::core::option::Option<*const D3D12_BOX>) {
        (::windows::core::Vtable::vtable(self).CopyTextureRegion)(::windows::core::Vtable::as_raw(self), pdst, dstx, dsty, dstz, psrc, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiles<P0, P1>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: P1, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).CopyTiles)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.into().abi(), bufferstartoffsetinbytes, flags)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), primitivetopology)
    }
    pub unsafe fn RSSetViewports(&self, pviewports: &[D3D12_VIEWPORT]) {
        (::windows::core::Vtable::vtable(self).RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.len() as _, ::core::mem::transmute(pviewports.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    pub unsafe fn OMSetBlendFactor(&self, blendfactor: ::core::option::Option<&[f32; 4]>) {
        (::windows::core::Vtable::vtable(self).OMSetBlendFactor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(blendfactor.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetStencilRef(&self, stencilref: u32) {
        (::windows::core::Vtable::vtable(self).OMSetStencilRef)(::windows::core::Vtable::as_raw(self), stencilref)
    }
    pub unsafe fn SetPipelineState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).SetPipelineState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    pub unsafe fn ExecuteBundle<P0>(&self, pcommandlist: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12GraphicsCommandList>>,
    {
        (::windows::core::Vtable::vtable(self).ExecuteBundle)(::windows::core::Vtable::as_raw(self), pcommandlist.into().abi())
    }
    pub unsafe fn SetDescriptorHeaps(&self, ppdescriptorheaps: &[ID3D12DescriptorHeap]) {
        (::windows::core::Vtable::vtable(self).SetDescriptorHeaps)(::windows::core::Vtable::as_raw(self), ppdescriptorheaps.len() as _, ::core::mem::transmute(ppdescriptorheaps.as_ptr()))
    }
    pub unsafe fn SetComputeRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).SetComputeRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetGraphicsRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).SetGraphicsRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetComputeRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).SetComputeRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetGraphicsRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).SetGraphicsRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetComputeRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).SetComputeRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).SetGraphicsRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).SetComputeRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).SetGraphicsRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).SetComputeRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).SetGraphicsRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).SetComputeRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).SetGraphicsRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).SetComputeRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).SetGraphicsRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer(&self, pview: ::core::option::Option<*const D3D12_INDEX_BUFFER_VIEW>) {
        (::windows::core::Vtable::vtable(self).IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pview.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_VERTEX_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn SOSetTargets(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_STREAM_OUTPUT_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).SOSetTargets)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OMSetRenderTargets<P0>(&self, numrendertargetdescriptors: u32, prendertargetdescriptors: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>, rtssinglehandletodescriptorrange: P0, pdepthstencildescriptor: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), numrendertargetdescriptors, ::core::mem::transmute(prendertargetdescriptors.unwrap_or(::std::ptr::null())), rtssinglehandletodescriptorrange.into(), ::core::mem::transmute(pdepthstencildescriptor.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearDepthStencilView(&self, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(depthstencilview), clearflags, depth, stencil, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearRenderTargetView(&self, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rendertargetview), colorrgba, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const u32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).ClearUnorderedAccessViewUint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const f32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).ClearUnorderedAccessViewFloat)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ExecuteIndirect<P0, P1, P2>(&self, pcommandsignature: P0, maxcommandcount: u32, pargumentbuffer: P1, argumentbufferoffset: u64, pcountbuffer: P2, countbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandSignature>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).ExecuteIndirect)(::windows::core::Vtable::as_raw(self), pcommandsignature.into().abi(), maxcommandcount, pargumentbuffer.into().abi(), argumentbufferoffset, pcountbuffer.into().abi(), countbufferoffset)
    }
}
::windows::core::interface_hierarchy!(ID3D12GraphicsCommandList, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList);
impl ::core::clone::Clone for ID3D12GraphicsCommandList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12GraphicsCommandList {}
unsafe impl ::core::marker::Sync for ID3D12GraphicsCommandList {}
unsafe impl ::windows::core::Vtable for ID3D12GraphicsCommandList {
    type Vtable = ID3D12GraphicsCommandList_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12GraphicsCommandList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b160d0f_ac1b_4185_8ba8_b3ae42a5a455);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12GraphicsCommandList_Vtbl {
    pub base__: ID3D12CommandList_Vtbl,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pallocator: *mut ::core::ffi::c_void, pinitialstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppipelinestate: *mut ::core::ffi::c_void),
    pub DrawInstanced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32),
    pub DrawIndexedInstanced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32),
    pub Dispatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32),
    pub CopyBufferRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstbuffer: *mut ::core::ffi::c_void, dstoffset: u64, psrcbuffer: *mut ::core::ffi::c_void, srcoffset: u64, numbytes: u64),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CopyTextureRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: *const D3D12_BOX),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CopyTextureRegion: usize,
    pub CopyResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, psrcresource: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptiledresource: *mut ::core::ffi::c_void, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: *mut ::core::ffi::c_void, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS),
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTiles: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResolveSubresource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResolveSubresource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub IASetPrimitiveTopology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    IASetPrimitiveTopology: usize,
    pub RSSetViewports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D12_VIEWPORT),
    #[cfg(feature = "Win32_Foundation")]
    pub RSSetScissorRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))]
    RSSetScissorRects: usize,
    pub OMSetBlendFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blendfactor: *const f32),
    pub OMSetStencilRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stencilref: u32),
    pub SetPipelineState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppipelinestate: *mut ::core::ffi::c_void),
    pub ResourceBarrier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numbarriers: u32, pbarriers: *const D3D12_RESOURCE_BARRIER),
    pub ExecuteBundle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommandlist: *mut ::core::ffi::c_void),
    pub SetDescriptorHeaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numdescriptorheaps: u32, ppdescriptorheaps: *const *mut ::core::ffi::c_void),
    pub SetComputeRootSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prootsignature: *mut ::core::ffi::c_void),
    pub SetGraphicsRootSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prootsignature: *mut ::core::ffi::c_void),
    pub SetComputeRootDescriptorTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE),
    pub SetGraphicsRootDescriptorTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE),
    pub SetComputeRoot32BitConstant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32),
    pub SetGraphicsRoot32BitConstant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32),
    pub SetComputeRoot32BitConstants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32),
    pub SetGraphicsRoot32BitConstants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32),
    pub SetComputeRootConstantBufferView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64),
    pub SetGraphicsRootConstantBufferView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64),
    pub SetComputeRootShaderResourceView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64),
    pub SetGraphicsRootShaderResourceView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64),
    pub SetComputeRootUnorderedAccessView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64),
    pub SetGraphicsRootUnorderedAccessView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub IASetIndexBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pview: *const D3D12_INDEX_BUFFER_VIEW),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    IASetIndexBuffer: usize,
    pub IASetVertexBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_VERTEX_BUFFER_VIEW),
    pub SOSetTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_STREAM_OUTPUT_BUFFER_VIEW),
    #[cfg(feature = "Win32_Foundation")]
    pub OMSetRenderTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numrendertargetdescriptors: u32, prendertargetdescriptors: *const D3D12_CPU_DESCRIPTOR_HANDLE, rtssinglehandletodescriptorrange: super::super::Foundation::BOOL, pdepthstencildescriptor: *const D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(not(feature = "Win32_Foundation"))]
    OMSetRenderTargets: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClearDepthStencilView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, numrects: u32, prects: *const super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))]
    ClearDepthStencilView: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClearRenderTargetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))]
    ClearRenderTargetView: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClearUnorderedAccessViewUint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: *mut ::core::ffi::c_void, values: *const u32, numrects: u32, prects: *const super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))]
    ClearUnorderedAccessViewUint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClearUnorderedAccessViewFloat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: *mut ::core::ffi::c_void, values: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))]
    ClearUnorderedAccessViewFloat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DiscardResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pregion: *const D3D12_DISCARD_REGION),
    #[cfg(not(feature = "Win32_Foundation"))]
    DiscardResource: usize,
    pub BeginQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqueryheap: *mut ::core::ffi::c_void, r#type: D3D12_QUERY_TYPE, index: u32),
    pub EndQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqueryheap: *mut ::core::ffi::c_void, r#type: D3D12_QUERY_TYPE, index: u32),
    pub ResolveQueryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqueryheap: *mut ::core::ffi::c_void, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: *mut ::core::ffi::c_void, aligneddestinationbufferoffset: u64),
    pub SetPredication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP),
    pub SetMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32),
    pub BeginEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadata: u32, pdata: *const ::core::ffi::c_void, size: u32),
    pub EndEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub ExecuteIndirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommandsignature: *mut ::core::ffi::c_void, maxcommandcount: u32, pargumentbuffer: *mut ::core::ffi::c_void, argumentbufferoffset: u64, pcountbuffer: *mut ::core::ffi::c_void, countbufferoffset: u64),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList1(::windows::core::IUnknown);
impl ID3D12GraphicsCommandList1 {
    pub unsafe fn AtomicCopyBufferUINT<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).AtomicCopyBufferUINT)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn AtomicCopyBufferUINT64<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).AtomicCopyBufferUINT64)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn OMSetDepthBounds(&self, min: f32, max: f32) {
        (::windows::core::Vtable::vtable(self).OMSetDepthBounds)(::windows::core::Vtable::as_raw(self), min, max)
    }
    pub unsafe fn SetSamplePositions(&self, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
        (::windows::core::Vtable::vtable(self).SetSamplePositions)(::windows::core::Vtable::as_raw(self), numsamplesperpixel, numpixels, psamplepositions)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn ResolveSubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: P1, srcsubresource: u32, psrcrect: ::core::option::Option<*const super::super::Foundation::RECT>, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).ResolveSubresourceRegion)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcrect.unwrap_or(::std::ptr::null())), format, resolvemode)
    }
    pub unsafe fn SetViewInstanceMask(&self, mask: u32) {
        (::windows::core::Vtable::vtable(self).SetViewInstanceMask)(::windows::core::Vtable::as_raw(self), mask)
    }
}
::windows::core::interface_hierarchy!(ID3D12GraphicsCommandList1, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList);
impl ::core::clone::Clone for ID3D12GraphicsCommandList1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12GraphicsCommandList1 {}
unsafe impl ::core::marker::Sync for ID3D12GraphicsCommandList1 {}
unsafe impl ::windows::core::Vtable for ID3D12GraphicsCommandList1 {
    type Vtable = ID3D12GraphicsCommandList1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12GraphicsCommandList1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x553103fb_1fe7_4557_bb38_946d7d0e7ca7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12GraphicsCommandList1_Vtbl {
    pub base__: ID3D12GraphicsCommandList_Vtbl,
    pub AtomicCopyBufferUINT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstbuffer: *mut ::core::ffi::c_void, dstoffset: u64, psrcbuffer: *mut ::core::ffi::c_void, srcoffset: u64, dependencies: u32, ppdependentresources: *const *mut ::core::ffi::c_void, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64),
    pub AtomicCopyBufferUINT64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstbuffer: *mut ::core::ffi::c_void, dstoffset: u64, psrcbuffer: *mut ::core::ffi::c_void, srcoffset: u64, dependencies: u32, ppdependentresources: *const *mut ::core::ffi::c_void, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64),
    pub OMSetDepthBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, min: f32, max: f32),
    pub SetSamplePositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub ResolveSubresourceRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, psrcrect: *const super::super::Foundation::RECT, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    ResolveSubresourceRegion: usize,
    pub SetViewInstanceMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: u32),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList2(::windows::core::IUnknown);
impl ID3D12GraphicsCommandList2 {
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: ::core::option::Option<*const D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (::windows::core::Vtable::vtable(self).WriteBufferImmediate)(::windows::core::Vtable::as_raw(self), count, pparams, ::core::mem::transmute(pmodes.unwrap_or(::std::ptr::null())))
    }
}
::windows::core::interface_hierarchy!(ID3D12GraphicsCommandList2, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1);
impl ::core::clone::Clone for ID3D12GraphicsCommandList2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12GraphicsCommandList2 {}
unsafe impl ::core::marker::Sync for ID3D12GraphicsCommandList2 {}
unsafe impl ::windows::core::Vtable for ID3D12GraphicsCommandList2 {
    type Vtable = ID3D12GraphicsCommandList2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12GraphicsCommandList2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38c3e585_ff17_412c_9150_4fc6f9d72a28);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12GraphicsCommandList2_Vtbl {
    pub base__: ID3D12GraphicsCommandList1_Vtbl,
    pub WriteBufferImmediate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const D3D12_WRITEBUFFERIMMEDIATE_MODE),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList3(::windows::core::IUnknown);
impl ID3D12GraphicsCommandList3 {
    pub unsafe fn SetProtectedResourceSession<P0>(&self, pprotectedresourcesession: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
    {
        (::windows::core::Vtable::vtable(self).SetProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pprotectedresourcesession.into().abi())
    }
}
::windows::core::interface_hierarchy!(ID3D12GraphicsCommandList3, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList2);
impl ::core::clone::Clone for ID3D12GraphicsCommandList3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12GraphicsCommandList3 {}
unsafe impl ::core::marker::Sync for ID3D12GraphicsCommandList3 {}
unsafe impl ::windows::core::Vtable for ID3D12GraphicsCommandList3 {
    type Vtable = ID3D12GraphicsCommandList3_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12GraphicsCommandList3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fda83a7_b84c_4e38_9ac8_c7bd22016b3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12GraphicsCommandList3_Vtbl {
    pub base__: ID3D12GraphicsCommandList2_Vtbl,
    pub SetProtectedResourceSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotectedresourcesession: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList4(::windows::core::IUnknown);
impl ID3D12GraphicsCommandList4 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn BeginRenderPass(&self, prendertargets: ::core::option::Option<&[D3D12_RENDER_PASS_RENDER_TARGET_DESC]>, pdepthstencil: ::core::option::Option<*const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC>, flags: D3D12_RENDER_PASS_FLAGS) {
        (::windows::core::Vtable::vtable(self).BeginRenderPass)(::windows::core::Vtable::as_raw(self), prendertargets.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(prendertargets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pdepthstencil.unwrap_or(::std::ptr::null())), flags)
    }
    pub unsafe fn EndRenderPass(&self) {
        (::windows::core::Vtable::vtable(self).EndRenderPass)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn InitializeMetaCommand<P0>(&self, pmetacommand: P0, pinitializationparametersdata: ::core::option::Option<*const ::core::ffi::c_void>, initializationparametersdatasizeinbytes: usize)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12MetaCommand>>,
    {
        (::windows::core::Vtable::vtable(self).InitializeMetaCommand)(::windows::core::Vtable::as_raw(self), pmetacommand.into().abi(), ::core::mem::transmute(pinitializationparametersdata.unwrap_or(::std::ptr::null())), initializationparametersdatasizeinbytes)
    }
    pub unsafe fn ExecuteMetaCommand<P0>(&self, pmetacommand: P0, pexecutionparametersdata: ::core::option::Option<*const ::core::ffi::c_void>, executionparametersdatasizeinbytes: usize)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12MetaCommand>>,
    {
        (::windows::core::Vtable::vtable(self).ExecuteMetaCommand)(::windows::core::Vtable::as_raw(self), pmetacommand.into().abi(), ::core::mem::transmute(pexecutionparametersdata.unwrap_or(::std::ptr::null())), executionparametersdatasizeinbytes)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn BuildRaytracingAccelerationStructure(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, ppostbuildinfodescs: ::core::option::Option<&[D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC]>) {
        (::windows::core::Vtable::vtable(self).BuildRaytracingAccelerationStructure)(::windows::core::Vtable::as_raw(self), pdesc, ppostbuildinfodescs.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppostbuildinfodescs.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn EmitRaytracingAccelerationStructurePostbuildInfo(&self, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, psourceaccelerationstructuredata: &[u64]) {
        (::windows::core::Vtable::vtable(self).EmitRaytracingAccelerationStructurePostbuildInfo)(::windows::core::Vtable::as_raw(self), pdesc, psourceaccelerationstructuredata.len() as _, ::core::mem::transmute(psourceaccelerationstructuredata.as_ptr()))
    }
    pub unsafe fn CopyRaytracingAccelerationStructure(&self, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) {
        (::windows::core::Vtable::vtable(self).CopyRaytracingAccelerationStructure)(::windows::core::Vtable::as_raw(self), destaccelerationstructuredata, sourceaccelerationstructuredata, mode)
    }
    pub unsafe fn SetPipelineState1<P0>(&self, pstateobject: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12StateObject>>,
    {
        (::windows::core::Vtable::vtable(self).SetPipelineState1)(::windows::core::Vtable::as_raw(self), pstateobject.into().abi())
    }
    pub unsafe fn DispatchRays(&self, pdesc: *const D3D12_DISPATCH_RAYS_DESC) {
        (::windows::core::Vtable::vtable(self).DispatchRays)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
::windows::core::interface_hierarchy!(ID3D12GraphicsCommandList4, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList2, ID3D12GraphicsCommandList3);
impl ::core::clone::Clone for ID3D12GraphicsCommandList4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12GraphicsCommandList4 {}
unsafe impl ::core::marker::Sync for ID3D12GraphicsCommandList4 {}
unsafe impl ::windows::core::Vtable for ID3D12GraphicsCommandList4 {
    type Vtable = ID3D12GraphicsCommandList4_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12GraphicsCommandList4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8754318e_d3a9_4541_98cf_645b50dc4874);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12GraphicsCommandList4_Vtbl {
    pub base__: ID3D12GraphicsCommandList3_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub BeginRenderPass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numrendertargets: u32, prendertargets: *const D3D12_RENDER_PASS_RENDER_TARGET_DESC, pdepthstencil: *const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC, flags: D3D12_RENDER_PASS_FLAGS),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    BeginRenderPass: usize,
    pub EndRenderPass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub InitializeMetaCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmetacommand: *mut ::core::ffi::c_void, pinitializationparametersdata: *const ::core::ffi::c_void, initializationparametersdatasizeinbytes: usize),
    pub ExecuteMetaCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmetacommand: *mut ::core::ffi::c_void, pexecutionparametersdata: *const ::core::ffi::c_void, executionparametersdatasizeinbytes: usize),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub BuildRaytracingAccelerationStructure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, numpostbuildinfodescs: u32, ppostbuildinfodescs: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    BuildRaytracingAccelerationStructure: usize,
    pub EmitRaytracingAccelerationStructurePostbuildInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, numsourceaccelerationstructures: u32, psourceaccelerationstructuredata: *const u64),
    pub CopyRaytracingAccelerationStructure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE),
    pub SetPipelineState1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstateobject: *mut ::core::ffi::c_void),
    pub DispatchRays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D12_DISPATCH_RAYS_DESC),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList5(::windows::core::IUnknown);
impl ID3D12GraphicsCommandList5 {
    pub unsafe fn RSSetShadingRate(&self, baseshadingrate: D3D12_SHADING_RATE, combiners: ::core::option::Option<*const D3D12_SHADING_RATE_COMBINER>) {
        (::windows::core::Vtable::vtable(self).RSSetShadingRate)(::windows::core::Vtable::as_raw(self), baseshadingrate, ::core::mem::transmute(combiners.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn RSSetShadingRateImage<P0>(&self, shadingrateimage: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).RSSetShadingRateImage)(::windows::core::Vtable::as_raw(self), shadingrateimage.into().abi())
    }
}
::windows::core::interface_hierarchy!(ID3D12GraphicsCommandList5, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList2, ID3D12GraphicsCommandList3, ID3D12GraphicsCommandList4);
impl ::core::clone::Clone for ID3D12GraphicsCommandList5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12GraphicsCommandList5 {}
unsafe impl ::core::marker::Sync for ID3D12GraphicsCommandList5 {}
unsafe impl ::windows::core::Vtable for ID3D12GraphicsCommandList5 {
    type Vtable = ID3D12GraphicsCommandList5_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12GraphicsCommandList5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55050859_4024_474c_87f5_6472eaee44ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12GraphicsCommandList5_Vtbl {
    pub base__: ID3D12GraphicsCommandList4_Vtbl,
    pub RSSetShadingRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseshadingrate: D3D12_SHADING_RATE, combiners: *const D3D12_SHADING_RATE_COMBINER),
    pub RSSetShadingRateImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shadingrateimage: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList6(::windows::core::IUnknown);
impl ID3D12GraphicsCommandList6 {
    pub unsafe fn DispatchMesh(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).DispatchMesh)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
}
::windows::core::interface_hierarchy!(ID3D12GraphicsCommandList6, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList2, ID3D12GraphicsCommandList3, ID3D12GraphicsCommandList4, ID3D12GraphicsCommandList5);
impl ::core::clone::Clone for ID3D12GraphicsCommandList6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12GraphicsCommandList6 {}
unsafe impl ::core::marker::Sync for ID3D12GraphicsCommandList6 {}
unsafe impl ::windows::core::Vtable for ID3D12GraphicsCommandList6 {
    type Vtable = ID3D12GraphicsCommandList6_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12GraphicsCommandList6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3827890_e548_4cfa_96cf_5689a9370f80);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12GraphicsCommandList6_Vtbl {
    pub base__: ID3D12GraphicsCommandList5_Vtbl,
    pub DispatchMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Heap(::windows::core::IUnknown);
impl ID3D12Heap {
    pub unsafe fn GetDesc(&self) -> D3D12_HEAP_DESC {
        let mut result__: D3D12_HEAP_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
::windows::core::interface_hierarchy!(ID3D12Heap, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ::core::clone::Clone for ID3D12Heap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Heap {}
unsafe impl ::core::marker::Sync for ID3D12Heap {}
unsafe impl ::windows::core::Vtable for ID3D12Heap {
    type Vtable = ID3D12Heap_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Heap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b3b2502_6e51_45b3_90ee_9884265e8df3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Heap_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_HEAP_DESC),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Heap1(::windows::core::IUnknown);
impl ID3D12Heap1 {
    pub unsafe fn GetProtectedResourceSession<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).GetProtectedResourceSession)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12Heap1, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable, ID3D12Heap);
impl ::core::clone::Clone for ID3D12Heap1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Heap1 {}
unsafe impl ::core::marker::Sync for ID3D12Heap1 {}
unsafe impl ::windows::core::Vtable for ID3D12Heap1 {
    type Vtable = ID3D12Heap1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Heap1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x572f7389_2168_49e3_9693_d6df5871bf6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Heap1_Vtbl {
    pub base__: ID3D12Heap_Vtbl,
    pub GetProtectedResourceSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12InfoQueue(::windows::core::IUnknown);
impl ID3D12InfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, messagecountlimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMessageCountLimit)(::windows::core::Vtable::as_raw(self), messagecountlimit).ok()
    }
    pub unsafe fn ClearStoredMessages(&self) {
        (::windows::core::Vtable::vtable(self).ClearStoredMessages)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMessage(&self, messageindex: u64, pmessage: ::core::option::Option<*mut D3D12_MESSAGE>, pmessagebytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetMessage)(::windows::core::Vtable::as_raw(self), messageindex, ::core::mem::transmute(pmessage.unwrap_or(::std::ptr::null_mut())), pmessagebytelength).ok()
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetNumMessagesAllowedByStorageFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetNumMessagesDeniedByStorageFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNumStoredMessages(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetNumStoredMessages)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetNumStoredMessagesAllowedByRetrievalFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetNumMessagesDiscardedByMessageCountLimit)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMessageCountLimit(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetMessageCountLimit)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AddStorageFilterEntries(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddStorageFilterEntries)(::windows::core::Vtable::as_raw(self), pfilter).ok()
    }
    pub unsafe fn GetStorageFilter(&self, pfilter: ::core::option::Option<*mut D3D12_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetStorageFilter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfilter.unwrap_or(::std::ptr::null_mut())), pfilterbytelength).ok()
    }
    pub unsafe fn ClearStorageFilter(&self) {
        (::windows::core::Vtable::vtable(self).ClearStorageFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PushEmptyStorageFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PushEmptyStorageFilter)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PushCopyOfStorageFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PushCopyOfStorageFilter)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PushStorageFilter(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PushStorageFilter)(::windows::core::Vtable::as_raw(self), pfilter).ok()
    }
    pub unsafe fn PopStorageFilter(&self) {
        (::windows::core::Vtable::vtable(self).PopStorageFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStorageFilterStackSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetStorageFilterStackSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddRetrievalFilterEntries)(::windows::core::Vtable::as_raw(self), pfilter).ok()
    }
    pub unsafe fn GetRetrievalFilter(&self, pfilter: ::core::option::Option<*mut D3D12_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetRetrievalFilter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfilter.unwrap_or(::std::ptr::null_mut())), pfilterbytelength).ok()
    }
    pub unsafe fn ClearRetrievalFilter(&self) {
        (::windows::core::Vtable::vtable(self).ClearRetrievalFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PushEmptyRetrievalFilter)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PushCopyOfRetrievalFilter)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PushRetrievalFilter(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PushRetrievalFilter)(::windows::core::Vtable::as_raw(self), pfilter).ok()
    }
    pub unsafe fn PopRetrievalFilter(&self) {
        (::windows::core::Vtable::vtable(self).PopRetrievalFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetRetrievalFilterStackSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AddMessage<P0>(&self, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).AddMessage)(::windows::core::Vtable::as_raw(self), category, severity, id, pdescription.into().abi()).ok()
    }
    pub unsafe fn AddApplicationMessage<P0>(&self, severity: D3D12_MESSAGE_SEVERITY, pdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).AddApplicationMessage)(::windows::core::Vtable::as_raw(self), severity, pdescription.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnCategory<P0>(&self, category: D3D12_MESSAGE_CATEGORY, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetBreakOnCategory)(::windows::core::Vtable::as_raw(self), category, benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnSeverity<P0>(&self, severity: D3D12_MESSAGE_SEVERITY, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetBreakOnSeverity)(::windows::core::Vtable::as_raw(self), severity, benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnID<P0>(&self, id: D3D12_MESSAGE_ID, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetBreakOnID)(::windows::core::Vtable::as_raw(self), id, benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnCategory(&self, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).GetBreakOnCategory)(::windows::core::Vtable::as_raw(self), category)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnSeverity(&self, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).GetBreakOnSeverity)(::windows::core::Vtable::as_raw(self), severity)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnID(&self, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).GetBreakOnID)(::windows::core::Vtable::as_raw(self), id)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMuteDebugOutput<P0>(&self, bmute: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetMuteDebugOutput)(::windows::core::Vtable::as_raw(self), bmute.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMuteDebugOutput(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).GetMuteDebugOutput)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12InfoQueue, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12InfoQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12InfoQueue {}
unsafe impl ::core::marker::Sync for ID3D12InfoQueue {}
unsafe impl ::windows::core::Vtable for ID3D12InfoQueue {
    type Vtable = ID3D12InfoQueue_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12InfoQueue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0742a90b_c387_483f_b946_30a7e4e61458);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12InfoQueue_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D12_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetNumStoredMessagesAllowedByRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub AddMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnCategory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnSeverity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: D3D12_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnCategory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnSeverity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMuteDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMuteDebugOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMuteDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMuteDebugOutput: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12InfoQueue1(::windows::core::IUnknown);
impl ID3D12InfoQueue1 {
    pub unsafe fn RegisterMessageCallback(&self, callbackfunc: D3D12MessageFunc, callbackfilterflags: D3D12_MESSAGE_CALLBACK_FLAGS, pcontext: *const ::core::ffi::c_void, pcallbackcookie: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RegisterMessageCallback)(::windows::core::Vtable::as_raw(self), callbackfunc, callbackfilterflags, pcontext, pcallbackcookie).ok()
    }
    pub unsafe fn UnregisterMessageCallback(&self, callbackcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterMessageCallback)(::windows::core::Vtable::as_raw(self), callbackcookie).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12InfoQueue1, ::windows::core::IUnknown, ID3D12InfoQueue);
impl ::core::clone::Clone for ID3D12InfoQueue1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12InfoQueue1 {}
unsafe impl ::core::marker::Sync for ID3D12InfoQueue1 {}
unsafe impl ::windows::core::Vtable for ID3D12InfoQueue1 {
    type Vtable = ID3D12InfoQueue1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12InfoQueue1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2852dd88_b484_4c0c_b6b1_67168500e600);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12InfoQueue1_Vtbl {
    pub base__: ID3D12InfoQueue_Vtbl,
    pub RegisterMessageCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callbackfunc: D3D12MessageFunc, callbackfilterflags: D3D12_MESSAGE_CALLBACK_FLAGS, pcontext: *const ::core::ffi::c_void, pcallbackcookie: *mut u32) -> ::windows::core::HRESULT,
    pub UnregisterMessageCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callbackcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12LibraryReflection(::windows::core::IUnknown);
impl ID3D12LibraryReflection {
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D12_LIBRARY_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFunctionByIndex(&self, functionindex: i32) -> ::core::option::Option<ID3D12FunctionReflection> {
        (::windows::core::Vtable::vtable(self).GetFunctionByIndex)(::windows::core::Vtable::as_raw(self), functionindex)
    }
}
::windows::core::interface_hierarchy!(ID3D12LibraryReflection, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12LibraryReflection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12LibraryReflection {}
unsafe impl ::core::marker::Sync for ID3D12LibraryReflection {}
unsafe impl ::windows::core::Vtable for ID3D12LibraryReflection {
    type Vtable = ID3D12LibraryReflection_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12LibraryReflection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e349d19_54db_4a56_9dc9_119d87bdb804);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12LibraryReflection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_LIBRARY_DESC) -> ::windows::core::HRESULT,
    pub GetFunctionByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionindex: i32) -> ::core::option::Option<ID3D12FunctionReflection>,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12LifetimeOwner(::windows::core::IUnknown);
impl ID3D12LifetimeOwner {
    pub unsafe fn LifetimeStateUpdated(&self, newstate: D3D12_LIFETIME_STATE) {
        (::windows::core::Vtable::vtable(self).LifetimeStateUpdated)(::windows::core::Vtable::as_raw(self), newstate)
    }
}
::windows::core::interface_hierarchy!(ID3D12LifetimeOwner, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12LifetimeOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12LifetimeOwner {}
unsafe impl ::core::marker::Sync for ID3D12LifetimeOwner {}
unsafe impl ::windows::core::Vtable for ID3D12LifetimeOwner {
    type Vtable = ID3D12LifetimeOwner_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12LifetimeOwner {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe667af9f_cd56_4f46_83ce_032e595d70a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12LifetimeOwner_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub LifetimeStateUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: D3D12_LIFETIME_STATE),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12LifetimeTracker(::windows::core::IUnknown);
impl ID3D12LifetimeTracker {
    pub unsafe fn DestroyOwnedObject<P0>(&self, pobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12DeviceChild>>,
    {
        (::windows::core::Vtable::vtable(self).DestroyOwnedObject)(::windows::core::Vtable::as_raw(self), pobject.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12LifetimeTracker, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ::core::clone::Clone for ID3D12LifetimeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12LifetimeTracker {}
unsafe impl ::core::marker::Sync for ID3D12LifetimeTracker {}
unsafe impl ::windows::core::Vtable for ID3D12LifetimeTracker {
    type Vtable = ID3D12LifetimeTracker_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12LifetimeTracker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fd03d36_4eb1_424a_a582_494ecb8ba813);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12LifetimeTracker_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
    pub DestroyOwnedObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12MetaCommand(::windows::core::IUnknown);
impl ID3D12MetaCommand {
    pub unsafe fn GetRequiredParameterResourceSize(&self, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameterindex: u32) -> u64 {
        (::windows::core::Vtable::vtable(self).GetRequiredParameterResourceSize)(::windows::core::Vtable::as_raw(self), stage, parameterindex)
    }
}
::windows::core::interface_hierarchy!(ID3D12MetaCommand, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ::core::clone::Clone for ID3D12MetaCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12MetaCommand {}
unsafe impl ::core::marker::Sync for ID3D12MetaCommand {}
unsafe impl ::windows::core::Vtable for ID3D12MetaCommand {
    type Vtable = ID3D12MetaCommand_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12MetaCommand {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbb84c27_36ce_4fc9_b801_f048c46ac570);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12MetaCommand_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub GetRequiredParameterResourceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameterindex: u32) -> u64,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Object(::windows::core::IUnknown);
impl ID3D12Object {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12Object, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12Object {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Object {}
unsafe impl ::core::marker::Sync for ID3D12Object {}
unsafe impl ::windows::core::Vtable for ID3D12Object {
    type Vtable = ID3D12Object_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Object {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4fec28f_7966_4e95_9f94_f431cb56c3b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Object_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Pageable(::windows::core::IUnknown);
impl ID3D12Pageable {}
::windows::core::interface_hierarchy!(ID3D12Pageable, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ::core::clone::Clone for ID3D12Pageable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Pageable {}
unsafe impl ::core::marker::Sync for ID3D12Pageable {}
unsafe impl ::windows::core::Vtable for ID3D12Pageable {
    type Vtable = ID3D12Pageable_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Pageable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63ee58fb_1268_4835_86da_f008ce62f0d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Pageable_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12PipelineLibrary(::windows::core::IUnknown);
impl ID3D12PipelineLibrary {
    pub unsafe fn StorePipeline<P0, P1>(&self, pname: P0, ppipeline: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).StorePipeline)(::windows::core::Vtable::as_raw(self), pname.into().abi(), ppipeline.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn LoadGraphicsPipeline<P0, T>(&self, pname: P0, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoadGraphicsPipeline)(::windows::core::Vtable::as_raw(self), pname.into().abi(), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LoadComputePipeline<P0, T>(&self, pname: P0, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoadComputePipeline)(::windows::core::Vtable::as_raw(self), pname.into().abi(), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSerializedSize(&self) -> usize {
        (::windows::core::Vtable::vtable(self).GetSerializedSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Serialize(&self, pdata: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Serialize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12PipelineLibrary, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ::core::clone::Clone for ID3D12PipelineLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12PipelineLibrary {}
unsafe impl ::core::marker::Sync for ID3D12PipelineLibrary {}
unsafe impl ::windows::core::Vtable for ID3D12PipelineLibrary {
    type Vtable = ID3D12PipelineLibrary_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12PipelineLibrary {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc64226a8_9201_46af_b4cc_53fb9ff7414f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12PipelineLibrary_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
    pub StorePipeline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, ppipeline: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub LoadGraphicsPipeline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    LoadGraphicsPipeline: usize,
    pub LoadComputePipeline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSerializedSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasizeinbytes: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12PipelineLibrary1(::windows::core::IUnknown);
impl ID3D12PipelineLibrary1 {
    pub unsafe fn LoadPipeline<P0, T>(&self, pname: P0, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoadPipeline)(::windows::core::Vtable::as_raw(self), pname.into().abi(), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID3D12PipelineLibrary1, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12PipelineLibrary);
impl ::core::clone::Clone for ID3D12PipelineLibrary1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12PipelineLibrary1 {}
unsafe impl ::core::marker::Sync for ID3D12PipelineLibrary1 {}
unsafe impl ::windows::core::Vtable for ID3D12PipelineLibrary1 {
    type Vtable = ID3D12PipelineLibrary1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12PipelineLibrary1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80eabf42_2568_4e5e_bd82_c37f86961dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12PipelineLibrary1_Vtbl {
    pub base__: ID3D12PipelineLibrary_Vtbl,
    pub LoadPipeline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const ::windows::core::GUID, pppipelinestate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12PipelineState(::windows::core::IUnknown);
impl ID3D12PipelineState {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetCachedBlob(&self) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedBlob)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ID3D12PipelineState, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ::core::clone::Clone for ID3D12PipelineState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12PipelineState {}
unsafe impl ::core::marker::Sync for ID3D12PipelineState {}
unsafe impl ::windows::core::Vtable for ID3D12PipelineState {
    type Vtable = ID3D12PipelineState_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12PipelineState {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x765a30f3_f624_4c6f_a828_ace948622445);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12PipelineState_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetCachedBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppblob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetCachedBlob: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12ProtectedResourceSession(::windows::core::IUnknown);
impl ID3D12ProtectedResourceSession {
    pub unsafe fn GetDesc(&self) -> D3D12_PROTECTED_RESOURCE_SESSION_DESC {
        let mut result__: D3D12_PROTECTED_RESOURCE_SESSION_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
::windows::core::interface_hierarchy!(ID3D12ProtectedResourceSession, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12ProtectedSession);
impl ::core::clone::Clone for ID3D12ProtectedResourceSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12ProtectedResourceSession {}
unsafe impl ::core::marker::Sync for ID3D12ProtectedResourceSession {}
unsafe impl ::windows::core::Vtable for ID3D12ProtectedResourceSession {
    type Vtable = ID3D12ProtectedResourceSession_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12ProtectedResourceSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cd696f4_f289_40cc_8091_5a6c0a099c3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12ProtectedResourceSession_Vtbl {
    pub base__: ID3D12ProtectedSession_Vtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12ProtectedResourceSession1(::windows::core::IUnknown);
impl ID3D12ProtectedResourceSession1 {
    pub unsafe fn GetDesc1(&self) -> D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
        let mut result__: D3D12_PROTECTED_RESOURCE_SESSION_DESC1 = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetDesc1)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
::windows::core::interface_hierarchy!(ID3D12ProtectedResourceSession1, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12ProtectedSession, ID3D12ProtectedResourceSession);
impl ::core::clone::Clone for ID3D12ProtectedResourceSession1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12ProtectedResourceSession1 {}
unsafe impl ::core::marker::Sync for ID3D12ProtectedResourceSession1 {}
unsafe impl ::windows::core::Vtable for ID3D12ProtectedResourceSession1 {
    type Vtable = ID3D12ProtectedResourceSession1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12ProtectedResourceSession1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6f12dd6_76fb_406e_8961_4296eefc0409);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12ProtectedResourceSession1_Vtbl {
    pub base__: ID3D12ProtectedResourceSession_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC1),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12ProtectedSession(::windows::core::IUnknown);
impl ID3D12ProtectedSession {
    pub unsafe fn GetStatusFence<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).GetStatusFence)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetSessionStatus(&self) -> D3D12_PROTECTED_SESSION_STATUS {
        (::windows::core::Vtable::vtable(self).GetSessionStatus)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12ProtectedSession, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ::core::clone::Clone for ID3D12ProtectedSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12ProtectedSession {}
unsafe impl ::core::marker::Sync for ID3D12ProtectedSession {}
unsafe impl ::windows::core::Vtable for ID3D12ProtectedSession {
    type Vtable = ID3D12ProtectedSession_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12ProtectedSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1533d18_0ac1_4084_85b9_89a96116806b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12ProtectedSession_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
    pub GetStatusFence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSessionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D3D12_PROTECTED_SESSION_STATUS,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12QueryHeap(::windows::core::IUnknown);
impl ID3D12QueryHeap {}
::windows::core::interface_hierarchy!(ID3D12QueryHeap, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ::core::clone::Clone for ID3D12QueryHeap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12QueryHeap {}
unsafe impl ::core::marker::Sync for ID3D12QueryHeap {}
unsafe impl ::windows::core::Vtable for ID3D12QueryHeap {
    type Vtable = ID3D12QueryHeap_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12QueryHeap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d9658ae_ed45_469e_a61d_970ec583cab4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12QueryHeap_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Resource(::windows::core::IUnknown);
impl ID3D12Resource {
    pub unsafe fn Map(&self, subresource: u32, preadrange: ::core::option::Option<*const D3D12_RANGE>, ppdata: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Map)(::windows::core::Vtable::as_raw(self), subresource, ::core::mem::transmute(preadrange.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Unmap(&self, subresource: u32, pwrittenrange: ::core::option::Option<*const D3D12_RANGE>) {
        (::windows::core::Vtable::vtable(self).Unmap)(::windows::core::Vtable::as_raw(self), subresource, ::core::mem::transmute(pwrittenrange.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self) -> D3D12_RESOURCE_DESC {
        let mut result__: D3D12_RESOURCE_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetGPUVirtualAddress(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetGPUVirtualAddress)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn WriteToSubresource(&self, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D12_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteToSubresource)(::windows::core::Vtable::as_raw(self), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch).ok()
    }
    pub unsafe fn ReadFromSubresource(&self, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D12_BOX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReadFromSubresource)(::windows::core::Vtable::as_raw(self), pdstdata, dstrowpitch, dstdepthpitch, srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetHeapProperties(&self, pheapproperties: ::core::option::Option<*mut D3D12_HEAP_PROPERTIES>, pheapflags: ::core::option::Option<*mut D3D12_HEAP_FLAGS>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetHeapProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pheapproperties.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pheapflags.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12Resource, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ::core::clone::Clone for ID3D12Resource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Resource {}
unsafe impl ::core::marker::Sync for ID3D12Resource {}
unsafe impl ::windows::core::Vtable for ID3D12Resource {
    type Vtable = ID3D12Resource_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Resource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x696442be_a72e_4059_bc79_5b5c98040fad);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Resource_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32, preadrange: *const D3D12_RANGE, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32, pwrittenrange: *const D3D12_RANGE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
    pub GetGPUVirtualAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub WriteToSubresource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D12_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> ::windows::core::HRESULT,
    pub ReadFromSubresource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: *const D3D12_BOX) -> ::windows::core::HRESULT,
    pub GetHeapProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheapproperties: *mut D3D12_HEAP_PROPERTIES, pheapflags: *mut D3D12_HEAP_FLAGS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Resource1(::windows::core::IUnknown);
impl ID3D12Resource1 {
    pub unsafe fn GetProtectedResourceSession<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).GetProtectedResourceSession)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12Resource1, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable, ID3D12Resource);
impl ::core::clone::Clone for ID3D12Resource1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Resource1 {}
unsafe impl ::core::marker::Sync for ID3D12Resource1 {}
unsafe impl ::windows::core::Vtable for ID3D12Resource1 {
    type Vtable = ID3D12Resource1_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Resource1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d5e227a_4430_4161_88b3_3eca6bb16e19);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Resource1_Vtbl {
    pub base__: ID3D12Resource_Vtbl,
    pub GetProtectedResourceSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppprotectedsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Resource2(::windows::core::IUnknown);
impl ID3D12Resource2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self) -> D3D12_RESOURCE_DESC1 {
        let mut result__: D3D12_RESOURCE_DESC1 = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetDesc1)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
::windows::core::interface_hierarchy!(ID3D12Resource2, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable, ID3D12Resource, ID3D12Resource1);
impl ::core::clone::Clone for ID3D12Resource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Resource2 {}
unsafe impl ::core::marker::Sync for ID3D12Resource2 {}
unsafe impl ::windows::core::Vtable for ID3D12Resource2 {
    type Vtable = ID3D12Resource2_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Resource2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe36ec3b_ea85_4aeb_a45a_e9d76404a495);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Resource2_Vtbl {
    pub base__: ID3D12Resource1_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC1),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc1: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12RootSignature(::windows::core::IUnknown);
impl ID3D12RootSignature {}
::windows::core::interface_hierarchy!(ID3D12RootSignature, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ::core::clone::Clone for ID3D12RootSignature {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12RootSignature {}
unsafe impl ::core::marker::Sync for ID3D12RootSignature {}
unsafe impl ::windows::core::Vtable for ID3D12RootSignature {
    type Vtable = ID3D12RootSignature_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12RootSignature {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc54a6b66_72df_4ee8_8be5_a946a1429214);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12RootSignature_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12RootSignatureDeserializer(::windows::core::IUnknown);
impl ID3D12RootSignatureDeserializer {
    pub unsafe fn GetRootSignatureDesc(&self) -> *mut D3D12_ROOT_SIGNATURE_DESC {
        (::windows::core::Vtable::vtable(self).GetRootSignatureDesc)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12RootSignatureDeserializer, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12RootSignatureDeserializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12RootSignatureDeserializer {}
unsafe impl ::core::marker::Sync for ID3D12RootSignatureDeserializer {}
unsafe impl ::windows::core::Vtable for ID3D12RootSignatureDeserializer {
    type Vtable = ID3D12RootSignatureDeserializer_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12RootSignatureDeserializer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34ab647b_3cc8_46ac_841b_c0965645c046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12RootSignatureDeserializer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRootSignatureDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut D3D12_ROOT_SIGNATURE_DESC,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12SDKConfiguration(::windows::core::IUnknown);
impl ID3D12SDKConfiguration {
    pub unsafe fn SetSDKVersion<P0>(&self, sdkversion: u32, sdkpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetSDKVersion)(::windows::core::Vtable::as_raw(self), sdkversion, sdkpath.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12SDKConfiguration, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12SDKConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12SDKConfiguration {}
unsafe impl ::core::marker::Sync for ID3D12SDKConfiguration {}
unsafe impl ::windows::core::Vtable for ID3D12SDKConfiguration {
    type Vtable = ID3D12SDKConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12SDKConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9eb5314_33aa_42b2_a718_d77f58b1f1c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12SDKConfiguration_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetSDKVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sdkversion: u32, sdkpath: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12ShaderCacheSession(::windows::core::IUnknown);
impl ID3D12ShaderCacheSession {
    pub unsafe fn FindValue(&self, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *mut ::core::ffi::c_void, pvaluesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FindValue)(::windows::core::Vtable::as_raw(self), pkey, keysize, pvalue, pvaluesize).ok()
    }
    pub unsafe fn StoreValue(&self, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StoreValue)(::windows::core::Vtable::as_raw(self), pkey, keysize, pvalue, valuesize).ok()
    }
    pub unsafe fn SetDeleteOnDestroy(&self) {
        (::windows::core::Vtable::vtable(self).SetDeleteOnDestroy)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> D3D12_SHADER_CACHE_SESSION_DESC {
        let mut result__: D3D12_SHADER_CACHE_SESSION_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
::windows::core::interface_hierarchy!(ID3D12ShaderCacheSession, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ::core::clone::Clone for ID3D12ShaderCacheSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12ShaderCacheSession {}
unsafe impl ::core::marker::Sync for ID3D12ShaderCacheSession {}
unsafe impl ::windows::core::Vtable for ID3D12ShaderCacheSession {
    type Vtable = ID3D12ShaderCacheSession_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12ShaderCacheSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28e2495d_0f64_4ae4_a6ec_129255dc49a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12ShaderCacheSession_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
    pub FindValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *mut ::core::ffi::c_void, pvaluesize: *mut u32) -> ::windows::core::HRESULT,
    pub StoreValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *const ::core::ffi::c_void, keysize: u32, pvalue: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::HRESULT,
    pub SetDeleteOnDestroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut D3D12_SHADER_CACHE_SESSION_DESC),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12ShaderReflection(::windows::core::IUnknown);
impl ID3D12ShaderReflection {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
        (::windows::core::Vtable::vtable(self).GetConstantBufferByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetConstantBufferByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetResourceBindingDesc)(::windows::core::Vtable::as_raw(self), resourceindex, pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetInputParameterDesc)(::windows::core::Vtable::as_raw(self), parameterindex, pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetOutputParameterDesc)(::windows::core::Vtable::as_raw(self), parameterindex, pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetPatchConstantParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPatchConstantParameterDesc)(::windows::core::Vtable::as_raw(self), parameterindex, pdesc).ok()
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D12ShaderReflectionVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetVariableByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetResourceBindingDescByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), pdesc).ok()
    }
    pub unsafe fn GetMovInstructionCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetMovInstructionCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMovcInstructionCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetMovcInstructionCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetConversionInstructionCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetConversionInstructionCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBitwiseInstructionCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetBitwiseInstructionCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetGSInputPrimitive(&self) -> super::Direct3D::D3D_PRIMITIVE {
        (::windows::core::Vtable::vtable(self).GetGSInputPrimitive)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSampleFrequencyShader(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsSampleFrequencyShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNumInterfaceSlots(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetNumInterfaceSlots)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetMinFeatureLevel(&self) -> ::windows::core::Result<super::Direct3D::D3D_FEATURE_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMinFeatureLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetThreadGroupSize(&self, psizex: ::core::option::Option<*mut u32>, psizey: ::core::option::Option<*mut u32>, psizez: ::core::option::Option<*mut u32>) -> u32 {
        (::windows::core::Vtable::vtable(self).GetThreadGroupSize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psizex.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(psizey.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(psizez.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GetRequiresFlags(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetRequiresFlags)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12ShaderReflection, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12ShaderReflection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12ShaderReflection {}
unsafe impl ::core::marker::Sync for ID3D12ShaderReflection {}
unsafe impl ::windows::core::Vtable for ID3D12ShaderReflection {
    type Vtable = ID3D12ShaderReflection_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12ShaderReflection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a58797d_a72c_478d_8ba2_efc6b0efe88e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12ShaderReflection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetInputParameterDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetInputParameterDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOutputParameterDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOutputParameterDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetPatchConstantParameterDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetPatchConstantParameterDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDescByName: usize,
    pub GetMovInstructionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetMovcInstructionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetConversionInstructionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetBitwiseInstructionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetGSInputPrimitive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetGSInputPrimitive: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSampleFrequencyShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSampleFrequencyShader: usize,
    pub GetNumInterfaceSlots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetMinFeatureLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetMinFeatureLevel: usize,
    pub GetThreadGroupSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32,
    pub GetRequiresFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12ShaderReflectionConstantBuffer(::std::ptr::NonNull<::std::ffi::c_void>);
impl ID3D12ShaderReflectionConstantBuffer {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionVariable> {
        (::windows::core::Vtable::vtable(self).GetVariableByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D12ShaderReflectionVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetVariableByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
}
impl ::core::clone::Clone for ID3D12ShaderReflectionConstantBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12ShaderReflectionConstantBuffer {}
unsafe impl ::core::marker::Sync for ID3D12ShaderReflectionConstantBuffer {}
unsafe impl ::windows::core::Vtable for ID3D12ShaderReflectionConstantBuffer {
    type Vtable = ID3D12ShaderReflectionConstantBuffer_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12ShaderReflectionConstantBuffer_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetVariableByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionVariable>,
    pub GetVariableByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionVariable>,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12ShaderReflectionType(::std::ptr::NonNull<::std::ffi::c_void>);
impl ID3D12ShaderReflectionType {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionType> {
        (::windows::core::Vtable::vtable(self).GetMemberTypeByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberTypeByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D12ShaderReflectionType>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetMemberTypeByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberTypeName(&self, index: u32) -> ::windows::core::PSTR {
        (::windows::core::Vtable::vtable(self).GetMemberTypeName)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn IsEqual<P0>(&self, ptype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ShaderReflectionType>>,
    {
        (::windows::core::Vtable::vtable(self).IsEqual)(::windows::core::Vtable::as_raw(self), ptype.into().abi()).ok()
    }
    pub unsafe fn GetSubType(&self) -> ::core::option::Option<ID3D12ShaderReflectionType> {
        (::windows::core::Vtable::vtable(self).GetSubType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBaseClass(&self) -> ::core::option::Option<ID3D12ShaderReflectionType> {
        (::windows::core::Vtable::vtable(self).GetBaseClass)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNumInterfaces(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetNumInterfaces)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetInterfaceByIndex(&self, uindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionType> {
        (::windows::core::Vtable::vtable(self).GetInterfaceByIndex)(::windows::core::Vtable::as_raw(self), uindex)
    }
    pub unsafe fn IsOfType<P0>(&self, ptype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ShaderReflectionType>>,
    {
        (::windows::core::Vtable::vtable(self).IsOfType)(::windows::core::Vtable::as_raw(self), ptype.into().abi()).ok()
    }
    pub unsafe fn ImplementsInterface<P0>(&self, pbase: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ShaderReflectionType>>,
    {
        (::windows::core::Vtable::vtable(self).ImplementsInterface)(::windows::core::Vtable::as_raw(self), pbase.into().abi()).ok()
    }
}
impl ::core::clone::Clone for ID3D12ShaderReflectionType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12ShaderReflectionType {}
unsafe impl ::core::marker::Sync for ID3D12ShaderReflectionType {}
unsafe impl ::windows::core::Vtable for ID3D12ShaderReflectionType {
    type Vtable = ID3D12ShaderReflectionType_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12ShaderReflectionType_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D12ShaderReflectionType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D12ShaderReflectionType>,
    pub GetMemberTypeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::PSTR,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSubType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType>,
    pub GetBaseClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType>,
    pub GetNumInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetInterfaceByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32) -> ::core::option::Option<ID3D12ShaderReflectionType>,
    pub IsOfType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ImplementsInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbase: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12ShaderReflectionVariable(::std::ptr::NonNull<::std::ffi::c_void>);
impl ID3D12ShaderReflectionVariable {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D12ShaderReflectionType> {
        (::windows::core::Vtable::vtable(self).GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBuffer(&self) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer> {
        (::windows::core::Vtable::vtable(self).GetBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).GetInterfaceSlot)(::windows::core::Vtable::as_raw(self), uarrayindex)
    }
}
impl ::core::clone::Clone for ID3D12ShaderReflectionVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12ShaderReflectionVariable {}
unsafe impl ::core::marker::Sync for ID3D12ShaderReflectionVariable {}
unsafe impl ::windows::core::Vtable for ID3D12ShaderReflectionVariable {
    type Vtable = ID3D12ShaderReflectionVariable_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12ShaderReflectionVariable_Vtbl {
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionType>,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D12ShaderReflectionConstantBuffer>,
    pub GetInterfaceSlot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uarrayindex: u32) -> u32,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12SharingContract(::windows::core::IUnknown);
impl ID3D12SharingContract {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present<P0, P1>(&self, presource: P0, subresource: u32, window: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).Present)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource, window.into())
    }
    pub unsafe fn SharedFenceSignal<P0>(&self, pfence: P0, fencevalue: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Fence>>,
    {
        (::windows::core::Vtable::vtable(self).SharedFenceSignal)(::windows::core::Vtable::as_raw(self), pfence.into().abi(), fencevalue)
    }
    pub unsafe fn BeginCapturableWork(&self, guid: *const ::windows::core::GUID) {
        (::windows::core::Vtable::vtable(self).BeginCapturableWork)(::windows::core::Vtable::as_raw(self), guid)
    }
    pub unsafe fn EndCapturableWork(&self, guid: *const ::windows::core::GUID) {
        (::windows::core::Vtable::vtable(self).EndCapturableWork)(::windows::core::Vtable::as_raw(self), guid)
    }
}
::windows::core::interface_hierarchy!(ID3D12SharingContract, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12SharingContract {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12SharingContract {}
unsafe impl ::core::marker::Sync for ID3D12SharingContract {}
unsafe impl ::windows::core::Vtable for ID3D12SharingContract {
    type Vtable = ID3D12SharingContract_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12SharingContract {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adf7d52_929c_4e61_addb_ffed30de66ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12SharingContract_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Present: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, window: super::super::Foundation::HWND),
    #[cfg(not(feature = "Win32_Foundation"))]
    Present: usize,
    pub SharedFenceSignal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfence: *mut ::core::ffi::c_void, fencevalue: u64),
    pub BeginCapturableWork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID),
    pub EndCapturableWork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12StateObject(::windows::core::IUnknown);
impl ID3D12StateObject {}
::windows::core::interface_hierarchy!(ID3D12StateObject, ::windows::core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ::core::clone::Clone for ID3D12StateObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12StateObject {}
unsafe impl ::core::marker::Sync for ID3D12StateObject {}
unsafe impl ::windows::core::Vtable for ID3D12StateObject {
    type Vtable = ID3D12StateObject_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12StateObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47016943_fca8_4594_93ea_af258b55346d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12StateObject_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12StateObjectProperties(::windows::core::IUnknown);
impl ID3D12StateObjectProperties {
    pub unsafe fn GetShaderIdentifier<P0>(&self, pexportname: P0) -> *mut ::core::ffi::c_void
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetShaderIdentifier)(::windows::core::Vtable::as_raw(self), pexportname.into().abi())
    }
    pub unsafe fn GetShaderStackSize<P0>(&self, pexportname: P0) -> u64
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetShaderStackSize)(::windows::core::Vtable::as_raw(self), pexportname.into().abi())
    }
    pub unsafe fn GetPipelineStackSize(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).GetPipelineStackSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetPipelineStackSize(&self, pipelinestacksizeinbytes: u64) {
        (::windows::core::Vtable::vtable(self).SetPipelineStackSize)(::windows::core::Vtable::as_raw(self), pipelinestacksizeinbytes)
    }
}
::windows::core::interface_hierarchy!(ID3D12StateObjectProperties, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12StateObjectProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12StateObjectProperties {}
unsafe impl ::core::marker::Sync for ID3D12StateObjectProperties {}
unsafe impl ::windows::core::Vtable for ID3D12StateObjectProperties {
    type Vtable = ID3D12StateObjectProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12StateObjectProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde5fa827_9bf9_4f26_89ff_d7f56fde3860);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12StateObjectProperties_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetShaderIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexportname: ::windows::core::PCWSTR) -> *mut ::core::ffi::c_void,
    pub GetShaderStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexportname: ::windows::core::PCWSTR) -> u64,
    pub GetPipelineStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub SetPipelineStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipelinestacksizeinbytes: u64),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12SwapChainAssistant(::windows::core::IUnknown);
impl ID3D12SwapChainAssistant {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLUID(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).GetLUID)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetSwapChainObject<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSwapChainObject)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentResourceAndCommandQueue<T>(&self, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentResourceAndCommandQueue)(::windows::core::Vtable::as_raw(self), riidresource, ppvresource, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertImplicitSync(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InsertImplicitSync)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ID3D12SwapChainAssistant, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12SwapChainAssistant {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12SwapChainAssistant {}
unsafe impl ::core::marker::Sync for ID3D12SwapChainAssistant {}
unsafe impl ::windows::core::Vtable for ID3D12SwapChainAssistant {
    type Vtable = ID3D12SwapChainAssistant_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12SwapChainAssistant {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1df64b6_57fd_49cd_8807_c0eb88b45c8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12SwapChainAssistant_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLUID: usize,
    pub GetSwapChainObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentResourceAndCommandQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riidresource: *const ::windows::core::GUID, ppvresource: *mut *mut ::core::ffi::c_void, riidqueue: *const ::windows::core::GUID, ppvqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertImplicitSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12Tools(::windows::core::IUnknown);
impl ID3D12Tools {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableShaderInstrumentation<P0>(&self, benable: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).EnableShaderInstrumentation)(::windows::core::Vtable::as_raw(self), benable.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShaderInstrumentationEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).ShaderInstrumentationEnabled)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12Tools, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12Tools {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12Tools {}
unsafe impl ::core::marker::Sync for ID3D12Tools {}
unsafe impl ::windows::core::Vtable for ID3D12Tools {
    type Vtable = ID3D12Tools_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12Tools {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7071e1f0_e84b_4b33_974f_12fa49de65c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Tools_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableShaderInstrumentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableShaderInstrumentation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShaderInstrumentationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShaderInstrumentationEnabled: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
pub struct ID3D12VersionedRootSignatureDeserializer(::windows::core::IUnknown);
impl ID3D12VersionedRootSignatureDeserializer {
    pub unsafe fn GetRootSignatureDescAtVersion(&self, converttoversion: D3D_ROOT_SIGNATURE_VERSION) -> ::windows::core::Result<*mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRootSignatureDescAtVersion)(::windows::core::Vtable::as_raw(self), converttoversion, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUnconvertedRootSignatureDesc(&self) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
        (::windows::core::Vtable::vtable(self).GetUnconvertedRootSignatureDesc)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(ID3D12VersionedRootSignatureDeserializer, ::windows::core::IUnknown);
impl ::core::clone::Clone for ID3D12VersionedRootSignatureDeserializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::core::marker::Send for ID3D12VersionedRootSignatureDeserializer {}
unsafe impl ::core::marker::Sync for ID3D12VersionedRootSignatureDeserializer {}
unsafe impl ::windows::core::Vtable for ID3D12VersionedRootSignatureDeserializer {
    type Vtable = ID3D12VersionedRootSignatureDeserializer_Vtbl;
}
unsafe impl ::windows::core::Interface for ID3D12VersionedRootSignatureDeserializer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f91ce67_090c_4bb7_b78e_ed8ff2e31da0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VersionedRootSignatureDeserializer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRootSignatureDescAtVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, converttoversion: D3D_ROOT_SIGNATURE_VERSION, ppdesc: *mut *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC) -> ::windows::core::HRESULT,
    pub GetUnconvertedRootSignatureDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const CLSID_D3D12Debug: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2352aeb_dd84_49fe_b97b_a9dcfdcc1b4f);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const CLSID_D3D12DeviceRemovedExtendedData: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a75bbc4_9ff4_4ad8_9f18_abae84dc5ff2);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const CLSID_D3D12SDKConfiguration: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cda6aca_a03e_49c8_9458_0334d20e07ce);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const CLSID_D3D12Tools: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe38216b1_3c8c_4833_aa09_0a06b65d96c8);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12ExperimentalShaderModels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76f5573e_f13a_40f5_b297_81ce9e18933f);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12MetaCommand: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc734c97e_8077_48c8_9fdc_d9d1dd31dd77);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12TiledResourceTier4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9c4725f_a81a_4f56_8c5b_c51039d694fb);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ANISOTROPIC_FILTERING_BIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_PARTIAL_UPDATE_EXTENTS_BYTE_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET00_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET00_MAX_NUM_THREADS_PER_GROUP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET01_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 240u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET01_MAX_NUM_THREADS_PER_GROUP: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET02_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 224u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET02_MAX_NUM_THREADS_PER_GROUP: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET03_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 208u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET03_MAX_NUM_THREADS_PER_GROUP: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET04_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET04_MAX_NUM_THREADS_PER_GROUP: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET05_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 176u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET05_MAX_NUM_THREADS_PER_GROUP: u32 = 92u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET06_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 160u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET06_MAX_NUM_THREADS_PER_GROUP: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET07_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 144u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET07_MAX_NUM_THREADS_PER_GROUP: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET08_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET08_MAX_NUM_THREADS_PER_GROUP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET09_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET09_MAX_NUM_THREADS_PER_GROUP: u32 = 144u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET10_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET10_MAX_NUM_THREADS_PER_GROUP: u32 = 168u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET11_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET11_MAX_NUM_THREADS_PER_GROUP: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET12_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET12_MAX_NUM_THREADS_PER_GROUP: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET13_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET13_MAX_NUM_THREADS_PER_GROUP: u32 = 340u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET14_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET14_MAX_NUM_THREADS_PER_GROUP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET15_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_BUCKET15_MAX_NUM_THREADS_PER_GROUP: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_DISPATCH_MAX_THREAD_GROUPS_IN_Z_DIMENSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_RAW_UAV_BYTE_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_X: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_Y: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_4_X_UAV_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_TGSM_REGISTER_COUNT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_TGSM_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_TGSM_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_TGSM_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREADGROUPID_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREADGROUPID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREADIDINGROUPFLATTENED_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREADIDINGROUPFLATTENED_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREADIDINGROUP_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREADIDINGROUP_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREADID_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREADID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREAD_GROUP_MAX_X: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREAD_GROUP_MAX_Y: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREAD_GROUP_MAX_Z: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREAD_GROUP_MIN_X: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREAD_GROUP_MIN_Y: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREAD_GROUP_MIN_Z: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CS_THREAD_LOCAL_TEMP_REGISTER_POOL: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_BLEND_FACTOR_RED: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_DEPTH_BIAS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_MAX_ANISOTROPY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_MIP_LOD_BIAS: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_SCISSOR_ENDX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_SCISSOR_ENDY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_SCISSOR_STARTX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_SCISSOR_STARTY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: u32 = 5768u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_STENCIL_READ_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_STENCIL_REFERENCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_STENCIL_WRITE_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_VIEWPORT_HEIGHT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEFAULT_VIEWPORT_WIDTH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRIVER_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967287u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRIVER_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967280u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_REDUCTION_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_REDUCTION_TYPE_SHIFT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FLOAT_TO_SRGB_OFFSET: f32 = 0.055f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FTOI_INSTRUCTION_MIN_INPUT: f32 = -2147483600f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_INSTANCE_ID_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_INSTANCE_ID_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_INPUT_REGISTER_VERTICES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_MAX_INSTANCE_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_MAX_OUTPUT_VERTEX_COUNT_ACROSS_INSTANCES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_OUTPUT_ELEMENTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_CONTROL_POINT_PHASE_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_CONTROL_POINT_PHASE_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_FORK_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_JOIN_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_MAXTESSFACTOR_LOWER_BOUND: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_MAXTESSFACTOR_UPPER_BOUND: f32 = 64f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_SCALAR_COMPONENTS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_INSTANCE_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_PATCH_MAX_CONTROL_POINT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_VERTEX_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_KEEP_UNORDERED_ACCESS_VIEWS: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LINEAR_GAMMA: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAG_FILTER_SHIFT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAJOR_VERSION: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_DEPTH: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_LIVE_STATIC_SAMPLERS: u32 = 2032u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_MAXANISOTROPY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_ROOT_COST: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_SHADER_VISIBLE_DESCRIPTOR_HEAP_SIZE_TIER_1: u32 = 1000000u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_SHADER_VISIBLE_DESCRIPTOR_HEAP_SIZE_TIER_2: u32 = 1000000u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_SHADER_VISIBLE_SAMPLER_HEAP_SIZE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MAX_VIEW_INSTANCE_COUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MINOR_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MIN_BORDER_COLOR_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MIN_DEPTH: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MIN_FILTER_SHIFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MIN_MAXANISOTROPY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MIP_FILTER_SHIFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MIP_LOD_BIAS_MAX: f32 = 15.99f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MIP_LOD_BIAS_MIN: f32 = -16f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MIP_LOD_RANGE_BIT_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_OS_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_OS_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967288u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PACKED_TILE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PROTECTED_RESOURCES_SESSION_HARDWARE_PROTECTED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62b0084e_c70e_4daa_a109_30ff8d5a0482);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_CS_UAV_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_CS_UAV_REGISTER_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_CS_UAV_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_CS_UAV_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_FRONTFACING_FALSE_VALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_OUTPUT_REGISTER_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAW_UAV_SRV_BYTE_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_AABB_BYTE_ALIGNMENT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BYTE_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_INSTANCE_DESCS_BYTE_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_MAX_ATTRIBUTE_SIZE_IN_BYTES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_MAX_DECLARABLE_TRACE_RECURSION_DEPTH: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_MAX_GEOMETRIES_PER_BOTTOM_LEVEL_ACCELERATION_STRUCTURE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_MAX_INSTANCES_PER_TOP_LEVEL_ACCELERATION_STRUCTURE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_MAX_PRIMITIVES_PER_BOTTOM_LEVEL_ACCELERATION_STRUCTURE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_MAX_RAY_GENERATION_SHADER_THREADS: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_MAX_SHADER_RECORD_STRIDE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_SHADER_RECORD_BYTE_ALIGNMENT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_SHADER_TABLE_BYTE_ALIGNMENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_TRANSFORM3X4_BYTE_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_BLEND_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_MAXANISOTROPY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_MIP_LEVELS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_RASTERIZER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_A_TERM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_B_TERM: f32 = 0.25f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_C_TERM: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_RESOURCE_VIEW_COUNT_PER_DEVICE_2_TO_EXP: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_SAMPLER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_SUBRESOURCES: u32 = 30720u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_TEXTURE1D_U_DIMENSION: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_REQ_TEXTURECUBE_DIMENSION: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RS_SET_SHADING_RATE_COMBINER_COUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SDK_VERSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_ALWAYS_SET_BIT_AVOIDING_ZEROMEM_MISTAKES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_MASK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_SHIFT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_MAJOR_VERSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_MAX_INSTANCES: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_MAX_INTERFACES: u32 = 253u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_MAX_INTERFACE_CALL_SITES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_MAX_TYPES: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_MINOR_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_VALID_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_X_AXIS_SHIFT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SMALL_MSAA_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SMALL_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SO_NO_RASTERIZED_STREAM: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SO_OUTPUT_COMPONENT_COUNT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SO_STREAM_COUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SPEC_DATE_DAY: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SPEC_DATE_MONTH: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SPEC_DATE_YEAR: u32 = 2014u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SPEC_VERSION: f64 = 1.16f64;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRGB_GAMMA: f32 = 2.2f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRGB_TO_FLOAT_OFFSET: f32 = 0.055f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STANDARD_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STANDARD_VECTOR_SIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SYSTEM_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SYSTEM_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967280u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TESSELLATOR_MAX_EVEN_TESSELLATION_FACTOR: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TESSELLATOR_MAX_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TESSELLATOR_MAX_ODD_TESSELLATION_FACTOR: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TESSELLATOR_MAX_TESSELLATION_FACTOR: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TESSELLATOR_MIN_EVEN_TESSELLATION_FACTOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TESSELLATOR_MIN_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TESSELLATOR_MIN_ODD_TESSELLATION_FACTOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_DATA_PITCH_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_DATA_PLACEMENT_ALIGNMENT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILED_RESOURCE_TILE_SIZE_IN_BYTES: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TRACKED_WORKLOAD_MAX_INSTANCES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_UAV_COUNTER_PLACEMENT_ALIGNMENT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_UAV_SLOT_COUNT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIDEO_DECODE_MAX_ARGUMENTS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIDEO_DECODE_MAX_HISTOGRAM_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIDEO_DECODE_MIN_BITSTREAM_OFFSET_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIDEO_DECODE_MIN_HISTOGRAM_OFFSET_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIDEO_DECODE_STATUS_MACROBLOCKS_AFFECTED_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIDEO_PROCESS_MAX_FILTERS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIDEO_PROCESS_STEREO_VIEWS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIEWPORT_BOUNDS_MAX: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIEWPORT_BOUNDS_MIN: i32 = -32768i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_DESCRIPTOR_HEAP_RESOURCE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_GROUP_SHARED: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_TYPED_RESOURCE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_BARYCENTRICS: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_DERIVATIVES_IN_MESH_AND_AMPLIFICATION_SHADERS: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_INNER_COVERAGE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_INT64_OPS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_NATIVE_16BIT_OPS: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_RAYTRACING_TIER_1_1: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_RESOURCE_DESCRIPTOR_HEAP_INDEXING: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_ROVS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_SAMPLER_DESCRIPTOR_HEAP_INDEXING: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_SAMPLER_FEEDBACK: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_SHADING_RATE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_STENCIL_REF: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_TYPED_UAV_LOAD_ADDITIONAL_FORMATS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_VIEWPORT_AND_RT_ARRAY_INDEX_FROM_ANY_SHADER_FEEDING_RASTERIZER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_VIEW_ID: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_WAVE_MMA: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_REQUIRES_WAVE_OPS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const DXGI_DEBUG_D3D12: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf59a98c_a950_4326_91ef_9bbaa17bfd95);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const LUID_DEFINED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const WKPDID_D3DAutoDebugObjectNameW: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4902e36_757a_4942_9594_b6769afa43cd);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_AUTO_BREADCRUMB_OP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_SETMARKER: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_BEGINEVENT: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_ENDEVENT: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_DRAWINSTANCED: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_DRAWINDEXEDINSTANCED: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEINDIRECT: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_DISPATCH: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_COPYBUFFERREGION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_COPYTEXTUREREGION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_COPYRESOURCE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_COPYTILES: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVESUBRESOURCE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_CLEARRENDERTARGETVIEW: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_CLEARUNORDEREDACCESSVIEW: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_CLEARDEPTHSTENCILVIEW: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_RESOURCEBARRIER: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEBUNDLE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_PRESENT: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVEQUERYDATA: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_BEGINSUBMISSION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_ENDSUBMISSION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_PROCESSFRAMES: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_ATOMICCOPYBUFFERUINT: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_ATOMICCOPYBUFFERUINT64: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVESUBRESOURCEREGION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_WRITEBUFFERIMMEDIATE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME1: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_SETPROTECTEDRESOURCESESSION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME2: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_PROCESSFRAMES1: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_BUILDRAYTRACINGACCELERATIONSTRUCTURE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(31i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_EMITRAYTRACINGACCELERATIONSTRUCTUREPOSTBUILDINFO: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_COPYRAYTRACINGACCELERATIONSTRUCTURE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_DISPATCHRAYS: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_INITIALIZEMETACOMMAND: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEMETACOMMAND: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_ESTIMATEMOTION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVEMOTIONVECTORHEAP: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_SETPIPELINESTATE1: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_INITIALIZEEXTENSIONCOMMAND: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(40i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEEXTENSIONCOMMAND: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(41i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_DISPATCHMESH: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(42i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_ENCODEFRAME: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(43i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVEENCODEROUTPUTMETADATA: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(44i32);
impl ::core::marker::Copy for D3D12_AUTO_BREADCRUMB_OP {}
impl ::core::clone::Clone for D3D12_AUTO_BREADCRUMB_OP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_AUTO_BREADCRUMB_OP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_AXIS_SHADING_RATE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AXIS_SHADING_RATE_1X: D3D12_AXIS_SHADING_RATE = D3D12_AXIS_SHADING_RATE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AXIS_SHADING_RATE_2X: D3D12_AXIS_SHADING_RATE = D3D12_AXIS_SHADING_RATE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_AXIS_SHADING_RATE_4X: D3D12_AXIS_SHADING_RATE = D3D12_AXIS_SHADING_RATE(2i32);
impl ::core::marker::Copy for D3D12_AXIS_SHADING_RATE {}
impl ::core::clone::Clone for D3D12_AXIS_SHADING_RATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_AXIS_SHADING_RATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_BACKGROUND_PROCESSING_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BACKGROUND_PROCESSING_MODE_ALLOWED: D3D12_BACKGROUND_PROCESSING_MODE = D3D12_BACKGROUND_PROCESSING_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BACKGROUND_PROCESSING_MODE_ALLOW_INTRUSIVE_MEASUREMENTS: D3D12_BACKGROUND_PROCESSING_MODE = D3D12_BACKGROUND_PROCESSING_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BACKGROUND_PROCESSING_MODE_DISABLE_BACKGROUND_WORK: D3D12_BACKGROUND_PROCESSING_MODE = D3D12_BACKGROUND_PROCESSING_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BACKGROUND_PROCESSING_MODE_DISABLE_PROFILING_BY_SYSTEM: D3D12_BACKGROUND_PROCESSING_MODE = D3D12_BACKGROUND_PROCESSING_MODE(3i32);
impl ::core::marker::Copy for D3D12_BACKGROUND_PROCESSING_MODE {}
impl ::core::clone::Clone for D3D12_BACKGROUND_PROCESSING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_BACKGROUND_PROCESSING_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_BLEND(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_ZERO: D3D12_BLEND = D3D12_BLEND(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_ONE: D3D12_BLEND = D3D12_BLEND(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_SRC_COLOR: D3D12_BLEND = D3D12_BLEND(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_INV_SRC_COLOR: D3D12_BLEND = D3D12_BLEND(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_SRC_ALPHA: D3D12_BLEND = D3D12_BLEND(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_INV_SRC_ALPHA: D3D12_BLEND = D3D12_BLEND(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_DEST_ALPHA: D3D12_BLEND = D3D12_BLEND(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_INV_DEST_ALPHA: D3D12_BLEND = D3D12_BLEND(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_DEST_COLOR: D3D12_BLEND = D3D12_BLEND(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_INV_DEST_COLOR: D3D12_BLEND = D3D12_BLEND(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_SRC_ALPHA_SAT: D3D12_BLEND = D3D12_BLEND(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_BLEND_FACTOR: D3D12_BLEND = D3D12_BLEND(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_INV_BLEND_FACTOR: D3D12_BLEND = D3D12_BLEND(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_SRC1_COLOR: D3D12_BLEND = D3D12_BLEND(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_INV_SRC1_COLOR: D3D12_BLEND = D3D12_BLEND(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_SRC1_ALPHA: D3D12_BLEND = D3D12_BLEND(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_INV_SRC1_ALPHA: D3D12_BLEND = D3D12_BLEND(19i32);
impl ::core::marker::Copy for D3D12_BLEND {}
impl ::core::clone::Clone for D3D12_BLEND {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_BLEND {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_BLEND_OP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_OP_ADD: D3D12_BLEND_OP = D3D12_BLEND_OP(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_OP_SUBTRACT: D3D12_BLEND_OP = D3D12_BLEND_OP(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_OP_REV_SUBTRACT: D3D12_BLEND_OP = D3D12_BLEND_OP(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_OP_MIN: D3D12_BLEND_OP = D3D12_BLEND_OP(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BLEND_OP_MAX: D3D12_BLEND_OP = D3D12_BLEND_OP(5i32);
impl ::core::marker::Copy for D3D12_BLEND_OP {}
impl ::core::clone::Clone for D3D12_BLEND_OP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_BLEND_OP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_BUFFER_SRV_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BUFFER_SRV_FLAG_NONE: D3D12_BUFFER_SRV_FLAGS = D3D12_BUFFER_SRV_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BUFFER_SRV_FLAG_RAW: D3D12_BUFFER_SRV_FLAGS = D3D12_BUFFER_SRV_FLAGS(1u32);
impl ::core::marker::Copy for D3D12_BUFFER_SRV_FLAGS {}
impl ::core::clone::Clone for D3D12_BUFFER_SRV_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_BUFFER_SRV_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_BUFFER_UAV_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BUFFER_UAV_FLAG_NONE: D3D12_BUFFER_UAV_FLAGS = D3D12_BUFFER_UAV_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_BUFFER_UAV_FLAG_RAW: D3D12_BUFFER_UAV_FLAGS = D3D12_BUFFER_UAV_FLAGS(1u32);
impl ::core::marker::Copy for D3D12_BUFFER_UAV_FLAGS {}
impl ::core::clone::Clone for D3D12_BUFFER_UAV_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_BUFFER_UAV_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_CLEAR_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CLEAR_FLAG_DEPTH: D3D12_CLEAR_FLAGS = D3D12_CLEAR_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CLEAR_FLAG_STENCIL: D3D12_CLEAR_FLAGS = D3D12_CLEAR_FLAGS(2u32);
impl ::core::marker::Copy for D3D12_CLEAR_FLAGS {}
impl ::core::clone::Clone for D3D12_CLEAR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_CLEAR_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_COLOR_WRITE_ENABLE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COLOR_WRITE_ENABLE_RED: D3D12_COLOR_WRITE_ENABLE = D3D12_COLOR_WRITE_ENABLE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COLOR_WRITE_ENABLE_GREEN: D3D12_COLOR_WRITE_ENABLE = D3D12_COLOR_WRITE_ENABLE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COLOR_WRITE_ENABLE_BLUE: D3D12_COLOR_WRITE_ENABLE = D3D12_COLOR_WRITE_ENABLE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COLOR_WRITE_ENABLE_ALPHA: D3D12_COLOR_WRITE_ENABLE = D3D12_COLOR_WRITE_ENABLE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COLOR_WRITE_ENABLE_ALL: D3D12_COLOR_WRITE_ENABLE = D3D12_COLOR_WRITE_ENABLE(15i32);
impl ::core::marker::Copy for D3D12_COLOR_WRITE_ENABLE {}
impl ::core::clone::Clone for D3D12_COLOR_WRITE_ENABLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_COLOR_WRITE_ENABLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_COMMAND_LIST_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_FLAG_NONE: D3D12_COMMAND_LIST_FLAGS = D3D12_COMMAND_LIST_FLAGS(0u32);
impl ::core::marker::Copy for D3D12_COMMAND_LIST_FLAGS {}
impl ::core::clone::Clone for D3D12_COMMAND_LIST_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_COMMAND_LIST_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_COMMAND_LIST_SUPPORT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_NONE: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_DIRECT: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_BUNDLE: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_COMPUTE: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_COPY: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_DECODE: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_PROCESS: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_ENCODE: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(64u32);
impl ::core::marker::Copy for D3D12_COMMAND_LIST_SUPPORT_FLAGS {}
impl ::core::clone::Clone for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_COMMAND_LIST_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_TYPE_DIRECT: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_TYPE_BUNDLE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_TYPE_COMPUTE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_TYPE_COPY: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(6i32);
impl ::core::marker::Copy for D3D12_COMMAND_LIST_TYPE {}
impl ::core::clone::Clone for D3D12_COMMAND_LIST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_COMMAND_LIST_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_COMMAND_POOL_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_POOL_FLAG_NONE: D3D12_COMMAND_POOL_FLAGS = D3D12_COMMAND_POOL_FLAGS(0u32);
impl ::core::marker::Copy for D3D12_COMMAND_POOL_FLAGS {}
impl ::core::clone::Clone for D3D12_COMMAND_POOL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_COMMAND_POOL_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_COMMAND_QUEUE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_QUEUE_FLAG_NONE: D3D12_COMMAND_QUEUE_FLAGS = D3D12_COMMAND_QUEUE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT: D3D12_COMMAND_QUEUE_FLAGS = D3D12_COMMAND_QUEUE_FLAGS(1u32);
impl ::core::marker::Copy for D3D12_COMMAND_QUEUE_FLAGS {}
impl ::core::clone::Clone for D3D12_COMMAND_QUEUE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_COMMAND_QUEUE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_COMMAND_QUEUE_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_QUEUE_PRIORITY_NORMAL: D3D12_COMMAND_QUEUE_PRIORITY = D3D12_COMMAND_QUEUE_PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_QUEUE_PRIORITY_HIGH: D3D12_COMMAND_QUEUE_PRIORITY = D3D12_COMMAND_QUEUE_PRIORITY(100i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME: D3D12_COMMAND_QUEUE_PRIORITY = D3D12_COMMAND_QUEUE_PRIORITY(10000i32);
impl ::core::marker::Copy for D3D12_COMMAND_QUEUE_PRIORITY {}
impl ::core::clone::Clone for D3D12_COMMAND_QUEUE_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_COMMAND_QUEUE_PRIORITY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_COMMAND_RECORDER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMMAND_RECORDER_FLAG_NONE: D3D12_COMMAND_RECORDER_FLAGS = D3D12_COMMAND_RECORDER_FLAGS(0u32);
impl ::core::marker::Copy for D3D12_COMMAND_RECORDER_FLAGS {}
impl ::core::clone::Clone for D3D12_COMMAND_RECORDER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_COMMAND_RECORDER_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_COMPARISON_FUNC(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMPARISON_FUNC_NEVER: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMPARISON_FUNC_LESS: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMPARISON_FUNC_EQUAL: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMPARISON_FUNC_LESS_EQUAL: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMPARISON_FUNC_GREATER: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMPARISON_FUNC_NOT_EQUAL: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMPARISON_FUNC_GREATER_EQUAL: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_COMPARISON_FUNC_ALWAYS: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(8i32);
impl ::core::marker::Copy for D3D12_COMPARISON_FUNC {}
impl ::core::clone::Clone for D3D12_COMPARISON_FUNC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_COMPARISON_FUNC {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_CONSERVATIVE_RASTERIZATION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF: D3D12_CONSERVATIVE_RASTERIZATION_MODE = D3D12_CONSERVATIVE_RASTERIZATION_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CONSERVATIVE_RASTERIZATION_MODE_ON: D3D12_CONSERVATIVE_RASTERIZATION_MODE = D3D12_CONSERVATIVE_RASTERIZATION_MODE(1i32);
impl ::core::marker::Copy for D3D12_CONSERVATIVE_RASTERIZATION_MODE {}
impl ::core::clone::Clone for D3D12_CONSERVATIVE_RASTERIZATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_CONSERVATIVE_RASTERIZATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_CONSERVATIVE_RASTERIZATION_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_NOT_SUPPORTED: D3D12_CONSERVATIVE_RASTERIZATION_TIER = D3D12_CONSERVATIVE_RASTERIZATION_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_1: D3D12_CONSERVATIVE_RASTERIZATION_TIER = D3D12_CONSERVATIVE_RASTERIZATION_TIER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_2: D3D12_CONSERVATIVE_RASTERIZATION_TIER = D3D12_CONSERVATIVE_RASTERIZATION_TIER(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_3: D3D12_CONSERVATIVE_RASTERIZATION_TIER = D3D12_CONSERVATIVE_RASTERIZATION_TIER(3i32);
impl ::core::marker::Copy for D3D12_CONSERVATIVE_RASTERIZATION_TIER {}
impl ::core::clone::Clone for D3D12_CONSERVATIVE_RASTERIZATION_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_CONSERVATIVE_RASTERIZATION_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_CPU_PAGE_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CPU_PAGE_PROPERTY_UNKNOWN: D3D12_CPU_PAGE_PROPERTY = D3D12_CPU_PAGE_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE: D3D12_CPU_PAGE_PROPERTY = D3D12_CPU_PAGE_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE: D3D12_CPU_PAGE_PROPERTY = D3D12_CPU_PAGE_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CPU_PAGE_PROPERTY_WRITE_BACK: D3D12_CPU_PAGE_PROPERTY = D3D12_CPU_PAGE_PROPERTY(3i32);
impl ::core::marker::Copy for D3D12_CPU_PAGE_PROPERTY {}
impl ::core::clone::Clone for D3D12_CPU_PAGE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_CPU_PAGE_PROPERTY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_CROSS_NODE_SHARING_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CROSS_NODE_SHARING_TIER_NOT_SUPPORTED: D3D12_CROSS_NODE_SHARING_TIER = D3D12_CROSS_NODE_SHARING_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CROSS_NODE_SHARING_TIER_1_EMULATED: D3D12_CROSS_NODE_SHARING_TIER = D3D12_CROSS_NODE_SHARING_TIER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CROSS_NODE_SHARING_TIER_1: D3D12_CROSS_NODE_SHARING_TIER = D3D12_CROSS_NODE_SHARING_TIER(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CROSS_NODE_SHARING_TIER_2: D3D12_CROSS_NODE_SHARING_TIER = D3D12_CROSS_NODE_SHARING_TIER(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CROSS_NODE_SHARING_TIER_3: D3D12_CROSS_NODE_SHARING_TIER = D3D12_CROSS_NODE_SHARING_TIER(4i32);
impl ::core::marker::Copy for D3D12_CROSS_NODE_SHARING_TIER {}
impl ::core::clone::Clone for D3D12_CROSS_NODE_SHARING_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_CROSS_NODE_SHARING_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_CULL_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CULL_MODE_NONE: D3D12_CULL_MODE = D3D12_CULL_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CULL_MODE_FRONT: D3D12_CULL_MODE = D3D12_CULL_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_CULL_MODE_BACK: D3D12_CULL_MODE = D3D12_CULL_MODE(3i32);
impl ::core::marker::Copy for D3D12_CULL_MODE {}
impl ::core::clone::Clone for D3D12_CULL_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_CULL_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEBUG_COMMAND_LIST_PARAMETER_GPU_BASED_VALIDATION_SETTINGS: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE = D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE(0i32);
impl ::core::marker::Copy for D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE {}
impl ::core::clone::Clone for D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DEBUG_DEVICE_PARAMETER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEBUG_DEVICE_PARAMETER_FEATURE_FLAGS: D3D12_DEBUG_DEVICE_PARAMETER_TYPE = D3D12_DEBUG_DEVICE_PARAMETER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEBUG_DEVICE_PARAMETER_GPU_BASED_VALIDATION_SETTINGS: D3D12_DEBUG_DEVICE_PARAMETER_TYPE = D3D12_DEBUG_DEVICE_PARAMETER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEBUG_DEVICE_PARAMETER_GPU_SLOWDOWN_PERFORMANCE_FACTOR: D3D12_DEBUG_DEVICE_PARAMETER_TYPE = D3D12_DEBUG_DEVICE_PARAMETER_TYPE(2i32);
impl ::core::marker::Copy for D3D12_DEBUG_DEVICE_PARAMETER_TYPE {}
impl ::core::clone::Clone for D3D12_DEBUG_DEVICE_PARAMETER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEBUG_DEVICE_PARAMETER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DEBUG_FEATURE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEBUG_FEATURE_NONE: D3D12_DEBUG_FEATURE = D3D12_DEBUG_FEATURE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEBUG_FEATURE_ALLOW_BEHAVIOR_CHANGING_DEBUG_AIDS: D3D12_DEBUG_FEATURE = D3D12_DEBUG_FEATURE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEBUG_FEATURE_CONSERVATIVE_RESOURCE_STATE_TRACKING: D3D12_DEBUG_FEATURE = D3D12_DEBUG_FEATURE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEBUG_FEATURE_DISABLE_VIRTUALIZED_BUNDLES_VALIDATION: D3D12_DEBUG_FEATURE = D3D12_DEBUG_FEATURE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEBUG_FEATURE_EMULATE_WINDOWS7: D3D12_DEBUG_FEATURE = D3D12_DEBUG_FEATURE(8i32);
impl ::core::marker::Copy for D3D12_DEBUG_FEATURE {}
impl ::core::clone::Clone for D3D12_DEBUG_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEBUG_FEATURE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DEPTH_WRITE_MASK(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEPTH_WRITE_MASK_ZERO: D3D12_DEPTH_WRITE_MASK = D3D12_DEPTH_WRITE_MASK(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DEPTH_WRITE_MASK_ALL: D3D12_DEPTH_WRITE_MASK = D3D12_DEPTH_WRITE_MASK(1i32);
impl ::core::marker::Copy for D3D12_DEPTH_WRITE_MASK {}
impl ::core::clone::Clone for D3D12_DEPTH_WRITE_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEPTH_WRITE_MASK {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DESCRIPTOR_HEAP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_HEAP_FLAG_NONE: D3D12_DESCRIPTOR_HEAP_FLAGS = D3D12_DESCRIPTOR_HEAP_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE: D3D12_DESCRIPTOR_HEAP_FLAGS = D3D12_DESCRIPTOR_HEAP_FLAGS(1u32);
impl ::core::marker::Copy for D3D12_DESCRIPTOR_HEAP_FLAGS {}
impl ::core::clone::Clone for D3D12_DESCRIPTOR_HEAP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DESCRIPTOR_HEAP_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DESCRIPTOR_HEAP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_HEAP_TYPE_RTV: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_HEAP_TYPE_DSV: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_HEAP_TYPE_NUM_TYPES: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE(4i32);
impl ::core::marker::Copy for D3D12_DESCRIPTOR_HEAP_TYPE {}
impl ::core::clone::Clone for D3D12_DESCRIPTOR_HEAP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DESCRIPTOR_HEAP_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DESCRIPTOR_RANGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_RANGE_FLAG_NONE: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DESCRIPTORS_VOLATILE: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DATA_VOLATILE: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DATA_STATIC_WHILE_SET_AT_EXECUTE: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DATA_STATIC: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DESCRIPTORS_STATIC_KEEPING_BUFFER_BOUNDS_CHECKS: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(65536u32);
impl ::core::marker::Copy for D3D12_DESCRIPTOR_RANGE_FLAGS {}
impl ::core::clone::Clone for D3D12_DESCRIPTOR_RANGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DESCRIPTOR_RANGE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DESCRIPTOR_RANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_RANGE_TYPE_SRV: D3D12_DESCRIPTOR_RANGE_TYPE = D3D12_DESCRIPTOR_RANGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_RANGE_TYPE_UAV: D3D12_DESCRIPTOR_RANGE_TYPE = D3D12_DESCRIPTOR_RANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_RANGE_TYPE_CBV: D3D12_DESCRIPTOR_RANGE_TYPE = D3D12_DESCRIPTOR_RANGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER: D3D12_DESCRIPTOR_RANGE_TYPE = D3D12_DESCRIPTOR_RANGE_TYPE(3i32);
impl ::core::marker::Copy for D3D12_DESCRIPTOR_RANGE_TYPE {}
impl ::core::clone::Clone for D3D12_DESCRIPTOR_RANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DESCRIPTOR_RANGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DRED_ALLOCATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_QUEUE: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_ALLOCATOR: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_PIPELINE_STATE: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_LIST: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_FENCE: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_DESCRIPTOR_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_QUERY_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_SIGNATURE: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_PIPELINE_LIBRARY: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_DECODER: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_PROCESSOR: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_RESOURCE: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_PASS: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_CRYPTOSESSION: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_CRYPTOSESSIONPOLICY: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_PROTECTEDRESOURCESESSION: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_DECODER_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_POOL: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(40i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_RECORDER: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(41i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_STATE_OBJECT: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(42i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_METACOMMAND: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(43i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_SCHEDULINGGROUP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(44i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_MOTION_ESTIMATOR: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(45i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_MOTION_VECTOR_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(46i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_EXTENSION_COMMAND: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(47i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_ENCODER: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(48i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_ENCODER_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(49i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ALLOCATION_TYPE_INVALID: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(-1i32);
impl ::core::marker::Copy for D3D12_DRED_ALLOCATION_TYPE {}
impl ::core::clone::Clone for D3D12_DRED_ALLOCATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_ALLOCATION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DRED_DEVICE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_DEVICE_STATE_UNKNOWN: D3D12_DRED_DEVICE_STATE = D3D12_DRED_DEVICE_STATE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_DEVICE_STATE_HUNG: D3D12_DRED_DEVICE_STATE = D3D12_DRED_DEVICE_STATE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_DEVICE_STATE_FAULT: D3D12_DRED_DEVICE_STATE = D3D12_DRED_DEVICE_STATE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_DEVICE_STATE_PAGEFAULT: D3D12_DRED_DEVICE_STATE = D3D12_DRED_DEVICE_STATE(7i32);
impl ::core::marker::Copy for D3D12_DRED_DEVICE_STATE {}
impl ::core::clone::Clone for D3D12_DRED_DEVICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_DEVICE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DRED_ENABLEMENT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ENABLEMENT_SYSTEM_CONTROLLED: D3D12_DRED_ENABLEMENT = D3D12_DRED_ENABLEMENT(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ENABLEMENT_FORCED_OFF: D3D12_DRED_ENABLEMENT = D3D12_DRED_ENABLEMENT(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_ENABLEMENT_FORCED_ON: D3D12_DRED_ENABLEMENT = D3D12_DRED_ENABLEMENT(2i32);
impl ::core::marker::Copy for D3D12_DRED_ENABLEMENT {}
impl ::core::clone::Clone for D3D12_DRED_ENABLEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_ENABLEMENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DRED_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_FLAG_NONE: D3D12_DRED_FLAGS = D3D12_DRED_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_FLAG_FORCE_ENABLE: D3D12_DRED_FLAGS = D3D12_DRED_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_FLAG_DISABLE_AUTOBREADCRUMBS: D3D12_DRED_FLAGS = D3D12_DRED_FLAGS(2u32);
impl ::core::marker::Copy for D3D12_DRED_FLAGS {}
impl ::core::clone::Clone for D3D12_DRED_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DRED_PAGE_FAULT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_PAGE_FAULT_FLAGS_NONE: D3D12_DRED_PAGE_FAULT_FLAGS = D3D12_DRED_PAGE_FAULT_FLAGS(0u32);
impl ::core::marker::Copy for D3D12_DRED_PAGE_FAULT_FLAGS {}
impl ::core::clone::Clone for D3D12_DRED_PAGE_FAULT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_PAGE_FAULT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DRED_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_VERSION_1_0: D3D12_DRED_VERSION = D3D12_DRED_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_VERSION_1_1: D3D12_DRED_VERSION = D3D12_DRED_VERSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_VERSION_1_2: D3D12_DRED_VERSION = D3D12_DRED_VERSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRED_VERSION_1_3: D3D12_DRED_VERSION = D3D12_DRED_VERSION(4i32);
impl ::core::marker::Copy for D3D12_DRED_VERSION {}
impl ::core::clone::Clone for D3D12_DRED_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_VERSION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_COMPATIBLE_WITH_DEVICE: D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS = D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_UNSUPPORTED_TYPE: D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS = D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_UNRECOGNIZED: D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS = D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_INCOMPATIBLE_VERSION: D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS = D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_INCOMPATIBLE_TYPE: D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS = D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(4i32);
impl ::core::marker::Copy for D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {}
impl ::core::clone::Clone for D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DSV_DIMENSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DSV_DIMENSION_UNKNOWN: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DSV_DIMENSION_TEXTURE1D: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DSV_DIMENSION_TEXTURE1DARRAY: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DSV_DIMENSION_TEXTURE2D: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DSV_DIMENSION_TEXTURE2DARRAY: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DSV_DIMENSION_TEXTURE2DMS: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(6i32);
impl ::core::marker::Copy for D3D12_DSV_DIMENSION {}
impl ::core::clone::Clone for D3D12_DSV_DIMENSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DSV_DIMENSION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_DSV_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DSV_FLAG_NONE: D3D12_DSV_FLAGS = D3D12_DSV_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DSV_FLAG_READ_ONLY_DEPTH: D3D12_DSV_FLAGS = D3D12_DSV_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_DSV_FLAG_READ_ONLY_STENCIL: D3D12_DSV_FLAGS = D3D12_DSV_FLAGS(2u32);
impl ::core::marker::Copy for D3D12_DSV_FLAGS {}
impl ::core::clone::Clone for D3D12_DSV_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DSV_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_ELEMENTS_LAYOUT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ELEMENTS_LAYOUT_ARRAY: D3D12_ELEMENTS_LAYOUT = D3D12_ELEMENTS_LAYOUT(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ELEMENTS_LAYOUT_ARRAY_OF_POINTERS: D3D12_ELEMENTS_LAYOUT = D3D12_ELEMENTS_LAYOUT(1i32);
impl ::core::marker::Copy for D3D12_ELEMENTS_LAYOUT {}
impl ::core::clone::Clone for D3D12_ELEMENTS_LAYOUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ELEMENTS_LAYOUT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_EXPORT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_EXPORT_FLAG_NONE: D3D12_EXPORT_FLAGS = D3D12_EXPORT_FLAGS(0u32);
impl ::core::marker::Copy for D3D12_EXPORT_FLAGS {}
impl ::core::clone::Clone for D3D12_EXPORT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_EXPORT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_FEATURE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS: D3D12_FEATURE = D3D12_FEATURE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_ARCHITECTURE: D3D12_FEATURE = D3D12_FEATURE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_FEATURE_LEVELS: D3D12_FEATURE = D3D12_FEATURE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_FORMAT_SUPPORT: D3D12_FEATURE = D3D12_FEATURE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS: D3D12_FEATURE = D3D12_FEATURE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_FORMAT_INFO: D3D12_FEATURE = D3D12_FEATURE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT: D3D12_FEATURE = D3D12_FEATURE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_SHADER_MODEL: D3D12_FEATURE = D3D12_FEATURE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS1: D3D12_FEATURE = D3D12_FEATURE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_SUPPORT: D3D12_FEATURE = D3D12_FEATURE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_ROOT_SIGNATURE: D3D12_FEATURE = D3D12_FEATURE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_ARCHITECTURE1: D3D12_FEATURE = D3D12_FEATURE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS2: D3D12_FEATURE = D3D12_FEATURE(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_SHADER_CACHE: D3D12_FEATURE = D3D12_FEATURE(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_COMMAND_QUEUE_PRIORITY: D3D12_FEATURE = D3D12_FEATURE(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS3: D3D12_FEATURE = D3D12_FEATURE(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_EXISTING_HEAPS: D3D12_FEATURE = D3D12_FEATURE(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS4: D3D12_FEATURE = D3D12_FEATURE(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_SERIALIZATION: D3D12_FEATURE = D3D12_FEATURE(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_CROSS_NODE: D3D12_FEATURE = D3D12_FEATURE(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS5: D3D12_FEATURE = D3D12_FEATURE(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_DISPLAYABLE: D3D12_FEATURE = D3D12_FEATURE(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS6: D3D12_FEATURE = D3D12_FEATURE(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_QUERY_META_COMMAND: D3D12_FEATURE = D3D12_FEATURE(31i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS7: D3D12_FEATURE = D3D12_FEATURE(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPE_COUNT: D3D12_FEATURE = D3D12_FEATURE(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPES: D3D12_FEATURE = D3D12_FEATURE(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS8: D3D12_FEATURE = D3D12_FEATURE(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS9: D3D12_FEATURE = D3D12_FEATURE(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS10: D3D12_FEATURE = D3D12_FEATURE(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FEATURE_D3D12_OPTIONS11: D3D12_FEATURE = D3D12_FEATURE(40i32);
impl ::core::marker::Copy for D3D12_FEATURE {}
impl ::core::clone::Clone for D3D12_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FEATURE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_FENCE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FENCE_FLAG_NONE: D3D12_FENCE_FLAGS = D3D12_FENCE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FENCE_FLAG_SHARED: D3D12_FENCE_FLAGS = D3D12_FENCE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FENCE_FLAG_SHARED_CROSS_ADAPTER: D3D12_FENCE_FLAGS = D3D12_FENCE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FENCE_FLAG_NON_MONITORED: D3D12_FENCE_FLAGS = D3D12_FENCE_FLAGS(4u32);
impl ::core::marker::Copy for D3D12_FENCE_FLAGS {}
impl ::core::clone::Clone for D3D12_FENCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FENCE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_FILL_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILL_MODE_WIREFRAME: D3D12_FILL_MODE = D3D12_FILL_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILL_MODE_SOLID: D3D12_FILL_MODE = D3D12_FILL_MODE(3i32);
impl ::core::marker::Copy for D3D12_FILL_MODE {}
impl ::core::clone::Clone for D3D12_FILL_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FILL_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_FILTER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MIN_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MIN_POINT_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MIN_LINEAR_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MIN_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MIN_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_ANISOTROPIC: D3D12_FILTER = D3D12_FILTER(85i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_COMPARISON_MIN_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(129i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(132i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(133i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(144i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(145i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(148i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(149i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_COMPARISON_ANISOTROPIC: D3D12_FILTER = D3D12_FILTER(213i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MINIMUM_MIN_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(256i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(257i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(260i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(261i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(272i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(273i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(276i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(277i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MINIMUM_ANISOTROPIC: D3D12_FILTER = D3D12_FILTER(341i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(384i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(385i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(388i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(389i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(400i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(401i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(404i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(405i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_MAXIMUM_ANISOTROPIC: D3D12_FILTER = D3D12_FILTER(469i32);
impl ::core::marker::Copy for D3D12_FILTER {}
impl ::core::clone::Clone for D3D12_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FILTER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_FILTER_REDUCTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_REDUCTION_TYPE_STANDARD: D3D12_FILTER_REDUCTION_TYPE = D3D12_FILTER_REDUCTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_REDUCTION_TYPE_COMPARISON: D3D12_FILTER_REDUCTION_TYPE = D3D12_FILTER_REDUCTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_REDUCTION_TYPE_MINIMUM: D3D12_FILTER_REDUCTION_TYPE = D3D12_FILTER_REDUCTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_REDUCTION_TYPE_MAXIMUM: D3D12_FILTER_REDUCTION_TYPE = D3D12_FILTER_REDUCTION_TYPE(3i32);
impl ::core::marker::Copy for D3D12_FILTER_REDUCTION_TYPE {}
impl ::core::clone::Clone for D3D12_FILTER_REDUCTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FILTER_REDUCTION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_FILTER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_TYPE_POINT: D3D12_FILTER_TYPE = D3D12_FILTER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FILTER_TYPE_LINEAR: D3D12_FILTER_TYPE = D3D12_FILTER_TYPE(1i32);
impl ::core::marker::Copy for D3D12_FILTER_TYPE {}
impl ::core::clone::Clone for D3D12_FILTER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FILTER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_FORMAT_SUPPORT1(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_NONE: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_BUFFER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_IA_VERTEX_BUFFER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_IA_INDEX_BUFFER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_SO_BUFFER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_TEXTURE1D: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_TEXTURE2D: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_TEXTURE3D: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_TEXTURECUBE: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_SHADER_LOAD: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_COMPARISON: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_MONO_TEXT: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(2048u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_MIP: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(4096u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_RENDER_TARGET: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(16384u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_BLENDABLE: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(32768u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_DEPTH_STENCIL: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(65536u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RESOLVE: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(262144u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_DISPLAY: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(524288u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_CAST_WITHIN_BIT_LAYOUT: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(1048576u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RENDERTARGET: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(2097152u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_MULTISAMPLE_LOAD: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(4194304u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_SHADER_GATHER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(8388608u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_BACK_BUFFER_CAST: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(16777216u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_TYPED_UNORDERED_ACCESS_VIEW: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(33554432u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_SHADER_GATHER_COMPARISON: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(67108864u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_DECODER_OUTPUT: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(134217728u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_OUTPUT: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(268435456u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_INPUT: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(536870912u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT1_VIDEO_ENCODER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(1073741824u32);
impl ::core::marker::Copy for D3D12_FORMAT_SUPPORT1 {}
impl ::core::clone::Clone for D3D12_FORMAT_SUPPORT1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FORMAT_SUPPORT1 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_FORMAT_SUPPORT2(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_NONE: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_ADD: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_UAV_TYPED_LOAD: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_UAV_TYPED_STORE: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_TILED: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_MULTIPLANE_OVERLAY: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(16384u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_FORMAT_SUPPORT2_SAMPLER_FEEDBACK: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(32768u32);
impl ::core::marker::Copy for D3D12_FORMAT_SUPPORT2 {}
impl ::core::clone::Clone for D3D12_FORMAT_SUPPORT2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FORMAT_SUPPORT2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_GPU_BASED_VALIDATION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GPU_BASED_VALIDATION_FLAGS_NONE: D3D12_GPU_BASED_VALIDATION_FLAGS = D3D12_GPU_BASED_VALIDATION_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GPU_BASED_VALIDATION_FLAGS_DISABLE_STATE_TRACKING: D3D12_GPU_BASED_VALIDATION_FLAGS = D3D12_GPU_BASED_VALIDATION_FLAGS(1i32);
impl ::core::marker::Copy for D3D12_GPU_BASED_VALIDATION_FLAGS {}
impl ::core::clone::Clone for D3D12_GPU_BASED_VALIDATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_GPU_BASED_VALIDATION_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_NONE: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS = D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_TRACKING_ONLY_SHADERS: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS = D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_UNGUARDED_VALIDATION_SHADERS: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS = D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_GUARDED_VALIDATION_SHADERS: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS = D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS_VALID_MASK: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS = D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(7i32);
impl ::core::marker::Copy for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {}
impl ::core::clone::Clone for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_NONE: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE = D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_STATE_TRACKING_ONLY: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE = D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_UNGUARDED_VALIDATION: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE = D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_GUARDED_VALIDATION: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE = D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const NUM_D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODES: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE = D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(4i32);
impl ::core::marker::Copy for D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE {}
impl ::core::clone::Clone for D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_GRAPHICS_STATES(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_NONE: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_IA_VERTEX_BUFFERS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_IA_INDEX_BUFFER: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_IA_PRIMITIVE_TOPOLOGY: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_DESCRIPTOR_HEAP: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_GRAPHICS_ROOT_SIGNATURE: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_COMPUTE_ROOT_SIGNATURE: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_RS_VIEWPORTS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_RS_SCISSOR_RECTS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_PREDICATION: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_OM_RENDER_TARGETS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_OM_STENCIL_REF: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_OM_BLEND_FACTOR: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(2048u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_PIPELINE_STATE: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(4096u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_SO_TARGETS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(8192u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_OM_DEPTH_BOUNDS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(16384u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_SAMPLE_POSITIONS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(32768u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_GRAPHICS_STATE_VIEW_INSTANCE_MASK: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(65536u32);
impl ::core::marker::Copy for D3D12_GRAPHICS_STATES {}
impl ::core::clone::Clone for D3D12_GRAPHICS_STATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_GRAPHICS_STATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_HEAP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_NONE: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_SHARED: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_DENY_BUFFERS: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_ALLOW_DISPLAY: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_SHARED_CROSS_ADAPTER: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_DENY_RT_DS_TEXTURES: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_DENY_NON_RT_DS_TEXTURES: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_HARDWARE_PROTECTED: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_ALLOW_WRITE_WATCH: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_ALLOW_SHADER_ATOMICS: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_CREATE_NOT_RESIDENT: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_CREATE_NOT_ZEROED: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_ALLOW_ALL_BUFFERS_AND_TEXTURES: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(192u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(68u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(132u32);
impl ::core::marker::Copy for D3D12_HEAP_FLAGS {}
impl ::core::clone::Clone for D3D12_HEAP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_HEAP_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_HEAP_SERIALIZATION_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_SERIALIZATION_TIER_0: D3D12_HEAP_SERIALIZATION_TIER = D3D12_HEAP_SERIALIZATION_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_SERIALIZATION_TIER_10: D3D12_HEAP_SERIALIZATION_TIER = D3D12_HEAP_SERIALIZATION_TIER(10i32);
impl ::core::marker::Copy for D3D12_HEAP_SERIALIZATION_TIER {}
impl ::core::clone::Clone for D3D12_HEAP_SERIALIZATION_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_HEAP_SERIALIZATION_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_HEAP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_TYPE_DEFAULT: D3D12_HEAP_TYPE = D3D12_HEAP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_TYPE_UPLOAD: D3D12_HEAP_TYPE = D3D12_HEAP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_TYPE_READBACK: D3D12_HEAP_TYPE = D3D12_HEAP_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HEAP_TYPE_CUSTOM: D3D12_HEAP_TYPE = D3D12_HEAP_TYPE(4i32);
impl ::core::marker::Copy for D3D12_HEAP_TYPE {}
impl ::core::clone::Clone for D3D12_HEAP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_HEAP_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_HIT_GROUP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HIT_GROUP_TYPE_TRIANGLES: D3D12_HIT_GROUP_TYPE = D3D12_HIT_GROUP_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HIT_GROUP_TYPE_PROCEDURAL_PRIMITIVE: D3D12_HIT_GROUP_TYPE = D3D12_HIT_GROUP_TYPE(1i32);
impl ::core::marker::Copy for D3D12_HIT_GROUP_TYPE {}
impl ::core::clone::Clone for D3D12_HIT_GROUP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_HIT_GROUP_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_HIT_KIND(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HIT_KIND_TRIANGLE_FRONT_FACE: D3D12_HIT_KIND = D3D12_HIT_KIND(254i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_HIT_KIND_TRIANGLE_BACK_FACE: D3D12_HIT_KIND = D3D12_HIT_KIND(255i32);
impl ::core::marker::Copy for D3D12_HIT_KIND {}
impl ::core::clone::Clone for D3D12_HIT_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_HIT_KIND {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_INDEX_BUFFER_STRIP_CUT_VALUE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFF: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFFFFFF: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE(2i32);
impl ::core::marker::Copy for D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {}
impl ::core::clone::Clone for D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_INDIRECT_ARGUMENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DRAW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDIRECT_ARGUMENT_TYPE_INDEX_BUFFER_VIEW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT_BUFFER_VIEW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDIRECT_ARGUMENT_TYPE_SHADER_RESOURCE_VIEW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDIRECT_ARGUMENT_TYPE_UNORDERED_ACCESS_VIEW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH_RAYS: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH_MESH: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(10i32);
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_TYPE {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INDIRECT_ARGUMENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_INPUT_CLASSIFICATION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA: D3D12_INPUT_CLASSIFICATION = D3D12_INPUT_CLASSIFICATION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA: D3D12_INPUT_CLASSIFICATION = D3D12_INPUT_CLASSIFICATION(1i32);
impl ::core::marker::Copy for D3D12_INPUT_CLASSIFICATION {}
impl ::core::clone::Clone for D3D12_INPUT_CLASSIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INPUT_CLASSIFICATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_LIFETIME_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LIFETIME_STATE_IN_USE: D3D12_LIFETIME_STATE = D3D12_LIFETIME_STATE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LIFETIME_STATE_NOT_IN_USE: D3D12_LIFETIME_STATE = D3D12_LIFETIME_STATE(1i32);
impl ::core::marker::Copy for D3D12_LIFETIME_STATE {}
impl ::core::clone::Clone for D3D12_LIFETIME_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_LIFETIME_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_LOGIC_OP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_CLEAR: D3D12_LOGIC_OP = D3D12_LOGIC_OP(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_SET: D3D12_LOGIC_OP = D3D12_LOGIC_OP(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_COPY: D3D12_LOGIC_OP = D3D12_LOGIC_OP(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_COPY_INVERTED: D3D12_LOGIC_OP = D3D12_LOGIC_OP(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_NOOP: D3D12_LOGIC_OP = D3D12_LOGIC_OP(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_INVERT: D3D12_LOGIC_OP = D3D12_LOGIC_OP(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_AND: D3D12_LOGIC_OP = D3D12_LOGIC_OP(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_NAND: D3D12_LOGIC_OP = D3D12_LOGIC_OP(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_OR: D3D12_LOGIC_OP = D3D12_LOGIC_OP(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_NOR: D3D12_LOGIC_OP = D3D12_LOGIC_OP(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_XOR: D3D12_LOGIC_OP = D3D12_LOGIC_OP(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_EQUIV: D3D12_LOGIC_OP = D3D12_LOGIC_OP(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_AND_REVERSE: D3D12_LOGIC_OP = D3D12_LOGIC_OP(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_AND_INVERTED: D3D12_LOGIC_OP = D3D12_LOGIC_OP(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_OR_REVERSE: D3D12_LOGIC_OP = D3D12_LOGIC_OP(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_LOGIC_OP_OR_INVERTED: D3D12_LOGIC_OP = D3D12_LOGIC_OP(15i32);
impl ::core::marker::Copy for D3D12_LOGIC_OP {}
impl ::core::clone::Clone for D3D12_LOGIC_OP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_LOGIC_OP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_MEASUREMENTS_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MEASUREMENTS_ACTION_KEEP_ALL: D3D12_MEASUREMENTS_ACTION = D3D12_MEASUREMENTS_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MEASUREMENTS_ACTION_COMMIT_RESULTS: D3D12_MEASUREMENTS_ACTION = D3D12_MEASUREMENTS_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MEASUREMENTS_ACTION_COMMIT_RESULTS_HIGH_PRIORITY: D3D12_MEASUREMENTS_ACTION = D3D12_MEASUREMENTS_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MEASUREMENTS_ACTION_DISCARD_PREVIOUS: D3D12_MEASUREMENTS_ACTION = D3D12_MEASUREMENTS_ACTION(3i32);
impl ::core::marker::Copy for D3D12_MEASUREMENTS_ACTION {}
impl ::core::clone::Clone for D3D12_MEASUREMENTS_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MEASUREMENTS_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_MEMORY_POOL(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MEMORY_POOL_UNKNOWN: D3D12_MEMORY_POOL = D3D12_MEMORY_POOL(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MEMORY_POOL_L0: D3D12_MEMORY_POOL = D3D12_MEMORY_POOL(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MEMORY_POOL_L1: D3D12_MEMORY_POOL = D3D12_MEMORY_POOL(2i32);
impl ::core::marker::Copy for D3D12_MEMORY_POOL {}
impl ::core::clone::Clone for D3D12_MEMORY_POOL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MEMORY_POOL {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_MESH_SHADER_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESH_SHADER_TIER_NOT_SUPPORTED: D3D12_MESH_SHADER_TIER = D3D12_MESH_SHADER_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESH_SHADER_TIER_1: D3D12_MESH_SHADER_TIER = D3D12_MESH_SHADER_TIER(10i32);
impl ::core::marker::Copy for D3D12_MESH_SHADER_TIER {}
impl ::core::clone::Clone for D3D12_MESH_SHADER_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MESH_SHADER_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_MESSAGE_CALLBACK_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CALLBACK_FLAG_NONE: D3D12_MESSAGE_CALLBACK_FLAGS = D3D12_MESSAGE_CALLBACK_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CALLBACK_IGNORE_FILTERS: D3D12_MESSAGE_CALLBACK_FLAGS = D3D12_MESSAGE_CALLBACK_FLAGS(1i32);
impl ::core::marker::Copy for D3D12_MESSAGE_CALLBACK_FLAGS {}
impl ::core::clone::Clone for D3D12_MESSAGE_CALLBACK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MESSAGE_CALLBACK_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_MESSAGE_CATEGORY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CATEGORY_APPLICATION_DEFINED: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CATEGORY_MISCELLANEOUS: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CATEGORY_INITIALIZATION: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CATEGORY_CLEANUP: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CATEGORY_COMPILATION: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CATEGORY_STATE_CREATION: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CATEGORY_STATE_SETTING: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CATEGORY_STATE_GETTING: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CATEGORY_EXECUTION: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_CATEGORY_SHADER: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(10i32);
impl ::core::marker::Copy for D3D12_MESSAGE_CATEGORY {}
impl ::core::clone::Clone for D3D12_MESSAGE_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MESSAGE_CATEGORY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_MESSAGE_ID(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_UNKNOWN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_STRING_FROM_APPLICATION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_THIS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER1: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER2: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER3: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER4: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER5: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER6: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER7: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER8: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER9: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER10: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER11: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER12: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER13: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER14: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER15: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CORRUPTED_MULTITHREADING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETPRIVATEDATA_MOREDATA: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDVIDEOPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(31i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDVIDEOPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(40i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(41i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(42i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(45i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(46i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(47i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(48i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(49i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(52i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(53i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(54i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(55i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(56i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(57i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(58i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(59i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(60i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(61i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(62i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(63i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(65i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(66i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(67i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(68i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(69i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(70i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(71i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(72i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(73i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(74i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(75i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(76i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(79i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(80i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(81i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(82i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(83i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(84i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(85i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(86i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(87i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(88i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(89i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(90i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(91i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(92i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(93i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(94i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(95i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(96i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(97i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(98i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(100i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(101i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(102i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(103i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(104i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(105i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(106i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(107i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(108i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(109i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(111i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(112i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(113i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(114i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(115i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(116i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(117i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(135i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_ROOT_SIGNATURE_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(200i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(201i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(202i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(209i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_TOO_SMALL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(210i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(211i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_FORMAT_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(212i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_TOO_SMALL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(213i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INVALID_PRIMITIVETOPOLOGY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(219i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_STRIDE_UNALIGNED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(221i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_OFFSET_UNALIGNED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(222i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(232i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(233i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(234i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(239i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(240i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(245i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(253i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_OBJECT_SUMMARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(255i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_DEVICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(274i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_SWAPCHAIN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(275i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(276i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(277i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(278i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAMTORASTERIZER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(280i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(283i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(284i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDENTRIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(285i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDSTRIDES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(286i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMSTRIDES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(287i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(289i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(290i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(291i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(292i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(294i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(295i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(296i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(297i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_UNMAP_NOTMAPPED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(310i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_MISMATCHED_DATA_SIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(318i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATECOMPUTESHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(321i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(322i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(323i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(331i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(332i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(333i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(334i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(335i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(336i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(337i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(340i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(341i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(342i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDVIDEOPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(343i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(344i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(345i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_UNRECOGNIZEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(346i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(354i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFORCEDSAMPLECOUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(401i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDLOGICOPS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(403i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(410i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(412i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(414i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(416i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(418i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(420i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(422i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(425i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(426i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(427i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(428i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(429i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(430i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(431i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDSOURCERECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(447i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CLEARVIEW_EMPTYRECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(448i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_UPDATETILEMAPPINGS_INVALID_PARAMETER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(493i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTILEMAPPINGS_INVALID_PARAMETER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(494i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEVICE_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(506i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEVICE_WARNING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(507i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(519i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_NULL_POINTER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(520i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_SUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(521i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_RESERVED_BITS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(522i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISSING_BIND_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(523i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_MISC_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(524i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MATCHING_STATES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(525i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMBINATION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(526i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_BEFORE_AFTER_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(527i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(528i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_SAMPLE_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(529i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(530i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMBINED_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(531i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAGS_FOR_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(532i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_SPLIT_BARRIER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(533i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_UNMATCHED_END: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(534i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_UNMATCHED_BEGIN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(535i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(536i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMMAND_LIST_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(537i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_INVALID_SUBRESOURCE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(538i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_CONTENTION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(540i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_RESET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(541i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_RESET_BUNDLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(542i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_CANNOT_RESET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(543i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_OPEN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(544i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_INVALID_BUNDLE_API: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(546i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_CLOSED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(547i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WRONG_COMMAND_ALLOCATOR_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(549i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_SYNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(552i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_SYNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(553i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_DESCRIPTOR_HEAP_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(554i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(557i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMANDALLOCATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(558i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_PIPELINESTATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(559i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMANDLIST12: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(560i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(562i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_DESCRIPTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(563i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_ROOTSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(564i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_LIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(565i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_HEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(566i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_MONITOREDFENCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(567i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_QUERYHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(568i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMANDSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(569i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_COMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(570i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_COMMANDALLOCATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(571i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_PIPELINESTATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(572i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_COMMANDLIST12: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(573i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(575i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_DESCRIPTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(576i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_ROOTSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(577i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_LIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(578i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_HEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(579i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_MONITOREDFENCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(580i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_QUERYHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(581i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_COMMANDSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(582i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(583i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDALLOCATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(584i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_PIPELINESTATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(585i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDLIST12: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(586i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(588i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_DESCRIPTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(589i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_ROOTSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(590i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_LIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(591i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_HEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(592i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_MONITOREDFENCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(593i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_QUERYHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(594i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(595i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(597i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDMISCFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(599i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDARG_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(602i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(603i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(604i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_POSSIBLY_INVALID_SUBRESOURCE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(607i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_INVALID_USE_OF_NON_RESIDENT_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(608i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_POSSIBLE_INVALID_USE_OF_NON_RESIDENT_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(609i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_BUNDLE_PIPELINE_STATE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(610i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_PRIMITIVE_TOPOLOGY_MISMATCH_PIPELINE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(611i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_TARGET_FORMAT_MISMATCH_PIPELINE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(613i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_TARGET_SAMPLE_DESC_MISMATCH_PIPELINE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(614i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEPTH_STENCIL_FORMAT_MISMATCH_PIPELINE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(615i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEPTH_STENCIL_SAMPLE_DESC_MISMATCH_PIPELINE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(616i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHADER_INVALIDBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(622i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHEAP_NULLDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(623i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDSIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(624i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDHEAPTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(625i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDCPUPAGEPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(626i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDMEMORYPOOL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(627i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(628i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDALIGNMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(629i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDMISCFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(630i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDMISCFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(631i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDARG_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(632i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEHEAP_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(633i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLHEAPPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(634i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDHEAPTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(635i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDCPUPAGEPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(636i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDMEMORYPOOL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(637i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDHEAPPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(638i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDHEAPMISCFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(639i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDHEAPMISCFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(640i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDARG_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(641i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(642i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETCUSTOMHEAPPROPERTIES_UNRECOGNIZEDHEAPTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(643i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETCUSTOMHEAPPROPERTIES_INVALIDHEAPTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(644i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_DESCRIPTOR_HEAP_INVALID_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(645i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_INVALID_DESCRIPTOR_HANDLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(646i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALID_CONSERVATIVERASTERMODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(647i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(649i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(650i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_UNORDEREDACCESS_VIEW_INVALID_COUNTER_USAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(652i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPY_DESCRIPTORS_INVALID_RANGES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(653i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPY_DESCRIPTORS_WRITE_ONLY_DESCRIPTOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(654i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RTV_FORMAT_NOT_UNKNOWN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(655i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_RENDER_TARGET_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(656i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VERTEX_SHADER_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(657i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INPUTLAYOUT_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(658i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_HS_DS_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(659i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_REGISTERINDEX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(660i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_COMPONENTTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(661i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_REGISTERMASK: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(662i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_SYSTEMVALUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(663i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(664i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_MINPRECISION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(665i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(666i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_XOR_DS_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(667i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HULL_SHADER_INPUT_TOPOLOGY_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(668i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_DS_CONTROL_POINT_COUNT_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(669i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_DS_TESSELLATOR_DOMAIN_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(670i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(671i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_USE_OF_FORCED_SAMPLE_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(672i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_PRIMITIVETOPOLOGY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(673i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_SYSTEMVALUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(674i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(675i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(676i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_OUTPUT_TYPE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(677i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_RENDER_TARGET_DOES_NOT_SUPPORT_LOGIC_OPS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(678i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RENDERTARGETVIEW_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(679i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_DEPTHSTENCILVIEW_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(680i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_GS_INPUT_PRIMITIVE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(681i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_POSITION_NOT_PRESENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(682i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MISSING_ROOT_SIGNATURE_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(683i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_INDEX_BUFFER_PROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(684i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_SAMPLE_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(685i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(686i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_DS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(687i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(688i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_GS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(689i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(690i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MISSING_ROOT_SIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(691i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_OPEN_BUNDLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(692i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_DESCRIPTOR_HEAP_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(693i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(694i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DRAW_EMPTY_SCISSOR_RECTANGLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(695i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_BLOB_NOT_FOUND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(696i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_DESERIALIZE_FAILED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(697i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_INVALID_CONFIGURATION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(698i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_NOT_SUPPORTED_ON_DEVICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(699i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLRESOURCEPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(700i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(701i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETRESOURCEALLOCATIONINFO_INVALIDRDESCS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(702i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MAKERESIDENT_NULLOBJECTARRAY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(703i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EVICT_NULLOBJECTARRAY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(705i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_DESCRIPTOR_TABLE_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(708i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_ROOT_CONSTANT_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(709i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_ROOT_CONSTANT_BUFFER_VIEW_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(710i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_ROOT_SHADER_RESOURCE_VIEW_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(711i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_ROOT_UNORDERED_ACCESS_VIEW_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(712i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_VERTEX_BUFFERS_INVALID_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(713i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_INDEX_BUFFER_INVALID_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(715i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_STREAM_OUTPUT_BUFFERS_INVALID_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(717i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDDIMENSIONALITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(718i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDLAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(719i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDIMENSIONALITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(720i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDALIGNMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(721i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDMIPLEVELS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(722i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDSAMPLEDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(723i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDLAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(724i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_INDEX_BUFFER_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(725i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_VERTEX_BUFFERS_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(726i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_STREAM_OUTPUT_BUFFERS_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(727i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_RENDER_TARGETS_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(728i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEQUERY_HEAP_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(729i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_BEGIN_END_QUERY_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(731i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CLOSE_COMMAND_LIST_OPEN_QUERY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(732i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVE_QUERY_DATA_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(733i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_PREDICATION_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(734i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_TIMESTAMPS_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(735i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(737i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(738i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_INVALIDSUBRESOURCERANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(739i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_INVALIDBASEOFFSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(740i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETCOPYABLELAYOUT_INVALIDSUBRESOURCERANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(739i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETCOPYABLELAYOUT_INVALIDBASEOFFSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(740i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_HEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(741i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_SAMPLER_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(742i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATECOMMANDSIGNATURE_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(743i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EXECUTE_INDIRECT_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(744i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETGPUVIRTUALADDRESS_INVALID_RESOURCE_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(745i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDCLEARVALUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(815i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDCLEARVALUEFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(816i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDCLEARVALUEFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(817i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATERESOURCE_CLEARVALUEDENORMFLUSH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(818i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CLEARRENDERTARGETVIEW_MISMATCHINGCLEARVALUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(820i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_MISMATCHINGCLEARVALUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(821i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MAP_INVALIDHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(822i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(823i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MAP_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(824i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(825i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MAP_INVALIDSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(826i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(827i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MAP_INVALIDRANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(828i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDRANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(829i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MAP_INVALIDDATAPOINTER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(832i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MAP_INVALIDARG_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(833i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MAP_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(834i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_BUNDLENOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(835i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_COMMANDLISTMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(836i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_OPENCOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(837i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_FAILEDCOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(838i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_NULLDST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(839i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDDSTRESOURCEDIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(840i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_DSTRANGEOUTOFBOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(841i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_NULLSRC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(842i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDSRCRESOURCEDIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(843i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_SRCRANGEOUTOFBOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(844i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDCOPYFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(845i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_NULLDST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(846i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDDSTTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(847i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTRESOURCEDIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(848i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(849i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(850i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTOFFSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(851i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDDSTFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(852i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(853i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(854i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTROWPITCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(855i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTPLACEMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(856i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTDSPLACEDFOOTPRINTFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(857i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_DSTREGIONOUTOFBOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(858i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_NULLSRC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(859i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDSRCTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(860i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCRESOURCEDIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(861i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(862i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(863i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCOFFSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(864i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDSRCFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(865i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(866i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(867i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCROWPITCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(868i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCPLACEMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(869i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCDSPLACEDFOOTPRINTFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(870i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_SRCREGIONOUTOFBOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(871i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTCOORDINATES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(872i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(873i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_FORMATMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(874i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_EMPTYBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(875i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDCOPYFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(876i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_SUBRESOURCE_INDEX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(877i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(878i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_RESOURCE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(879i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_SAMPLE_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(880i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_INVALID_SHADER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(881i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_CS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(882i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_MISSING_ROOT_SIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(883i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALIDCACHEDBLOB: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(884i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBADAPTERMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(885i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBDRIVERVERSIONMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(886i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBDESCMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(887i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBIGNORED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(888i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(889i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(890i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(891i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(892i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_EMPTYBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(893i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(894i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(895i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(896i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(897i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_EMPTYBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(898i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_TOO_MANY_NODES_SPECIFIED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(899i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_INVALID_NODE_INDEX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(900i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETHEAPPROPERTIES_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(901i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_NODE_MASK_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(902i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(903i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_MULTIPLE_SWAPCHAIN_BUFFER_REFERENCES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(904i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_TOO_MANY_SWAPCHAIN_REFERENCES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(905i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_QUEUE_TOO_MANY_SWAPCHAIN_REFERENCES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(906i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_WRONGSWAPCHAINBUFFERREFERENCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(907i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_SETRENDERTARGETS_INVALIDNUMRENDERTARGETS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(908i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(909i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(910i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHAREDRESOURCE_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(911i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHAREDRESOURCE_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(912i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHAREDHEAP_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(913i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_UNRECOGNIZEDPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(914i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_INVALIDSIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(915i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_INVALIDOBJECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(916i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_KEYEDMUTEX_INVALIDOBJECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(917i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_KEYEDMUTEX_INVALIDKEY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(918i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_KEYEDMUTEX_WRONGSTATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(919i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_PRIORITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(920i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_OBJECT_DELETED_WHILE_STILL_IN_USE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(921i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(922i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_HEAP_ADDRESS_RANGE_HAS_NO_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(923i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_RENDER_TARGET_DELETED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(924i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_ALL_RENDER_TARGETS_HAVE_UNKNOWN_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(925i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_HEAP_ADDRESS_RANGE_INTERSECTS_MULTIPLE_BUFFERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(926i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_GPU_WRITTEN_READBACK_RESOURCE_MAPPED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(927i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_UNMAP_RANGE_NOT_EMPTY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(929i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MAP_INVALID_NULLRANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(930i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_UNMAP_INVALID_NULLRANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(931i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_NO_GRAPHICS_API_SUPPORT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(932i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_NO_COMPUTE_API_SUPPORT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(933i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_RESOURCE_FLAGS_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(934i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_ROOT_ARGUMENT_UNINITIALIZED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(935i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_HEAP_INDEX_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(936i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_TABLE_REGISTER_INDEX_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(937i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_UNINITIALIZED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(938i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_TYPE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(939i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_SRV_RESOURCE_DIMENSION_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(940i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_UAV_RESOURCE_DIMENSION_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(941i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_INCOMPATIBLE_RESOURCE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(942i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYRESOURCE_NULLDST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(943i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYRESOURCE_INVALIDDSTRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(944i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYRESOURCE_NULLSRC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(945i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYRESOURCE_INVALIDSRCRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(946i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_NULLDST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(947i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALIDDSTRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(948i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_NULLSRC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(949i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALIDSRCRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(950i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_PIPELINE_STATE_TYPE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(951i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DISPATCH_ROOT_SIGNATURE_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(952i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DISPATCH_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(953i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_ZERO_BARRIERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(954i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_BEGIN_END_EVENT_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(955i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_POSSIBLE_BEFORE_AFTER_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(956i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_BEGIN_END: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(957i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_INVALID_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(958i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_USE_OF_ZERO_REFCOUNT_OBJECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(959i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_OBJECT_EVICTED_WHILE_STILL_IN_USE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(960i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_ROOT_DESCRIPTOR_ACCESS_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(961i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_INVALIDLIBRARYBLOB: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(962i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_DRIVERVERSIONMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(963i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_ADAPTERVERSIONMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(964i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(965i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_PIPELINELIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(966i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_PIPELINELIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(967i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_PIPELINELIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(968i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_STOREPIPELINE_NONAME: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(969i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_STOREPIPELINE_DUPLICATENAME: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(970i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LOADPIPELINE_NAMENOTFOUND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(971i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LOADPIPELINE_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(972i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_PIPELINELIBRARY_SERIALIZE_NOTENOUGHMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(973i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_OUTPUT_RT_OUTPUT_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(974i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SETEVENTONMULTIPLEFENCECOMPLETION_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(975i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_VIDEO_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(976i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_ALLOCATOR_VIDEO_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(977i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEQUERY_HEAP_VIDEO_DECODE_STATISTICS_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(978i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(979i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(980i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODESTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(981i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(982i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(983i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODESTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(984i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(985i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(986i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODESTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(987i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DECODE_FRAME_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(988i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEPRECATED_API: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(989i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_COMMAND_LIST_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(990i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DESCRIPTOR_TABLE_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(991i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_CONSTANT_BUFFER_VIEW_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(992i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_SHADER_RESOURCE_VIEW_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(993i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_UNORDERED_ACCESS_VIEW_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(994i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DISCARD_INVALID_SUBRESOURCE_RANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(995i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DISCARD_ONE_SUBRESOURCE_FOR_MIPS_WITH_RECTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(996i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DISCARD_NO_RECTS_FOR_NON_TEXTURE2D: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(997i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPY_ON_SAME_SUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(998i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SETRESIDENCYPRIORITY_INVALID_PAGEABLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(999i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1000i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_STATIC_DESCRIPTOR_INVALID_DESCRIPTOR_CHANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1001i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DATA_STATIC_DESCRIPTOR_INVALID_DATA_CHANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1002i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DATA_STATIC_WHILE_SET_AT_EXECUTE_DESCRIPTOR_INVALID_DATA_CHANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1003i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_STATIC_DESCRIPTOR_DATA_STATIC_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1004i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_RESOURCE_ACCESS_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1005i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_SAMPLER_MODE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1006i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_FENCE_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1007i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_DUPLICATE_SUBRESOURCE_TRANSITIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1008i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SETRESIDENCYPRIORITY_INVALID_PRIORITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1009i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_DESCRIPTOR_HEAP_LARGE_NUM_DESCRIPTORS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1013i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_BEGIN_EVENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1014i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_END_EVENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1015i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEVICE_DEBUG_LAYER_STARTUP_OPTIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1016i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_DEPTHBOUNDSTEST_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1017i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_DUPLICATE_SUBOBJECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1018i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_UNKNOWN_SUBOBJECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1019i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_ZERO_SIZE_STREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1020i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALID_STREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1021i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CANNOT_DEDUCE_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1022i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_STATIC_DESCRIPTOR_RESOURCE_DIMENSION_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1023i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_QUEUE_INSUFFICIENT_PRIVILEGE_FOR_GLOBAL_REALTIME: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1024i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_QUEUE_INSUFFICIENT_HARDWARE_SUPPORT_FOR_GLOBAL_REALTIME: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1025i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_ARCHITECTURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1026i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1027i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DST_RESOURCE_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1028i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DST_RANGE_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1029i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_SRC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1030i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_SRC_RESOURCE_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1031i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_SRC_RANGE_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1032i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_OFFSET_ALIGNMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1033i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DEPENDENT_RESOURCES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1034i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DEPENDENT_SUBRESOURCE_RANGES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1035i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DEPENDENT_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1036i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DEPENDENT_SUBRESOURCE_RANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1037i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DEPENDENT_SUBRESOURCE_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1038i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DEPENDENT_RANGE_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1039i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_ZERO_DEPENDENCIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1040i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DEVICE_CREATE_SHARED_HANDLE_INVALIDARG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1041i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESCRIPTOR_HANDLE_WITH_INVALID_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1042i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SETDEPTHBOUNDS_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1043i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_RESOURCE_STATE_IMPRECISE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1044i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_PIPELINE_STATE_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1045i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_MODEL_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1046i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_OBJECT_ACCESSED_WHILE_STILL_IN_USE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1047i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_PROGRAMMABLE_MSAA_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1048i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SETSAMPLEPOSITIONS_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1049i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCEREGION_INVALID_RECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1050i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1051i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSCOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1052i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSCOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1053i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1054i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSCOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1055i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSCOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1056i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1057i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSCOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1058i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSCOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1059i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1060i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSSTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1061i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1062i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSSTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1063i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1064i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSSTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1065i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_PROCESS_FRAME_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1066i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPY_INVALIDLAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1067i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_CRYPTO_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1068i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_CRYPTO_SESSION_POLICY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1069i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_PROTECTED_RESOURCE_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1070i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_CRYPTO_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1071i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_CRYPTO_SESSION_POLICY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1072i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_PROTECTED_RESOURCE_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1073i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_CRYPTO_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1074i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_CRYPTO_SESSION_POLICY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1075i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_PROTECTED_RESOURCE_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1076i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_PROTECTED_RESOURCE_SESSION_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1077i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_FENCE_INVALIDOPERATION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1078i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEQUERY_HEAP_COPY_QUEUE_TIMESTAMPS_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1079i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_DEFERRED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1080i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_RECORDTIME_ASSUMEDFROMFIRSTUSE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1081i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_RECORDTIME_ASSUMEDFROMCLEAR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1082i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1083i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1084i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1085i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDARG_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1086i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1087i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDADDRESS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1088i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDHANDLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1089i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_DEST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1090i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_MODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1091i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_ALIGNMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1092i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1093i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SETVIEWINSTANCEMASK_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1094i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VIEW_INSTANCING_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1095i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VIEW_INSTANCING_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1096i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_MISMATCH_DECODE_REFERENCE_ONLY_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1097i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYRESOURCE_MISMATCH_DECODE_REFERENCE_ONLY_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1098i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_DECODE_HEAP_CAPS_FAILURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1099i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_DECODE_HEAP_CAPS_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1100i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VIDEO_DECODE_SUPPORT_INVALID_INPUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1101i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_DECODER_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1102i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_METADATA_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1103i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VIEW_INSTANCING_VERTEX_SIZE_EXCEEDED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1104i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RUNTIME_INTERNAL_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1105i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_NO_VIDEO_API_SUPPORT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1106i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VIDEO_PROCESS_SUPPORT_INVALID_INPUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1107i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_PROCESSOR_CAPS_FAILURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1108i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VIDEO_PROCESS_SUPPORT_UNSUPPORTED_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1109i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VIDEO_DECODE_FRAME_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1110i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ENQUEUE_MAKE_RESIDENT_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1111i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1112i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VIDEO_PROCESS_FRAMES_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1113i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VIDEO_DECODE_SUPPORT_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1114i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMANDRECORDER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1115i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_COMMANDRECORDER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1116i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDRECORDER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1117i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_VIDEO_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1118i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_INVALID_SUPPORT_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1119i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1120i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_MORE_RECORDERS_THAN_LOGICAL_PROCESSORS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1121i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMANDPOOL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1122i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_COMMANDPOOL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1123i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDPOOL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1124i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_POOL_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1125i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_VIDEO_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1126i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_RECORDER_SUPPORT_FLAGS_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1127i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_RECORDER_CONTENTION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1128i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_RECORDER_USAGE_WITH_CREATECOMMANDLIST_COMMAND_LIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1129i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_USAGE_WITH_CREATECOMMANDLIST1_COMMAND_LIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1130i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CANNOT_EXECUTE_EMPTY_COMMAND_LIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1131i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CANNOT_RESET_COMMAND_POOL_WITH_OPEN_COMMAND_LISTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1132i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CANNOT_USE_COMMAND_RECORDER_WITHOUT_CURRENT_TARGET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1133i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CANNOT_CHANGE_COMMAND_RECORDER_TARGET_WHILE_RECORDING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1134i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_POOL_SYNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1135i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EVICT_UNDERFLOW: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1136i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_META_COMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1137i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_META_COMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1138i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_META_COMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1139i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALID_DST_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1140i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALID_SRC_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1141i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DST_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1142i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_SRC_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1143i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_NULL_BUFFER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1144i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_NULL_RESOURCE_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1145i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1146i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1147i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1148i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_OFFSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1149i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_RESOURCE_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1150i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_RESOURCE_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1151i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1152i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CANNOT_CREATE_GRAPHICS_AND_VIDEO_COMMAND_RECORDER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1153i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_UPDATETILEMAPPINGS_POSSIBLY_MISMATCHING_PROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1154i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_INVALID_COMMAND_LIST_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1155i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INCOMPATIBLE_WITH_STRUCTURED_BUFFERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1156i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMPUTE_ONLY_DEVICE_OPERATION_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1157i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1158i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1159i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPY_RAYTRACING_ACCELERATION_STRUCTURE_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1160i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DISPATCH_RAYS_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1161i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GET_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1162i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_LIFETIMETRACKER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1163i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_LIFETIMETRACKER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1164i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_LIFETIMETRACKER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1165i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROYOWNEDOBJECT_OBJECTNOTOWNED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1166i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_TRACKEDWORKLOAD: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1167i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_TRACKEDWORKLOAD: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1168i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_TRACKEDWORKLOAD: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1169i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1170i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_META_COMMAND_ID_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1171i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_META_COMMAND_UNSUPPORTED_PARAMS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1172i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_META_COMMAND_FAILED_ENUMERATION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1173i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_META_COMMAND_PARAMETER_SIZE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1174i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_UNINITIALIZED_META_COMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1175i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_META_COMMAND_INVALID_GPU_VIRTUAL_ADDRESS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1176i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1177i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1178i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1179i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1180i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1181i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1182i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEOMOTIONESTIMATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1183i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEOMOTIONESTIMATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1184i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOMOTIONESTIMATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1185i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEOMOTIONVECTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1186i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEOMOTIONVECTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1187i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOMOTIONVECTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1188i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOADS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1189i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOAD_PAIRS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1190i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_OUT_OF_ORDER_TRACKED_WORKLOAD_PAIR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1191i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CANNOT_ADD_TRACKED_WORKLOAD: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1192i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_INCOMPLETE_TRACKED_WORKLOAD_PAIR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1193i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_STATE_OBJECT_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1194i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GET_SHADER_IDENTIFIER_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1195i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GET_SHADER_STACK_SIZE_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1196i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GET_PIPELINE_STACK_SIZE_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1197i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_PIPELINE_STACK_SIZE_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1198i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GET_SHADER_IDENTIFIER_SIZE_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1199i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CHECK_DRIVER_MATCHING_IDENTIFIER_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1200i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CHECK_DRIVER_MATCHING_IDENTIFIER_DRIVER_REPORTED_ISSUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1201i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_INVALID_RESOURCE_BARRIER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1202i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_DISALLOWED_API_CALLED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1203i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_NEST_RENDER_PASSES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1204i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_END_WITHOUT_BEGIN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1205i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_CLOSE_COMMAND_LIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1206i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_GPU_WORK_WHILE_SUSPENDED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1207i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_SUSPEND_RESUME: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1208i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_NO_PRIOR_SUSPEND_WITHIN_EXECUTECOMMANDLISTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1209i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_NO_SUBSEQUENT_RESUME_WITHIN_EXECUTECOMMANDLISTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1210i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_TRACKED_WORKLOAD_COMMAND_QUEUE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1211i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_TRACKED_WORKLOAD_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1212i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_NO_ACCESS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1213i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RENDER_PASS_UNSUPPORTED_RESOLVE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1214i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INVALID_RESOURCE_PTR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1215i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WINDOWS7_FENCE_OUTOFORDER_SIGNAL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1216i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WINDOWS7_FENCE_OUTOFORDER_WAIT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1217i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VIDEO_CREATE_MOTION_ESTIMATOR_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1218i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VIDEO_CREATE_MOTION_VECTOR_HEAP_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1219i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ESTIMATE_MOTION_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1220i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVE_MOTION_VECTOR_HEAP_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1221i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETGPUVIRTUALADDRESS_INVALID_HEAP_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1222i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SET_BACKGROUND_PROCESSING_MODE_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1223i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_INVALID_COMMAND_LIST_TYPE_FOR_FEATURE_LEVEL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1224i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEOEXTENSIONCOMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1225i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEOEXTENSIONCOMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1226i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOEXTENSIONCOMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1227i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_INVALID_VIDEO_EXTENSION_COMMAND_ID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1228i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VIDEO_EXTENSION_COMMAND_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1229i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_NOT_UNIQUE_IN_DXIL_LIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1230i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VARIABLE_SHADING_RATE_NOT_ALLOWED_WITH_TIR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1231i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GEOMETRY_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1232i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RSSETSHADING_RATE_INVALID_SHADING_RATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1233i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RSSETSHADING_RATE_SHADING_RATE_NOT_PERMITTED_BY_CAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1234i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RSSETSHADING_RATE_INVALID_COMBINER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1235i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RSSETSHADINGRATEIMAGE_REQUIRES_TIER_2: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1236i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RSSETSHADINGRATE_REQUIRES_TIER_1: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1237i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1238i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_ARRAY_SIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1239i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_MIP_LEVEL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1240i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_SAMPLE_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1241i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_SAMPLE_QUALITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1242i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_NON_RETAIL_SHADER_MODEL_WONT_VALIDATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1243i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_AS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1244i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1245i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ADD_TO_STATE_OBJECT_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1246i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_PROTECTED_RESOURCE_SESSION_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1247i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_PSO_DESC_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1248i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_MS_INCOMPLETE_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1249i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_AS_NOT_MS_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1250i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_NOT_PS_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1251i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_NONZERO_SAMPLER_FEEDBACK_MIP_REGION_WITH_INCOMPATIBLE_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1252i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INPUTLAYOUT_SHADER_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1253i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_EMPTY_DISPATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1254i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOURCE_FORMAT_REQUIRES_SAMPLER_FEEDBACK_CAPABILITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1255i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_MIP_REGION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1256i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1257i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_SAMPLE_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1258i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_SAMPLE_QUALITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1259i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_LAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1260i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_REQUIRES_UNORDERED_ACCESS_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1261i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_NULL_ARGUMENTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1262i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_UAV_REQUIRES_SAMPLER_FEEDBACK_CAPABILITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1263i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_REQUIRES_FEEDBACK_MAP_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1264i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1265i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1266i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEMESHSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1267i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_TRANSCODE_INVALID_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1268i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_INVALID_MIP_LEVEL_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1269i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_TRANSCODE_ARRAY_SIZE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1270i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_MISMATCHING_TARGETED_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1271i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_OUTPUTEXCEEDSMAXSIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1272i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_GROUPSHAREDEXCEEDSMAXSIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1273i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VERTEX_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1274i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_MESH_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1275i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_MISMATCHEDASMSPAYLOADSIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1276i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_UNBOUNDED_STATIC_DESCRIPTORS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1277i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEAMPLIFICATIONSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1278i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEAMPLIFICATIONSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1279i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_SHADERCACHESESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1280i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_SHADERCACHESESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1281i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_SHADERCACHESESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1282i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1283i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_DISABLED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1284i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_ALREADYOPEN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1285i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_DEVELOPERMODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1286i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1287i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_STATEALREADYSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1288i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_IGNOREDFLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1289i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_ALREADYPRESENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1290i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_HASHCOLLISION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1291i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_CACHEFULL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1292i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_FINDVALUE_NOTFOUND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1293i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_CORRUPT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1294i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_DISABLED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1295i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_OVERSIZED_DISPATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1296i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1297i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1298i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1299i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1300i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1301i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1302i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_MISMATCH_ENCODE_REFERENCE_ONLY_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1303i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COPYRESOURCE_MISMATCH_ENCODE_REFERENCE_ONLY_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1304i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ENCODE_FRAME_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1305i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_ENCODE_FRAME_UNSUPPORTED_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1306i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVE_ENCODER_OUTPUT_METADATA_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1307i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVE_ENCODER_OUTPUT_METADATA_UNSUPPORTED_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1308i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1309i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_UNSUPPORTED_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1310i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_HEAP_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1311i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_HEAP_UNSUPPORTED_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1312i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATECOMMANDLIST_NULL_COMMANDALLOCATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1313i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CLEAR_UNORDERED_ACCESS_VIEW_INVALID_DESCRIPTOR_HANDLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1314i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_DESCRIPTOR_HEAP_NOT_SHADER_VISIBLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1315i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_BLENDOP_WARNING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1316i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_BLENDOPALPHA_WARNING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1317i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_WRITE_COMBINE_PERFORMANCE_WARNING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1318i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_RESOLVE_QUERY_INVALID_QUERY_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1319i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_NO_ACCESS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1320i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_COMMAND_LIST_STATIC_DESCRIPTOR_SAMPLER_MODE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1321i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_UNSUPPORTED_BUFFER_WIDTH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1322i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_TOPOLOGY_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1323i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_VRS_SUM_COMBINER_REQUIRES_CAPABILITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1324i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SETTING_SHADING_RATE_FROM_MS_REQUIRES_CAPABILITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1325i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_SHADERCACHEDELETE_NOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1326i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_SHADERCACHECLEAR_NOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1327i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_ID_D3D12_MESSAGES_END: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1328i32);
impl ::core::marker::Copy for D3D12_MESSAGE_ID {}
impl ::core::clone::Clone for D3D12_MESSAGE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MESSAGE_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_MESSAGE_SEVERITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_SEVERITY_CORRUPTION: D3D12_MESSAGE_SEVERITY = D3D12_MESSAGE_SEVERITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_SEVERITY_ERROR: D3D12_MESSAGE_SEVERITY = D3D12_MESSAGE_SEVERITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_SEVERITY_WARNING: D3D12_MESSAGE_SEVERITY = D3D12_MESSAGE_SEVERITY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_SEVERITY_INFO: D3D12_MESSAGE_SEVERITY = D3D12_MESSAGE_SEVERITY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MESSAGE_SEVERITY_MESSAGE: D3D12_MESSAGE_SEVERITY = D3D12_MESSAGE_SEVERITY(4i32);
impl ::core::marker::Copy for D3D12_MESSAGE_SEVERITY {}
impl ::core::clone::Clone for D3D12_MESSAGE_SEVERITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MESSAGE_SEVERITY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_META_COMMAND_PARAMETER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_META_COMMAND_PARAMETER_FLAG_INPUT: D3D12_META_COMMAND_PARAMETER_FLAGS = D3D12_META_COMMAND_PARAMETER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_META_COMMAND_PARAMETER_FLAG_OUTPUT: D3D12_META_COMMAND_PARAMETER_FLAGS = D3D12_META_COMMAND_PARAMETER_FLAGS(2u32);
impl ::core::marker::Copy for D3D12_META_COMMAND_PARAMETER_FLAGS {}
impl ::core::clone::Clone for D3D12_META_COMMAND_PARAMETER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_META_COMMAND_PARAMETER_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_META_COMMAND_PARAMETER_STAGE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_META_COMMAND_PARAMETER_STAGE_CREATION: D3D12_META_COMMAND_PARAMETER_STAGE = D3D12_META_COMMAND_PARAMETER_STAGE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_META_COMMAND_PARAMETER_STAGE_INITIALIZATION: D3D12_META_COMMAND_PARAMETER_STAGE = D3D12_META_COMMAND_PARAMETER_STAGE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_META_COMMAND_PARAMETER_STAGE_EXECUTION: D3D12_META_COMMAND_PARAMETER_STAGE = D3D12_META_COMMAND_PARAMETER_STAGE(2i32);
impl ::core::marker::Copy for D3D12_META_COMMAND_PARAMETER_STAGE {}
impl ::core::clone::Clone for D3D12_META_COMMAND_PARAMETER_STAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_META_COMMAND_PARAMETER_STAGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_META_COMMAND_PARAMETER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_META_COMMAND_PARAMETER_TYPE_FLOAT: D3D12_META_COMMAND_PARAMETER_TYPE = D3D12_META_COMMAND_PARAMETER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_META_COMMAND_PARAMETER_TYPE_UINT64: D3D12_META_COMMAND_PARAMETER_TYPE = D3D12_META_COMMAND_PARAMETER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_META_COMMAND_PARAMETER_TYPE_GPU_VIRTUAL_ADDRESS: D3D12_META_COMMAND_PARAMETER_TYPE = D3D12_META_COMMAND_PARAMETER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_META_COMMAND_PARAMETER_TYPE_CPU_DESCRIPTOR_HANDLE_HEAP_TYPE_CBV_SRV_UAV: D3D12_META_COMMAND_PARAMETER_TYPE = D3D12_META_COMMAND_PARAMETER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_META_COMMAND_PARAMETER_TYPE_GPU_DESCRIPTOR_HANDLE_HEAP_TYPE_CBV_SRV_UAV: D3D12_META_COMMAND_PARAMETER_TYPE = D3D12_META_COMMAND_PARAMETER_TYPE(4i32);
impl ::core::marker::Copy for D3D12_META_COMMAND_PARAMETER_TYPE {}
impl ::core::clone::Clone for D3D12_META_COMMAND_PARAMETER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_META_COMMAND_PARAMETER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_MULTIPLE_FENCE_WAIT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MULTIPLE_FENCE_WAIT_FLAG_NONE: D3D12_MULTIPLE_FENCE_WAIT_FLAGS = D3D12_MULTIPLE_FENCE_WAIT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MULTIPLE_FENCE_WAIT_FLAG_ANY: D3D12_MULTIPLE_FENCE_WAIT_FLAGS = D3D12_MULTIPLE_FENCE_WAIT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MULTIPLE_FENCE_WAIT_FLAG_ALL: D3D12_MULTIPLE_FENCE_WAIT_FLAGS = D3D12_MULTIPLE_FENCE_WAIT_FLAGS(0u32);
impl ::core::marker::Copy for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {}
impl ::core::clone::Clone for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_NONE: D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS = D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_TILED_RESOURCE: D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS = D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS(1u32);
impl ::core::marker::Copy for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {}
impl ::core::clone::Clone for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_PIPELINE_STATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_FLAG_NONE: D3D12_PIPELINE_STATE_FLAGS = D3D12_PIPELINE_STATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_FLAG_TOOL_DEBUG: D3D12_PIPELINE_STATE_FLAGS = D3D12_PIPELINE_STATE_FLAGS(1u32);
impl ::core::marker::Copy for D3D12_PIPELINE_STATE_FLAGS {}
impl ::core::clone::Clone for D3D12_PIPELINE_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PIPELINE_STATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL1: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VIEW_INSTANCING: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_AS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MAX_VALID: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(26i32);
impl ::core::marker::Copy for D3D12_PIPELINE_STATE_SUBOBJECT_TYPE {}
impl ::core::clone::Clone for D3D12_PIPELINE_STATE_SUBOBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PIPELINE_STATE_SUBOBJECT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_PREDICATION_OP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PREDICATION_OP_EQUAL_ZERO: D3D12_PREDICATION_OP = D3D12_PREDICATION_OP(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PREDICATION_OP_NOT_EQUAL_ZERO: D3D12_PREDICATION_OP = D3D12_PREDICATION_OP(1i32);
impl ::core::marker::Copy for D3D12_PREDICATION_OP {}
impl ::core::clone::Clone for D3D12_PREDICATION_OP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PREDICATION_OP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_PRIMITIVE_TOPOLOGY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_UNDEFINED: D3D12_PRIMITIVE_TOPOLOGY_TYPE = D3D12_PRIMITIVE_TOPOLOGY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_POINT: D3D12_PRIMITIVE_TOPOLOGY_TYPE = D3D12_PRIMITIVE_TOPOLOGY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE: D3D12_PRIMITIVE_TOPOLOGY_TYPE = D3D12_PRIMITIVE_TOPOLOGY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE: D3D12_PRIMITIVE_TOPOLOGY_TYPE = D3D12_PRIMITIVE_TOPOLOGY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_PATCH: D3D12_PRIMITIVE_TOPOLOGY_TYPE = D3D12_PRIMITIVE_TOPOLOGY_TYPE(4i32);
impl ::core::marker::Copy for D3D12_PRIMITIVE_TOPOLOGY_TYPE {}
impl ::core::clone::Clone for D3D12_PRIMITIVE_TOPOLOGY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PRIMITIVE_TOPOLOGY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_NOT_SUPPORTED: D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_1: D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_2: D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER(2i32);
impl ::core::marker::Copy for D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER {}
impl ::core::clone::Clone for D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_PROTECTED_RESOURCE_SESSION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PROTECTED_RESOURCE_SESSION_FLAG_NONE: D3D12_PROTECTED_RESOURCE_SESSION_FLAGS = D3D12_PROTECTED_RESOURCE_SESSION_FLAGS(0u32);
impl ::core::marker::Copy for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {}
impl ::core::clone::Clone for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAG_NONE: D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS = D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAG_SUPPORTED: D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS = D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS(1u32);
impl ::core::marker::Copy for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {}
impl ::core::clone::Clone for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_PROTECTED_SESSION_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PROTECTED_SESSION_STATUS_OK: D3D12_PROTECTED_SESSION_STATUS = D3D12_PROTECTED_SESSION_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_PROTECTED_SESSION_STATUS_INVALID: D3D12_PROTECTED_SESSION_STATUS = D3D12_PROTECTED_SESSION_STATUS(1i32);
impl ::core::marker::Copy for D3D12_PROTECTED_SESSION_STATUS {}
impl ::core::clone::Clone for D3D12_PROTECTED_SESSION_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PROTECTED_SESSION_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_QUERY_HEAP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_HEAP_TYPE_OCCLUSION: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_HEAP_TYPE_TIMESTAMP: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_HEAP_TYPE_SO_STATISTICS: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_HEAP_TYPE_VIDEO_DECODE_STATISTICS: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_HEAP_TYPE_COPY_QUEUE_TIMESTAMP: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS1: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(7i32);
impl ::core::marker::Copy for D3D12_QUERY_HEAP_TYPE {}
impl ::core::clone::Clone for D3D12_QUERY_HEAP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_QUERY_HEAP_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_QUERY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_TYPE_OCCLUSION: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_TYPE_BINARY_OCCLUSION: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_TYPE_TIMESTAMP: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_TYPE_PIPELINE_STATISTICS: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM0: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM1: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM2: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM3: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_TYPE_VIDEO_DECODE_STATISTICS: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_QUERY_TYPE_PIPELINE_STATISTICS1: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(10i32);
impl ::core::marker::Copy for D3D12_QUERY_TYPE {}
impl ::core::clone::Clone for D3D12_QUERY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_QUERY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_NONE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_UPDATE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_COMPACTION: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_TRACE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_BUILD: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_MINIMIZE_MEMORY: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PERFORM_UPDATE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(32u32);
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_CLONE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_COMPACT: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_VISUALIZATION_DECODE_FOR_TOOLS: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_SERIALIZE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_DESERIALIZE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(4i32);
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE(3i32);
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE(1i32);
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RAYTRACING_GEOMETRY_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_GEOMETRY_FLAG_NONE: D3D12_RAYTRACING_GEOMETRY_FLAGS = D3D12_RAYTRACING_GEOMETRY_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_GEOMETRY_FLAG_OPAQUE: D3D12_RAYTRACING_GEOMETRY_FLAGS = D3D12_RAYTRACING_GEOMETRY_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_GEOMETRY_FLAG_NO_DUPLICATE_ANYHIT_INVOCATION: D3D12_RAYTRACING_GEOMETRY_FLAGS = D3D12_RAYTRACING_GEOMETRY_FLAGS(2u32);
impl ::core::marker::Copy for D3D12_RAYTRACING_GEOMETRY_FLAGS {}
impl ::core::clone::Clone for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RAYTRACING_GEOMETRY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_GEOMETRY_TYPE_TRIANGLES: D3D12_RAYTRACING_GEOMETRY_TYPE = D3D12_RAYTRACING_GEOMETRY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_GEOMETRY_TYPE_PROCEDURAL_PRIMITIVE_AABBS: D3D12_RAYTRACING_GEOMETRY_TYPE = D3D12_RAYTRACING_GEOMETRY_TYPE(1i32);
impl ::core::marker::Copy for D3D12_RAYTRACING_GEOMETRY_TYPE {}
impl ::core::clone::Clone for D3D12_RAYTRACING_GEOMETRY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_GEOMETRY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RAYTRACING_INSTANCE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_INSTANCE_FLAG_NONE: D3D12_RAYTRACING_INSTANCE_FLAGS = D3D12_RAYTRACING_INSTANCE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_INSTANCE_FLAG_TRIANGLE_CULL_DISABLE: D3D12_RAYTRACING_INSTANCE_FLAGS = D3D12_RAYTRACING_INSTANCE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_INSTANCE_FLAG_TRIANGLE_FRONT_COUNTERCLOCKWISE: D3D12_RAYTRACING_INSTANCE_FLAGS = D3D12_RAYTRACING_INSTANCE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_INSTANCE_FLAG_FORCE_OPAQUE: D3D12_RAYTRACING_INSTANCE_FLAGS = D3D12_RAYTRACING_INSTANCE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_INSTANCE_FLAG_FORCE_NON_OPAQUE: D3D12_RAYTRACING_INSTANCE_FLAGS = D3D12_RAYTRACING_INSTANCE_FLAGS(8u32);
impl ::core::marker::Copy for D3D12_RAYTRACING_INSTANCE_FLAGS {}
impl ::core::clone::Clone for D3D12_RAYTRACING_INSTANCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_INSTANCE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RAYTRACING_PIPELINE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_PIPELINE_FLAG_NONE: D3D12_RAYTRACING_PIPELINE_FLAGS = D3D12_RAYTRACING_PIPELINE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_PIPELINE_FLAG_SKIP_TRIANGLES: D3D12_RAYTRACING_PIPELINE_FLAGS = D3D12_RAYTRACING_PIPELINE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_PIPELINE_FLAG_SKIP_PROCEDURAL_PRIMITIVES: D3D12_RAYTRACING_PIPELINE_FLAGS = D3D12_RAYTRACING_PIPELINE_FLAGS(512u32);
impl ::core::marker::Copy for D3D12_RAYTRACING_PIPELINE_FLAGS {}
impl ::core::clone::Clone for D3D12_RAYTRACING_PIPELINE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_PIPELINE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RAYTRACING_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_TIER_NOT_SUPPORTED: D3D12_RAYTRACING_TIER = D3D12_RAYTRACING_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_TIER_1_0: D3D12_RAYTRACING_TIER = D3D12_RAYTRACING_TIER(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAYTRACING_TIER_1_1: D3D12_RAYTRACING_TIER = D3D12_RAYTRACING_TIER(11i32);
impl ::core::marker::Copy for D3D12_RAYTRACING_TIER {}
impl ::core::clone::Clone for D3D12_RAYTRACING_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RAY_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAY_FLAG_NONE: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAY_FLAG_FORCE_OPAQUE: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAY_FLAG_FORCE_NON_OPAQUE: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAY_FLAG_ACCEPT_FIRST_HIT_AND_END_SEARCH: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAY_FLAG_SKIP_CLOSEST_HIT_SHADER: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAY_FLAG_CULL_BACK_FACING_TRIANGLES: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAY_FLAG_CULL_FRONT_FACING_TRIANGLES: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAY_FLAG_CULL_OPAQUE: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAY_FLAG_CULL_NON_OPAQUE: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAY_FLAG_SKIP_TRIANGLES: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RAY_FLAG_SKIP_PROCEDURAL_PRIMITIVES: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(512u32);
impl ::core::marker::Copy for D3D12_RAY_FLAGS {}
impl ::core::clone::Clone for D3D12_RAY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAY_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_DISCARD: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_PRESERVE: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_CLEAR: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_NO_ACCESS: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(3i32);
impl ::core::marker::Copy for D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE {}
impl ::core::clone::Clone for D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_DISCARD: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_PRESERVE: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_RESOLVE: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_NO_ACCESS: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(3i32);
impl ::core::marker::Copy for D3D12_RENDER_PASS_ENDING_ACCESS_TYPE {}
impl ::core::clone::Clone for D3D12_RENDER_PASS_ENDING_ACCESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_ENDING_ACCESS_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RENDER_PASS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_FLAG_NONE: D3D12_RENDER_PASS_FLAGS = D3D12_RENDER_PASS_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_FLAG_ALLOW_UAV_WRITES: D3D12_RENDER_PASS_FLAGS = D3D12_RENDER_PASS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_FLAG_SUSPENDING_PASS: D3D12_RENDER_PASS_FLAGS = D3D12_RENDER_PASS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_FLAG_RESUMING_PASS: D3D12_RENDER_PASS_FLAGS = D3D12_RENDER_PASS_FLAGS(4u32);
impl ::core::marker::Copy for D3D12_RENDER_PASS_FLAGS {}
impl ::core::clone::Clone for D3D12_RENDER_PASS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RENDER_PASS_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_TIER_0: D3D12_RENDER_PASS_TIER = D3D12_RENDER_PASS_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_TIER_1: D3D12_RENDER_PASS_TIER = D3D12_RENDER_PASS_TIER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RENDER_PASS_TIER_2: D3D12_RENDER_PASS_TIER = D3D12_RENDER_PASS_TIER(2i32);
impl ::core::marker::Copy for D3D12_RENDER_PASS_TIER {}
impl ::core::clone::Clone for D3D12_RENDER_PASS_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RESIDENCY_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESIDENCY_FLAG_NONE: D3D12_RESIDENCY_FLAGS = D3D12_RESIDENCY_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESIDENCY_FLAG_DENY_OVERBUDGET: D3D12_RESIDENCY_FLAGS = D3D12_RESIDENCY_FLAGS(1u32);
impl ::core::marker::Copy for D3D12_RESIDENCY_FLAGS {}
impl ::core::clone::Clone for D3D12_RESIDENCY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESIDENCY_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RESIDENCY_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESIDENCY_PRIORITY_MINIMUM: D3D12_RESIDENCY_PRIORITY = D3D12_RESIDENCY_PRIORITY(671088640i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESIDENCY_PRIORITY_LOW: D3D12_RESIDENCY_PRIORITY = D3D12_RESIDENCY_PRIORITY(1342177280i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESIDENCY_PRIORITY_NORMAL: D3D12_RESIDENCY_PRIORITY = D3D12_RESIDENCY_PRIORITY(2013265920i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESIDENCY_PRIORITY_HIGH: D3D12_RESIDENCY_PRIORITY = D3D12_RESIDENCY_PRIORITY(-1610547200i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESIDENCY_PRIORITY_MAXIMUM: D3D12_RESIDENCY_PRIORITY = D3D12_RESIDENCY_PRIORITY(-939524096i32);
impl ::core::marker::Copy for D3D12_RESIDENCY_PRIORITY {}
impl ::core::clone::Clone for D3D12_RESIDENCY_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESIDENCY_PRIORITY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RESOLVE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOLVE_MODE_DECOMPRESS: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOLVE_MODE_MIN: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOLVE_MODE_MAX: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOLVE_MODE_AVERAGE: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOLVE_MODE_ENCODE_SAMPLER_FEEDBACK: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOLVE_MODE_DECODE_SAMPLER_FEEDBACK: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(5i32);
impl ::core::marker::Copy for D3D12_RESOLVE_MODE {}
impl ::core::clone::Clone for D3D12_RESOLVE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOLVE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RESOURCE_BARRIER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_BARRIER_FLAG_NONE: D3D12_RESOURCE_BARRIER_FLAGS = D3D12_RESOURCE_BARRIER_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_BARRIER_FLAG_BEGIN_ONLY: D3D12_RESOURCE_BARRIER_FLAGS = D3D12_RESOURCE_BARRIER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_BARRIER_FLAG_END_ONLY: D3D12_RESOURCE_BARRIER_FLAGS = D3D12_RESOURCE_BARRIER_FLAGS(2u32);
impl ::core::marker::Copy for D3D12_RESOURCE_BARRIER_FLAGS {}
impl ::core::clone::Clone for D3D12_RESOURCE_BARRIER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_BARRIER_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RESOURCE_BARRIER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_BARRIER_TYPE_TRANSITION: D3D12_RESOURCE_BARRIER_TYPE = D3D12_RESOURCE_BARRIER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_BARRIER_TYPE_ALIASING: D3D12_RESOURCE_BARRIER_TYPE = D3D12_RESOURCE_BARRIER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_BARRIER_TYPE_UAV: D3D12_RESOURCE_BARRIER_TYPE = D3D12_RESOURCE_BARRIER_TYPE(2i32);
impl ::core::marker::Copy for D3D12_RESOURCE_BARRIER_TYPE {}
impl ::core::clone::Clone for D3D12_RESOURCE_BARRIER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_BARRIER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RESOURCE_BINDING_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_BINDING_TIER_1: D3D12_RESOURCE_BINDING_TIER = D3D12_RESOURCE_BINDING_TIER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_BINDING_TIER_2: D3D12_RESOURCE_BINDING_TIER = D3D12_RESOURCE_BINDING_TIER(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_BINDING_TIER_3: D3D12_RESOURCE_BINDING_TIER = D3D12_RESOURCE_BINDING_TIER(3i32);
impl ::core::marker::Copy for D3D12_RESOURCE_BINDING_TIER {}
impl ::core::clone::Clone for D3D12_RESOURCE_BINDING_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_BINDING_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RESOURCE_DIMENSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_DIMENSION_UNKNOWN: D3D12_RESOURCE_DIMENSION = D3D12_RESOURCE_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_DIMENSION_BUFFER: D3D12_RESOURCE_DIMENSION = D3D12_RESOURCE_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_DIMENSION_TEXTURE1D: D3D12_RESOURCE_DIMENSION = D3D12_RESOURCE_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_DIMENSION_TEXTURE2D: D3D12_RESOURCE_DIMENSION = D3D12_RESOURCE_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_DIMENSION_TEXTURE3D: D3D12_RESOURCE_DIMENSION = D3D12_RESOURCE_DIMENSION(4i32);
impl ::core::marker::Copy for D3D12_RESOURCE_DIMENSION {}
impl ::core::clone::Clone for D3D12_RESOURCE_DIMENSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_DIMENSION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RESOURCE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_FLAG_NONE: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_FLAG_ALLOW_CROSS_ADAPTER: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_FLAG_VIDEO_DECODE_REFERENCE_ONLY: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_FLAG_VIDEO_ENCODE_REFERENCE_ONLY: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(128u32);
impl ::core::marker::Copy for D3D12_RESOURCE_FLAGS {}
impl ::core::clone::Clone for D3D12_RESOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RESOURCE_HEAP_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_HEAP_TIER_1: D3D12_RESOURCE_HEAP_TIER = D3D12_RESOURCE_HEAP_TIER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_HEAP_TIER_2: D3D12_RESOURCE_HEAP_TIER = D3D12_RESOURCE_HEAP_TIER(2i32);
impl ::core::marker::Copy for D3D12_RESOURCE_HEAP_TIER {}
impl ::core::clone::Clone for D3D12_RESOURCE_HEAP_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_HEAP_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RESOURCE_STATES(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_COMMON: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_INDEX_BUFFER: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_RENDER_TARGET: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_UNORDERED_ACCESS: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_DEPTH_WRITE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_DEPTH_READ: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_NON_PIXEL_SHADER_RESOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_STREAM_OUT: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_INDIRECT_ARGUMENT: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_COPY_DEST: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_COPY_SOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(2048u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_RESOLVE_DEST: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(4096u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_RESOLVE_SOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(8192u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(4194304u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_SHADING_RATE_SOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(16777216u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_GENERIC_READ: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(2755u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_ALL_SHADER_RESOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(192u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_PRESENT: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_PREDICATION: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_VIDEO_DECODE_READ: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(65536u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_VIDEO_DECODE_WRITE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(131072u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_VIDEO_PROCESS_READ: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(262144u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_VIDEO_PROCESS_WRITE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(524288u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_VIDEO_ENCODE_READ: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(2097152u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RESOURCE_STATE_VIDEO_ENCODE_WRITE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(8388608u32);
impl ::core::marker::Copy for D3D12_RESOURCE_STATES {}
impl ::core::clone::Clone for D3D12_RESOURCE_STATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_STATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RLDO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RLDO_NONE: D3D12_RLDO_FLAGS = D3D12_RLDO_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RLDO_SUMMARY: D3D12_RLDO_FLAGS = D3D12_RLDO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RLDO_DETAIL: D3D12_RLDO_FLAGS = D3D12_RLDO_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RLDO_IGNORE_INTERNAL: D3D12_RLDO_FLAGS = D3D12_RLDO_FLAGS(4i32);
impl ::core::marker::Copy for D3D12_RLDO_FLAGS {}
impl ::core::clone::Clone for D3D12_RLDO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RLDO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_ROOT_DESCRIPTOR_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_DESCRIPTOR_FLAG_NONE: D3D12_ROOT_DESCRIPTOR_FLAGS = D3D12_ROOT_DESCRIPTOR_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_DESCRIPTOR_FLAG_DATA_VOLATILE: D3D12_ROOT_DESCRIPTOR_FLAGS = D3D12_ROOT_DESCRIPTOR_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_DESCRIPTOR_FLAG_DATA_STATIC_WHILE_SET_AT_EXECUTE: D3D12_ROOT_DESCRIPTOR_FLAGS = D3D12_ROOT_DESCRIPTOR_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_DESCRIPTOR_FLAG_DATA_STATIC: D3D12_ROOT_DESCRIPTOR_FLAGS = D3D12_ROOT_DESCRIPTOR_FLAGS(8u32);
impl ::core::marker::Copy for D3D12_ROOT_DESCRIPTOR_FLAGS {}
impl ::core::clone::Clone for D3D12_ROOT_DESCRIPTOR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_DESCRIPTOR_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_ROOT_PARAMETER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_PARAMETER_TYPE_CBV: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_PARAMETER_TYPE_SRV: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_PARAMETER_TYPE_UAV: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE(4i32);
impl ::core::marker::Copy for D3D12_ROOT_PARAMETER_TYPE {}
impl ::core::clone::Clone for D3D12_ROOT_PARAMETER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_PARAMETER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_ROOT_SIGNATURE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_NONE: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_LOCAL_ROOT_SIGNATURE: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_AMPLIFICATION_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_MESH_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(2048u32);
impl ::core::marker::Copy for D3D12_ROOT_SIGNATURE_FLAGS {}
impl ::core::clone::Clone for D3D12_ROOT_SIGNATURE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_SIGNATURE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_RTV_DIMENSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RTV_DIMENSION_UNKNOWN: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RTV_DIMENSION_BUFFER: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RTV_DIMENSION_TEXTURE1D: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RTV_DIMENSION_TEXTURE1DARRAY: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RTV_DIMENSION_TEXTURE2D: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RTV_DIMENSION_TEXTURE2DARRAY: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RTV_DIMENSION_TEXTURE2DMS: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_RTV_DIMENSION_TEXTURE3D: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(8i32);
impl ::core::marker::Copy for D3D12_RTV_DIMENSION {}
impl ::core::clone::Clone for D3D12_RTV_DIMENSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RTV_DIMENSION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SAMPLER_FEEDBACK_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SAMPLER_FEEDBACK_TIER_NOT_SUPPORTED: D3D12_SAMPLER_FEEDBACK_TIER = D3D12_SAMPLER_FEEDBACK_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SAMPLER_FEEDBACK_TIER_0_9: D3D12_SAMPLER_FEEDBACK_TIER = D3D12_SAMPLER_FEEDBACK_TIER(90i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SAMPLER_FEEDBACK_TIER_1_0: D3D12_SAMPLER_FEEDBACK_TIER = D3D12_SAMPLER_FEEDBACK_TIER(100i32);
impl ::core::marker::Copy for D3D12_SAMPLER_FEEDBACK_TIER {}
impl ::core::clone::Clone for D3D12_SAMPLER_FEEDBACK_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SAMPLER_FEEDBACK_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SERIALIZED_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SERIALIZED_DATA_RAYTRACING_ACCELERATION_STRUCTURE: D3D12_SERIALIZED_DATA_TYPE = D3D12_SERIALIZED_DATA_TYPE(0i32);
impl ::core::marker::Copy for D3D12_SERIALIZED_DATA_TYPE {}
impl ::core::clone::Clone for D3D12_SERIALIZED_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SERIALIZED_DATA_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHADER_CACHE_CONTROL_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_CONTROL_FLAG_DISABLE: D3D12_SHADER_CACHE_CONTROL_FLAGS = D3D12_SHADER_CACHE_CONTROL_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_CONTROL_FLAG_ENABLE: D3D12_SHADER_CACHE_CONTROL_FLAGS = D3D12_SHADER_CACHE_CONTROL_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_CONTROL_FLAG_CLEAR: D3D12_SHADER_CACHE_CONTROL_FLAGS = D3D12_SHADER_CACHE_CONTROL_FLAGS(4u32);
impl ::core::marker::Copy for D3D12_SHADER_CACHE_CONTROL_FLAGS {}
impl ::core::clone::Clone for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHADER_CACHE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_FLAG_NONE: D3D12_SHADER_CACHE_FLAGS = D3D12_SHADER_CACHE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_FLAG_DRIVER_VERSIONED: D3D12_SHADER_CACHE_FLAGS = D3D12_SHADER_CACHE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_FLAG_USE_WORKING_DIR: D3D12_SHADER_CACHE_FLAGS = D3D12_SHADER_CACHE_FLAGS(2u32);
impl ::core::marker::Copy for D3D12_SHADER_CACHE_FLAGS {}
impl ::core::clone::Clone for D3D12_SHADER_CACHE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_CACHE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHADER_CACHE_KIND_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_D3D_CACHE_FOR_DRIVER: D3D12_SHADER_CACHE_KIND_FLAGS = D3D12_SHADER_CACHE_KIND_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_D3D_CONVERSIONS: D3D12_SHADER_CACHE_KIND_FLAGS = D3D12_SHADER_CACHE_KIND_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_DRIVER_MANAGED: D3D12_SHADER_CACHE_KIND_FLAGS = D3D12_SHADER_CACHE_KIND_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_KIND_FLAG_APPLICATION_MANAGED: D3D12_SHADER_CACHE_KIND_FLAGS = D3D12_SHADER_CACHE_KIND_FLAGS(8u32);
impl ::core::marker::Copy for D3D12_SHADER_CACHE_KIND_FLAGS {}
impl ::core::clone::Clone for D3D12_SHADER_CACHE_KIND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_CACHE_KIND_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHADER_CACHE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_MODE_MEMORY: D3D12_SHADER_CACHE_MODE = D3D12_SHADER_CACHE_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_MODE_DISK: D3D12_SHADER_CACHE_MODE = D3D12_SHADER_CACHE_MODE(1i32);
impl ::core::marker::Copy for D3D12_SHADER_CACHE_MODE {}
impl ::core::clone::Clone for D3D12_SHADER_CACHE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_CACHE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHADER_CACHE_SUPPORT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_SUPPORT_NONE: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_SUPPORT_SINGLE_PSO: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_SUPPORT_LIBRARY: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_INPROC_CACHE: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_DISK_CACHE: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_SUPPORT_DRIVER_MANAGED_CACHE: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_SUPPORT_SHADER_CONTROL_CLEAR: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_CACHE_SUPPORT_SHADER_SESSION_DELETE: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(64u32);
impl ::core::marker::Copy for D3D12_SHADER_CACHE_SUPPORT_FLAGS {}
impl ::core::clone::Clone for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHADER_COMPONENT_MAPPING(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_0: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_1: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_2: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_3: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_FORCE_VALUE_0: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_FORCE_VALUE_1: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(5i32);
impl ::core::marker::Copy for D3D12_SHADER_COMPONENT_MAPPING {}
impl ::core::clone::Clone for D3D12_SHADER_COMPONENT_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_COMPONENT_MAPPING {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHADER_MIN_PRECISION_SUPPORT(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_MIN_PRECISION_SUPPORT_NONE: D3D12_SHADER_MIN_PRECISION_SUPPORT = D3D12_SHADER_MIN_PRECISION_SUPPORT(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_MIN_PRECISION_SUPPORT_10_BIT: D3D12_SHADER_MIN_PRECISION_SUPPORT = D3D12_SHADER_MIN_PRECISION_SUPPORT(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_MIN_PRECISION_SUPPORT_16_BIT: D3D12_SHADER_MIN_PRECISION_SUPPORT = D3D12_SHADER_MIN_PRECISION_SUPPORT(2u32);
impl ::core::marker::Copy for D3D12_SHADER_MIN_PRECISION_SUPPORT {}
impl ::core::clone::Clone for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHADER_VERSION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHVER_PIXEL_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHVER_VERTEX_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHVER_GEOMETRY_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHVER_HULL_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHVER_DOMAIN_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHVER_COMPUTE_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHVER_RESERVED0: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(65520i32);
impl ::core::marker::Copy for D3D12_SHADER_VERSION_TYPE {}
impl ::core::clone::Clone for D3D12_SHADER_VERSION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_VERSION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHADER_VISIBILITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_VISIBILITY_ALL: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_VISIBILITY_VERTEX: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_VISIBILITY_HULL: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_VISIBILITY_DOMAIN: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_VISIBILITY_GEOMETRY: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_VISIBILITY_PIXEL: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_VISIBILITY_AMPLIFICATION: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADER_VISIBILITY_MESH: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(7i32);
impl ::core::marker::Copy for D3D12_SHADER_VISIBILITY {}
impl ::core::clone::Clone for D3D12_SHADER_VISIBILITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_VISIBILITY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHADING_RATE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_1X1: D3D12_SHADING_RATE = D3D12_SHADING_RATE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_1X2: D3D12_SHADING_RATE = D3D12_SHADING_RATE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_2X1: D3D12_SHADING_RATE = D3D12_SHADING_RATE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_2X2: D3D12_SHADING_RATE = D3D12_SHADING_RATE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_2X4: D3D12_SHADING_RATE = D3D12_SHADING_RATE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_4X2: D3D12_SHADING_RATE = D3D12_SHADING_RATE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_4X4: D3D12_SHADING_RATE = D3D12_SHADING_RATE(10i32);
impl ::core::marker::Copy for D3D12_SHADING_RATE {}
impl ::core::clone::Clone for D3D12_SHADING_RATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADING_RATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHADING_RATE_COMBINER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_COMBINER_PASSTHROUGH: D3D12_SHADING_RATE_COMBINER = D3D12_SHADING_RATE_COMBINER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_COMBINER_OVERRIDE: D3D12_SHADING_RATE_COMBINER = D3D12_SHADING_RATE_COMBINER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_COMBINER_MIN: D3D12_SHADING_RATE_COMBINER = D3D12_SHADING_RATE_COMBINER(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_COMBINER_MAX: D3D12_SHADING_RATE_COMBINER = D3D12_SHADING_RATE_COMBINER(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHADING_RATE_COMBINER_SUM: D3D12_SHADING_RATE_COMBINER = D3D12_SHADING_RATE_COMBINER(4i32);
impl ::core::marker::Copy for D3D12_SHADING_RATE_COMBINER {}
impl ::core::clone::Clone for D3D12_SHADING_RATE_COMBINER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADING_RATE_COMBINER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_0: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER = D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_1: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER = D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_2: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER = D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER(2i32);
impl ::core::marker::Copy for D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER {}
impl ::core::clone::Clone for D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_SRV_DIMENSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_UNKNOWN: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_BUFFER: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_TEXTURE1D: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_TEXTURE1DARRAY: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_TEXTURE2D: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_TEXTURE2DARRAY: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_TEXTURE2DMS: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_TEXTURE3D: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_TEXTURECUBE: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_TEXTURECUBEARRAY: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(11i32);
impl ::core::marker::Copy for D3D12_SRV_DIMENSION {}
impl ::core::clone::Clone for D3D12_SRV_DIMENSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SRV_DIMENSION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_STATE_OBJECT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_OBJECT_FLAG_NONE: D3D12_STATE_OBJECT_FLAGS = D3D12_STATE_OBJECT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_OBJECT_FLAG_ALLOW_LOCAL_DEPENDENCIES_ON_EXTERNAL_DEFINITIONS: D3D12_STATE_OBJECT_FLAGS = D3D12_STATE_OBJECT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_OBJECT_FLAG_ALLOW_EXTERNAL_DEPENDENCIES_ON_LOCAL_DEFINITIONS: D3D12_STATE_OBJECT_FLAGS = D3D12_STATE_OBJECT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_OBJECT_FLAG_ALLOW_STATE_OBJECT_ADDITIONS: D3D12_STATE_OBJECT_FLAGS = D3D12_STATE_OBJECT_FLAGS(4u32);
impl ::core::marker::Copy for D3D12_STATE_OBJECT_FLAGS {}
impl ::core::clone::Clone for D3D12_STATE_OBJECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_STATE_OBJECT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_STATE_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_OBJECT_TYPE_COLLECTION: D3D12_STATE_OBJECT_TYPE = D3D12_STATE_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_OBJECT_TYPE_RAYTRACING_PIPELINE: D3D12_STATE_OBJECT_TYPE = D3D12_STATE_OBJECT_TYPE(3i32);
impl ::core::marker::Copy for D3D12_STATE_OBJECT_TYPE {}
impl ::core::clone::Clone for D3D12_STATE_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_STATE_OBJECT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_STATE_SUBOBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_STATE_OBJECT_CONFIG: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_GLOBAL_ROOT_SIGNATURE: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_LOCAL_ROOT_SIGNATURE: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_NODE_MASK: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_DXIL_LIBRARY: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_EXISTING_COLLECTION: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_SUBOBJECT_TO_EXPORTS_ASSOCIATION: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_SHADER_CONFIG: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_HIT_GROUP: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG1: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATE_SUBOBJECT_TYPE_MAX_VALID: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(13i32);
impl ::core::marker::Copy for D3D12_STATE_SUBOBJECT_TYPE {}
impl ::core::clone::Clone for D3D12_STATE_SUBOBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_STATE_SUBOBJECT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_STATIC_BORDER_COLOR(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK: D3D12_STATIC_BORDER_COLOR = D3D12_STATIC_BORDER_COLOR(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK: D3D12_STATIC_BORDER_COLOR = D3D12_STATIC_BORDER_COLOR(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE: D3D12_STATIC_BORDER_COLOR = D3D12_STATIC_BORDER_COLOR(2i32);
impl ::core::marker::Copy for D3D12_STATIC_BORDER_COLOR {}
impl ::core::clone::Clone for D3D12_STATIC_BORDER_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_STATIC_BORDER_COLOR {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_STENCIL_OP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STENCIL_OP_KEEP: D3D12_STENCIL_OP = D3D12_STENCIL_OP(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STENCIL_OP_ZERO: D3D12_STENCIL_OP = D3D12_STENCIL_OP(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STENCIL_OP_REPLACE: D3D12_STENCIL_OP = D3D12_STENCIL_OP(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STENCIL_OP_INCR_SAT: D3D12_STENCIL_OP = D3D12_STENCIL_OP(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STENCIL_OP_DECR_SAT: D3D12_STENCIL_OP = D3D12_STENCIL_OP(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STENCIL_OP_INVERT: D3D12_STENCIL_OP = D3D12_STENCIL_OP(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STENCIL_OP_INCR: D3D12_STENCIL_OP = D3D12_STENCIL_OP(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_STENCIL_OP_DECR: D3D12_STENCIL_OP = D3D12_STENCIL_OP(8i32);
impl ::core::marker::Copy for D3D12_STENCIL_OP {}
impl ::core::clone::Clone for D3D12_STENCIL_OP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_STENCIL_OP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_TEXTURE_ADDRESS_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_ADDRESS_MODE_WRAP: D3D12_TEXTURE_ADDRESS_MODE = D3D12_TEXTURE_ADDRESS_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_ADDRESS_MODE_MIRROR: D3D12_TEXTURE_ADDRESS_MODE = D3D12_TEXTURE_ADDRESS_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_ADDRESS_MODE_CLAMP: D3D12_TEXTURE_ADDRESS_MODE = D3D12_TEXTURE_ADDRESS_MODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_ADDRESS_MODE_BORDER: D3D12_TEXTURE_ADDRESS_MODE = D3D12_TEXTURE_ADDRESS_MODE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_ADDRESS_MODE_MIRROR_ONCE: D3D12_TEXTURE_ADDRESS_MODE = D3D12_TEXTURE_ADDRESS_MODE(5i32);
impl ::core::marker::Copy for D3D12_TEXTURE_ADDRESS_MODE {}
impl ::core::clone::Clone for D3D12_TEXTURE_ADDRESS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEXTURE_ADDRESS_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_TEXTURE_COPY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX: D3D12_TEXTURE_COPY_TYPE = D3D12_TEXTURE_COPY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT: D3D12_TEXTURE_COPY_TYPE = D3D12_TEXTURE_COPY_TYPE(1i32);
impl ::core::marker::Copy for D3D12_TEXTURE_COPY_TYPE {}
impl ::core::clone::Clone for D3D12_TEXTURE_COPY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEXTURE_COPY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_TEXTURE_LAYOUT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_LAYOUT_UNKNOWN: D3D12_TEXTURE_LAYOUT = D3D12_TEXTURE_LAYOUT(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_LAYOUT_ROW_MAJOR: D3D12_TEXTURE_LAYOUT = D3D12_TEXTURE_LAYOUT(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_LAYOUT_64KB_UNDEFINED_SWIZZLE: D3D12_TEXTURE_LAYOUT = D3D12_TEXTURE_LAYOUT(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TEXTURE_LAYOUT_64KB_STANDARD_SWIZZLE: D3D12_TEXTURE_LAYOUT = D3D12_TEXTURE_LAYOUT(3i32);
impl ::core::marker::Copy for D3D12_TEXTURE_LAYOUT {}
impl ::core::clone::Clone for D3D12_TEXTURE_LAYOUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEXTURE_LAYOUT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_TILED_RESOURCES_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILED_RESOURCES_TIER_NOT_SUPPORTED: D3D12_TILED_RESOURCES_TIER = D3D12_TILED_RESOURCES_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILED_RESOURCES_TIER_1: D3D12_TILED_RESOURCES_TIER = D3D12_TILED_RESOURCES_TIER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILED_RESOURCES_TIER_2: D3D12_TILED_RESOURCES_TIER = D3D12_TILED_RESOURCES_TIER(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILED_RESOURCES_TIER_3: D3D12_TILED_RESOURCES_TIER = D3D12_TILED_RESOURCES_TIER(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILED_RESOURCES_TIER_4: D3D12_TILED_RESOURCES_TIER = D3D12_TILED_RESOURCES_TIER(4i32);
impl ::core::marker::Copy for D3D12_TILED_RESOURCES_TIER {}
impl ::core::clone::Clone for D3D12_TILED_RESOURCES_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TILED_RESOURCES_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_TILE_COPY_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILE_COPY_FLAG_NONE: D3D12_TILE_COPY_FLAGS = D3D12_TILE_COPY_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILE_COPY_FLAG_NO_HAZARD: D3D12_TILE_COPY_FLAGS = D3D12_TILE_COPY_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILE_COPY_FLAG_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE: D3D12_TILE_COPY_FLAGS = D3D12_TILE_COPY_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILE_COPY_FLAG_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER: D3D12_TILE_COPY_FLAGS = D3D12_TILE_COPY_FLAGS(4u32);
impl ::core::marker::Copy for D3D12_TILE_COPY_FLAGS {}
impl ::core::clone::Clone for D3D12_TILE_COPY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TILE_COPY_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_TILE_MAPPING_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILE_MAPPING_FLAG_NONE: D3D12_TILE_MAPPING_FLAGS = D3D12_TILE_MAPPING_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILE_MAPPING_FLAG_NO_HAZARD: D3D12_TILE_MAPPING_FLAGS = D3D12_TILE_MAPPING_FLAGS(1u32);
impl ::core::marker::Copy for D3D12_TILE_MAPPING_FLAGS {}
impl ::core::clone::Clone for D3D12_TILE_MAPPING_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TILE_MAPPING_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_TILE_RANGE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILE_RANGE_FLAG_NONE: D3D12_TILE_RANGE_FLAGS = D3D12_TILE_RANGE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILE_RANGE_FLAG_NULL: D3D12_TILE_RANGE_FLAGS = D3D12_TILE_RANGE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILE_RANGE_FLAG_SKIP: D3D12_TILE_RANGE_FLAGS = D3D12_TILE_RANGE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_TILE_RANGE_FLAG_REUSE_SINGLE_TILE: D3D12_TILE_RANGE_FLAGS = D3D12_TILE_RANGE_FLAGS(4i32);
impl ::core::marker::Copy for D3D12_TILE_RANGE_FLAGS {}
impl ::core::clone::Clone for D3D12_TILE_RANGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TILE_RANGE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_UAV_DIMENSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_UAV_DIMENSION_UNKNOWN: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_UAV_DIMENSION_BUFFER: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_UAV_DIMENSION_TEXTURE1D: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_UAV_DIMENSION_TEXTURE1DARRAY: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_UAV_DIMENSION_TEXTURE2D: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_UAV_DIMENSION_TEXTURE2DARRAY: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_UAV_DIMENSION_TEXTURE3D: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(8i32);
impl ::core::marker::Copy for D3D12_UAV_DIMENSION {}
impl ::core::clone::Clone for D3D12_UAV_DIMENSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_UAV_DIMENSION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_VARIABLE_SHADING_RATE_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VARIABLE_SHADING_RATE_TIER_NOT_SUPPORTED: D3D12_VARIABLE_SHADING_RATE_TIER = D3D12_VARIABLE_SHADING_RATE_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VARIABLE_SHADING_RATE_TIER_1: D3D12_VARIABLE_SHADING_RATE_TIER = D3D12_VARIABLE_SHADING_RATE_TIER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VARIABLE_SHADING_RATE_TIER_2: D3D12_VARIABLE_SHADING_RATE_TIER = D3D12_VARIABLE_SHADING_RATE_TIER(2i32);
impl ::core::marker::Copy for D3D12_VARIABLE_SHADING_RATE_TIER {}
impl ::core::clone::Clone for D3D12_VARIABLE_SHADING_RATE_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_VARIABLE_SHADING_RATE_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_VIEW_INSTANCING_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIEW_INSTANCING_FLAG_NONE: D3D12_VIEW_INSTANCING_FLAGS = D3D12_VIEW_INSTANCING_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIEW_INSTANCING_FLAG_ENABLE_VIEW_INSTANCE_MASKING: D3D12_VIEW_INSTANCING_FLAGS = D3D12_VIEW_INSTANCING_FLAGS(1u32);
impl ::core::marker::Copy for D3D12_VIEW_INSTANCING_FLAGS {}
impl ::core::clone::Clone for D3D12_VIEW_INSTANCING_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_VIEW_INSTANCING_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_VIEW_INSTANCING_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIEW_INSTANCING_TIER_NOT_SUPPORTED: D3D12_VIEW_INSTANCING_TIER = D3D12_VIEW_INSTANCING_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIEW_INSTANCING_TIER_1: D3D12_VIEW_INSTANCING_TIER = D3D12_VIEW_INSTANCING_TIER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIEW_INSTANCING_TIER_2: D3D12_VIEW_INSTANCING_TIER = D3D12_VIEW_INSTANCING_TIER(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_VIEW_INSTANCING_TIER_3: D3D12_VIEW_INSTANCING_TIER = D3D12_VIEW_INSTANCING_TIER(3i32);
impl ::core::marker::Copy for D3D12_VIEW_INSTANCING_TIER {}
impl ::core::clone::Clone for D3D12_VIEW_INSTANCING_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_VIEW_INSTANCING_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_WAVE_MMA_TIER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_WAVE_MMA_TIER_NOT_SUPPORTED: D3D12_WAVE_MMA_TIER = D3D12_WAVE_MMA_TIER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_WAVE_MMA_TIER_1_0: D3D12_WAVE_MMA_TIER = D3D12_WAVE_MMA_TIER(10i32);
impl ::core::marker::Copy for D3D12_WAVE_MMA_TIER {}
impl ::core::clone::Clone for D3D12_WAVE_MMA_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_WAVE_MMA_TIER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D12_WRITEBUFFERIMMEDIATE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_WRITEBUFFERIMMEDIATE_MODE_DEFAULT: D3D12_WRITEBUFFERIMMEDIATE_MODE = D3D12_WRITEBUFFERIMMEDIATE_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_WRITEBUFFERIMMEDIATE_MODE_MARKER_IN: D3D12_WRITEBUFFERIMMEDIATE_MODE = D3D12_WRITEBUFFERIMMEDIATE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D12_WRITEBUFFERIMMEDIATE_MODE_MARKER_OUT: D3D12_WRITEBUFFERIMMEDIATE_MODE = D3D12_WRITEBUFFERIMMEDIATE_MODE(2i32);
impl ::core::marker::Copy for D3D12_WRITEBUFFERIMMEDIATE_MODE {}
impl ::core::clone::Clone for D3D12_WRITEBUFFERIMMEDIATE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_WRITEBUFFERIMMEDIATE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_ROOT_SIGNATURE_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_ROOT_SIGNATURE_VERSION_1: D3D_ROOT_SIGNATURE_VERSION = D3D_ROOT_SIGNATURE_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_ROOT_SIGNATURE_VERSION_1_0: D3D_ROOT_SIGNATURE_VERSION = D3D_ROOT_SIGNATURE_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_ROOT_SIGNATURE_VERSION_1_1: D3D_ROOT_SIGNATURE_VERSION = D3D_ROOT_SIGNATURE_VERSION(2i32);
impl ::core::marker::Copy for D3D_ROOT_SIGNATURE_VERSION {}
impl ::core::clone::Clone for D3D_ROOT_SIGNATURE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D_ROOT_SIGNATURE_VERSION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_SHADER_MODEL(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_MODEL_5_1: D3D_SHADER_MODEL = D3D_SHADER_MODEL(81i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_MODEL_6_0: D3D_SHADER_MODEL = D3D_SHADER_MODEL(96i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_MODEL_6_1: D3D_SHADER_MODEL = D3D_SHADER_MODEL(97i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_MODEL_6_2: D3D_SHADER_MODEL = D3D_SHADER_MODEL(98i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_MODEL_6_3: D3D_SHADER_MODEL = D3D_SHADER_MODEL(99i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_MODEL_6_4: D3D_SHADER_MODEL = D3D_SHADER_MODEL(100i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_MODEL_6_5: D3D_SHADER_MODEL = D3D_SHADER_MODEL(101i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_MODEL_6_6: D3D_SHADER_MODEL = D3D_SHADER_MODEL(102i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub const D3D_SHADER_MODEL_6_7: D3D_SHADER_MODEL = D3D_SHADER_MODEL(103i32);
impl ::core::marker::Copy for D3D_SHADER_MODEL {}
impl ::core::clone::Clone for D3D_SHADER_MODEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D_SHADER_MODEL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_AUTO_BREADCRUMB_NODE {
    pub pCommandListDebugNameA: *const u8,
    pub pCommandListDebugNameW: ::windows::core::PCWSTR,
    pub pCommandQueueDebugNameA: *const u8,
    pub pCommandQueueDebugNameW: ::windows::core::PCWSTR,
    pub pCommandList: ::windows::core::ManuallyDrop<ID3D12GraphicsCommandList>,
    pub pCommandQueue: ::windows::core::ManuallyDrop<ID3D12CommandQueue>,
    pub BreadcrumbCount: u32,
    pub pLastBreadcrumbValue: *const u32,
    pub pCommandHistory: *const D3D12_AUTO_BREADCRUMB_OP,
    pub pNext: *const D3D12_AUTO_BREADCRUMB_NODE,
}
impl ::core::clone::Clone for D3D12_AUTO_BREADCRUMB_NODE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_AUTO_BREADCRUMB_NODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_AUTO_BREADCRUMB_NODE1 {
    pub pCommandListDebugNameA: *const u8,
    pub pCommandListDebugNameW: ::windows::core::PCWSTR,
    pub pCommandQueueDebugNameA: *const u8,
    pub pCommandQueueDebugNameW: ::windows::core::PCWSTR,
    pub pCommandList: ::windows::core::ManuallyDrop<ID3D12GraphicsCommandList>,
    pub pCommandQueue: ::windows::core::ManuallyDrop<ID3D12CommandQueue>,
    pub BreadcrumbCount: u32,
    pub pLastBreadcrumbValue: *const u32,
    pub pCommandHistory: *const D3D12_AUTO_BREADCRUMB_OP,
    pub pNext: *const D3D12_AUTO_BREADCRUMB_NODE1,
    pub BreadcrumbContextsCount: u32,
    pub pBreadcrumbContexts: *mut D3D12_DRED_BREADCRUMB_CONTEXT,
}
impl ::core::clone::Clone for D3D12_AUTO_BREADCRUMB_NODE1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_AUTO_BREADCRUMB_NODE1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_BLEND_DESC {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub IndependentBlendEnable: super::super::Foundation::BOOL,
    pub RenderTarget: [D3D12_RENDER_TARGET_BLEND_DESC; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_BLEND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_BLEND_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
impl ::core::marker::Copy for D3D12_BOX {}
impl ::core::clone::Clone for D3D12_BOX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_BOX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_BUFFER_RTV {
    pub FirstElement: u64,
    pub NumElements: u32,
}
impl ::core::marker::Copy for D3D12_BUFFER_RTV {}
impl ::core::clone::Clone for D3D12_BUFFER_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_BUFFER_RTV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_BUFFER_SRV {
    pub FirstElement: u64,
    pub NumElements: u32,
    pub StructureByteStride: u32,
    pub Flags: D3D12_BUFFER_SRV_FLAGS,
}
impl ::core::marker::Copy for D3D12_BUFFER_SRV {}
impl ::core::clone::Clone for D3D12_BUFFER_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_BUFFER_SRV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_BUFFER_UAV {
    pub FirstElement: u64,
    pub NumElements: u32,
    pub StructureByteStride: u32,
    pub CounterOffsetInBytes: u64,
    pub Flags: D3D12_BUFFER_UAV_FLAGS,
}
impl ::core::marker::Copy for D3D12_BUFFER_UAV {}
impl ::core::clone::Clone for D3D12_BUFFER_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_BUFFER_UAV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC {
    pub DestAccelerationStructureData: u64,
    pub Inputs: D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS,
    pub SourceAccelerationStructureData: u64,
    pub ScratchAccelerationStructureData: u64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
    pub Type: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE,
    pub Flags: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS,
    pub NumDescs: u32,
    pub DescsLayout: D3D12_ELEMENTS_LAYOUT,
    pub Anonymous: D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {
    pub InstanceDescs: u64,
    pub pGeometryDescs: *const D3D12_RAYTRACING_GEOMETRY_DESC,
    pub ppGeometryDescs: *const *const D3D12_RAYTRACING_GEOMETRY_DESC,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {
    pub Type: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE,
    pub NumDescs: u32,
}
impl ::core::marker::Copy for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {}
impl ::core::clone::Clone for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_CACHED_PIPELINE_STATE {
    pub pCachedBlob: *const ::core::ffi::c_void,
    pub CachedBlobSizeInBytes: usize,
}
impl ::core::marker::Copy for D3D12_CACHED_PIPELINE_STATE {}
impl ::core::clone::Clone for D3D12_CACHED_PIPELINE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_CACHED_PIPELINE_STATE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_CLEAR_VALUE {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Anonymous: D3D12_CLEAR_VALUE_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_CLEAR_VALUE {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_CLEAR_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_CLEAR_VALUE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_CLEAR_VALUE_0 {
    pub Color: [f32; 4],
    pub DepthStencil: D3D12_DEPTH_STENCIL_VALUE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_CLEAR_VALUE_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_CLEAR_VALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_CLEAR_VALUE_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_COMMAND_QUEUE_DESC {
    pub Type: D3D12_COMMAND_LIST_TYPE,
    pub Priority: i32,
    pub Flags: D3D12_COMMAND_QUEUE_FLAGS,
    pub NodeMask: u32,
}
impl ::core::marker::Copy for D3D12_COMMAND_QUEUE_DESC {}
impl ::core::clone::Clone for D3D12_COMMAND_QUEUE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_COMMAND_QUEUE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_COMMAND_SIGNATURE_DESC {
    pub ByteStride: u32,
    pub NumArgumentDescs: u32,
    pub pArgumentDescs: *const D3D12_INDIRECT_ARGUMENT_DESC,
    pub NodeMask: u32,
}
impl ::core::marker::Copy for D3D12_COMMAND_SIGNATURE_DESC {}
impl ::core::clone::Clone for D3D12_COMMAND_SIGNATURE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_COMMAND_SIGNATURE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_COMPUTE_PIPELINE_STATE_DESC {
    pub pRootSignature: ::windows::core::ManuallyDrop<ID3D12RootSignature>,
    pub CS: D3D12_SHADER_BYTECODE,
    pub NodeMask: u32,
    pub CachedPSO: D3D12_CACHED_PIPELINE_STATE,
    pub Flags: D3D12_PIPELINE_STATE_FLAGS,
}
impl ::core::clone::Clone for D3D12_COMPUTE_PIPELINE_STATE_DESC {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_COMPUTE_PIPELINE_STATE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_CONSTANT_BUFFER_VIEW_DESC {
    pub BufferLocation: u64,
    pub SizeInBytes: u32,
}
impl ::core::marker::Copy for D3D12_CONSTANT_BUFFER_VIEW_DESC {}
impl ::core::clone::Clone for D3D12_CONSTANT_BUFFER_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_CONSTANT_BUFFER_VIEW_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_CPU_DESCRIPTOR_HANDLE {
    pub ptr: usize,
}
impl ::core::marker::Copy for D3D12_CPU_DESCRIPTOR_HANDLE {}
impl ::core::clone::Clone for D3D12_CPU_DESCRIPTOR_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_CPU_DESCRIPTOR_HANDLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {
    pub ShaderPatchMode: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE,
}
impl ::core::marker::Copy for D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {}
impl ::core::clone::Clone for D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {
    pub MaxMessagesPerCommandList: u32,
    pub DefaultShaderPatchMode: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE,
    pub PipelineStateCreateFlags: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS,
}
impl ::core::marker::Copy for D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {}
impl ::core::clone::Clone for D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {
    pub SlowdownFactor: f32,
}
impl ::core::marker::Copy for D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {}
impl ::core::clone::Clone for D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D12_STENCIL_OP,
    pub StencilDepthFailOp: D3D12_STENCIL_OP,
    pub StencilPassOp: D3D12_STENCIL_OP,
    pub StencilFunc: D3D12_COMPARISON_FUNC,
}
impl ::core::marker::Copy for D3D12_DEPTH_STENCILOP_DESC {}
impl ::core::clone::Clone for D3D12_DEPTH_STENCILOP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEPTH_STENCILOP_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DEPTH_STENCIL_DESC {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D12_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D12_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D12_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D12_DEPTH_STENCILOP_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DEPTH_STENCIL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DEPTH_STENCIL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_DEPTH_STENCIL_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DEPTH_STENCIL_DESC1 {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D12_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D12_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D12_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D12_DEPTH_STENCILOP_DESC,
    pub DepthBoundsTestEnable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DEPTH_STENCIL_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DEPTH_STENCIL_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_DEPTH_STENCIL_DESC1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DEPTH_STENCIL_VALUE {
    pub Depth: f32,
    pub Stencil: u8,
}
impl ::core::marker::Copy for D3D12_DEPTH_STENCIL_VALUE {}
impl ::core::clone::Clone for D3D12_DEPTH_STENCIL_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEPTH_STENCIL_VALUE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_DSV_DIMENSION,
    pub Flags: D3D12_DSV_FLAGS,
    pub Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_DEPTH_STENCIL_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_DEPTH_STENCIL_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_DEPTH_STENCIL_VIEW_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D12_TEX1D_DSV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D12_TEX2D_DSV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D12_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D12_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_DEPTH_STENCIL_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DESCRIPTOR_HEAP_DESC {
    pub Type: D3D12_DESCRIPTOR_HEAP_TYPE,
    pub NumDescriptors: u32,
    pub Flags: D3D12_DESCRIPTOR_HEAP_FLAGS,
    pub NodeMask: u32,
}
impl ::core::marker::Copy for D3D12_DESCRIPTOR_HEAP_DESC {}
impl ::core::clone::Clone for D3D12_DESCRIPTOR_HEAP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DESCRIPTOR_HEAP_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DESCRIPTOR_RANGE {
    pub RangeType: D3D12_DESCRIPTOR_RANGE_TYPE,
    pub NumDescriptors: u32,
    pub BaseShaderRegister: u32,
    pub RegisterSpace: u32,
    pub OffsetInDescriptorsFromTableStart: u32,
}
impl ::core::marker::Copy for D3D12_DESCRIPTOR_RANGE {}
impl ::core::clone::Clone for D3D12_DESCRIPTOR_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DESCRIPTOR_RANGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DESCRIPTOR_RANGE1 {
    pub RangeType: D3D12_DESCRIPTOR_RANGE_TYPE,
    pub NumDescriptors: u32,
    pub BaseShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Flags: D3D12_DESCRIPTOR_RANGE_FLAGS,
    pub OffsetInDescriptorsFromTableStart: u32,
}
impl ::core::marker::Copy for D3D12_DESCRIPTOR_RANGE1 {}
impl ::core::clone::Clone for D3D12_DESCRIPTOR_RANGE1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DESCRIPTOR_RANGE1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA {
    pub Flags: D3D12_DRED_FLAGS,
    pub pHeadAutoBreadcrumbNode: *mut D3D12_AUTO_BREADCRUMB_NODE,
}
impl ::core::marker::Copy for D3D12_DEVICE_REMOVED_EXTENDED_DATA {}
impl ::core::clone::Clone for D3D12_DEVICE_REMOVED_EXTENDED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEVICE_REMOVED_EXTENDED_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {
    pub DeviceRemovedReason: ::windows::core::HRESULT,
    pub AutoBreadcrumbsOutput: D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT,
    pub PageFaultOutput: D3D12_DRED_PAGE_FAULT_OUTPUT,
}
impl ::core::marker::Copy for D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {}
impl ::core::clone::Clone for D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {
    pub DeviceRemovedReason: ::windows::core::HRESULT,
    pub AutoBreadcrumbsOutput: D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1,
    pub PageFaultOutput: D3D12_DRED_PAGE_FAULT_OUTPUT1,
}
impl ::core::marker::Copy for D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {}
impl ::core::clone::Clone for D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {
    pub DeviceRemovedReason: ::windows::core::HRESULT,
    pub AutoBreadcrumbsOutput: D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1,
    pub PageFaultOutput: D3D12_DRED_PAGE_FAULT_OUTPUT2,
    pub DeviceState: D3D12_DRED_DEVICE_STATE,
}
impl ::core::marker::Copy for D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {}
impl ::core::clone::Clone for D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DISCARD_REGION {
    pub NumRects: u32,
    pub pRects: *const super::super::Foundation::RECT,
    pub FirstSubresource: u32,
    pub NumSubresources: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DISCARD_REGION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DISCARD_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_DISCARD_REGION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DISPATCH_ARGUMENTS {
    pub ThreadGroupCountX: u32,
    pub ThreadGroupCountY: u32,
    pub ThreadGroupCountZ: u32,
}
impl ::core::marker::Copy for D3D12_DISPATCH_ARGUMENTS {}
impl ::core::clone::Clone for D3D12_DISPATCH_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DISPATCH_ARGUMENTS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DISPATCH_MESH_ARGUMENTS {
    pub ThreadGroupCountX: u32,
    pub ThreadGroupCountY: u32,
    pub ThreadGroupCountZ: u32,
}
impl ::core::marker::Copy for D3D12_DISPATCH_MESH_ARGUMENTS {}
impl ::core::clone::Clone for D3D12_DISPATCH_MESH_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DISPATCH_MESH_ARGUMENTS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DISPATCH_RAYS_DESC {
    pub RayGenerationShaderRecord: D3D12_GPU_VIRTUAL_ADDRESS_RANGE,
    pub MissShaderTable: D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE,
    pub HitGroupTable: D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE,
    pub CallableShaderTable: D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE,
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
}
impl ::core::marker::Copy for D3D12_DISPATCH_RAYS_DESC {}
impl ::core::clone::Clone for D3D12_DISPATCH_RAYS_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DISPATCH_RAYS_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DRAW_ARGUMENTS {
    pub VertexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartVertexLocation: u32,
    pub StartInstanceLocation: u32,
}
impl ::core::marker::Copy for D3D12_DRAW_ARGUMENTS {}
impl ::core::clone::Clone for D3D12_DRAW_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRAW_ARGUMENTS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DRAW_INDEXED_ARGUMENTS {
    pub IndexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartIndexLocation: u32,
    pub BaseVertexLocation: i32,
    pub StartInstanceLocation: u32,
}
impl ::core::marker::Copy for D3D12_DRAW_INDEXED_ARGUMENTS {}
impl ::core::clone::Clone for D3D12_DRAW_INDEXED_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRAW_INDEXED_ARGUMENTS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DRED_ALLOCATION_NODE {
    pub ObjectNameA: *const u8,
    pub ObjectNameW: ::windows::core::PCWSTR,
    pub AllocationType: D3D12_DRED_ALLOCATION_TYPE,
    pub pNext: *const D3D12_DRED_ALLOCATION_NODE,
}
impl ::core::marker::Copy for D3D12_DRED_ALLOCATION_NODE {}
impl ::core::clone::Clone for D3D12_DRED_ALLOCATION_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_ALLOCATION_NODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DRED_ALLOCATION_NODE1 {
    pub ObjectNameA: *const u8,
    pub ObjectNameW: ::windows::core::PCWSTR,
    pub AllocationType: D3D12_DRED_ALLOCATION_TYPE,
    pub pNext: *const D3D12_DRED_ALLOCATION_NODE1,
    pub pObject: ::windows::core::ManuallyDrop<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for D3D12_DRED_ALLOCATION_NODE1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_ALLOCATION_NODE1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {
    pub pHeadAutoBreadcrumbNode: *const D3D12_AUTO_BREADCRUMB_NODE,
}
impl ::core::marker::Copy for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {}
impl ::core::clone::Clone for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {
    pub pHeadAutoBreadcrumbNode: *const D3D12_AUTO_BREADCRUMB_NODE1,
}
impl ::core::marker::Copy for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {}
impl ::core::clone::Clone for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DRED_BREADCRUMB_CONTEXT {
    pub BreadcrumbIndex: u32,
    pub pContextString: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for D3D12_DRED_BREADCRUMB_CONTEXT {}
impl ::core::clone::Clone for D3D12_DRED_BREADCRUMB_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_BREADCRUMB_CONTEXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT {
    pub PageFaultVA: u64,
    pub pHeadExistingAllocationNode: *const D3D12_DRED_ALLOCATION_NODE,
    pub pHeadRecentFreedAllocationNode: *const D3D12_DRED_ALLOCATION_NODE,
}
impl ::core::marker::Copy for D3D12_DRED_PAGE_FAULT_OUTPUT {}
impl ::core::clone::Clone for D3D12_DRED_PAGE_FAULT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_PAGE_FAULT_OUTPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT1 {
    pub PageFaultVA: u64,
    pub pHeadExistingAllocationNode: *const D3D12_DRED_ALLOCATION_NODE1,
    pub pHeadRecentFreedAllocationNode: *const D3D12_DRED_ALLOCATION_NODE1,
}
impl ::core::marker::Copy for D3D12_DRED_PAGE_FAULT_OUTPUT1 {}
impl ::core::clone::Clone for D3D12_DRED_PAGE_FAULT_OUTPUT1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_PAGE_FAULT_OUTPUT1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT2 {
    pub PageFaultVA: u64,
    pub pHeadExistingAllocationNode: *const D3D12_DRED_ALLOCATION_NODE1,
    pub pHeadRecentFreedAllocationNode: *const D3D12_DRED_ALLOCATION_NODE1,
    pub PageFaultFlags: D3D12_DRED_PAGE_FAULT_FLAGS,
}
impl ::core::marker::Copy for D3D12_DRED_PAGE_FAULT_OUTPUT2 {}
impl ::core::clone::Clone for D3D12_DRED_PAGE_FAULT_OUTPUT2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DRED_PAGE_FAULT_OUTPUT2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DXIL_LIBRARY_DESC {
    pub DXILLibrary: D3D12_SHADER_BYTECODE,
    pub NumExports: u32,
    pub pExports: *mut D3D12_EXPORT_DESC,
}
impl ::core::marker::Copy for D3D12_DXIL_LIBRARY_DESC {}
impl ::core::clone::Clone for D3D12_DXIL_LIBRARY_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DXIL_LIBRARY_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    pub SubobjectToAssociate: ::windows::core::PCWSTR,
    pub NumExports: u32,
    pub pExports: *mut ::windows::core::PWSTR,
}
impl ::core::marker::Copy for D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {}
impl ::core::clone::Clone for D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_EXISTING_COLLECTION_DESC {
    pub pExistingCollection: ::windows::core::ManuallyDrop<ID3D12StateObject>,
    pub NumExports: u32,
    pub pExports: *mut D3D12_EXPORT_DESC,
}
impl ::core::clone::Clone for D3D12_EXISTING_COLLECTION_DESC {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_EXISTING_COLLECTION_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_EXPORT_DESC {
    pub Name: ::windows::core::PCWSTR,
    pub ExportToRename: ::windows::core::PCWSTR,
    pub Flags: D3D12_EXPORT_FLAGS,
}
impl ::core::marker::Copy for D3D12_EXPORT_DESC {}
impl ::core::clone::Clone for D3D12_EXPORT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_EXPORT_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_ARCHITECTURE {
    pub NodeIndex: u32,
    pub TileBasedRenderer: super::super::Foundation::BOOL,
    pub UMA: super::super::Foundation::BOOL,
    pub CacheCoherentUMA: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_ARCHITECTURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_ARCHITECTURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_ARCHITECTURE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_ARCHITECTURE1 {
    pub NodeIndex: u32,
    pub TileBasedRenderer: super::super::Foundation::BOOL,
    pub UMA: super::super::Foundation::BOOL,
    pub CacheCoherentUMA: super::super::Foundation::BOOL,
    pub IsolatedMMU: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_ARCHITECTURE1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_ARCHITECTURE1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_ARCHITECTURE1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
    pub CommandListType: D3D12_COMMAND_LIST_TYPE,
    pub Priority: u32,
    pub PriorityForTypeIsSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_CROSS_NODE {
    pub SharingTier: D3D12_CROSS_NODE_SHARING_TIER,
    pub AtomicShaderInstructions: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_CROSS_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_CROSS_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_CROSS_NODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS {
    pub DoublePrecisionFloatShaderOps: super::super::Foundation::BOOL,
    pub OutputMergerLogicOp: super::super::Foundation::BOOL,
    pub MinPrecisionSupport: D3D12_SHADER_MIN_PRECISION_SUPPORT,
    pub TiledResourcesTier: D3D12_TILED_RESOURCES_TIER,
    pub ResourceBindingTier: D3D12_RESOURCE_BINDING_TIER,
    pub PSSpecifiedStencilRefSupported: super::super::Foundation::BOOL,
    pub TypedUAVLoadAdditionalFormats: super::super::Foundation::BOOL,
    pub ROVsSupported: super::super::Foundation::BOOL,
    pub ConservativeRasterizationTier: D3D12_CONSERVATIVE_RASTERIZATION_TIER,
    pub MaxGPUVirtualAddressBitsPerResource: u32,
    pub StandardSwizzle64KBSupported: super::super::Foundation::BOOL,
    pub CrossNodeSharingTier: D3D12_CROSS_NODE_SHARING_TIER,
    pub CrossAdapterRowMajorTextureSupported: super::super::Foundation::BOOL,
    pub VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation: super::super::Foundation::BOOL,
    pub ResourceHeapTier: D3D12_RESOURCE_HEAP_TIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
    pub WaveOps: super::super::Foundation::BOOL,
    pub WaveLaneCountMin: u32,
    pub WaveLaneCountMax: u32,
    pub TotalLaneCount: u32,
    pub ExpandedComputeResourceStates: super::super::Foundation::BOOL,
    pub Int64ShaderOps: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
    pub VariableRateShadingSumCombinerSupported: super::super::Foundation::BOOL,
    pub MeshShaderPerPrimitiveShadingRateSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
    pub AtomicInt64OnDescriptorHeapResourceSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS11 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
    pub DepthBoundsTestSupported: super::super::Foundation::BOOL,
    pub ProgrammableSamplePositionsTier: D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
    pub CopyQueueTimestampQueriesSupported: super::super::Foundation::BOOL,
    pub CastingFullyTypedFormatSupported: super::super::Foundation::BOOL,
    pub WriteBufferImmediateSupportFlags: D3D12_COMMAND_LIST_SUPPORT_FLAGS,
    pub ViewInstancingTier: D3D12_VIEW_INSTANCING_TIER,
    pub BarycentricsSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
    pub MSAA64KBAlignedTextureSupported: super::super::Foundation::BOOL,
    pub SharedResourceCompatibilityTier: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER,
    pub Native16BitShaderOpsSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
    pub SRVOnlyTiledResourceTier3: super::super::Foundation::BOOL,
    pub RenderPassesTier: D3D12_RENDER_PASS_TIER,
    pub RaytracingTier: D3D12_RAYTRACING_TIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
    pub AdditionalShadingRatesSupported: super::super::Foundation::BOOL,
    pub PerPrimitiveShadingRateSupportedWithViewportIndexing: super::super::Foundation::BOOL,
    pub VariableShadingRateTier: D3D12_VARIABLE_SHADING_RATE_TIER,
    pub ShadingRateImageTileSize: u32,
    pub BackgroundProcessingSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
    pub MeshShaderTier: D3D12_MESH_SHADER_TIER,
    pub SamplerFeedbackTier: D3D12_SAMPLER_FEEDBACK_TIER,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS7 {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
    pub UnalignedBlockTexturesSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
    pub MeshShaderPipelineStatsSupported: super::super::Foundation::BOOL,
    pub MeshShaderSupportsFullRangeRenderTargetArrayIndex: super::super::Foundation::BOOL,
    pub AtomicInt64OnTypedResourceSupported: super::super::Foundation::BOOL,
    pub AtomicInt64OnGroupSharedSupported: super::super::Foundation::BOOL,
    pub DerivativesInMeshAndAmplificationShadersSupported: super::super::Foundation::BOOL,
    pub WaveMMATier: D3D12_WAVE_MMA_TIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS9 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_DISPLAYABLE {
    pub DisplayableTexture: super::super::Foundation::BOOL,
    pub SharedResourceCompatibilityTier: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_DISPLAYABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_DISPLAYABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_DISPLAYABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_EXISTING_HEAPS {
    pub Supported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_EXISTING_HEAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_EXISTING_HEAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_EXISTING_HEAPS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D12_FEATURE_DATA_FEATURE_LEVELS {
    pub NumFeatureLevels: u32,
    pub pFeatureLevelsRequested: *const super::Direct3D::D3D_FEATURE_LEVEL,
    pub MaxSupportedFeatureLevel: super::Direct3D::D3D_FEATURE_LEVEL,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_FEATURE_LEVELS {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_FEATURE_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_FEATURE_LEVELS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_FORMAT_INFO {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub PlaneCount: u8,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_FORMAT_INFO {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_FORMAT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_FORMAT_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Support1: D3D12_FORMAT_SUPPORT1,
    pub Support2: D3D12_FORMAT_SUPPORT2,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_FORMAT_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    pub MaxGPUVirtualAddressBitsPerResource: u32,
    pub MaxGPUVirtualAddressBitsPerProcess: u32,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleCount: u32,
    pub Flags: D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS,
    pub NumQualityLevels: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
    pub NodeIndex: u32,
    pub Support: D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
    pub NodeIndex: u32,
    pub Count: u32,
    pub pTypes: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
    pub NodeIndex: u32,
    pub Count: u32,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_FEATURE_DATA_QUERY_META_COMMAND {
    pub CommandId: ::windows::core::GUID,
    pub NodeMask: u32,
    pub pQueryInputData: *const ::core::ffi::c_void,
    pub QueryInputDataSizeInBytes: usize,
    pub pQueryOutputData: *mut ::core::ffi::c_void,
    pub QueryOutputDataSizeInBytes: usize,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_QUERY_META_COMMAND {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_QUERY_META_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_QUERY_META_COMMAND {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_FEATURE_DATA_ROOT_SIGNATURE {
    pub HighestVersion: D3D_ROOT_SIGNATURE_VERSION,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_ROOT_SIGNATURE {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_ROOT_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_ROOT_SIGNATURE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_FEATURE_DATA_SERIALIZATION {
    pub NodeIndex: u32,
    pub HeapSerializationTier: D3D12_HEAP_SERIALIZATION_TIER,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_SERIALIZATION {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_SERIALIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_SERIALIZATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_FEATURE_DATA_SHADER_CACHE {
    pub SupportFlags: D3D12_SHADER_CACHE_SUPPORT_FLAGS,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_SHADER_CACHE {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_SHADER_CACHE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_SHADER_CACHE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_FEATURE_DATA_SHADER_MODEL {
    pub HighestShaderModel: D3D_SHADER_MODEL,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_SHADER_MODEL {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_SHADER_MODEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_FEATURE_DATA_SHADER_MODEL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D12_FUNCTION_DESC {
    pub Version: u32,
    pub Creator: ::windows::core::PCSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
    pub InstructionCount: u32,
    pub TempRegisterCount: u32,
    pub TempArrayCount: u32,
    pub DefCount: u32,
    pub DclCount: u32,
    pub TextureNormalInstructions: u32,
    pub TextureLoadInstructions: u32,
    pub TextureCompInstructions: u32,
    pub TextureBiasInstructions: u32,
    pub TextureGradientInstructions: u32,
    pub FloatInstructionCount: u32,
    pub IntInstructionCount: u32,
    pub UintInstructionCount: u32,
    pub StaticFlowControlCount: u32,
    pub DynamicFlowControlCount: u32,
    pub MacroInstructionCount: u32,
    pub ArrayInstructionCount: u32,
    pub MovInstructionCount: u32,
    pub MovcInstructionCount: u32,
    pub ConversionInstructionCount: u32,
    pub BitwiseInstructionCount: u32,
    pub MinFeatureLevel: super::Direct3D::D3D_FEATURE_LEVEL,
    pub RequiredFeatureFlags: u64,
    pub Name: ::windows::core::PCSTR,
    pub FunctionParameterCount: i32,
    pub HasReturn: super::super::Foundation::BOOL,
    pub Has10Level9VertexShader: super::super::Foundation::BOOL,
    pub Has10Level9PixelShader: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D12_FUNCTION_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D12_FUNCTION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
unsafe impl ::windows::core::Abi for D3D12_FUNCTION_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_GLOBAL_ROOT_SIGNATURE {
    pub pGlobalRootSignature: ::windows::core::ManuallyDrop<ID3D12RootSignature>,
}
impl ::core::clone::Clone for D3D12_GLOBAL_ROOT_SIGNATURE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_GLOBAL_ROOT_SIGNATURE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_GPU_DESCRIPTOR_HANDLE {
    pub ptr: u64,
}
impl ::core::marker::Copy for D3D12_GPU_DESCRIPTOR_HANDLE {}
impl ::core::clone::Clone for D3D12_GPU_DESCRIPTOR_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_GPU_DESCRIPTOR_HANDLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    pub StartAddress: u64,
    pub StrideInBytes: u64,
}
impl ::core::marker::Copy for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {}
impl ::core::clone::Clone for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    pub StartAddress: u64,
    pub SizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {}
impl ::core::clone::Clone for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {
    pub StartAddress: u64,
    pub SizeInBytes: u64,
    pub StrideInBytes: u64,
}
impl ::core::marker::Copy for D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {}
impl ::core::clone::Clone for D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    pub pRootSignature: ::windows::core::ManuallyDrop<ID3D12RootSignature>,
    pub VS: D3D12_SHADER_BYTECODE,
    pub PS: D3D12_SHADER_BYTECODE,
    pub DS: D3D12_SHADER_BYTECODE,
    pub HS: D3D12_SHADER_BYTECODE,
    pub GS: D3D12_SHADER_BYTECODE,
    pub StreamOutput: D3D12_STREAM_OUTPUT_DESC,
    pub BlendState: D3D12_BLEND_DESC,
    pub SampleMask: u32,
    pub RasterizerState: D3D12_RASTERIZER_DESC,
    pub DepthStencilState: D3D12_DEPTH_STENCIL_DESC,
    pub InputLayout: D3D12_INPUT_LAYOUT_DESC,
    pub IBStripCutValue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE,
    pub PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE,
    pub NumRenderTargets: u32,
    pub RTVFormats: [super::Dxgi::Common::DXGI_FORMAT; 8],
    pub DSVFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub NodeMask: u32,
    pub CachedPSO: D3D12_CACHED_PIPELINE_STATE,
    pub Flags: D3D12_PIPELINE_STATE_FLAGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_HEAP_DESC {
    pub SizeInBytes: u64,
    pub Properties: D3D12_HEAP_PROPERTIES,
    pub Alignment: u64,
    pub Flags: D3D12_HEAP_FLAGS,
}
impl ::core::marker::Copy for D3D12_HEAP_DESC {}
impl ::core::clone::Clone for D3D12_HEAP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_HEAP_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_HEAP_PROPERTIES {
    pub Type: D3D12_HEAP_TYPE,
    pub CPUPageProperty: D3D12_CPU_PAGE_PROPERTY,
    pub MemoryPoolPreference: D3D12_MEMORY_POOL,
    pub CreationNodeMask: u32,
    pub VisibleNodeMask: u32,
}
impl ::core::marker::Copy for D3D12_HEAP_PROPERTIES {}
impl ::core::clone::Clone for D3D12_HEAP_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_HEAP_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_HIT_GROUP_DESC {
    pub HitGroupExport: ::windows::core::PCWSTR,
    pub Type: D3D12_HIT_GROUP_TYPE,
    pub AnyHitShaderImport: ::windows::core::PCWSTR,
    pub ClosestHitShaderImport: ::windows::core::PCWSTR,
    pub IntersectionShaderImport: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for D3D12_HIT_GROUP_DESC {}
impl ::core::clone::Clone for D3D12_HIT_GROUP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_HIT_GROUP_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_INDEX_BUFFER_VIEW {
    pub BufferLocation: u64,
    pub SizeInBytes: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_INDEX_BUFFER_VIEW {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_INDEX_BUFFER_VIEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_INDEX_BUFFER_VIEW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_INDIRECT_ARGUMENT_DESC {
    pub Type: D3D12_INDIRECT_ARGUMENT_TYPE,
    pub Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INDIRECT_ARGUMENT_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub union D3D12_INDIRECT_ARGUMENT_DESC_0 {
    pub VertexBuffer: D3D12_INDIRECT_ARGUMENT_DESC_0_4,
    pub Constant: D3D12_INDIRECT_ARGUMENT_DESC_0_1,
    pub ConstantBufferView: D3D12_INDIRECT_ARGUMENT_DESC_0_0,
    pub ShaderResourceView: D3D12_INDIRECT_ARGUMENT_DESC_0_2,
    pub UnorderedAccessView: D3D12_INDIRECT_ARGUMENT_DESC_0_3,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INDIRECT_ARGUMENT_DESC_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_0 {
    pub RootParameterIndex: u32,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0_0 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INDIRECT_ARGUMENT_DESC_0_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
    pub RootParameterIndex: u32,
    pub DestOffsetIn32BitValues: u32,
    pub Num32BitValuesToSet: u32,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0_1 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_2 {
    pub RootParameterIndex: u32,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0_2 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INDIRECT_ARGUMENT_DESC_0_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_3 {
    pub RootParameterIndex: u32,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0_3 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INDIRECT_ARGUMENT_DESC_0_3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_4 {
    pub Slot: u32,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0_4 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INDIRECT_ARGUMENT_DESC_0_4 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_INFO_QUEUE_FILTER {
    pub AllowList: D3D12_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D12_INFO_QUEUE_FILTER_DESC,
}
impl ::core::marker::Copy for D3D12_INFO_QUEUE_FILTER {}
impl ::core::clone::Clone for D3D12_INFO_QUEUE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INFO_QUEUE_FILTER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D12_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D12_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D12_MESSAGE_ID,
}
impl ::core::marker::Copy for D3D12_INFO_QUEUE_FILTER_DESC {}
impl ::core::clone::Clone for D3D12_INFO_QUEUE_FILTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_INFO_QUEUE_FILTER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_INPUT_ELEMENT_DESC {
    pub SemanticName: ::windows::core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D12_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_INPUT_ELEMENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_INPUT_ELEMENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_INPUT_ELEMENT_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_INPUT_LAYOUT_DESC {
    pub pInputElementDescs: *const D3D12_INPUT_ELEMENT_DESC,
    pub NumElements: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_INPUT_LAYOUT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_INPUT_LAYOUT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_INPUT_LAYOUT_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_LIBRARY_DESC {
    pub Creator: ::windows::core::PCSTR,
    pub Flags: u32,
    pub FunctionCount: u32,
}
impl ::core::marker::Copy for D3D12_LIBRARY_DESC {}
impl ::core::clone::Clone for D3D12_LIBRARY_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_LIBRARY_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_LOCAL_ROOT_SIGNATURE {
    pub pLocalRootSignature: ::windows::core::ManuallyDrop<ID3D12RootSignature>,
}
impl ::core::clone::Clone for D3D12_LOCAL_ROOT_SIGNATURE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_LOCAL_ROOT_SIGNATURE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_MEMCPY_DEST {
    pub pData: *mut ::core::ffi::c_void,
    pub RowPitch: usize,
    pub SlicePitch: usize,
}
impl ::core::marker::Copy for D3D12_MEMCPY_DEST {}
impl ::core::clone::Clone for D3D12_MEMCPY_DEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MEMCPY_DEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_MESSAGE {
    pub Category: D3D12_MESSAGE_CATEGORY,
    pub Severity: D3D12_MESSAGE_SEVERITY,
    pub ID: D3D12_MESSAGE_ID,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}
impl ::core::marker::Copy for D3D12_MESSAGE {}
impl ::core::clone::Clone for D3D12_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MESSAGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_META_COMMAND_DESC {
    pub Id: ::windows::core::GUID,
    pub Name: ::windows::core::PCWSTR,
    pub InitializationDirtyState: D3D12_GRAPHICS_STATES,
    pub ExecutionDirtyState: D3D12_GRAPHICS_STATES,
}
impl ::core::marker::Copy for D3D12_META_COMMAND_DESC {}
impl ::core::clone::Clone for D3D12_META_COMMAND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_META_COMMAND_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_META_COMMAND_PARAMETER_DESC {
    pub Name: ::windows::core::PCWSTR,
    pub Type: D3D12_META_COMMAND_PARAMETER_TYPE,
    pub Flags: D3D12_META_COMMAND_PARAMETER_FLAGS,
    pub RequiredResourceState: D3D12_RESOURCE_STATES,
    pub StructureOffset: u32,
}
impl ::core::marker::Copy for D3D12_META_COMMAND_PARAMETER_DESC {}
impl ::core::clone::Clone for D3D12_META_COMMAND_PARAMETER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_META_COMMAND_PARAMETER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_MIP_REGION {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
}
impl ::core::marker::Copy for D3D12_MIP_REGION {}
impl ::core::clone::Clone for D3D12_MIP_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_MIP_REGION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_NODE_MASK {
    pub NodeMask: u32,
}
impl ::core::marker::Copy for D3D12_NODE_MASK {}
impl ::core::clone::Clone for D3D12_NODE_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_NODE_MASK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_PACKED_MIP_INFO {
    pub NumStandardMips: u8,
    pub NumPackedMips: u8,
    pub NumTilesForPackedMips: u32,
    pub StartTileIndexInOverallResource: u32,
}
impl ::core::marker::Copy for D3D12_PACKED_MIP_INFO {}
impl ::core::clone::Clone for D3D12_PACKED_MIP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PACKED_MIP_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D12_PARAMETER_DESC {
    pub Name: ::windows::core::PCSTR,
    pub SemanticName: ::windows::core::PCSTR,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Rows: u32,
    pub Columns: u32,
    pub InterpolationMode: super::Direct3D::D3D_INTERPOLATION_MODE,
    pub Flags: super::Direct3D::D3D_PARAMETER_FLAGS,
    pub FirstInRegister: u32,
    pub FirstInComponent: u32,
    pub FirstOutRegister: u32,
    pub FirstOutComponent: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D12_PARAMETER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D12_PARAMETER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D12_PARAMETER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_PIPELINE_STATE_STREAM_DESC {
    pub SizeInBytes: usize,
    pub pPipelineStateSubobjectStream: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for D3D12_PIPELINE_STATE_STREAM_DESC {}
impl ::core::clone::Clone for D3D12_PIPELINE_STATE_STREAM_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PIPELINE_STATE_STREAM_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    pub Offset: u64,
    pub Footprint: D3D12_SUBRESOURCE_FOOTPRINT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_PROTECTED_RESOURCE_SESSION_DESC {
    pub NodeMask: u32,
    pub Flags: D3D12_PROTECTED_RESOURCE_SESSION_FLAGS,
}
impl ::core::marker::Copy for D3D12_PROTECTED_RESOURCE_SESSION_DESC {}
impl ::core::clone::Clone for D3D12_PROTECTED_RESOURCE_SESSION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PROTECTED_RESOURCE_SESSION_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
    pub NodeMask: u32,
    pub Flags: D3D12_PROTECTED_RESOURCE_SESSION_FLAGS,
    pub ProtectionType: ::windows::core::GUID,
}
impl ::core::marker::Copy for D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {}
impl ::core::clone::Clone for D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_QUERY_DATA_PIPELINE_STATISTICS {
    pub IAVertices: u64,
    pub IAPrimitives: u64,
    pub VSInvocations: u64,
    pub GSInvocations: u64,
    pub GSPrimitives: u64,
    pub CInvocations: u64,
    pub CPrimitives: u64,
    pub PSInvocations: u64,
    pub HSInvocations: u64,
    pub DSInvocations: u64,
    pub CSInvocations: u64,
}
impl ::core::marker::Copy for D3D12_QUERY_DATA_PIPELINE_STATISTICS {}
impl ::core::clone::Clone for D3D12_QUERY_DATA_PIPELINE_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_QUERY_DATA_PIPELINE_STATISTICS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {
    pub IAVertices: u64,
    pub IAPrimitives: u64,
    pub VSInvocations: u64,
    pub GSInvocations: u64,
    pub GSPrimitives: u64,
    pub CInvocations: u64,
    pub CPrimitives: u64,
    pub PSInvocations: u64,
    pub HSInvocations: u64,
    pub DSInvocations: u64,
    pub CSInvocations: u64,
    pub ASInvocations: u64,
    pub MSInvocations: u64,
    pub MSPrimitives: u64,
}
impl ::core::marker::Copy for D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {}
impl ::core::clone::Clone for D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
impl ::core::marker::Copy for D3D12_QUERY_DATA_SO_STATISTICS {}
impl ::core::clone::Clone for D3D12_QUERY_DATA_SO_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_QUERY_DATA_SO_STATISTICS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_QUERY_HEAP_DESC {
    pub Type: D3D12_QUERY_HEAP_TYPE,
    pub Count: u32,
    pub NodeMask: u32,
}
impl ::core::marker::Copy for D3D12_QUERY_HEAP_DESC {}
impl ::core::clone::Clone for D3D12_QUERY_HEAP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_QUERY_HEAP_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RANGE {
    pub Begin: usize,
    pub End: usize,
}
impl ::core::marker::Copy for D3D12_RANGE {}
impl ::core::clone::Clone for D3D12_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RANGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RANGE_UINT64 {
    pub Begin: u64,
    pub End: u64,
}
impl ::core::marker::Copy for D3D12_RANGE_UINT64 {}
impl ::core::clone::Clone for D3D12_RANGE_UINT64 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RANGE_UINT64 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_RASTERIZER_DESC {
    pub FillMode: D3D12_FILL_MODE,
    pub CullMode: D3D12_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_RASTERIZER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_RASTERIZER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_RASTERIZER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_AABB {
    pub MinX: f32,
    pub MinY: f32,
    pub MinZ: f32,
    pub MaxX: f32,
    pub MaxY: f32,
    pub MaxZ: f32,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_AABB {}
impl ::core::clone::Clone for D3D12_RAYTRACING_AABB {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_AABB {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {
    pub CompactedSizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {
    pub CurrentSizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {
    pub DestBuffer: u64,
    pub InfoType: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {
    pub SerializedSizeInBytes: u64,
    pub NumBottomLevelAccelerationStructurePointers: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {
    pub DecodedSizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {
    pub ResultDataMaxSizeInBytes: u64,
    pub ScratchDataSizeInBytes: u64,
    pub UpdateScratchDataSizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
    pub Location: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
    pub AABBCount: u64,
    pub AABBs: D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RAYTRACING_GEOMETRY_DESC {
    pub Type: D3D12_RAYTRACING_GEOMETRY_TYPE,
    pub Flags: D3D12_RAYTRACING_GEOMETRY_FLAGS,
    pub Anonymous: D3D12_RAYTRACING_GEOMETRY_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RAYTRACING_GEOMETRY_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RAYTRACING_GEOMETRY_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_GEOMETRY_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_RAYTRACING_GEOMETRY_DESC_0 {
    pub Triangles: D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC,
    pub AABBs: D3D12_RAYTRACING_GEOMETRY_AABBS_DESC,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RAYTRACING_GEOMETRY_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RAYTRACING_GEOMETRY_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_GEOMETRY_DESC_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
    pub Transform3x4: u64,
    pub IndexFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub VertexFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub IndexCount: u32,
    pub VertexCount: u32,
    pub IndexBuffer: u64,
    pub VertexBuffer: D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_INSTANCE_DESC {
    pub Transform: [f32; 12],
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub AccelerationStructure: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_INSTANCE_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_INSTANCE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_INSTANCE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_PIPELINE_CONFIG {
    pub MaxTraceRecursionDepth: u32,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_PIPELINE_CONFIG {}
impl ::core::clone::Clone for D3D12_RAYTRACING_PIPELINE_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_PIPELINE_CONFIG {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_PIPELINE_CONFIG1 {
    pub MaxTraceRecursionDepth: u32,
    pub Flags: D3D12_RAYTRACING_PIPELINE_FLAGS,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_PIPELINE_CONFIG1 {}
impl ::core::clone::Clone for D3D12_RAYTRACING_PIPELINE_CONFIG1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_PIPELINE_CONFIG1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RAYTRACING_SHADER_CONFIG {
    pub MaxPayloadSizeInBytes: u32,
    pub MaxAttributeSizeInBytes: u32,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_SHADER_CONFIG {}
impl ::core::clone::Clone for D3D12_RAYTRACING_SHADER_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RAYTRACING_SHADER_CONFIG {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS {
    pub Type: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE,
    pub Anonymous: D3D12_RENDER_PASS_BEGINNING_ACCESS_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RENDER_PASS_BEGINNING_ACCESS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RENDER_PASS_BEGINNING_ACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_BEGINNING_ACCESS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_RENDER_PASS_BEGINNING_ACCESS_0 {
    pub Clear: D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RENDER_PASS_BEGINNING_ACCESS_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RENDER_PASS_BEGINNING_ACCESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_BEGINNING_ACCESS_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {
    pub ClearValue: D3D12_CLEAR_VALUE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
    pub cpuDescriptor: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub DepthBeginningAccess: D3D12_RENDER_PASS_BEGINNING_ACCESS,
    pub StencilBeginningAccess: D3D12_RENDER_PASS_BEGINNING_ACCESS,
    pub DepthEndingAccess: D3D12_RENDER_PASS_ENDING_ACCESS,
    pub StencilEndingAccess: D3D12_RENDER_PASS_ENDING_ACCESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS {
    pub Type: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE,
    pub Anonymous: D3D12_RENDER_PASS_ENDING_ACCESS_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_RENDER_PASS_ENDING_ACCESS {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_ENDING_ACCESS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub union D3D12_RENDER_PASS_ENDING_ACCESS_0 {
    pub Resolve: ::std::mem::ManuallyDrop<D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_RENDER_PASS_ENDING_ACCESS_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_ENDING_ACCESS_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    pub pSrcResource: ::windows::core::ManuallyDrop<ID3D12Resource>,
    pub pDstResource: ::windows::core::ManuallyDrop<ID3D12Resource>,
    pub SubresourceCount: u32,
    pub pSubresourceParameters: *const D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ResolveMode: D3D12_RESOLVE_MODE,
    pub PreserveResolveSource: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {
    pub SrcSubresource: u32,
    pub DstSubresource: u32,
    pub DstX: u32,
    pub DstY: u32,
    pub SrcRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_RENDER_PASS_RENDER_TARGET_DESC {
    pub cpuDescriptor: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub BeginningAccess: D3D12_RENDER_PASS_BEGINNING_ACCESS,
    pub EndingAccess: D3D12_RENDER_PASS_ENDING_ACCESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_RENDER_PASS_RENDER_TARGET_DESC {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D3D12_RENDER_PASS_RENDER_TARGET_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: super::super::Foundation::BOOL,
    pub LogicOpEnable: super::super::Foundation::BOOL,
    pub SrcBlend: D3D12_BLEND,
    pub DestBlend: D3D12_BLEND,
    pub BlendOp: D3D12_BLEND_OP,
    pub SrcBlendAlpha: D3D12_BLEND,
    pub DestBlendAlpha: D3D12_BLEND,
    pub BlendOpAlpha: D3D12_BLEND_OP,
    pub LogicOp: D3D12_LOGIC_OP,
    pub RenderTargetWriteMask: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_RENDER_TARGET_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_RENDER_TARGET_BLEND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_RENDER_TARGET_BLEND_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RENDER_TARGET_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_RTV_DIMENSION,
    pub Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RENDER_TARGET_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RENDER_TARGET_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_RENDER_TARGET_VIEW_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_RENDER_TARGET_VIEW_DESC_0 {
    pub Buffer: D3D12_BUFFER_RTV,
    pub Texture1D: D3D12_TEX1D_RTV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D12_TEX2D_RTV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_RTV,
    pub Texture2DMS: D3D12_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D12_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D12_TEX3D_RTV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RENDER_TARGET_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RENDER_TARGET_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_RENDER_TARGET_VIEW_DESC_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RESOURCE_ALIASING_BARRIER {
    pub pResourceBefore: ::windows::core::ManuallyDrop<ID3D12Resource>,
    pub pResourceAfter: ::windows::core::ManuallyDrop<ID3D12Resource>,
}
impl ::core::clone::Clone for D3D12_RESOURCE_ALIASING_BARRIER {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_ALIASING_BARRIER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RESOURCE_ALLOCATION_INFO {
    pub SizeInBytes: u64,
    pub Alignment: u64,
}
impl ::core::marker::Copy for D3D12_RESOURCE_ALLOCATION_INFO {}
impl ::core::clone::Clone for D3D12_RESOURCE_ALLOCATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_ALLOCATION_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RESOURCE_ALLOCATION_INFO1 {
    pub Offset: u64,
    pub Alignment: u64,
    pub SizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_RESOURCE_ALLOCATION_INFO1 {}
impl ::core::clone::Clone for D3D12_RESOURCE_ALLOCATION_INFO1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_ALLOCATION_INFO1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RESOURCE_BARRIER {
    pub Type: D3D12_RESOURCE_BARRIER_TYPE,
    pub Flags: D3D12_RESOURCE_BARRIER_FLAGS,
    pub Anonymous: D3D12_RESOURCE_BARRIER_0,
}
impl ::core::clone::Clone for D3D12_RESOURCE_BARRIER {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_BARRIER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub union D3D12_RESOURCE_BARRIER_0 {
    pub Transition: ::std::mem::ManuallyDrop<D3D12_RESOURCE_TRANSITION_BARRIER>,
    pub Aliasing: ::std::mem::ManuallyDrop<D3D12_RESOURCE_ALIASING_BARRIER>,
    pub UAV: ::std::mem::ManuallyDrop<D3D12_RESOURCE_UAV_BARRIER>,
}
impl ::core::clone::Clone for D3D12_RESOURCE_BARRIER_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_BARRIER_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RESOURCE_DESC {
    pub Dimension: D3D12_RESOURCE_DIMENSION,
    pub Alignment: u64,
    pub Width: u64,
    pub Height: u32,
    pub DepthOrArraySize: u16,
    pub MipLevels: u16,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Layout: D3D12_TEXTURE_LAYOUT,
    pub Flags: D3D12_RESOURCE_FLAGS,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RESOURCE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RESOURCE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RESOURCE_DESC1 {
    pub Dimension: D3D12_RESOURCE_DIMENSION,
    pub Alignment: u64,
    pub Width: u64,
    pub Height: u32,
    pub DepthOrArraySize: u16,
    pub MipLevels: u16,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Layout: D3D12_TEXTURE_LAYOUT,
    pub Flags: D3D12_RESOURCE_FLAGS,
    pub SamplerFeedbackMipRegion: D3D12_MIP_REGION,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RESOURCE_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RESOURCE_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_DESC1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RESOURCE_TRANSITION_BARRIER {
    pub pResource: ::windows::core::ManuallyDrop<ID3D12Resource>,
    pub Subresource: u32,
    pub StateBefore: D3D12_RESOURCE_STATES,
    pub StateAfter: D3D12_RESOURCE_STATES,
}
impl ::core::clone::Clone for D3D12_RESOURCE_TRANSITION_BARRIER {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_TRANSITION_BARRIER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_RESOURCE_UAV_BARRIER {
    pub pResource: ::windows::core::ManuallyDrop<ID3D12Resource>,
}
impl ::core::clone::Clone for D3D12_RESOURCE_UAV_BARRIER {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for D3D12_RESOURCE_UAV_BARRIER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_ROOT_CONSTANTS {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Num32BitValues: u32,
}
impl ::core::marker::Copy for D3D12_ROOT_CONSTANTS {}
impl ::core::clone::Clone for D3D12_ROOT_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_CONSTANTS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_ROOT_DESCRIPTOR {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
}
impl ::core::marker::Copy for D3D12_ROOT_DESCRIPTOR {}
impl ::core::clone::Clone for D3D12_ROOT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_DESCRIPTOR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_ROOT_DESCRIPTOR1 {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Flags: D3D12_ROOT_DESCRIPTOR_FLAGS,
}
impl ::core::marker::Copy for D3D12_ROOT_DESCRIPTOR1 {}
impl ::core::clone::Clone for D3D12_ROOT_DESCRIPTOR1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_DESCRIPTOR1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_ROOT_DESCRIPTOR_TABLE {
    pub NumDescriptorRanges: u32,
    pub pDescriptorRanges: *const D3D12_DESCRIPTOR_RANGE,
}
impl ::core::marker::Copy for D3D12_ROOT_DESCRIPTOR_TABLE {}
impl ::core::clone::Clone for D3D12_ROOT_DESCRIPTOR_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_DESCRIPTOR_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_ROOT_DESCRIPTOR_TABLE1 {
    pub NumDescriptorRanges: u32,
    pub pDescriptorRanges: *const D3D12_DESCRIPTOR_RANGE1,
}
impl ::core::marker::Copy for D3D12_ROOT_DESCRIPTOR_TABLE1 {}
impl ::core::clone::Clone for D3D12_ROOT_DESCRIPTOR_TABLE1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_DESCRIPTOR_TABLE1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_ROOT_PARAMETER {
    pub ParameterType: D3D12_ROOT_PARAMETER_TYPE,
    pub Anonymous: D3D12_ROOT_PARAMETER_0,
    pub ShaderVisibility: D3D12_SHADER_VISIBILITY,
}
impl ::core::marker::Copy for D3D12_ROOT_PARAMETER {}
impl ::core::clone::Clone for D3D12_ROOT_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_PARAMETER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub union D3D12_ROOT_PARAMETER_0 {
    pub DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE,
    pub Constants: D3D12_ROOT_CONSTANTS,
    pub Descriptor: D3D12_ROOT_DESCRIPTOR,
}
impl ::core::marker::Copy for D3D12_ROOT_PARAMETER_0 {}
impl ::core::clone::Clone for D3D12_ROOT_PARAMETER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_PARAMETER_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_ROOT_PARAMETER1 {
    pub ParameterType: D3D12_ROOT_PARAMETER_TYPE,
    pub Anonymous: D3D12_ROOT_PARAMETER1_0,
    pub ShaderVisibility: D3D12_SHADER_VISIBILITY,
}
impl ::core::marker::Copy for D3D12_ROOT_PARAMETER1 {}
impl ::core::clone::Clone for D3D12_ROOT_PARAMETER1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_PARAMETER1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub union D3D12_ROOT_PARAMETER1_0 {
    pub DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE1,
    pub Constants: D3D12_ROOT_CONSTANTS,
    pub Descriptor: D3D12_ROOT_DESCRIPTOR1,
}
impl ::core::marker::Copy for D3D12_ROOT_PARAMETER1_0 {}
impl ::core::clone::Clone for D3D12_ROOT_PARAMETER1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_PARAMETER1_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_ROOT_SIGNATURE_DESC {
    pub NumParameters: u32,
    pub pParameters: *const D3D12_ROOT_PARAMETER,
    pub NumStaticSamplers: u32,
    pub pStaticSamplers: *const D3D12_STATIC_SAMPLER_DESC,
    pub Flags: D3D12_ROOT_SIGNATURE_FLAGS,
}
impl ::core::marker::Copy for D3D12_ROOT_SIGNATURE_DESC {}
impl ::core::clone::Clone for D3D12_ROOT_SIGNATURE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_SIGNATURE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_ROOT_SIGNATURE_DESC1 {
    pub NumParameters: u32,
    pub pParameters: *const D3D12_ROOT_PARAMETER1,
    pub NumStaticSamplers: u32,
    pub pStaticSamplers: *const D3D12_STATIC_SAMPLER_DESC,
    pub Flags: D3D12_ROOT_SIGNATURE_FLAGS,
}
impl ::core::marker::Copy for D3D12_ROOT_SIGNATURE_DESC1 {}
impl ::core::clone::Clone for D3D12_ROOT_SIGNATURE_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_ROOT_SIGNATURE_DESC1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RT_FORMAT_ARRAY {
    pub RTFormats: [super::Dxgi::Common::DXGI_FORMAT; 8],
    pub NumRenderTargets: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RT_FORMAT_ARRAY {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RT_FORMAT_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_RT_FORMAT_ARRAY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SAMPLER_DESC {
    pub Filter: D3D12_FILTER,
    pub AddressU: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D12_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D12_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
impl ::core::marker::Copy for D3D12_SAMPLER_DESC {}
impl ::core::clone::Clone for D3D12_SAMPLER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SAMPLER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SAMPLE_POSITION {
    pub X: i8,
    pub Y: i8,
}
impl ::core::marker::Copy for D3D12_SAMPLE_POSITION {}
impl ::core::clone::Clone for D3D12_SAMPLE_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SAMPLE_POSITION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {
    pub DriverOpaqueGUID: ::windows::core::GUID,
    pub DriverOpaqueVersioningData: [u8; 16],
}
impl ::core::marker::Copy for D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {}
impl ::core::clone::Clone for D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {
    pub DriverMatchingIdentifier: D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER,
    pub SerializedSizeInBytesIncludingHeader: u64,
    pub DeserializedSizeInBytes: u64,
    pub NumBottomLevelAccelerationStructurePointersAfterHeader: u64,
}
impl ::core::marker::Copy for D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {}
impl ::core::clone::Clone for D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D12_SHADER_BUFFER_DESC {
    pub Name: ::windows::core::PCSTR,
    pub Type: super::Direct3D::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D12_SHADER_BUFFER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D12_SHADER_BUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D12_SHADER_BUFFER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SHADER_BYTECODE {
    pub pShaderBytecode: *const ::core::ffi::c_void,
    pub BytecodeLength: usize,
}
impl ::core::marker::Copy for D3D12_SHADER_BYTECODE {}
impl ::core::clone::Clone for D3D12_SHADER_BYTECODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_BYTECODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SHADER_CACHE_SESSION_DESC {
    pub Identifier: ::windows::core::GUID,
    pub Mode: D3D12_SHADER_CACHE_MODE,
    pub Flags: D3D12_SHADER_CACHE_FLAGS,
    pub MaximumInMemoryCacheSizeBytes: u32,
    pub MaximumInMemoryCacheEntries: u32,
    pub MaximumValueFileSizeBytes: u32,
    pub Version: u64,
}
impl ::core::marker::Copy for D3D12_SHADER_CACHE_SESSION_DESC {}
impl ::core::clone::Clone for D3D12_SHADER_CACHE_SESSION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_CACHE_SESSION_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D12_SHADER_DESC {
    pub Version: u32,
    pub Creator: ::windows::core::PCSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
    pub InputParameters: u32,
    pub OutputParameters: u32,
    pub InstructionCount: u32,
    pub TempRegisterCount: u32,
    pub TempArrayCount: u32,
    pub DefCount: u32,
    pub DclCount: u32,
    pub TextureNormalInstructions: u32,
    pub TextureLoadInstructions: u32,
    pub TextureCompInstructions: u32,
    pub TextureBiasInstructions: u32,
    pub TextureGradientInstructions: u32,
    pub FloatInstructionCount: u32,
    pub IntInstructionCount: u32,
    pub UintInstructionCount: u32,
    pub StaticFlowControlCount: u32,
    pub DynamicFlowControlCount: u32,
    pub MacroInstructionCount: u32,
    pub ArrayInstructionCount: u32,
    pub CutInstructionCount: u32,
    pub EmitInstructionCount: u32,
    pub GSOutputTopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY,
    pub GSMaxOutputVertexCount: u32,
    pub InputPrimitive: super::Direct3D::D3D_PRIMITIVE,
    pub PatchConstantParameters: u32,
    pub cGSInstanceCount: u32,
    pub cControlPoints: u32,
    pub HSOutputPrimitive: super::Direct3D::D3D_TESSELLATOR_OUTPUT_PRIMITIVE,
    pub HSPartitioning: super::Direct3D::D3D_TESSELLATOR_PARTITIONING,
    pub TessellatorDomain: super::Direct3D::D3D_TESSELLATOR_DOMAIN,
    pub cBarrierInstructions: u32,
    pub cInterlockedInstructions: u32,
    pub cTextureStoreInstructions: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D12_SHADER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D12_SHADER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D12_SHADER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D12_SHADER_INPUT_BIND_DESC {
    pub Name: ::windows::core::PCSTR,
    pub Type: super::Direct3D::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::Direct3D::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
    pub Space: u32,
    pub uID: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D12_SHADER_INPUT_BIND_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D12_SHADER_INPUT_BIND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D12_SHADER_INPUT_BIND_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_SRV_DIMENSION,
    pub Shader4ComponentMapping: u32,
    pub Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_SHADER_RESOURCE_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_SHADER_RESOURCE_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_SHADER_RESOURCE_VIEW_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
    pub Buffer: D3D12_BUFFER_SRV,
    pub Texture1D: D3D12_TEX1D_SRV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D12_TEX2D_SRV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D12_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D12_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D12_TEX3D_SRV,
    pub TextureCube: D3D12_TEXCUBE_SRV,
    pub TextureCubeArray: D3D12_TEXCUBE_ARRAY_SRV,
    pub RaytracingAccelerationStructure: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_SHADER_RESOURCE_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D12_SHADER_TYPE_DESC {
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
    pub Name: ::windows::core::PCSTR,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D12_SHADER_TYPE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D12_SHADER_TYPE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D12_SHADER_TYPE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SHADER_VARIABLE_DESC {
    pub Name: ::windows::core::PCSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut ::core::ffi::c_void,
    pub StartTexture: u32,
    pub TextureSize: u32,
    pub StartSampler: u32,
    pub SamplerSize: u32,
}
impl ::core::marker::Copy for D3D12_SHADER_VARIABLE_DESC {}
impl ::core::clone::Clone for D3D12_SHADER_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SHADER_VARIABLE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D12_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: ::windows::core::PCSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: super::Direct3D::D3D_NAME,
    pub ComponentType: super::Direct3D::D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
    pub Stream: u32,
    pub MinPrecision: super::Direct3D::D3D_MIN_PRECISION,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D12_SIGNATURE_PARAMETER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D12_SIGNATURE_PARAMETER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D12_SIGNATURE_PARAMETER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SO_DECLARATION_ENTRY {
    pub Stream: u32,
    pub SemanticName: ::windows::core::PCSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
impl ::core::marker::Copy for D3D12_SO_DECLARATION_ENTRY {}
impl ::core::clone::Clone for D3D12_SO_DECLARATION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SO_DECLARATION_ENTRY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_STATE_OBJECT_CONFIG {
    pub Flags: D3D12_STATE_OBJECT_FLAGS,
}
impl ::core::marker::Copy for D3D12_STATE_OBJECT_CONFIG {}
impl ::core::clone::Clone for D3D12_STATE_OBJECT_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_STATE_OBJECT_CONFIG {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_STATE_OBJECT_DESC {
    pub Type: D3D12_STATE_OBJECT_TYPE,
    pub NumSubobjects: u32,
    pub pSubobjects: *const D3D12_STATE_SUBOBJECT,
}
impl ::core::marker::Copy for D3D12_STATE_OBJECT_DESC {}
impl ::core::clone::Clone for D3D12_STATE_OBJECT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_STATE_OBJECT_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_STATE_SUBOBJECT {
    pub Type: D3D12_STATE_SUBOBJECT_TYPE,
    pub pDesc: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for D3D12_STATE_SUBOBJECT {}
impl ::core::clone::Clone for D3D12_STATE_SUBOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_STATE_SUBOBJECT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_STATIC_SAMPLER_DESC {
    pub Filter: D3D12_FILTER,
    pub AddressU: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D12_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D12_COMPARISON_FUNC,
    pub BorderColor: D3D12_STATIC_BORDER_COLOR,
    pub MinLOD: f32,
    pub MaxLOD: f32,
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub ShaderVisibility: D3D12_SHADER_VISIBILITY,
}
impl ::core::marker::Copy for D3D12_STATIC_SAMPLER_DESC {}
impl ::core::clone::Clone for D3D12_STATIC_SAMPLER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_STATIC_SAMPLER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    pub BufferLocation: u64,
    pub SizeInBytes: u64,
    pub BufferFilledSizeLocation: u64,
}
impl ::core::marker::Copy for D3D12_STREAM_OUTPUT_BUFFER_VIEW {}
impl ::core::clone::Clone for D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_STREAM_OUTPUT_DESC {
    pub pSODeclaration: *const D3D12_SO_DECLARATION_ENTRY,
    pub NumEntries: u32,
    pub pBufferStrides: *const u32,
    pub NumStrides: u32,
    pub RasterizedStream: u32,
}
impl ::core::marker::Copy for D3D12_STREAM_OUTPUT_DESC {}
impl ::core::clone::Clone for D3D12_STREAM_OUTPUT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_STREAM_OUTPUT_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    pub pSubobjectToAssociate: *const D3D12_STATE_SUBOBJECT,
    pub NumExports: u32,
    pub pExports: *mut ::windows::core::PWSTR,
}
impl ::core::marker::Copy for D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {}
impl ::core::clone::Clone for D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SUBRESOURCE_DATA {
    pub pData: *const ::core::ffi::c_void,
    pub RowPitch: isize,
    pub SlicePitch: isize,
}
impl ::core::marker::Copy for D3D12_SUBRESOURCE_DATA {}
impl ::core::clone::Clone for D3D12_SUBRESOURCE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SUBRESOURCE_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_SUBRESOURCE_FOOTPRINT {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub RowPitch: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_SUBRESOURCE_FOOTPRINT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_SUBRESOURCE_FOOTPRINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_SUBRESOURCE_FOOTPRINT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SUBRESOURCE_INFO {
    pub Offset: u64,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
impl ::core::marker::Copy for D3D12_SUBRESOURCE_INFO {}
impl ::core::clone::Clone for D3D12_SUBRESOURCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SUBRESOURCE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SUBRESOURCE_RANGE_UINT64 {
    pub Subresource: u32,
    pub Range: D3D12_RANGE_UINT64,
}
impl ::core::marker::Copy for D3D12_SUBRESOURCE_RANGE_UINT64 {}
impl ::core::clone::Clone for D3D12_SUBRESOURCE_RANGE_UINT64 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SUBRESOURCE_RANGE_UINT64 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_SUBRESOURCE_TILING {
    pub WidthInTiles: u32,
    pub HeightInTiles: u16,
    pub DepthInTiles: u16,
    pub StartTileIndexInOverallResource: u32,
}
impl ::core::marker::Copy for D3D12_SUBRESOURCE_TILING {}
impl ::core::clone::Clone for D3D12_SUBRESOURCE_TILING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_SUBRESOURCE_TILING {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_ARRAY_DSV {}
impl ::core::clone::Clone for D3D12_TEX1D_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX1D_ARRAY_DSV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_ARRAY_RTV {}
impl ::core::clone::Clone for D3D12_TEX1D_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX1D_ARRAY_RTV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEX1D_ARRAY_SRV {}
impl ::core::clone::Clone for D3D12_TEX1D_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX1D_ARRAY_SRV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX1D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_ARRAY_UAV {}
impl ::core::clone::Clone for D3D12_TEX1D_ARRAY_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX1D_ARRAY_UAV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX1D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_DSV {}
impl ::core::clone::Clone for D3D12_TEX1D_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX1D_DSV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX1D_RTV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_RTV {}
impl ::core::clone::Clone for D3D12_TEX1D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX1D_RTV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEX1D_SRV {}
impl ::core::clone::Clone for D3D12_TEX1D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX1D_SRV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX1D_UAV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_UAV {}
impl ::core::clone::Clone for D3D12_TEX1D_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX1D_UAV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_ARRAY_DSV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2DMS_ARRAY_DSV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_ARRAY_RTV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2DMS_ARRAY_RTV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_ARRAY_SRV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2DMS_ARRAY_SRV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_DSV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2DMS_DSV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_RTV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2DMS_RTV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_SRV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2DMS_SRV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_ARRAY_DSV {}
impl ::core::clone::Clone for D3D12_TEX2D_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2D_ARRAY_DSV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_ARRAY_RTV {}
impl ::core::clone::Clone for D3D12_TEX2D_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2D_ARRAY_RTV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEX2D_ARRAY_SRV {}
impl ::core::clone::Clone for D3D12_TEX2D_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2D_ARRAY_SRV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_ARRAY_UAV {}
impl ::core::clone::Clone for D3D12_TEX2D_ARRAY_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2D_ARRAY_UAV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_DSV {}
impl ::core::clone::Clone for D3D12_TEX2D_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2D_DSV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2D_RTV {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_RTV {}
impl ::core::clone::Clone for D3D12_TEX2D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2D_RTV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub PlaneSlice: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEX2D_SRV {}
impl ::core::clone::Clone for D3D12_TEX2D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2D_SRV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX2D_UAV {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_UAV {}
impl ::core::clone::Clone for D3D12_TEX2D_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX2D_UAV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl ::core::marker::Copy for D3D12_TEX3D_RTV {}
impl ::core::clone::Clone for D3D12_TEX3D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX3D_RTV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEX3D_SRV {}
impl ::core::clone::Clone for D3D12_TEX3D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX3D_SRV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEX3D_UAV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl ::core::marker::Copy for D3D12_TEX3D_UAV {}
impl ::core::clone::Clone for D3D12_TEX3D_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEX3D_UAV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEXCUBE_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEXCUBE_ARRAY_SRV {}
impl ::core::clone::Clone for D3D12_TEXCUBE_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEXCUBE_ARRAY_SRV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEXCUBE_SRV {}
impl ::core::clone::Clone for D3D12_TEXCUBE_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TEXCUBE_SRV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_TEXTURE_COPY_LOCATION {
    pub pResource: ::windows::core::ManuallyDrop<ID3D12Resource>,
    pub Type: D3D12_TEXTURE_COPY_TYPE,
    pub Anonymous: D3D12_TEXTURE_COPY_LOCATION_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_TEXTURE_COPY_LOCATION {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_TEXTURE_COPY_LOCATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_TEXTURE_COPY_LOCATION_0 {
    pub PlacedFootprint: D3D12_PLACED_SUBRESOURCE_FOOTPRINT,
    pub SubresourceIndex: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_TEXTURE_COPY_LOCATION_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_TEXTURE_COPY_LOCATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_TEXTURE_COPY_LOCATION_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TILED_RESOURCE_COORDINATE {
    pub X: u32,
    pub Y: u32,
    pub Z: u32,
    pub Subresource: u32,
}
impl ::core::marker::Copy for D3D12_TILED_RESOURCE_COORDINATE {}
impl ::core::clone::Clone for D3D12_TILED_RESOURCE_COORDINATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TILED_RESOURCE_COORDINATE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_TILE_REGION_SIZE {
    pub NumTiles: u32,
    pub UseBox: super::super::Foundation::BOOL,
    pub Width: u32,
    pub Height: u16,
    pub Depth: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_TILE_REGION_SIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_TILE_REGION_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D12_TILE_REGION_SIZE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_TILE_SHAPE {
    pub WidthInTexels: u32,
    pub HeightInTexels: u32,
    pub DepthInTexels: u32,
}
impl ::core::marker::Copy for D3D12_TILE_SHAPE {}
impl ::core::clone::Clone for D3D12_TILE_SHAPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_TILE_SHAPE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_UAV_DIMENSION,
    pub Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_UNORDERED_ACCESS_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_UNORDERED_ACCESS_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_UNORDERED_ACCESS_VIEW_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
    pub Buffer: D3D12_BUFFER_UAV,
    pub Texture1D: D3D12_TEX1D_UAV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D12_TEX2D_UAV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_UAV,
    pub Texture3D: D3D12_TEX3D_UAV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA {
    pub Version: D3D12_DRED_VERSION,
    pub Anonymous: D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0,
}
impl ::core::marker::Copy for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA {}
impl ::core::clone::Clone for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub union D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0 {
    pub Dred_1_0: D3D12_DEVICE_REMOVED_EXTENDED_DATA,
    pub Dred_1_1: D3D12_DEVICE_REMOVED_EXTENDED_DATA1,
    pub Dred_1_2: D3D12_DEVICE_REMOVED_EXTENDED_DATA2,
    pub Dred_1_3: D3D12_DEVICE_REMOVED_EXTENDED_DATA3,
}
impl ::core::marker::Copy for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0 {}
impl ::core::clone::Clone for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
    pub Version: D3D_ROOT_SIGNATURE_VERSION,
    pub Anonymous: D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0,
}
impl ::core::marker::Copy for D3D12_VERSIONED_ROOT_SIGNATURE_DESC {}
impl ::core::clone::Clone for D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub union D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
    pub Desc_1_0: D3D12_ROOT_SIGNATURE_DESC,
    pub Desc_1_1: D3D12_ROOT_SIGNATURE_DESC1,
}
impl ::core::marker::Copy for D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {}
impl ::core::clone::Clone for D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_VERTEX_BUFFER_VIEW {
    pub BufferLocation: u64,
    pub SizeInBytes: u32,
    pub StrideInBytes: u32,
}
impl ::core::marker::Copy for D3D12_VERTEX_BUFFER_VIEW {}
impl ::core::clone::Clone for D3D12_VERTEX_BUFFER_VIEW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_VERTEX_BUFFER_VIEW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_VIEWPORT {
    pub TopLeftX: f32,
    pub TopLeftY: f32,
    pub Width: f32,
    pub Height: f32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
impl ::core::marker::Copy for D3D12_VIEWPORT {}
impl ::core::clone::Clone for D3D12_VIEWPORT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_VIEWPORT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_VIEW_INSTANCE_LOCATION {
    pub ViewportArrayIndex: u32,
    pub RenderTargetArrayIndex: u32,
}
impl ::core::marker::Copy for D3D12_VIEW_INSTANCE_LOCATION {}
impl ::core::clone::Clone for D3D12_VIEW_INSTANCE_LOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_VIEW_INSTANCE_LOCATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_VIEW_INSTANCING_DESC {
    pub ViewInstanceCount: u32,
    pub pViewInstanceLocations: *const D3D12_VIEW_INSTANCE_LOCATION,
    pub Flags: D3D12_VIEW_INSTANCING_FLAGS,
}
impl ::core::marker::Copy for D3D12_VIEW_INSTANCING_DESC {}
impl ::core::clone::Clone for D3D12_VIEW_INSTANCING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_VIEW_INSTANCING_DESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub struct D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {
    pub Dest: u64,
    pub Value: u32,
}
impl ::core::marker::Copy for D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {}
impl ::core::clone::Clone for D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub type D3D12MessageFunc = ::core::option::Option<unsafe extern "system" fn(category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: ::windows::core::PCSTR, pcontext: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub type PFN_D3D12_CREATE_DEVICE = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<::windows::core::IUnknown>, param1: super::Direct3D::D3D_FEATURE_LEVEL, param2: *const ::windows::core::GUID, param3: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub type PFN_D3D12_CREATE_ROOT_SIGNATURE_DESERIALIZER = ::core::option::Option<unsafe extern "system" fn(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows::core::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub type PFN_D3D12_CREATE_VERSIONED_ROOT_SIGNATURE_DESERIALIZER = ::core::option::Option<unsafe extern "system" fn(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows::core::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub type PFN_D3D12_GET_DEBUG_INTERFACE = ::core::option::Option<unsafe extern "system" fn(param0: *const ::windows::core::GUID, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
pub type PFN_D3D12_GET_INTERFACE = ::core::option::Option<unsafe extern "system" fn(param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub type PFN_D3D12_SERIALIZE_ROOT_SIGNATURE = ::core::option::Option<unsafe extern "system" fn(prootsignature: *const D3D12_ROOT_SIGNATURE_DESC, version: D3D_ROOT_SIGNATURE_VERSION, ppblob: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrorblob: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub type PFN_D3D12_SERIALIZE_VERSIONED_ROOT_SIGNATURE = ::core::option::Option<unsafe extern "system" fn(prootsignature: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppblob: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrorblob: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
