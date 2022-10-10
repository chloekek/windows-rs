#[doc(hidden)]
#[repr(transparent)]
pub struct IWidgetActionInvokedArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWidgetActionInvokedArgs {
    type Vtable = IWidgetActionInvokedArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWidgetActionInvokedArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc593cc57_04b9_52ca_88ad_46fea21ea340);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetActionInvokedArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CustomState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWidgetContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWidgetContext {
    type Vtable = IWidgetContext_Vtbl;
}
unsafe impl ::windows::core::Interface for IWidgetContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x903c518b_40bc_5bc6_88f7_af9d81c0cdc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DefinitionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::WidgetSize) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWidgetContextChangedArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWidgetContextChangedArgs {
    type Vtable = IWidgetContextChangedArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWidgetContextChangedArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c226d54_2252_576b_a197_370b28d25c2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetContextChangedArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWidgetInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWidgetInfo {
    type Vtable = IWidgetInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IWidgetInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcea11f42_a020_5db5_89e2_b7dece4ae5cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Template: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CustomState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LastUpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::Foundation::DateTime) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IWidgetManager(::windows::core::IUnknown);
impl IWidgetManager {
    pub fn UpdateWidget<'a, P0>(&self, widgetupdaterequestoptions: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, WidgetUpdateRequestOptions>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateWidget)(::windows::core::Vtable::as_raw(this), widgetupdaterequestoptions.into().abi()).ok() }
    }
    pub fn GetWidgetIds(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetWidgetIds)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<::windows::core::HSTRING>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn GetWidgetInfo(&self, widgetid: &::windows::core::HSTRING) -> ::windows::core::Result<WidgetInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetWidgetInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(widgetid), result__.as_mut_ptr()).from_abi::<WidgetInfo>(result__)
        }
    }
    pub fn GetWidgetInfos(&self) -> ::windows::core::Result<::windows::core::Array<WidgetInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetWidgetInfos)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<WidgetInfo>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn DeleteWidget(&self, widgetid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).DeleteWidget)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(widgetid)).ok() }
    }
}
impl ::core::convert::From<IWidgetManager> for ::windows::core::IUnknown {
    fn from(value: IWidgetManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWidgetManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWidgetManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWidgetManager> for ::windows::core::IUnknown {
    fn from(value: &IWidgetManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWidgetManager> for ::windows::core::IInspectable {
    fn from(value: IWidgetManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWidgetManager> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IWidgetManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWidgetManager> for ::windows::core::IInspectable {
    fn from(value: &IWidgetManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWidgetManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWidgetManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWidgetManager {}
impl ::core::fmt::Debug for IWidgetManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWidgetManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWidgetManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{71cb10c0-671e-48e3-b995-207940397123}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IWidgetManager {
    type Vtable = IWidgetManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IWidgetManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71cb10c0_671e_48e3_b995_207940397123);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UpdateWidget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, widgetupdaterequestoptions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetWidgetIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetWidgetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, widgetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetWidgetInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteWidget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, widgetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWidgetManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWidgetManagerStatics {
    type Vtable = IWidgetManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IWidgetManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f233b06_28e5_5e2b_8c04_a4fa747c28c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IWidgetProvider(::windows::core::IUnknown);
impl IWidgetProvider {
    pub fn CreateWidget<'a, P0>(&self, widgetcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, WidgetContext>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CreateWidget)(::windows::core::Vtable::as_raw(this), widgetcontext.into().abi()).ok() }
    }
    pub fn DeleteWidget(&self, widgetid: &::windows::core::HSTRING, customstate: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).DeleteWidget)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(widgetid), ::core::mem::transmute_copy(customstate)).ok() }
    }
    pub fn OnActionInvoked<'a, P0>(&self, actioninvokedargs: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, WidgetActionInvokedArgs>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnActionInvoked)(::windows::core::Vtable::as_raw(this), actioninvokedargs.into().abi()).ok() }
    }
    pub fn OnWidgetContextChanged<'a, P0>(&self, contextchangedargs: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, WidgetContextChangedArgs>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnWidgetContextChanged)(::windows::core::Vtable::as_raw(this), contextchangedargs.into().abi()).ok() }
    }
    pub fn Activate<'a, P0>(&self, widgetcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, WidgetContext>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Activate)(::windows::core::Vtable::as_raw(this), widgetcontext.into().abi()).ok() }
    }
    pub fn Deactivate(&self, widgetid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Deactivate)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(widgetid)).ok() }
    }
}
impl ::core::convert::From<IWidgetProvider> for ::windows::core::IUnknown {
    fn from(value: IWidgetProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWidgetProvider> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWidgetProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWidgetProvider> for ::windows::core::IUnknown {
    fn from(value: &IWidgetProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWidgetProvider> for ::windows::core::IInspectable {
    fn from(value: IWidgetProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWidgetProvider> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IWidgetProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWidgetProvider> for ::windows::core::IInspectable {
    fn from(value: &IWidgetProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWidgetProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWidgetProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWidgetProvider {}
impl ::core::fmt::Debug for IWidgetProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWidgetProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWidgetProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5c5774cc-72a0-452d-b9ed-075c0dd25eed}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IWidgetProvider {
    type Vtable = IWidgetProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IWidgetProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c5774cc_72a0_452d_b9ed_075c0dd25eed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWidget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, widgetcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteWidget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, widgetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customstate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub OnActionInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actioninvokedargs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnWidgetContextChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contextchangedargs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, widgetcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, widgetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWidgetUpdateRequestOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWidgetUpdateRequestOptions {
    type Vtable = IWidgetUpdateRequestOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IWidgetUpdateRequestOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb09ca8f7_7424_5687_baaf_7dd6fa639672);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetUpdateRequestOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub WidgetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Template: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CustomState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCustomState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWidgetUpdateRequestOptionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWidgetUpdateRequestOptionsFactory {
    type Vtable = IWidgetUpdateRequestOptionsFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IWidgetUpdateRequestOptionsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0e00af8_1d10_57a8_9419_3f568e854daa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetUpdateRequestOptionsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, widgetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWidgetUpdateRequestOptionsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWidgetUpdateRequestOptionsStatics {
    type Vtable = IWidgetUpdateRequestOptionsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IWidgetUpdateRequestOptionsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4645b5e3_d332_5d11_82f0_3607e5df6018);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetUpdateRequestOptionsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UnsetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct WidgetActionInvokedArgs(::windows::core::IUnknown);
impl WidgetActionInvokedArgs {
    pub fn WidgetContext(&self) -> ::windows::core::Result<WidgetContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WidgetContext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<WidgetContext>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Verb)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CustomState(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for WidgetActionInvokedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WidgetActionInvokedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WidgetActionInvokedArgs {}
impl ::core::fmt::Debug for WidgetActionInvokedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WidgetActionInvokedArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WidgetActionInvokedArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Microsoft.Windows.Widgets.Providers.WidgetActionInvokedArgs;{c593cc57-04b9-52ca-88ad-46fea21ea340})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WidgetActionInvokedArgs {
    type Vtable = IWidgetActionInvokedArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WidgetActionInvokedArgs {
    const IID: ::windows::core::GUID = <IWidgetActionInvokedArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WidgetActionInvokedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetActionInvokedArgs";
}
impl ::core::convert::From<WidgetActionInvokedArgs> for ::windows::core::IUnknown {
    fn from(value: WidgetActionInvokedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetActionInvokedArgs> for ::windows::core::IUnknown {
    fn from(value: &WidgetActionInvokedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetActionInvokedArgs> for &::windows::core::IUnknown {
    fn from(value: &WidgetActionInvokedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WidgetActionInvokedArgs> for ::windows::core::IInspectable {
    fn from(value: WidgetActionInvokedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetActionInvokedArgs> for ::windows::core::IInspectable {
    fn from(value: &WidgetActionInvokedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetActionInvokedArgs> for &::windows::core::IInspectable {
    fn from(value: &WidgetActionInvokedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for WidgetActionInvokedArgs {}
unsafe impl ::core::marker::Sync for WidgetActionInvokedArgs {}
#[repr(transparent)]
pub struct WidgetContext(::windows::core::IUnknown);
impl WidgetContext {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DefinitionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DefinitionId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<super::WidgetSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::WidgetSize>(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsActive)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for WidgetContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WidgetContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WidgetContext {}
impl ::core::fmt::Debug for WidgetContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WidgetContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WidgetContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Microsoft.Windows.Widgets.Providers.WidgetContext;{903c518b-40bc-5bc6-88f7-af9d81c0cdc1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WidgetContext {
    type Vtable = IWidgetContext_Vtbl;
}
unsafe impl ::windows::core::Interface for WidgetContext {
    const IID: ::windows::core::GUID = <IWidgetContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WidgetContext {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetContext";
}
impl ::core::convert::From<WidgetContext> for ::windows::core::IUnknown {
    fn from(value: WidgetContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetContext> for ::windows::core::IUnknown {
    fn from(value: &WidgetContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetContext> for &::windows::core::IUnknown {
    fn from(value: &WidgetContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WidgetContext> for ::windows::core::IInspectable {
    fn from(value: WidgetContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetContext> for ::windows::core::IInspectable {
    fn from(value: &WidgetContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetContext> for &::windows::core::IInspectable {
    fn from(value: &WidgetContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for WidgetContext {}
unsafe impl ::core::marker::Sync for WidgetContext {}
#[repr(transparent)]
pub struct WidgetContextChangedArgs(::windows::core::IUnknown);
impl WidgetContextChangedArgs {
    pub fn WidgetContext(&self) -> ::windows::core::Result<WidgetContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WidgetContext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<WidgetContext>(result__)
        }
    }
}
impl ::core::clone::Clone for WidgetContextChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WidgetContextChangedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WidgetContextChangedArgs {}
impl ::core::fmt::Debug for WidgetContextChangedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WidgetContextChangedArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WidgetContextChangedArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Microsoft.Windows.Widgets.Providers.WidgetContextChangedArgs;{2c226d54-2252-576b-a197-370b28d25c2f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WidgetContextChangedArgs {
    type Vtable = IWidgetContextChangedArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WidgetContextChangedArgs {
    const IID: ::windows::core::GUID = <IWidgetContextChangedArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WidgetContextChangedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetContextChangedArgs";
}
impl ::core::convert::From<WidgetContextChangedArgs> for ::windows::core::IUnknown {
    fn from(value: WidgetContextChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetContextChangedArgs> for ::windows::core::IUnknown {
    fn from(value: &WidgetContextChangedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetContextChangedArgs> for &::windows::core::IUnknown {
    fn from(value: &WidgetContextChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WidgetContextChangedArgs> for ::windows::core::IInspectable {
    fn from(value: WidgetContextChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetContextChangedArgs> for ::windows::core::IInspectable {
    fn from(value: &WidgetContextChangedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetContextChangedArgs> for &::windows::core::IInspectable {
    fn from(value: &WidgetContextChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for WidgetContextChangedArgs {}
unsafe impl ::core::marker::Sync for WidgetContextChangedArgs {}
#[repr(transparent)]
pub struct WidgetInfo(::windows::core::IUnknown);
impl WidgetInfo {
    pub fn WidgetContext(&self) -> ::windows::core::Result<WidgetContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WidgetContext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<WidgetContext>(result__)
        }
    }
    pub fn Template(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Template)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CustomState(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn LastUpdateTime(&self) -> ::windows::core::Result<::windows::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastUpdateTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for WidgetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WidgetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WidgetInfo {}
impl ::core::fmt::Debug for WidgetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WidgetInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WidgetInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Microsoft.Windows.Widgets.Providers.WidgetInfo;{cea11f42-a020-5db5-89e2-b7dece4ae5cb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WidgetInfo {
    type Vtable = IWidgetInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for WidgetInfo {
    const IID: ::windows::core::GUID = <IWidgetInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WidgetInfo {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetInfo";
}
impl ::core::convert::From<WidgetInfo> for ::windows::core::IUnknown {
    fn from(value: WidgetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetInfo> for ::windows::core::IUnknown {
    fn from(value: &WidgetInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetInfo> for &::windows::core::IUnknown {
    fn from(value: &WidgetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WidgetInfo> for ::windows::core::IInspectable {
    fn from(value: WidgetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetInfo> for ::windows::core::IInspectable {
    fn from(value: &WidgetInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetInfo> for &::windows::core::IInspectable {
    fn from(value: &WidgetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for WidgetInfo {}
unsafe impl ::core::marker::Sync for WidgetInfo {}
#[repr(transparent)]
pub struct WidgetManager(::windows::core::IUnknown);
impl WidgetManager {
    pub fn UpdateWidget<'a, P0>(&self, widgetupdaterequestoptions: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, WidgetUpdateRequestOptions>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateWidget)(::windows::core::Vtable::as_raw(this), widgetupdaterequestoptions.into().abi()).ok() }
    }
    pub fn GetWidgetIds(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetWidgetIds)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<::windows::core::HSTRING>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn GetWidgetInfo(&self, widgetid: &::windows::core::HSTRING) -> ::windows::core::Result<WidgetInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetWidgetInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(widgetid), result__.as_mut_ptr()).from_abi::<WidgetInfo>(result__)
        }
    }
    pub fn GetWidgetInfos(&self) -> ::windows::core::Result<::windows::core::Array<WidgetInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetWidgetInfos)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<WidgetInfo>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn DeleteWidget(&self, widgetid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).DeleteWidget)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(widgetid)).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<WidgetManager> {
        Self::IWidgetManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<WidgetManager>(result__)
        })
    }
    pub fn IWidgetManagerStatics<R, F: FnOnce(&IWidgetManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WidgetManager, IWidgetManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WidgetManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WidgetManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WidgetManager {}
impl ::core::fmt::Debug for WidgetManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WidgetManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WidgetManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Microsoft.Windows.Widgets.Providers.WidgetManager;{71cb10c0-671e-48e3-b995-207940397123})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WidgetManager {
    type Vtable = IWidgetManager_Vtbl;
}
unsafe impl ::windows::core::Interface for WidgetManager {
    const IID: ::windows::core::GUID = <IWidgetManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WidgetManager {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetManager";
}
impl ::core::convert::From<WidgetManager> for ::windows::core::IUnknown {
    fn from(value: WidgetManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetManager> for ::windows::core::IUnknown {
    fn from(value: &WidgetManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetManager> for &::windows::core::IUnknown {
    fn from(value: &WidgetManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WidgetManager> for ::windows::core::IInspectable {
    fn from(value: WidgetManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetManager> for ::windows::core::IInspectable {
    fn from(value: &WidgetManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetManager> for &::windows::core::IInspectable {
    fn from(value: &WidgetManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<WidgetManager> for IWidgetManager {
    type Error = ::windows::core::Error;
    fn try_from(value: WidgetManager) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WidgetManager> for IWidgetManager {
    type Error = ::windows::core::Error;
    fn try_from(value: &WidgetManager) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WidgetManager> for ::windows::core::InParam<'a, IWidgetManager> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WidgetManager) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for WidgetManager {}
unsafe impl ::core::marker::Sync for WidgetManager {}
#[repr(transparent)]
pub struct WidgetUpdateRequestOptions(::windows::core::IUnknown);
impl WidgetUpdateRequestOptions {
    pub fn WidgetId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WidgetId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Template(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Template)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTemplate(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTemplate)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CustomState(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCustomState(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCustomState)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateInstance(widgetid: &::windows::core::HSTRING) -> ::windows::core::Result<WidgetUpdateRequestOptions> {
        Self::IWidgetUpdateRequestOptionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(widgetid), result__.as_mut_ptr()).from_abi::<WidgetUpdateRequestOptions>(result__)
        })
    }
    pub fn UnsetValue() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWidgetUpdateRequestOptionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnsetValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IWidgetUpdateRequestOptionsFactory<R, F: FnOnce(&IWidgetUpdateRequestOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WidgetUpdateRequestOptions, IWidgetUpdateRequestOptionsFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IWidgetUpdateRequestOptionsStatics<R, F: FnOnce(&IWidgetUpdateRequestOptionsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WidgetUpdateRequestOptions, IWidgetUpdateRequestOptionsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WidgetUpdateRequestOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WidgetUpdateRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WidgetUpdateRequestOptions {}
impl ::core::fmt::Debug for WidgetUpdateRequestOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WidgetUpdateRequestOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WidgetUpdateRequestOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Microsoft.Windows.Widgets.Providers.WidgetUpdateRequestOptions;{b09ca8f7-7424-5687-baaf-7dd6fa639672})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WidgetUpdateRequestOptions {
    type Vtable = IWidgetUpdateRequestOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for WidgetUpdateRequestOptions {
    const IID: ::windows::core::GUID = <IWidgetUpdateRequestOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WidgetUpdateRequestOptions {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetUpdateRequestOptions";
}
impl ::core::convert::From<WidgetUpdateRequestOptions> for ::windows::core::IUnknown {
    fn from(value: WidgetUpdateRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetUpdateRequestOptions> for ::windows::core::IUnknown {
    fn from(value: &WidgetUpdateRequestOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetUpdateRequestOptions> for &::windows::core::IUnknown {
    fn from(value: &WidgetUpdateRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WidgetUpdateRequestOptions> for ::windows::core::IInspectable {
    fn from(value: WidgetUpdateRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WidgetUpdateRequestOptions> for ::windows::core::IInspectable {
    fn from(value: &WidgetUpdateRequestOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WidgetUpdateRequestOptions> for &::windows::core::IInspectable {
    fn from(value: &WidgetUpdateRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for WidgetUpdateRequestOptions {}
unsafe impl ::core::marker::Sync for WidgetUpdateRequestOptions {}
::core::include!("impl.rs");
