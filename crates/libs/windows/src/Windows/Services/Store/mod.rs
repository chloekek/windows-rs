#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAcquireLicenseResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreAcquireLicenseResult {
    type Vtable = IStoreAcquireLicenseResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreAcquireLicenseResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbd7946d_f040_4cb3_9a39_29bcecdbe22d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAcquireLicenseResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StorePackageLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAppLicense(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreAppLicense {
    type Vtable = IStoreAppLicense_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreAppLicense {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf389f9de_73c0_45ce_9bab_b2fe3e5eafd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAppLicense_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SkuStoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddOnLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddOnLicenses: usize,
    #[cfg(feature = "Foundation")]
    pub TrialTimeRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrialTimeRemaining: usize,
    pub IsTrialOwnedByThisUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TrialUniqueId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAppLicense2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreAppLicense2 {
    type Vtable = IStoreAppLicense2_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreAppLicense2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4666e91_4443_40b3_993f_28904435bdc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAppLicense2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsDiscLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAvailability(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreAvailability {
    type Vtable = IStoreAvailability_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreAvailability {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa060325_0ffd_4493_ad43_f1f9918f69fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAvailability_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndDate: usize,
    pub Price: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepurchaseproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreCanAcquireLicenseResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreCanAcquireLicenseResult {
    type Vtable = IStoreCanAcquireLicenseResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreCanAcquireLicenseResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a693db3_0088_482f_86d5_bd46522663ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreCanAcquireLicenseResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub LicensableSku: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreCanLicenseStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreCollectionData(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreCollectionData {
    type Vtable = IStoreCollectionData_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreCollectionData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8aa4c3b3_5bb3_441a_2ab4_4dab73d5ce67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreCollectionData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeveloperOfferId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AcquiredDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcquiredDate: usize,
    #[cfg(feature = "Foundation")]
    pub StartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDate: usize,
    #[cfg(feature = "Foundation")]
    pub EndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndDate: usize,
    #[cfg(feature = "Foundation")]
    pub TrialTimeRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrialTimeRemaining: usize,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConsumableResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreConsumableResult {
    type Vtable = IStoreConsumableResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreConsumableResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea5dab72_6a00_4052_be5b_bfdab4433352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConsumableResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreConsumableStatus) -> ::windows::core::HRESULT,
    pub TrackingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub BalanceRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreContext {
    type Vtable = IStoreContext_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac98b6be_f4fd_4912_babd_5035e5e8bcab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "Foundation")]
    pub OfflineLicensesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OfflineLicensesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOfflineLicensesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOfflineLicensesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub GetCustomerPurchaseIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceticket: *mut ::core::ffi::c_void, publisheruserid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomerPurchaseIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCustomerCollectionsIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceticket: *mut ::core::ffi::c_void, publisheruserid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomerCollectionsIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppLicenseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppLicenseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetStoreProductForCurrentAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStoreProductForCurrentAppAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStoreProductsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: *mut ::core::ffi::c_void, storeids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStoreProductsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAssociatedStoreProductsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAssociatedStoreProductsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAssociatedStoreProductsWithPagingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: *mut ::core::ffi::c_void, maxitemstoretrieveperpage: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAssociatedStoreProductsWithPagingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUserCollectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUserCollectionAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUserCollectionWithPagingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: *mut ::core::ffi::c_void, maxitemstoretrieveperpage: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUserCollectionWithPagingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productstoreid: *mut ::core::ffi::c_void, quantity: u32, trackingid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportConsumableFulfillmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConsumableBalanceRemainingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productstoreid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConsumableBalanceRemainingAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub AcquireStoreLicenseForOptionalPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    AcquireStoreLicenseForOptionalPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: *mut ::core::ffi::c_void, storepurchaseproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppAndOptionalStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppAndOptionalStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadAndInstallStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadAndInstallStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadAndInstallStorePackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadAndInstallStorePackagesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreContext2 {
    type Vtable = IStoreContext2_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreContext2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18bc54da_7bd9_452c_9116_3bbd06ffc63a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindStoreProductForPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindStoreProductForPackageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreContext3 {
    type Vtable = IStoreContext3_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreContext3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe26226ca_1a01_4730_85a6_ecc896e4ae38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanSilentlyDownloadStorePackageUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TrySilentDownloadStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrySilentDownloadStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TrySilentDownloadAndInstallStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrySilentDownloadAndInstallStorePackageUpdatesAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub CanAcquireStoreLicenseForOptionalPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    CanAcquireStoreLicenseForOptionalPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CanAcquireStoreLicenseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productstoreid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanAcquireStoreLicenseAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStoreProductsWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: *mut ::core::ffi::c_void, storeids: *mut ::core::ffi::c_void, storeproductoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStoreProductsWithOptionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAssociatedStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAssociatedStoreQueueItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStoreQueueItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: *mut ::core::ffi::c_void, storepackageinstalloptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DownloadAndInstallStorePackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DownloadAndInstallStorePackagesAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub RequestUninstallStorePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    RequestUninstallStorePackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUninstallStorePackageByStoreIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUninstallStorePackageByStoreIdAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub UninstallStorePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    UninstallStorePackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UninstallStorePackageByStoreIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UninstallStorePackageByStoreIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreContext4 {
    type Vtable = IStoreContext4_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreContext4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf9c6f69_bea1_4bf4_8e74_ae03e206c6b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestRateAndReviewAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRateAndReviewAppAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetInstallOrderForAssociatedStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, items: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetInstallOrderForAssociatedStoreQueueItemsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContextStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreContextStatics {
    type Vtable = IStoreContextStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreContextStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c06ee5f_15c0_4e72_9330_d6191cebd19c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContextStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreImage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreImage {
    type Vtable = IStoreImage_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreImage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x081fd248_adb4_4b64_a993_784789926ed5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreImage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub ImagePurposeTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreLicense(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreLicense {
    type Vtable = IStoreLicense_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreLicense {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26dc9579_4c4f_4f30_bc89_649f60e36055);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreLicense_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SkuStoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InAppOfferToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageInstallOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePackageInstallOptions {
    type Vtable = IStorePackageInstallOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePackageInstallOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d3d630c_0ccd_44dd_8c59_80810a729973);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageInstallOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageLicense(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePackageLicense {
    type Vtable = IStorePackageLicense_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePackageLicense {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c465714_14e1_4973_bd14_f77724271e99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageLicense_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LicenseLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LicenseLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLicenseLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLicenseLost: usize,
    #[cfg(feature = "ApplicationModel")]
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Package: usize,
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ReleaseLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageUpdate(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePackageUpdate {
    type Vtable = IStorePackageUpdate_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePackageUpdate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x140fa150_3cbf_4a35_b91f_48271c31b072);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageUpdate_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Package: usize,
    pub Mandatory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageUpdateResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePackageUpdateResult {
    type Vtable = IStorePackageUpdateResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePackageUpdateResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79142ed_61f9_4893_b4fe_cf191603af7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageUpdateResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OverallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorePackageUpdateState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StorePackageUpdateStatuses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorePackageUpdateStatuses: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageUpdateResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePackageUpdateResult2 {
    type Vtable = IStorePackageUpdateResult2_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePackageUpdateResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x071d012e_bc62_4f2e_87ea_99d801aeaf98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageUpdateResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub StoreQueueItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StoreQueueItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePrice(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePrice {
    type Vtable = IStorePrice_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePrice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55ba94c4_15f1_407c_8f06_006380f4df0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePrice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FormattedBasePrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FormattedPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsOnSale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaleEndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaleEndDate: usize,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FormattedRecurrencePrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProduct(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreProduct {
    type Vtable = IStoreProduct_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreProduct {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x320e2c52_d760_450a_a42b_67d1e901ac90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProduct_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProductKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HasDigitalDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Images: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Images: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Videos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Videos: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Skus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Skus: usize,
    pub IsInUserCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Price: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LinkUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LinkUri: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsAnySkuInstalledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsAnySkuInstalledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepurchaseproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
    pub InAppOfferToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreProductOptions {
    type Vtable = IStoreProductOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreProductOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b34a0f9_a113_4811_8326_16199c927f31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ActionFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActionFilters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductPagedQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreProductPagedQueryResult {
    type Vtable = IStoreProductPagedQueryResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreProductPagedQueryResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc92718c5_4dd5_4869_a462_ecc6872e43c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductPagedQueryResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Products: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Products: usize,
    pub HasMoreResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetNextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetNextAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreProductQueryResult {
    type Vtable = IStoreProductQueryResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreProductQueryResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd805e6c5_d456_4ff6_8049_9076d5165f73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductQueryResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Products: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Products: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreProductResult {
    type Vtable = IStoreProductResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreProductResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7674f73_3c87_4ee1_8201_f428359bd3af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Product: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePurchaseProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePurchaseProperties {
    type Vtable = IStorePurchaseProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePurchaseProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x836278f3_ff87_4364_a5b4_fd2153ebe43b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePurchaseProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePurchasePropertiesFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePurchasePropertiesFactory {
    type Vtable = IStorePurchasePropertiesFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePurchasePropertiesFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa768f59e_fefd_489f_9a17_22a593e68b9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePurchasePropertiesFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePurchaseResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePurchaseResult {
    type Vtable = IStorePurchaseResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePurchaseResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadd28552_f96a_463d_a7bb_c20b4fca6952);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePurchaseResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorePurchaseStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreQueueItem {
    type Vtable = IStoreQueueItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreQueueItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56d5c32b_f830_4293_9188_cad2dcde7357);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InstallKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemKind) -> ::windows::core::HRESULT,
    pub GetCurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItem2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreQueueItem2 {
    type Vtable = IStoreQueueItem2_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreQueueItem2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69491ca8_1ad4_447c_ad8c_a95035f64d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItem2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CancelInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PauseInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeInstallAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItemCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreQueueItemCompletedEventArgs {
    type Vtable = IStoreQueueItemCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreQueueItemCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1247df6c_b44a_439b_bb07_1d3003d005c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItemCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItemStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreQueueItemStatus {
    type Vtable = IStoreQueueItemStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreQueueItemStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bd6796f_9cc3_4ec3_b2ef_7be433b30174);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItemStatus_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PackageInstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemState) -> ::windows::core::HRESULT,
    pub PackageInstallExtendedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemExtendedState) -> ::windows::core::HRESULT,
    pub UpdateStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::ManuallyDrop<StorePackageUpdateStatus>) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreRateAndReviewResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreRateAndReviewResult {
    type Vtable = IStoreRateAndReviewResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreRateAndReviewResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d209d56_a6b5_4121_9b61_ee6d0fbdbdbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreRateAndReviewResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WasUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreRateAndReviewStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreRequestHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreRequestHelperStatics {
    type Vtable = IStoreRequestHelperStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreRequestHelperStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ce5e5f9_a0c9_4b2c_96a6_a171c630038d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreRequestHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, requestkind: u32, parametersasjson: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendRequestAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSendRequestResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreSendRequestResult {
    type Vtable = IStoreSendRequestResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreSendRequestResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc73abe60_8272_4502_8a69_6e75153a4299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSendRequestResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSendRequestResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreSendRequestResult2 {
    type Vtable = IStoreSendRequestResult2_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreSendRequestResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2901296f_c0b0_49d0_8e8d_aa940af9c10b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSendRequestResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Http")]
    pub HttpStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Web::Http::HttpStatusCode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpStatusCode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSku(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreSku {
    type Vtable = IStoreSku_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreSku {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x397e6f55_4440_4f03_863c_91f3fec83d79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSku_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CustomDeveloperData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Images: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Images: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Videos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Videos: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Availabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Availabilities: usize,
    pub Price: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsInUserCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BundledSkus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BundledSkus: usize,
    pub CollectionData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetIsInstalledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsInstalledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepurchaseproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
    pub IsSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SubscriptionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSubscriptionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreSubscriptionInfo {
    type Vtable = IStoreSubscriptionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreSubscriptionInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4189776a_0559_43ac_a9c6_3ab0011fb8eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSubscriptionInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BillingPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub BillingPeriodUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreDurationUnit) -> ::windows::core::HRESULT,
    pub HasTrialPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TrialPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TrialPeriodUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreDurationUnit) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreUninstallStorePackageResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreUninstallStorePackageResult {
    type Vtable = IStoreUninstallStorePackageResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreUninstallStorePackageResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fca39fd_126f_4cda_b801_1346b8d0a260);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreUninstallStorePackageResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreUninstallStorePackageStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreVideo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreVideo {
    type Vtable = IStoreVideo_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreVideo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf26cb184_6f5e_4dc2_886c_3c63083c2f94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreVideo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub VideoPurposeTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PreviewImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreAcquireLicenseResult(::windows::core::IUnknown);
impl StoreAcquireLicenseResult {
    pub fn StorePackageLicense(&self) -> ::windows::core::Result<StorePackageLicense> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StorePackageLicense)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreAcquireLicenseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreAcquireLicenseResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreAcquireLicenseResult;{fbd7946d-f040-4cb3-9a39-29bcecdbe22d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreAcquireLicenseResult {
    type Vtable = IStoreAcquireLicenseResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreAcquireLicenseResult {
    const IID: ::windows::core::GUID = <IStoreAcquireLicenseResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreAcquireLicenseResult {
    const NAME: &'static str = "Windows.Services.Store.StoreAcquireLicenseResult";
}
::windows::core::interface_hierarchy!(StoreAcquireLicenseResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreAcquireLicenseResult {}
unsafe impl ::core::marker::Sync for StoreAcquireLicenseResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreAppLicense(::windows::core::IUnknown);
impl StoreAppLicense {
    pub fn SkuStoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SkuStoreId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsActive)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsTrial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTrial)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedJsonData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddOnLicenses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreLicense>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddOnLicenses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrialTimeRemaining(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrialTimeRemaining)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsTrialOwnedByThisUser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTrialOwnedByThisUser)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TrialUniqueId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrialUniqueId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDiscLicense(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStoreAppLicense2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDiscLicense)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreAppLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreAppLicense {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreAppLicense;{f389f9de-73c0-45ce-9bab-b2fe3e5eafd3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreAppLicense {
    type Vtable = IStoreAppLicense_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreAppLicense {
    const IID: ::windows::core::GUID = <IStoreAppLicense as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreAppLicense {
    const NAME: &'static str = "Windows.Services.Store.StoreAppLicense";
}
::windows::core::interface_hierarchy!(StoreAppLicense, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreAppLicense {}
unsafe impl ::core::marker::Sync for StoreAppLicense {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreAvailability(::windows::core::IUnknown);
impl StoreAvailability {
    pub fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StoreId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EndDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Price(&self) -> ::windows::core::Result<StorePrice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Price)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedJsonData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestPurchaseAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseWithPurchasePropertiesAsync(&self, storepurchaseproperties: &StorePurchaseProperties) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(storepurchaseproperties), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreAvailability {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreAvailability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreAvailability;{fa060325-0ffd-4493-ad43-f1f9918f69fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreAvailability {
    type Vtable = IStoreAvailability_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreAvailability {
    const IID: ::windows::core::GUID = <IStoreAvailability as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreAvailability {
    const NAME: &'static str = "Windows.Services.Store.StoreAvailability";
}
::windows::core::interface_hierarchy!(StoreAvailability, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreAvailability {}
unsafe impl ::core::marker::Sync for StoreAvailability {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreCanAcquireLicenseResult(::windows::core::IUnknown);
impl StoreCanAcquireLicenseResult {
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LicensableSku(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LicensableSku)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<StoreCanLicenseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreCanAcquireLicenseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreCanAcquireLicenseResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreCanAcquireLicenseResult;{3a693db3-0088-482f-86d5-bd46522663ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreCanAcquireLicenseResult {
    type Vtable = IStoreCanAcquireLicenseResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreCanAcquireLicenseResult {
    const IID: ::windows::core::GUID = <IStoreCanAcquireLicenseResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreCanAcquireLicenseResult {
    const NAME: &'static str = "Windows.Services.Store.StoreCanAcquireLicenseResult";
}
::windows::core::interface_hierarchy!(StoreCanAcquireLicenseResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreCanAcquireLicenseResult {}
unsafe impl ::core::marker::Sync for StoreCanAcquireLicenseResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreCollectionData(::windows::core::IUnknown);
impl StoreCollectionData {
    pub fn IsTrial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTrial)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CampaignId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeveloperOfferId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeveloperOfferId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcquiredDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AcquiredDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EndDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrialTimeRemaining(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrialTimeRemaining)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedJsonData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreCollectionData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreCollectionData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreCollectionData;{8aa4c3b3-5bb3-441a-2ab4-4dab73d5ce67})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreCollectionData {
    type Vtable = IStoreCollectionData_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreCollectionData {
    const IID: ::windows::core::GUID = <IStoreCollectionData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreCollectionData {
    const NAME: &'static str = "Windows.Services.Store.StoreCollectionData";
}
::windows::core::interface_hierarchy!(StoreCollectionData, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreCollectionData {}
unsafe impl ::core::marker::Sync for StoreCollectionData {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreConsumableResult(::windows::core::IUnknown);
impl StoreConsumableResult {
    pub fn Status(&self) -> ::windows::core::Result<StoreConsumableStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TrackingId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrackingId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BalanceRemaining(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BalanceRemaining)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreConsumableResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreConsumableResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreConsumableResult;{ea5dab72-6a00-4052-be5b-bfdab4433352})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreConsumableResult {
    type Vtable = IStoreConsumableResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreConsumableResult {
    const IID: ::windows::core::GUID = <IStoreConsumableResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreConsumableResult {
    const NAME: &'static str = "Windows.Services.Store.StoreConsumableResult";
}
::windows::core::interface_hierarchy!(StoreConsumableResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreConsumableResult {}
unsafe impl ::core::marker::Sync for StoreConsumableResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreContext(::windows::core::IUnknown);
impl StoreContext {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OfflineLicensesChanged(&self, handler: &super::super::Foundation::TypedEventHandler<StoreContext, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OfflineLicensesChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOfflineLicensesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveOfflineLicensesChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCustomerPurchaseIdAsync(&self, serviceticket: &::windows::core::HSTRING, publisheruserid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCustomerPurchaseIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(serviceticket), ::core::mem::transmute_copy(publisheruserid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCustomerCollectionsIdAsync(&self, serviceticket: &::windows::core::HSTRING, publisheruserid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCustomerCollectionsIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(serviceticket), ::core::mem::transmute_copy(publisheruserid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppLicenseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreAppLicense>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAppLicenseAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetStoreProductForCurrentAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStoreProductForCurrentAppAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStoreProductsAsync<P0, E0, P1, E1>(&self, productkinds: P0, storeids: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStoreProductsAsync)(::windows::core::Vtable::as_raw(this), productkinds.try_into().map_err(|e| e.into())?.abi(), storeids.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAssociatedStoreProductsAsync<P0, E0>(&self, productkinds: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAssociatedStoreProductsAsync)(::windows::core::Vtable::as_raw(this), productkinds.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAssociatedStoreProductsWithPagingAsync<P0, E0>(&self, productkinds: P0, maxitemstoretrieveperpage: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAssociatedStoreProductsWithPagingAsync)(::windows::core::Vtable::as_raw(this), productkinds.try_into().map_err(|e| e.into())?.abi(), maxitemstoretrieveperpage, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUserCollectionAsync<P0, E0>(&self, productkinds: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUserCollectionAsync)(::windows::core::Vtable::as_raw(this), productkinds.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUserCollectionWithPagingAsync<P0, E0>(&self, productkinds: P0, maxitemstoretrieveperpage: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUserCollectionWithPagingAsync)(::windows::core::Vtable::as_raw(this), productkinds.try_into().map_err(|e| e.into())?.abi(), maxitemstoretrieveperpage, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportConsumableFulfillmentAsync(&self, productstoreid: &::windows::core::HSTRING, quantity: u32, trackingid: ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreConsumableResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportConsumableFulfillmentAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productstoreid), quantity, trackingid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConsumableBalanceRemainingAsync(&self, productstoreid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreConsumableResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetConsumableBalanceRemainingAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productstoreid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn AcquireStoreLicenseForOptionalPackageAsync(&self, optionalpackage: &super::super::ApplicationModel::Package) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreAcquireLicenseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AcquireStoreLicenseForOptionalPackageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(optionalpackage), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseAsync(&self, storeid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestPurchaseAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(storeid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseWithPurchasePropertiesAsync(&self, storeid: &::windows::core::HSTRING, storepurchaseproperties: &StorePurchaseProperties) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(storeid), ::core::mem::transmute_copy(storepurchaseproperties), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppAndOptionalStorePackageUpdatesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StorePackageUpdate>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAppAndOptionalStorePackageUpdatesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestDownloadStorePackageUpdatesAsync<P0, E0>(&self, storepackageupdates: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestDownloadStorePackageUpdatesAsync)(::windows::core::Vtable::as_raw(this), storepackageupdates.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestDownloadAndInstallStorePackageUpdatesAsync<P0, E0>(&self, storepackageupdates: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestDownloadAndInstallStorePackageUpdatesAsync)(::windows::core::Vtable::as_raw(this), storepackageupdates.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestDownloadAndInstallStorePackagesAsync<P0, E0>(&self, storeids: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestDownloadAndInstallStorePackagesAsync)(::windows::core::Vtable::as_raw(this), storeids.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindStoreProductForPackageAsync<P0, E0>(&self, productkinds: P0, package: &super::super::ApplicationModel::Package) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStoreContext2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindStoreProductForPackageAsync)(::windows::core::Vtable::as_raw(this), productkinds.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(package), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanSilentlyDownloadStorePackageUpdates(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanSilentlyDownloadStorePackageUpdates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrySilentDownloadStorePackageUpdatesAsync<P0, E0>(&self, storepackageupdates: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySilentDownloadStorePackageUpdatesAsync)(::windows::core::Vtable::as_raw(this), storepackageupdates.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrySilentDownloadAndInstallStorePackageUpdatesAsync<P0, E0>(&self, storepackageupdates: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySilentDownloadAndInstallStorePackageUpdatesAsync)(::windows::core::Vtable::as_raw(this), storepackageupdates.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn CanAcquireStoreLicenseForOptionalPackageAsync(&self, optionalpackage: &super::super::ApplicationModel::Package) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreCanAcquireLicenseResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanAcquireStoreLicenseForOptionalPackageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(optionalpackage), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CanAcquireStoreLicenseAsync(&self, productstoreid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreCanAcquireLicenseResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanAcquireStoreLicenseAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productstoreid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStoreProductsWithOptionsAsync<P0, E0, P1, E1>(&self, productkinds: P0, storeids: P1, storeproductoptions: &StoreProductOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStoreProductsWithOptionsAsync)(::windows::core::Vtable::as_raw(this), productkinds.try_into().map_err(|e| e.into())?.abi(), storeids.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(storeproductoptions), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAssociatedStoreQueueItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAssociatedStoreQueueItemsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStoreQueueItemsAsync<P0, E0>(&self, storeids: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStoreQueueItemsAsync)(::windows::core::Vtable::as_raw(this), storeids.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync<P0, E0>(&self, storeids: P0, storepackageinstalloptions: &StorePackageInstallOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync)(::windows::core::Vtable::as_raw(this), storeids.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(storepackageinstalloptions), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DownloadAndInstallStorePackagesAsync<P0, E0>(&self, storeids: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DownloadAndInstallStorePackagesAsync)(::windows::core::Vtable::as_raw(this), storeids.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn RequestUninstallStorePackageAsync(&self, package: &super::super::ApplicationModel::Package) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestUninstallStorePackageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(package), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUninstallStorePackageByStoreIdAsync(&self, storeid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestUninstallStorePackageByStoreIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(storeid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn UninstallStorePackageAsync(&self, package: &super::super::ApplicationModel::Package) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UninstallStorePackageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(package), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UninstallStorePackageByStoreIdAsync(&self, storeid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UninstallStorePackageByStoreIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(storeid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestRateAndReviewAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreRateAndReviewResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestRateAndReviewAppAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetInstallOrderForAssociatedStoreQueueItemsAsync<P0, E0>(&self, items: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<StoreQueueItem>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStoreContext4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetInstallOrderForAssociatedStoreQueueItemsAsync)(::windows::core::Vtable::as_raw(this), items.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<StoreContext> {
        Self::IStoreContextStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<StoreContext> {
        Self::IStoreContextStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStoreContextStatics<R, F: FnOnce(&IStoreContextStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StoreContext, IStoreContextStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for StoreContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreContext;{ac98b6be-f4fd-4912-babd-5035e5e8bcab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreContext {
    type Vtable = IStoreContext_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreContext {
    const IID: ::windows::core::GUID = <IStoreContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreContext {
    const NAME: &'static str = "Windows.Services.Store.StoreContext";
}
::windows::core::interface_hierarchy!(StoreContext, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreContext {}
unsafe impl ::core::marker::Sync for StoreContext {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreImage(::windows::core::IUnknown);
impl StoreImage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ImagePurposeTag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImagePurposeTag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Width)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Height)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Caption)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreImage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreImage;{081fd248-adb4-4b64-a993-784789926ed5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreImage {
    type Vtable = IStoreImage_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreImage {
    const IID: ::windows::core::GUID = <IStoreImage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreImage {
    const NAME: &'static str = "Windows.Services.Store.StoreImage";
}
::windows::core::interface_hierarchy!(StoreImage, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreImage {}
unsafe impl ::core::marker::Sync for StoreImage {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreLicense(::windows::core::IUnknown);
impl StoreLicense {
    pub fn SkuStoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SkuStoreId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsActive)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedJsonData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InAppOfferToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InAppOfferToken)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreLicense {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreLicense;{26dc9579-4c4f-4f30-bc89-649f60e36055})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreLicense {
    type Vtable = IStoreLicense_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreLicense {
    const IID: ::windows::core::GUID = <IStoreLicense as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreLicense {
    const NAME: &'static str = "Windows.Services.Store.StoreLicense";
}
::windows::core::interface_hierarchy!(StoreLicense, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreLicense {}
unsafe impl ::core::marker::Sync for StoreLicense {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePackageInstallOptions(::windows::core::IUnknown);
impl StorePackageInstallOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorePackageInstallOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowForcedAppRestart)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowForcedAppRestart)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for StorePackageInstallOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StorePackageInstallOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageInstallOptions;{1d3d630c-0ccd-44dd-8c59-80810a729973})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorePackageInstallOptions {
    type Vtable = IStorePackageInstallOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for StorePackageInstallOptions {
    const IID: ::windows::core::GUID = <IStorePackageInstallOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePackageInstallOptions {
    const NAME: &'static str = "Windows.Services.Store.StorePackageInstallOptions";
}
::windows::core::interface_hierarchy!(StorePackageInstallOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorePackageInstallOptions {}
unsafe impl ::core::marker::Sync for StorePackageInstallOptions {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePackageLicense(::windows::core::IUnknown);
impl StorePackageLicense {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LicenseLost(&self, handler: &super::super::Foundation::TypedEventHandler<StorePackageLicense, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LicenseLost)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLicenseLost(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveLicenseLost)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Package(&self) -> ::windows::core::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsValid(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsValid)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReleaseLicense(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ReleaseLicense)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for StorePackageLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StorePackageLicense {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageLicense;{0c465714-14e1-4973-bd14-f77724271e99})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorePackageLicense {
    type Vtable = IStorePackageLicense_Vtbl;
}
unsafe impl ::windows::core::Interface for StorePackageLicense {
    const IID: ::windows::core::GUID = <IStorePackageLicense as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePackageLicense {
    const NAME: &'static str = "Windows.Services.Store.StorePackageLicense";
}
::windows::core::interface_hierarchy!(StorePackageLicense, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<StorePackageLicense> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: StorePackageLicense) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&StorePackageLicense> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorePackageLicense) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&StorePackageLicense> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorePackageLicense) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for StorePackageLicense {}
unsafe impl ::core::marker::Sync for StorePackageLicense {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePackageUpdate(::windows::core::IUnknown);
impl StorePackageUpdate {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Package(&self) -> ::windows::core::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Mandatory(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mandatory)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StorePackageUpdate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StorePackageUpdate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageUpdate;{140fa150-3cbf-4a35-b91f-48271c31b072})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorePackageUpdate {
    type Vtable = IStorePackageUpdate_Vtbl;
}
unsafe impl ::windows::core::Interface for StorePackageUpdate {
    const IID: ::windows::core::GUID = <IStorePackageUpdate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePackageUpdate {
    const NAME: &'static str = "Windows.Services.Store.StorePackageUpdate";
}
::windows::core::interface_hierarchy!(StorePackageUpdate, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorePackageUpdate {}
unsafe impl ::core::marker::Sync for StorePackageUpdate {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePackageUpdateResult(::windows::core::IUnknown);
impl StorePackageUpdateResult {
    pub fn OverallState(&self) -> ::windows::core::Result<StorePackageUpdateState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OverallState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StorePackageUpdateStatuses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorePackageUpdateStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StorePackageUpdateStatuses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StoreQueueItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreQueueItem>> {
        let this = &::windows::core::Interface::cast::<IStorePackageUpdateResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StoreQueueItems)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StorePackageUpdateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StorePackageUpdateResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageUpdateResult;{e79142ed-61f9-4893-b4fe-cf191603af7b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorePackageUpdateResult {
    type Vtable = IStorePackageUpdateResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StorePackageUpdateResult {
    const IID: ::windows::core::GUID = <IStorePackageUpdateResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePackageUpdateResult {
    const NAME: &'static str = "Windows.Services.Store.StorePackageUpdateResult";
}
::windows::core::interface_hierarchy!(StorePackageUpdateResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorePackageUpdateResult {}
unsafe impl ::core::marker::Sync for StorePackageUpdateResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePrice(::windows::core::IUnknown);
impl StorePrice {
    pub fn FormattedBasePrice(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FormattedBasePrice)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FormattedPrice(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FormattedPrice)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsOnSale(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsOnSale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaleEndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaleEndDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CurrencyCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrencyCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FormattedRecurrencePrice(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FormattedRecurrencePrice)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StorePrice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StorePrice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePrice;{55ba94c4-15f1-407c-8f06-006380f4df0b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorePrice {
    type Vtable = IStorePrice_Vtbl;
}
unsafe impl ::windows::core::Interface for StorePrice {
    const IID: ::windows::core::GUID = <IStorePrice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePrice {
    const NAME: &'static str = "Windows.Services.Store.StorePrice";
}
::windows::core::interface_hierarchy!(StorePrice, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorePrice {}
unsafe impl ::core::marker::Sync for StorePrice {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreProduct(::windows::core::IUnknown);
impl StoreProduct {
    pub fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StoreId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProductKind(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProductKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasDigitalDownload(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasDigitalDownload)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Keywords)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Images(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreImage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Images)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Videos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreVideo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Videos)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Skus(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreSku>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Skus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsInUserCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInUserCollection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Price(&self) -> ::windows::core::Result<StorePrice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Price)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedJsonData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LinkUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LinkUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsAnySkuInstalledAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetIsAnySkuInstalledAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestPurchaseAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseWithPurchasePropertiesAsync(&self, storepurchaseproperties: &StorePurchaseProperties) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(storepurchaseproperties), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InAppOfferToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InAppOfferToken)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProduct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreProduct {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProduct;{320e2c52-d760-450a-a42b-67d1e901ac90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreProduct {
    type Vtable = IStoreProduct_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreProduct {
    const IID: ::windows::core::GUID = <IStoreProduct as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreProduct {
    const NAME: &'static str = "Windows.Services.Store.StoreProduct";
}
::windows::core::interface_hierarchy!(StoreProduct, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreProduct {}
unsafe impl ::core::marker::Sync for StoreProduct {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreProductOptions(::windows::core::IUnknown);
impl StoreProductOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StoreProductOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActionFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActionFilters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProductOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreProductOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductOptions;{5b34a0f9-a113-4811-8326-16199c927f31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreProductOptions {
    type Vtable = IStoreProductOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreProductOptions {
    const IID: ::windows::core::GUID = <IStoreProductOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreProductOptions {
    const NAME: &'static str = "Windows.Services.Store.StoreProductOptions";
}
::windows::core::interface_hierarchy!(StoreProductOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreProductOptions {}
unsafe impl ::core::marker::Sync for StoreProductOptions {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreProductPagedQueryResult(::windows::core::IUnknown);
impl StoreProductPagedQueryResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Products(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreProduct>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Products)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasMoreResults(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasMoreResults)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetNextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNextAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProductPagedQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreProductPagedQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductPagedQueryResult;{c92718c5-4dd5-4869-a462-ecc6872e43c5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreProductPagedQueryResult {
    type Vtable = IStoreProductPagedQueryResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreProductPagedQueryResult {
    const IID: ::windows::core::GUID = <IStoreProductPagedQueryResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreProductPagedQueryResult {
    const NAME: &'static str = "Windows.Services.Store.StoreProductPagedQueryResult";
}
::windows::core::interface_hierarchy!(StoreProductPagedQueryResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreProductPagedQueryResult {}
unsafe impl ::core::marker::Sync for StoreProductPagedQueryResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreProductQueryResult(::windows::core::IUnknown);
impl StoreProductQueryResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Products(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreProduct>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Products)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProductQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreProductQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductQueryResult;{d805e6c5-d456-4ff6-8049-9076d5165f73})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreProductQueryResult {
    type Vtable = IStoreProductQueryResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreProductQueryResult {
    const IID: ::windows::core::GUID = <IStoreProductQueryResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreProductQueryResult {
    const NAME: &'static str = "Windows.Services.Store.StoreProductQueryResult";
}
::windows::core::interface_hierarchy!(StoreProductQueryResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreProductQueryResult {}
unsafe impl ::core::marker::Sync for StoreProductQueryResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreProductResult(::windows::core::IUnknown);
impl StoreProductResult {
    pub fn Product(&self) -> ::windows::core::Result<StoreProduct> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Product)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProductResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreProductResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductResult;{b7674f73-3c87-4ee1-8201-f428359bd3af})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreProductResult {
    type Vtable = IStoreProductResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreProductResult {
    const IID: ::windows::core::GUID = <IStoreProductResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreProductResult {
    const NAME: &'static str = "Windows.Services.Store.StoreProductResult";
}
::windows::core::interface_hierarchy!(StoreProductResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreProductResult {}
unsafe impl ::core::marker::Sync for StoreProductResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePurchaseProperties(::windows::core::IUnknown);
impl StorePurchaseProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorePurchaseProperties, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedJsonData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetExtendedJsonData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetExtendedJsonData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(name: &::windows::core::HSTRING) -> ::windows::core::Result<StorePurchaseProperties> {
        Self::IStorePurchasePropertiesFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorePurchasePropertiesFactory<R, F: FnOnce(&IStorePurchasePropertiesFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorePurchaseProperties, IStorePurchasePropertiesFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for StorePurchaseProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StorePurchaseProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePurchaseProperties;{836278f3-ff87-4364-a5b4-fd2153ebe43b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorePurchaseProperties {
    type Vtable = IStorePurchaseProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for StorePurchaseProperties {
    const IID: ::windows::core::GUID = <IStorePurchaseProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePurchaseProperties {
    const NAME: &'static str = "Windows.Services.Store.StorePurchaseProperties";
}
::windows::core::interface_hierarchy!(StorePurchaseProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorePurchaseProperties {}
unsafe impl ::core::marker::Sync for StorePurchaseProperties {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePurchaseResult(::windows::core::IUnknown);
impl StorePurchaseResult {
    pub fn Status(&self) -> ::windows::core::Result<StorePurchaseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StorePurchaseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StorePurchaseResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePurchaseResult;{add28552-f96a-463d-a7bb-c20b4fca6952})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorePurchaseResult {
    type Vtable = IStorePurchaseResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StorePurchaseResult {
    const IID: ::windows::core::GUID = <IStorePurchaseResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePurchaseResult {
    const NAME: &'static str = "Windows.Services.Store.StorePurchaseResult";
}
::windows::core::interface_hierarchy!(StorePurchaseResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorePurchaseResult {}
unsafe impl ::core::marker::Sync for StorePurchaseResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreQueueItem(::windows::core::IUnknown);
impl StoreQueueItem {
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProductId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageFamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InstallKind(&self) -> ::windows::core::Result<StoreQueueItemKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstallKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetCurrentStatus(&self) -> ::windows::core::Result<StoreQueueItemStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self, handler: &super::super::Foundation::TypedEventHandler<StoreQueueItem, StoreQueueItemCompletedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Completed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged(&self, handler: &super::super::Foundation::TypedEventHandler<StoreQueueItem, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStatusChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CancelInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStoreQueueItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CancelInstallAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PauseInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStoreQueueItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PauseInstallAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStoreQueueItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResumeInstallAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreQueueItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreQueueItem;{56d5c32b-f830-4293-9188-cad2dcde7357})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreQueueItem {
    type Vtable = IStoreQueueItem_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreQueueItem {
    const IID: ::windows::core::GUID = <IStoreQueueItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreQueueItem {
    const NAME: &'static str = "Windows.Services.Store.StoreQueueItem";
}
::windows::core::interface_hierarchy!(StoreQueueItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreQueueItem {}
unsafe impl ::core::marker::Sync for StoreQueueItem {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreQueueItemCompletedEventArgs(::windows::core::IUnknown);
impl StoreQueueItemCompletedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<StoreQueueItemStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreQueueItemCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItemCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreQueueItemCompletedEventArgs;{1247df6c-b44a-439b-bb07-1d3003d005c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreQueueItemCompletedEventArgs {
    type Vtable = IStoreQueueItemCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreQueueItemCompletedEventArgs {
    const IID: ::windows::core::GUID = <IStoreQueueItemCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreQueueItemCompletedEventArgs {
    const NAME: &'static str = "Windows.Services.Store.StoreQueueItemCompletedEventArgs";
}
::windows::core::interface_hierarchy!(StoreQueueItemCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreQueueItemCompletedEventArgs {}
unsafe impl ::core::marker::Sync for StoreQueueItemCompletedEventArgs {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreQueueItemStatus(::windows::core::IUnknown);
impl StoreQueueItemStatus {
    pub fn PackageInstallState(&self) -> ::windows::core::Result<StoreQueueItemState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageInstallState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PackageInstallExtendedState(&self) -> ::windows::core::Result<StoreQueueItemExtendedState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageInstallExtendedState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UpdateStatus(&self) -> ::windows::core::Result<StorePackageUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreQueueItemStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItemStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreQueueItemStatus;{9bd6796f-9cc3-4ec3-b2ef-7be433b30174})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreQueueItemStatus {
    type Vtable = IStoreQueueItemStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreQueueItemStatus {
    const IID: ::windows::core::GUID = <IStoreQueueItemStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreQueueItemStatus {
    const NAME: &'static str = "Windows.Services.Store.StoreQueueItemStatus";
}
::windows::core::interface_hierarchy!(StoreQueueItemStatus, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreQueueItemStatus {}
unsafe impl ::core::marker::Sync for StoreQueueItemStatus {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreRateAndReviewResult(::windows::core::IUnknown);
impl StoreRateAndReviewResult {
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedJsonData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn WasUpdated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WasUpdated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<StoreRateAndReviewStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreRateAndReviewResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreRateAndReviewResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreRateAndReviewResult;{9d209d56-a6b5-4121-9b61-ee6d0fbdbdbb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreRateAndReviewResult {
    type Vtable = IStoreRateAndReviewResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreRateAndReviewResult {
    const IID: ::windows::core::GUID = <IStoreRateAndReviewResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreRateAndReviewResult {
    const NAME: &'static str = "Windows.Services.Store.StoreRateAndReviewResult";
}
::windows::core::interface_hierarchy!(StoreRateAndReviewResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreRateAndReviewResult {}
unsafe impl ::core::marker::Sync for StoreRateAndReviewResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
pub struct StoreRequestHelper;
impl StoreRequestHelper {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestAsync(context: &StoreContext, requestkind: u32, parametersasjson: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreSendRequestResult>> {
        Self::IStoreRequestHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendRequestAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(context), requestkind, ::core::mem::transmute_copy(parametersasjson), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStoreRequestHelperStatics<R, F: FnOnce(&IStoreRequestHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StoreRequestHelper, IStoreRequestHelperStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for StoreRequestHelper {
    const NAME: &'static str = "Windows.Services.Store.StoreRequestHelper";
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreSendRequestResult(::windows::core::IUnknown);
impl StoreSendRequestResult {
    pub fn Response(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Response)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn HttpStatusCode(&self) -> ::windows::core::Result<super::super::Web::Http::HttpStatusCode> {
        let this = &::windows::core::Interface::cast::<IStoreSendRequestResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HttpStatusCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreSendRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreSendRequestResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreSendRequestResult;{c73abe60-8272-4502-8a69-6e75153a4299})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreSendRequestResult {
    type Vtable = IStoreSendRequestResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreSendRequestResult {
    const IID: ::windows::core::GUID = <IStoreSendRequestResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreSendRequestResult {
    const NAME: &'static str = "Windows.Services.Store.StoreSendRequestResult";
}
::windows::core::interface_hierarchy!(StoreSendRequestResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreSendRequestResult {}
unsafe impl ::core::marker::Sync for StoreSendRequestResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreSku(::windows::core::IUnknown);
impl StoreSku {
    pub fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StoreId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsTrial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTrial)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CustomDeveloperData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomDeveloperData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Images(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreImage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Images)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Videos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreVideo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Videos)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Availabilities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreAvailability>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Availabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Price(&self) -> ::windows::core::Result<StorePrice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Price)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedJsonData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsInUserCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInUserCollection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BundledSkus(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BundledSkus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CollectionData(&self) -> ::windows::core::Result<StoreCollectionData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CollectionData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsInstalledAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetIsInstalledAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestPurchaseAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseWithPurchasePropertiesAsync(&self, storepurchaseproperties: &StorePurchaseProperties) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(storepurchaseproperties), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSubscription(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSubscription)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SubscriptionInfo(&self) -> ::windows::core::Result<StoreSubscriptionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubscriptionInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreSku {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreSku {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreSku;{397e6f55-4440-4f03-863c-91f3fec83d79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreSku {
    type Vtable = IStoreSku_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreSku {
    const IID: ::windows::core::GUID = <IStoreSku as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreSku {
    const NAME: &'static str = "Windows.Services.Store.StoreSku";
}
::windows::core::interface_hierarchy!(StoreSku, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreSku {}
unsafe impl ::core::marker::Sync for StoreSku {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreSubscriptionInfo(::windows::core::IUnknown);
impl StoreSubscriptionInfo {
    pub fn BillingPeriod(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BillingPeriod)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BillingPeriodUnit(&self) -> ::windows::core::Result<StoreDurationUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BillingPeriodUnit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasTrialPeriod(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasTrialPeriod)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TrialPeriod(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrialPeriod)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TrialPeriodUnit(&self) -> ::windows::core::Result<StoreDurationUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrialPeriodUnit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreSubscriptionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreSubscriptionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreSubscriptionInfo;{4189776a-0559-43ac-a9c6-3ab0011fb8eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreSubscriptionInfo {
    type Vtable = IStoreSubscriptionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreSubscriptionInfo {
    const IID: ::windows::core::GUID = <IStoreSubscriptionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreSubscriptionInfo {
    const NAME: &'static str = "Windows.Services.Store.StoreSubscriptionInfo";
}
::windows::core::interface_hierarchy!(StoreSubscriptionInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreSubscriptionInfo {}
unsafe impl ::core::marker::Sync for StoreSubscriptionInfo {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreUninstallStorePackageResult(::windows::core::IUnknown);
impl StoreUninstallStorePackageResult {
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<StoreUninstallStorePackageStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreUninstallStorePackageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreUninstallStorePackageResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreUninstallStorePackageResult;{9fca39fd-126f-4cda-b801-1346b8d0a260})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreUninstallStorePackageResult {
    type Vtable = IStoreUninstallStorePackageResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreUninstallStorePackageResult {
    const IID: ::windows::core::GUID = <IStoreUninstallStorePackageResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreUninstallStorePackageResult {
    const NAME: &'static str = "Windows.Services.Store.StoreUninstallStorePackageResult";
}
::windows::core::interface_hierarchy!(StoreUninstallStorePackageResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreUninstallStorePackageResult {}
unsafe impl ::core::marker::Sync for StoreUninstallStorePackageResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreVideo(::windows::core::IUnknown);
impl StoreVideo {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn VideoPurposeTag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VideoPurposeTag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Width)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Height)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Caption)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviewImage(&self) -> ::windows::core::Result<StoreImage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviewImage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreVideo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreVideo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreVideo;{f26cb184-6f5e-4dc2-886c-3c63083c2f94})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreVideo {
    type Vtable = IStoreVideo_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreVideo {
    const IID: ::windows::core::GUID = <IStoreVideo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreVideo {
    const NAME: &'static str = "Windows.Services.Store.StoreVideo";
}
::windows::core::interface_hierarchy!(StoreVideo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreVideo {}
unsafe impl ::core::marker::Sync for StoreVideo {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreCanLicenseStatus(pub i32);
impl StoreCanLicenseStatus {
    pub const NotLicensableToUser: Self = Self(0i32);
    pub const Licensable: Self = Self(1i32);
    pub const LicenseActionNotApplicableToProduct: Self = Self(2i32);
    pub const NetworkError: Self = Self(3i32);
    pub const ServerError: Self = Self(4i32);
}
impl ::core::marker::Copy for StoreCanLicenseStatus {}
impl ::core::clone::Clone for StoreCanLicenseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StoreCanLicenseStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StoreCanLicenseStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreCanLicenseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreConsumableStatus(pub i32);
impl StoreConsumableStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const InsufficentQuantity: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
impl ::core::marker::Copy for StoreConsumableStatus {}
impl ::core::clone::Clone for StoreConsumableStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StoreConsumableStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StoreConsumableStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreConsumableStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreDurationUnit(pub i32);
impl StoreDurationUnit {
    pub const Minute: Self = Self(0i32);
    pub const Hour: Self = Self(1i32);
    pub const Day: Self = Self(2i32);
    pub const Week: Self = Self(3i32);
    pub const Month: Self = Self(4i32);
    pub const Year: Self = Self(5i32);
}
impl ::core::marker::Copy for StoreDurationUnit {}
impl ::core::clone::Clone for StoreDurationUnit {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StoreDurationUnit {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StoreDurationUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreDurationUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorePackageUpdateState(pub i32);
impl StorePackageUpdateState {
    pub const Pending: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Deploying: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const OtherError: Self = Self(5i32);
    pub const ErrorLowBattery: Self = Self(6i32);
    pub const ErrorWiFiRecommended: Self = Self(7i32);
    pub const ErrorWiFiRequired: Self = Self(8i32);
}
impl ::core::marker::Copy for StorePackageUpdateState {}
impl ::core::clone::Clone for StorePackageUpdateState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StorePackageUpdateState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorePackageUpdateState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StorePackageUpdateState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorePurchaseStatus(pub i32);
impl StorePurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotPurchased: Self = Self(2i32);
    pub const NetworkError: Self = Self(3i32);
    pub const ServerError: Self = Self(4i32);
}
impl ::core::marker::Copy for StorePurchaseStatus {}
impl ::core::clone::Clone for StorePurchaseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StorePurchaseStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorePurchaseStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StorePurchaseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreQueueItemExtendedState(pub i32);
impl StoreQueueItemExtendedState {
    pub const ActivePending: Self = Self(0i32);
    pub const ActiveStarting: Self = Self(1i32);
    pub const ActiveAcquiringLicense: Self = Self(2i32);
    pub const ActiveDownloading: Self = Self(3i32);
    pub const ActiveRestoringData: Self = Self(4i32);
    pub const ActiveInstalling: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedPackagesInUse: Self = Self(10i32);
    pub const PausedLowBattery: Self = Self(11i32);
    pub const PausedWiFiRecommended: Self = Self(12i32);
    pub const PausedWiFiRequired: Self = Self(13i32);
    pub const PausedReadyToInstall: Self = Self(14i32);
}
impl ::core::marker::Copy for StoreQueueItemExtendedState {}
impl ::core::clone::Clone for StoreQueueItemExtendedState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StoreQueueItemExtendedState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItemExtendedState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreQueueItemExtendedState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreQueueItemKind(pub i32);
impl StoreQueueItemKind {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
impl ::core::marker::Copy for StoreQueueItemKind {}
impl ::core::clone::Clone for StoreQueueItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StoreQueueItemKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItemKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreQueueItemKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreQueueItemState(pub i32);
impl StoreQueueItemState {
    pub const Active: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for StoreQueueItemState {}
impl ::core::clone::Clone for StoreQueueItemState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StoreQueueItemState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItemState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreQueueItemState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreRateAndReviewStatus(pub i32);
impl StoreRateAndReviewStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const CanceledByUser: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
}
impl ::core::marker::Copy for StoreRateAndReviewStatus {}
impl ::core::clone::Clone for StoreRateAndReviewStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StoreRateAndReviewStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StoreRateAndReviewStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreRateAndReviewStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreUninstallStorePackageStatus(pub i32);
impl StoreUninstallStorePackageStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const CanceledByUser: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const UninstallNotApplicable: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl ::core::marker::Copy for StoreUninstallStorePackageStatus {}
impl ::core::clone::Clone for StoreUninstallStorePackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StoreUninstallStorePackageStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StoreUninstallStorePackageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreUninstallStorePackageStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Services_Store\"`*"]
pub struct StorePackageUpdateStatus {
    pub PackageFamilyName: ::windows::core::HSTRING,
    pub PackageDownloadSizeInBytes: u64,
    pub PackageBytesDownloaded: u64,
    pub PackageDownloadProgress: f64,
    pub TotalDownloadProgress: f64,
    pub PackageUpdateState: StorePackageUpdateState,
}
impl ::core::clone::Clone for StorePackageUpdateStatus {
    fn clone(&self) -> Self {
        Self {
            PackageFamilyName: self.PackageFamilyName.clone(),
            PackageDownloadSizeInBytes: self.PackageDownloadSizeInBytes,
            PackageBytesDownloaded: self.PackageBytesDownloaded,
            PackageDownloadProgress: self.PackageDownloadProgress,
            TotalDownloadProgress: self.TotalDownloadProgress,
            PackageUpdateState: self.PackageUpdateState,
        }
    }
}
unsafe impl ::windows::core::Abi for StorePackageUpdateStatus {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for StorePackageUpdateStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Services.Store.StorePackageUpdateStatus;string;u8;u8;f8;f8;enum(Windows.Services.Store.StorePackageUpdateState;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(from.clone())
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
