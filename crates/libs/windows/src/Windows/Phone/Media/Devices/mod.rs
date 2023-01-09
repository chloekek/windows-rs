#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioRoutingManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAudioRoutingManager {
    type Vtable = IAudioRoutingManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IAudioRoutingManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79340d20_71cc_4526_9f29_fc8d2486418b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRoutingManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioRoutingEndpoint) -> ::windows::core::HRESULT,
    pub SetAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: AudioRoutingEndpoint) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioEndpointChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointchangehandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioEndpointChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioEndpointChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioEndpointChanged: usize,
    pub AvailableAudioEndpoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AvailableAudioRoutingEndpoints) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioRoutingManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAudioRoutingManagerStatics {
    type Vtable = IAudioRoutingManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAudioRoutingManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x977fb2a4_5590_4a6f_adde_6a3d0ad58250);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRoutingManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioRoutingManager(::windows::core::IUnknown);
impl AudioRoutingManager {
    pub fn GetAudioEndpoint(&self) -> ::windows::core::Result<AudioRoutingEndpoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAudioEndpoint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAudioEndpoint(&self, endpoint: AudioRoutingEndpoint) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAudioEndpoint)(::windows::core::Vtable::as_raw(this), endpoint).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioEndpointChanged(&self, endpointchangehandler: &super::super::super::Foundation::TypedEventHandler<AudioRoutingManager, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioEndpointChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(endpointchangehandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioEndpointChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAudioEndpointChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn AvailableAudioEndpoints(&self) -> ::windows::core::Result<AvailableAudioRoutingEndpoints> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AvailableAudioEndpoints)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<AudioRoutingManager> {
        Self::IAudioRoutingManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioRoutingManagerStatics<R, F: FnOnce(&IAudioRoutingManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AudioRoutingManager, IAudioRoutingManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AudioRoutingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for AudioRoutingManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Media.Devices.AudioRoutingManager;{79340d20-71cc-4526-9f29-fc8d2486418b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AudioRoutingManager {
    type Vtable = IAudioRoutingManager_Vtbl;
}
unsafe impl ::windows::core::Interface for AudioRoutingManager {
    const IID: ::windows::core::GUID = <IAudioRoutingManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioRoutingManager {
    const NAME: &'static str = "Windows.Phone.Media.Devices.AudioRoutingManager";
}
::windows::core::interface_hierarchy!(AudioRoutingManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioRoutingManager {}
unsafe impl ::core::marker::Sync for AudioRoutingManager {}
#[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioRoutingEndpoint(pub i32);
impl AudioRoutingEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Earpiece: Self = Self(1i32);
    pub const Speakerphone: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
    pub const WiredHeadset: Self = Self(4i32);
    pub const WiredHeadsetSpeakerOnly: Self = Self(5i32);
    pub const BluetoothWithNoiseAndEchoCancellation: Self = Self(6i32);
    pub const BluetoothPreferred: Self = Self(7i32);
}
impl ::core::marker::Copy for AudioRoutingEndpoint {}
impl ::core::clone::Clone for AudioRoutingEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AudioRoutingEndpoint {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioRoutingEndpoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Media.Devices.AudioRoutingEndpoint;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AvailableAudioRoutingEndpoints(pub u32);
impl AvailableAudioRoutingEndpoints {
    pub const None: Self = Self(0u32);
    pub const Earpiece: Self = Self(1u32);
    pub const Speakerphone: Self = Self(2u32);
    pub const Bluetooth: Self = Self(4u32);
}
impl ::core::marker::Copy for AvailableAudioRoutingEndpoints {}
impl ::core::clone::Clone for AvailableAudioRoutingEndpoints {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AvailableAudioRoutingEndpoints {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AvailableAudioRoutingEndpoints {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Media.Devices.AvailableAudioRoutingEndpoints;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
