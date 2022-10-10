pub trait IWidgetManager_Impl: Sized {
    fn UpdateWidget(
        &self,
        widgetupdaterequestoptions: &::core::option::Option<WidgetUpdateRequestOptions>,
    ) -> ::windows::core::Result<()>;
    fn GetWidgetIds(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
    fn GetWidgetInfo(
        &self,
        widgetid: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<WidgetInfo>;
    fn GetWidgetInfos(&self) -> ::windows::core::Result<::windows::core::Array<WidgetInfo>>;
    fn DeleteWidget(&self, widgetid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWidgetManager {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetManager";
}
impl IWidgetManager_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IWidgetManager_Impl,
        const OFFSET: isize,
    >() -> IWidgetManager_Vtbl {
        unsafe extern "system" fn UpdateWidget<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetupdaterequestoptions: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateWidget(::core::mem::transmute(&widgetupdaterequestoptions))
                .into()
        }
        unsafe extern "system" fn GetWidgetIds<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWidgetIds() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWidgetInfo<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWidgetInfo(::core::mem::transmute(&widgetid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWidgetInfos<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWidgetInfos() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteWidget<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteWidget(::core::mem::transmute(&widgetid)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IWidgetManager, OFFSET>(),
            UpdateWidget: UpdateWidget::<Identity, Impl, OFFSET>,
            GetWidgetIds: GetWidgetIds::<Identity, Impl, OFFSET>,
            GetWidgetInfo: GetWidgetInfo::<Identity, Impl, OFFSET>,
            GetWidgetInfos: GetWidgetInfos::<Identity, Impl, OFFSET>,
            DeleteWidget: DeleteWidget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWidgetManager as ::windows::core::Interface>::IID
    }
}
pub trait IWidgetProvider_Impl: Sized {
    fn CreateWidget(
        &self,
        widgetcontext: &::core::option::Option<WidgetContext>,
    ) -> ::windows::core::Result<()>;
    fn DeleteWidget(
        &self,
        widgetid: &::windows::core::HSTRING,
        customstate: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()>;
    fn OnActionInvoked(
        &self,
        actioninvokedargs: &::core::option::Option<WidgetActionInvokedArgs>,
    ) -> ::windows::core::Result<()>;
    fn OnWidgetContextChanged(
        &self,
        contextchangedargs: &::core::option::Option<WidgetContextChangedArgs>,
    ) -> ::windows::core::Result<()>;
    fn Activate(
        &self,
        widgetcontext: &::core::option::Option<WidgetContext>,
    ) -> ::windows::core::Result<()>;
    fn Deactivate(&self, widgetid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWidgetProvider {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProvider";
}
impl IWidgetProvider_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IWidgetProvider_Impl,
        const OFFSET: isize,
    >() -> IWidgetProvider_Vtbl {
        unsafe extern "system" fn CreateWidget<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetcontext: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateWidget(::core::mem::transmute(&widgetcontext))
                .into()
        }
        unsafe extern "system" fn DeleteWidget<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            customstate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteWidget(
                ::core::mem::transmute(&widgetid),
                ::core::mem::transmute(&customstate),
            )
            .into()
        }
        unsafe extern "system" fn OnActionInvoked<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            actioninvokedargs: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActionInvoked(::core::mem::transmute(&actioninvokedargs))
                .into()
        }
        unsafe extern "system" fn OnWidgetContextChanged<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            contextchangedargs: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWidgetContextChanged(::core::mem::transmute(&contextchangedargs))
                .into()
        }
        unsafe extern "system" fn Activate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetcontext: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate(::core::mem::transmute(&widgetcontext)).into()
        }
        unsafe extern "system" fn Deactivate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate(::core::mem::transmute(&widgetid)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IWidgetProvider, OFFSET>(),
            CreateWidget: CreateWidget::<Identity, Impl, OFFSET>,
            DeleteWidget: DeleteWidget::<Identity, Impl, OFFSET>,
            OnActionInvoked: OnActionInvoked::<Identity, Impl, OFFSET>,
            OnWidgetContextChanged: OnWidgetContextChanged::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWidgetProvider as ::windows::core::Interface>::IID
    }
}
