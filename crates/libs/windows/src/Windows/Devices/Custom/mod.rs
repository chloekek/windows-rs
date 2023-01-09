#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICustomDevice {
    type Vtable = ICustomDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for ICustomDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd30251f_c48b_43bd_bcb1_dec88f15143e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendIOControlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iocontrolcode: *mut ::core::ffi::c_void, inputbuffer: *mut ::core::ffi::c_void, outputbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendIOControlAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TrySendIOControlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iocontrolcode: *mut ::core::ffi::c_void, inputbuffer: *mut ::core::ffi::c_void, outputbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TrySendIOControlAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICustomDeviceStatics {
    type Vtable = ICustomDeviceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICustomDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8220312_ef4c_46b1_a58e_eeb308dc8917);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classguid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct IIOControlCode(::windows::core::IUnknown);
impl IIOControlCode {
    pub fn AccessMode(&self) -> ::windows::core::Result<IOControlAccessMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BufferingMethod(&self) -> ::windows::core::Result<IOControlBufferingMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferingMethod)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Function(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Function)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeviceType(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ControlCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ControlCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IIOControlCode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IIOControlCode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for IIOControlCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0e9559e7-60c8-4375-a761-7f8808066c60}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IIOControlCode {
    type Vtable = IIOControlCode_Vtbl;
}
unsafe impl ::windows::core::Interface for IIOControlCode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e9559e7_60c8_4375_a761_7f8808066c60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIOControlCode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AccessMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IOControlAccessMode) -> ::windows::core::HRESULT,
    pub BufferingMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IOControlBufferingMethod) -> ::windows::core::HRESULT,
    pub Function: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub DeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ControlCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIOControlCodeFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIOControlCodeFactory {
    type Vtable = IIOControlCodeFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IIOControlCodeFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x856a7cf0_4c11_44ae_afc6_b8d4a212788f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIOControlCodeFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateIOControlCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownDeviceTypesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKnownDeviceTypesStatics {
    type Vtable = IKnownDeviceTypesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IKnownDeviceTypesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee5479c2_5448_45da_ad1b_24948c239094);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownDeviceTypesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Unknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct CustomDevice(::windows::core::IUnknown);
impl CustomDevice {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OutputStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendIOControlAsync<P0, E0, P1, E1, P2, E2>(&self, iocontrolcode: P0, inputbuffer: P1, outputbuffer: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IIOControlCode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendIOControlAsync)(::windows::core::Vtable::as_raw(this), iocontrolcode.try_into().map_err(|e| e.into())?.abi(), inputbuffer.try_into().map_err(|e| e.into())?.abi(), outputbuffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TrySendIOControlAsync<P0, E0, P1, E1, P2, E2>(&self, iocontrolcode: P0, inputbuffer: P1, outputbuffer: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IIOControlCode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySendIOControlAsync)(::windows::core::Vtable::as_raw(this), iocontrolcode.try_into().map_err(|e| e.into())?.abi(), inputbuffer.try_into().map_err(|e| e.into())?.abi(), outputbuffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector(classguid: ::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICustomDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), classguid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CustomDevice>> {
        Self::ICustomDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), desiredaccess, sharingmode, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICustomDeviceStatics<R, F: FnOnce(&ICustomDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CustomDevice, ICustomDeviceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CustomDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for CustomDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Custom.CustomDevice;{dd30251f-c48b-43bd-bcb1-dec88f15143e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CustomDevice {
    type Vtable = ICustomDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for CustomDevice {
    const IID: ::windows::core::GUID = <ICustomDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CustomDevice {
    const NAME: &'static str = "Windows.Devices.Custom.CustomDevice";
}
::windows::core::interface_hierarchy!(CustomDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CustomDevice {}
unsafe impl ::core::marker::Sync for CustomDevice {}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct IOControlCode(::windows::core::IUnknown);
impl IOControlCode {
    pub fn AccessMode(&self) -> ::windows::core::Result<IOControlAccessMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BufferingMethod(&self) -> ::windows::core::Result<IOControlBufferingMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferingMethod)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Function(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Function)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeviceType(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ControlCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ControlCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateIOControlCode(devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod) -> ::windows::core::Result<IOControlCode> {
        Self::IIOControlCodeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateIOControlCode)(::windows::core::Vtable::as_raw(this), devicetype, function, accessmode, bufferingmethod, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIOControlCodeFactory<R, F: FnOnce(&IIOControlCodeFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IOControlCode, IIOControlCodeFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for IOControlCode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for IOControlCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Custom.IOControlCode;{0e9559e7-60c8-4375-a761-7f8808066c60})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IOControlCode {
    type Vtable = IIOControlCode_Vtbl;
}
unsafe impl ::windows::core::Interface for IOControlCode {
    const IID: ::windows::core::GUID = <IIOControlCode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IOControlCode {
    const NAME: &'static str = "Windows.Devices.Custom.IOControlCode";
}
::windows::core::interface_hierarchy!(IOControlCode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<IOControlCode> for IIOControlCode {
    type Error = ::windows::core::Error;
    fn try_from(value: IOControlCode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IOControlCode> for IIOControlCode {
    type Error = ::windows::core::Error;
    fn try_from(value: &IOControlCode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IOControlCode> for ::windows::core::InParam<IIOControlCode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IOControlCode) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for IOControlCode {}
unsafe impl ::core::marker::Sync for IOControlCode {}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
pub struct KnownDeviceTypes;
impl KnownDeviceTypes {
    pub fn Unknown() -> ::windows::core::Result<u16> {
        Self::IKnownDeviceTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Unknown)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownDeviceTypesStatics<R, F: FnOnce(&IKnownDeviceTypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownDeviceTypes, IKnownDeviceTypesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KnownDeviceTypes {
    const NAME: &'static str = "Windows.Devices.Custom.KnownDeviceTypes";
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceAccessMode(pub i32);
impl DeviceAccessMode {
    pub const Read: Self = Self(0i32);
    pub const Write: Self = Self(1i32);
    pub const ReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for DeviceAccessMode {}
impl ::core::clone::Clone for DeviceAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DeviceAccessMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeviceAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.DeviceAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceSharingMode(pub i32);
impl DeviceSharingMode {
    pub const Shared: Self = Self(0i32);
    pub const Exclusive: Self = Self(1i32);
}
impl ::core::marker::Copy for DeviceSharingMode {}
impl ::core::clone::Clone for DeviceSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DeviceSharingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeviceSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.DeviceSharingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IOControlAccessMode(pub i32);
impl IOControlAccessMode {
    pub const Any: Self = Self(0i32);
    pub const Read: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
    pub const ReadWrite: Self = Self(3i32);
}
impl ::core::marker::Copy for IOControlAccessMode {}
impl ::core::clone::Clone for IOControlAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IOControlAccessMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IOControlAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.IOControlAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IOControlBufferingMethod(pub i32);
impl IOControlBufferingMethod {
    pub const Buffered: Self = Self(0i32);
    pub const DirectInput: Self = Self(1i32);
    pub const DirectOutput: Self = Self(2i32);
    pub const Neither: Self = Self(3i32);
}
impl ::core::marker::Copy for IOControlBufferingMethod {}
impl ::core::clone::Clone for IOControlBufferingMethod {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IOControlBufferingMethod {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IOControlBufferingMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.IOControlBufferingMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
