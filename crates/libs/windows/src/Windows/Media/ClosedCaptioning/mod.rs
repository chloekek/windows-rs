#[doc(hidden)]
#[repr(transparent)]
pub struct IClosedCaptionPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClosedCaptionPropertiesStatics {
    type Vtable = IClosedCaptionPropertiesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IClosedCaptionPropertiesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10aa1f84_cc30_4141_b503_5272289e0c20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosedCaptionPropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FontColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedFontColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedFontColor: usize,
    pub FontOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT,
    pub FontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionSize) -> ::windows::core::HRESULT,
    pub FontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionStyle) -> ::windows::core::HRESULT,
    pub FontEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionEdgeEffect) -> ::windows::core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedBackgroundColor: usize,
    pub BackgroundOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT,
    pub RegionColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedRegionColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedRegionColor: usize,
    pub RegionOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
pub struct ClosedCaptionProperties;
impl ClosedCaptionProperties {
    pub fn FontColor() -> ::windows::core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn ComputedFontColor() -> ::windows::core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ComputedFontColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn FontOpacity() -> ::windows::core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontOpacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn FontSize() -> ::windows::core::Result<ClosedCaptionSize> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn FontStyle() -> ::windows::core::Result<ClosedCaptionStyle> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn FontEffect() -> ::windows::core::Result<ClosedCaptionEdgeEffect> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontEffect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn BackgroundColor() -> ::windows::core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn ComputedBackgroundColor() -> ::windows::core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ComputedBackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn BackgroundOpacity() -> ::windows::core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundOpacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn RegionColor() -> ::windows::core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegionColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn ComputedRegionColor() -> ::windows::core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ComputedRegionColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn RegionOpacity() -> ::windows::core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegionOpacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IClosedCaptionPropertiesStatics<R, F: FnOnce(&IClosedCaptionPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ClosedCaptionProperties, IClosedCaptionPropertiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ClosedCaptionProperties {
    const NAME: &'static str = "Windows.Media.ClosedCaptioning.ClosedCaptionProperties";
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ClosedCaptionColor(pub i32);
impl ClosedCaptionColor {
    pub const Default: Self = Self(0i32);
    pub const White: Self = Self(1i32);
    pub const Black: Self = Self(2i32);
    pub const Red: Self = Self(3i32);
    pub const Green: Self = Self(4i32);
    pub const Blue: Self = Self(5i32);
    pub const Yellow: Self = Self(6i32);
    pub const Magenta: Self = Self(7i32);
    pub const Cyan: Self = Self(8i32);
}
impl ::core::marker::Copy for ClosedCaptionColor {}
impl ::core::clone::Clone for ClosedCaptionColor {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionColor {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionColor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionColor;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ClosedCaptionEdgeEffect(pub i32);
impl ClosedCaptionEdgeEffect {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Raised: Self = Self(2i32);
    pub const Depressed: Self = Self(3i32);
    pub const Uniform: Self = Self(4i32);
    pub const DropShadow: Self = Self(5i32);
}
impl ::core::marker::Copy for ClosedCaptionEdgeEffect {}
impl ::core::clone::Clone for ClosedCaptionEdgeEffect {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionEdgeEffect {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionEdgeEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionEdgeEffect;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ClosedCaptionOpacity(pub i32);
impl ClosedCaptionOpacity {
    pub const Default: Self = Self(0i32);
    pub const OneHundredPercent: Self = Self(1i32);
    pub const SeventyFivePercent: Self = Self(2i32);
    pub const TwentyFivePercent: Self = Self(3i32);
    pub const ZeroPercent: Self = Self(4i32);
}
impl ::core::marker::Copy for ClosedCaptionOpacity {}
impl ::core::clone::Clone for ClosedCaptionOpacity {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionOpacity {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionOpacity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionOpacity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ClosedCaptionSize(pub i32);
impl ClosedCaptionSize {
    pub const Default: Self = Self(0i32);
    pub const FiftyPercent: Self = Self(1i32);
    pub const OneHundredPercent: Self = Self(2i32);
    pub const OneHundredFiftyPercent: Self = Self(3i32);
    pub const TwoHundredPercent: Self = Self(4i32);
}
impl ::core::marker::Copy for ClosedCaptionSize {}
impl ::core::clone::Clone for ClosedCaptionSize {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionSize {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionSize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ClosedCaptionStyle(pub i32);
impl ClosedCaptionStyle {
    pub const Default: Self = Self(0i32);
    pub const MonospacedWithSerifs: Self = Self(1i32);
    pub const ProportionalWithSerifs: Self = Self(2i32);
    pub const MonospacedWithoutSerifs: Self = Self(3i32);
    pub const ProportionalWithoutSerifs: Self = Self(4i32);
    pub const Casual: Self = Self(5i32);
    pub const Cursive: Self = Self(6i32);
    pub const SmallCapitals: Self = Self(7i32);
}
impl ::core::marker::Copy for ClosedCaptionStyle {}
impl ::core::clone::Clone for ClosedCaptionStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionStyle {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
