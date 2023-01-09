#[cfg(feature = "Win32_Security_Authentication_Identity_Provider")]
pub mod Provider;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn AcceptSecurityContext(phcredential: ::core::option::Option<*const super::super::Credentials::SecHandle>, phcontext: ::core::option::Option<*const super::super::Credentials::SecHandle>, pinput: ::core::option::Option<*const SecBufferDesc>, fcontextreq: ASC_REQ_FLAGS, targetdatarep: u32, phnewcontext: ::core::option::Option<*mut super::super::Credentials::SecHandle>, poutput: ::core::option::Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: ::core::option::Option<*mut i64>) -> ::windows::core::HRESULT {
    ::windows::core::link ! ( "secur32.dll""system" fn AcceptSecurityContext ( phcredential : *const super::super::Credentials:: SecHandle , phcontext : *const super::super::Credentials:: SecHandle , pinput : *const SecBufferDesc , fcontextreq : ASC_REQ_FLAGS , targetdatarep : u32 , phnewcontext : *mut super::super::Credentials:: SecHandle , poutput : *mut SecBufferDesc , pfcontextattr : *mut u32 , ptsexpiry : *mut i64 ) -> :: windows::core::HRESULT );
    AcceptSecurityContext(::core::mem::transmute(phcredential.unwrap_or(::std::ptr::null())), ::core::mem::transmute(phcontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pinput.unwrap_or(::std::ptr::null())), fcontextreq, targetdatarep, ::core::mem::transmute(phnewcontext.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(poutput.unwrap_or(::std::ptr::null_mut())), pfcontextattr, ::core::mem::transmute(ptsexpiry.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn AcquireCredentialsHandleA<P0, P1>(pszprincipal: P0, pszpackage: P1, fcredentialuse: SECPKG_CRED, pvlogonid: ::core::option::Option<*const ::core::ffi::c_void>, pauthdata: ::core::option::Option<*const ::core::ffi::c_void>, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: ::core::option::Option<*const ::core::ffi::c_void>, phcredential: *mut super::super::Credentials::SecHandle, ptsexpiry: ::core::option::Option<*mut i64>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn AcquireCredentialsHandleA ( pszprincipal : :: windows::core::PCSTR , pszpackage : :: windows::core::PCSTR , fcredentialuse : SECPKG_CRED , pvlogonid : *const ::core::ffi::c_void , pauthdata : *const ::core::ffi::c_void , pgetkeyfn : SEC_GET_KEY_FN , pvgetkeyargument : *const ::core::ffi::c_void , phcredential : *mut super::super::Credentials:: SecHandle , ptsexpiry : *mut i64 ) -> :: windows::core::HRESULT );
    AcquireCredentialsHandleA(pszprincipal.into().abi(), pszpackage.into().abi(), fcredentialuse, ::core::mem::transmute(pvlogonid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pauthdata.unwrap_or(::std::ptr::null())), pgetkeyfn, ::core::mem::transmute(pvgetkeyargument.unwrap_or(::std::ptr::null())), phcredential, ::core::mem::transmute(ptsexpiry.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn AcquireCredentialsHandleW<P0, P1>(pszprincipal: P0, pszpackage: P1, fcredentialuse: SECPKG_CRED, pvlogonid: ::core::option::Option<*const ::core::ffi::c_void>, pauthdata: ::core::option::Option<*const ::core::ffi::c_void>, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: ::core::option::Option<*const ::core::ffi::c_void>, phcredential: *mut super::super::Credentials::SecHandle, ptsexpiry: ::core::option::Option<*mut i64>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn AcquireCredentialsHandleW ( pszprincipal : :: windows::core::PCWSTR , pszpackage : :: windows::core::PCWSTR , fcredentialuse : SECPKG_CRED , pvlogonid : *const ::core::ffi::c_void , pauthdata : *const ::core::ffi::c_void , pgetkeyfn : SEC_GET_KEY_FN , pvgetkeyargument : *const ::core::ffi::c_void , phcredential : *mut super::super::Credentials:: SecHandle , ptsexpiry : *mut i64 ) -> :: windows::core::HRESULT );
    AcquireCredentialsHandleW(pszprincipal.into().abi(), pszpackage.into().abi(), fcredentialuse, ::core::mem::transmute(pvlogonid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pauthdata.unwrap_or(::std::ptr::null())), pgetkeyfn, ::core::mem::transmute(pvgetkeyargument.unwrap_or(::std::ptr::null())), phcredential, ::core::mem::transmute(ptsexpiry.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn AddCredentialsA<P0, P1>(hcredentials: *const super::super::Credentials::SecHandle, pszprincipal: P0, pszpackage: P1, fcredentialuse: u32, pauthdata: ::core::option::Option<*const ::core::ffi::c_void>, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: ::core::option::Option<*const ::core::ffi::c_void>, ptsexpiry: ::core::option::Option<*mut i64>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn AddCredentialsA ( hcredentials : *const super::super::Credentials:: SecHandle , pszprincipal : :: windows::core::PCSTR , pszpackage : :: windows::core::PCSTR , fcredentialuse : u32 , pauthdata : *const ::core::ffi::c_void , pgetkeyfn : SEC_GET_KEY_FN , pvgetkeyargument : *const ::core::ffi::c_void , ptsexpiry : *mut i64 ) -> :: windows::core::HRESULT );
    AddCredentialsA(hcredentials, pszprincipal.into().abi(), pszpackage.into().abi(), fcredentialuse, ::core::mem::transmute(pauthdata.unwrap_or(::std::ptr::null())), pgetkeyfn, ::core::mem::transmute(pvgetkeyargument.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ptsexpiry.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn AddCredentialsW<P0, P1>(hcredentials: *const super::super::Credentials::SecHandle, pszprincipal: P0, pszpackage: P1, fcredentialuse: u32, pauthdata: ::core::option::Option<*const ::core::ffi::c_void>, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: ::core::option::Option<*const ::core::ffi::c_void>, ptsexpiry: ::core::option::Option<*mut i64>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn AddCredentialsW ( hcredentials : *const super::super::Credentials:: SecHandle , pszprincipal : :: windows::core::PCWSTR , pszpackage : :: windows::core::PCWSTR , fcredentialuse : u32 , pauthdata : *const ::core::ffi::c_void , pgetkeyfn : SEC_GET_KEY_FN , pvgetkeyargument : *const ::core::ffi::c_void , ptsexpiry : *mut i64 ) -> :: windows::core::HRESULT );
    AddCredentialsW(hcredentials, pszprincipal.into().abi(), pszpackage.into().abi(), fcredentialuse, ::core::mem::transmute(pauthdata.unwrap_or(::std::ptr::null())), pgetkeyfn, ::core::mem::transmute(pvgetkeyargument.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ptsexpiry.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn AddSecurityPackageA<P0>(pszpackagename: P0, poptions: ::core::option::Option<*const SECURITY_PACKAGE_OPTIONS>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn AddSecurityPackageA ( pszpackagename : :: windows::core::PCSTR , poptions : *const SECURITY_PACKAGE_OPTIONS ) -> :: windows::core::HRESULT );
    AddSecurityPackageA(pszpackagename.into().abi(), ::core::mem::transmute(poptions.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn AddSecurityPackageW<P0>(pszpackagename: P0, poptions: ::core::option::Option<*const SECURITY_PACKAGE_OPTIONS>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn AddSecurityPackageW ( pszpackagename : :: windows::core::PCWSTR , poptions : *const SECURITY_PACKAGE_OPTIONS ) -> :: windows::core::HRESULT );
    AddSecurityPackageW(pszpackagename.into().abi(), ::core::mem::transmute(poptions.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn ApplyControlToken(phcontext: *const super::super::Credentials::SecHandle, pinput: *const SecBufferDesc) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn ApplyControlToken ( phcontext : *const super::super::Credentials:: SecHandle , pinput : *const SecBufferDesc ) -> :: windows::core::HRESULT );
    ApplyControlToken(phcontext, pinput).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditComputeEffectivePolicyBySid<P0>(psid: P0, psubcategoryguids: &[::windows::core::GUID], ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<super::super::super::Foundation::PSID>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditComputeEffectivePolicyBySid ( psid : super::super::super::Foundation:: PSID , psubcategoryguids : *const :: windows::core::GUID , dwpolicycount : u32 , ppauditpolicy : *mut *mut AUDIT_POLICY_INFORMATION ) -> super::super::super::Foundation:: BOOLEAN );
    AuditComputeEffectivePolicyBySid(psid.into(), ::core::mem::transmute(psubcategoryguids.as_ptr()), psubcategoryguids.len() as _, ppauditpolicy)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditComputeEffectivePolicyByToken<P0>(htokenhandle: P0, psubcategoryguids: &[::windows::core::GUID], ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditComputeEffectivePolicyByToken ( htokenhandle : super::super::super::Foundation:: HANDLE , psubcategoryguids : *const :: windows::core::GUID , dwpolicycount : u32 , ppauditpolicy : *mut *mut AUDIT_POLICY_INFORMATION ) -> super::super::super::Foundation:: BOOLEAN );
    AuditComputeEffectivePolicyByToken(htokenhandle.into(), ::core::mem::transmute(psubcategoryguids.as_ptr()), psubcategoryguids.len() as _, ppauditpolicy)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditEnumerateCategories(ppauditcategoriesarray: *mut *mut ::windows::core::GUID, pdwcountreturned: *mut u32) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditEnumerateCategories ( ppauditcategoriesarray : *mut *mut :: windows::core::GUID , pdwcountreturned : *mut u32 ) -> super::super::super::Foundation:: BOOLEAN );
    AuditEnumerateCategories(ppauditcategoriesarray, pdwcountreturned)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditEnumeratePerUserPolicy(ppauditsidarray: *mut *mut POLICY_AUDIT_SID_ARRAY) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditEnumeratePerUserPolicy ( ppauditsidarray : *mut *mut POLICY_AUDIT_SID_ARRAY ) -> super::super::super::Foundation:: BOOLEAN );
    AuditEnumeratePerUserPolicy(ppauditsidarray)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditEnumerateSubCategories<P0>(pauditcategoryguid: ::core::option::Option<*const ::windows::core::GUID>, bretrieveallsubcategories: P0, ppauditsubcategoriesarray: *mut *mut ::windows::core::GUID, pdwcountreturned: *mut u32) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<super::super::super::Foundation::BOOLEAN>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditEnumerateSubCategories ( pauditcategoryguid : *const :: windows::core::GUID , bretrieveallsubcategories : super::super::super::Foundation:: BOOLEAN , ppauditsubcategoriesarray : *mut *mut :: windows::core::GUID , pdwcountreturned : *mut u32 ) -> super::super::super::Foundation:: BOOLEAN );
    AuditEnumerateSubCategories(::core::mem::transmute(pauditcategoryguid.unwrap_or(::std::ptr::null())), bretrieveallsubcategories.into(), ppauditsubcategoriesarray, pdwcountreturned)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn AuditFree(buffer: *const ::core::ffi::c_void) {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditFree ( buffer : *const ::core::ffi::c_void ) -> ( ) );
    AuditFree(buffer)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditLookupCategoryGuidFromCategoryId(auditcategoryid: POLICY_AUDIT_EVENT_TYPE, pauditcategoryguid: *mut ::windows::core::GUID) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditLookupCategoryGuidFromCategoryId ( auditcategoryid : POLICY_AUDIT_EVENT_TYPE , pauditcategoryguid : *mut :: windows::core::GUID ) -> super::super::super::Foundation:: BOOLEAN );
    AuditLookupCategoryGuidFromCategoryId(auditcategoryid, pauditcategoryguid)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditLookupCategoryIdFromCategoryGuid(pauditcategoryguid: *const ::windows::core::GUID, pauditcategoryid: *mut POLICY_AUDIT_EVENT_TYPE) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditLookupCategoryIdFromCategoryGuid ( pauditcategoryguid : *const :: windows::core::GUID , pauditcategoryid : *mut POLICY_AUDIT_EVENT_TYPE ) -> super::super::super::Foundation:: BOOLEAN );
    AuditLookupCategoryIdFromCategoryGuid(pauditcategoryguid, pauditcategoryid)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditLookupCategoryNameA(pauditcategoryguid: *const ::windows::core::GUID, ppszcategoryname: *mut ::windows::core::PSTR) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditLookupCategoryNameA ( pauditcategoryguid : *const :: windows::core::GUID , ppszcategoryname : *mut :: windows::core::PSTR ) -> super::super::super::Foundation:: BOOLEAN );
    AuditLookupCategoryNameA(pauditcategoryguid, ppszcategoryname)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditLookupCategoryNameW(pauditcategoryguid: *const ::windows::core::GUID, ppszcategoryname: *mut ::windows::core::PWSTR) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditLookupCategoryNameW ( pauditcategoryguid : *const :: windows::core::GUID , ppszcategoryname : *mut :: windows::core::PWSTR ) -> super::super::super::Foundation:: BOOLEAN );
    AuditLookupCategoryNameW(pauditcategoryguid, ppszcategoryname)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditLookupSubCategoryNameA(pauditsubcategoryguid: *const ::windows::core::GUID, ppszsubcategoryname: *mut ::windows::core::PSTR) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditLookupSubCategoryNameA ( pauditsubcategoryguid : *const :: windows::core::GUID , ppszsubcategoryname : *mut :: windows::core::PSTR ) -> super::super::super::Foundation:: BOOLEAN );
    AuditLookupSubCategoryNameA(pauditsubcategoryguid, ppszsubcategoryname)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditLookupSubCategoryNameW(pauditsubcategoryguid: *const ::windows::core::GUID, ppszsubcategoryname: *mut ::windows::core::PWSTR) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditLookupSubCategoryNameW ( pauditsubcategoryguid : *const :: windows::core::GUID , ppszsubcategoryname : *mut :: windows::core::PWSTR ) -> super::super::super::Foundation:: BOOLEAN );
    AuditLookupSubCategoryNameW(pauditsubcategoryguid, ppszsubcategoryname)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditQueryGlobalSaclA<P0>(objecttypename: P0, acl: *mut *mut super::super::ACL) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditQueryGlobalSaclA ( objecttypename : :: windows::core::PCSTR , acl : *mut *mut super::super:: ACL ) -> super::super::super::Foundation:: BOOLEAN );
    AuditQueryGlobalSaclA(objecttypename.into().abi(), acl)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditQueryGlobalSaclW<P0>(objecttypename: P0, acl: *mut *mut super::super::ACL) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditQueryGlobalSaclW ( objecttypename : :: windows::core::PCWSTR , acl : *mut *mut super::super:: ACL ) -> super::super::super::Foundation:: BOOLEAN );
    AuditQueryGlobalSaclW(objecttypename.into().abi(), acl)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditQueryPerUserPolicy<P0>(psid: P0, psubcategoryguids: &[::windows::core::GUID], ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<super::super::super::Foundation::PSID>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditQueryPerUserPolicy ( psid : super::super::super::Foundation:: PSID , psubcategoryguids : *const :: windows::core::GUID , dwpolicycount : u32 , ppauditpolicy : *mut *mut AUDIT_POLICY_INFORMATION ) -> super::super::super::Foundation:: BOOLEAN );
    AuditQueryPerUserPolicy(psid.into(), ::core::mem::transmute(psubcategoryguids.as_ptr()), psubcategoryguids.len() as _, ppauditpolicy)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditQuerySecurity(securityinformation: u32, ppsecuritydescriptor: *mut super::super::PSECURITY_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditQuerySecurity ( securityinformation : u32 , ppsecuritydescriptor : *mut super::super:: PSECURITY_DESCRIPTOR ) -> super::super::super::Foundation:: BOOLEAN );
    AuditQuerySecurity(securityinformation, ppsecuritydescriptor)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditQuerySystemPolicy(psubcategoryguids: &[::windows::core::GUID], ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditQuerySystemPolicy ( psubcategoryguids : *const :: windows::core::GUID , dwpolicycount : u32 , ppauditpolicy : *mut *mut AUDIT_POLICY_INFORMATION ) -> super::super::super::Foundation:: BOOLEAN );
    AuditQuerySystemPolicy(::core::mem::transmute(psubcategoryguids.as_ptr()), psubcategoryguids.len() as _, ppauditpolicy)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditSetGlobalSaclA<P0>(objecttypename: P0, acl: ::core::option::Option<*const super::super::ACL>) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditSetGlobalSaclA ( objecttypename : :: windows::core::PCSTR , acl : *const super::super:: ACL ) -> super::super::super::Foundation:: BOOLEAN );
    AuditSetGlobalSaclA(objecttypename.into().abi(), ::core::mem::transmute(acl.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditSetGlobalSaclW<P0>(objecttypename: P0, acl: ::core::option::Option<*const super::super::ACL>) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditSetGlobalSaclW ( objecttypename : :: windows::core::PCWSTR , acl : *const super::super:: ACL ) -> super::super::super::Foundation:: BOOLEAN );
    AuditSetGlobalSaclW(objecttypename.into().abi(), ::core::mem::transmute(acl.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditSetPerUserPolicy<P0>(psid: P0, pauditpolicy: &[AUDIT_POLICY_INFORMATION]) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<super::super::super::Foundation::PSID>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditSetPerUserPolicy ( psid : super::super::super::Foundation:: PSID , pauditpolicy : *const AUDIT_POLICY_INFORMATION , dwpolicycount : u32 ) -> super::super::super::Foundation:: BOOLEAN );
    AuditSetPerUserPolicy(psid.into(), ::core::mem::transmute(pauditpolicy.as_ptr()), pauditpolicy.len() as _)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditSetSecurity<P0>(securityinformation: u32, psecuritydescriptor: P0) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<super::super::PSECURITY_DESCRIPTOR>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditSetSecurity ( securityinformation : u32 , psecuritydescriptor : super::super:: PSECURITY_DESCRIPTOR ) -> super::super::super::Foundation:: BOOLEAN );
    AuditSetSecurity(securityinformation, psecuritydescriptor.into())
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuditSetSystemPolicy(pauditpolicy: &[AUDIT_POLICY_INFORMATION]) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn AuditSetSystemPolicy ( pauditpolicy : *const AUDIT_POLICY_INFORMATION , dwpolicycount : u32 ) -> super::super::super::Foundation:: BOOLEAN );
    AuditSetSystemPolicy(::core::mem::transmute(pauditpolicy.as_ptr()), pauditpolicy.len() as _)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeAccountPasswordA<P0>(pszpackagename: *const i8, pszdomainname: *const i8, pszaccountname: *const i8, pszoldpassword: *const i8, psznewpassword: *const i8, bimpersonating: P0, dwreserved: u32, poutput: *mut SecBufferDesc) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Foundation::BOOLEAN>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn ChangeAccountPasswordA ( pszpackagename : *const i8 , pszdomainname : *const i8 , pszaccountname : *const i8 , pszoldpassword : *const i8 , psznewpassword : *const i8 , bimpersonating : super::super::super::Foundation:: BOOLEAN , dwreserved : u32 , poutput : *mut SecBufferDesc ) -> :: windows::core::HRESULT );
    ChangeAccountPasswordA(pszpackagename, pszdomainname, pszaccountname, pszoldpassword, psznewpassword, bimpersonating.into(), dwreserved, poutput).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeAccountPasswordW<P0>(pszpackagename: *const u16, pszdomainname: *const u16, pszaccountname: *const u16, pszoldpassword: *const u16, psznewpassword: *const u16, bimpersonating: P0, dwreserved: u32, poutput: *mut SecBufferDesc) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Foundation::BOOLEAN>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn ChangeAccountPasswordW ( pszpackagename : *const u16 , pszdomainname : *const u16 , pszaccountname : *const u16 , pszoldpassword : *const u16 , psznewpassword : *const u16 , bimpersonating : super::super::super::Foundation:: BOOLEAN , dwreserved : u32 , poutput : *mut SecBufferDesc ) -> :: windows::core::HRESULT );
    ChangeAccountPasswordW(pszpackagename, pszdomainname, pszaccountname, pszoldpassword, psznewpassword, bimpersonating.into(), dwreserved, poutput).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn CompleteAuthToken(phcontext: *const super::super::Credentials::SecHandle, ptoken: *const SecBufferDesc) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn CompleteAuthToken ( phcontext : *const super::super::Credentials:: SecHandle , ptoken : *const SecBufferDesc ) -> :: windows::core::HRESULT );
    CompleteAuthToken(phcontext, ptoken).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
#[inline]
pub unsafe fn CredMarshalTargetInfo(intargetinfo: *const super::super::Credentials::CREDENTIAL_TARGET_INFORMATIONW, buffer: *mut *mut u16, buffersize: *mut u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn CredMarshalTargetInfo ( intargetinfo : *const super::super::Credentials:: CREDENTIAL_TARGET_INFORMATIONW , buffer : *mut *mut u16 , buffersize : *mut u32 ) -> super::super::super::Foundation:: NTSTATUS );
    CredMarshalTargetInfo(intargetinfo, buffer, buffersize).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
#[inline]
pub unsafe fn CredUnmarshalTargetInfo(buffer: *const u16, buffersize: u32, rettargetinfo: ::core::option::Option<*mut *mut super::super::Credentials::CREDENTIAL_TARGET_INFORMATIONW>, retactualsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn CredUnmarshalTargetInfo ( buffer : *const u16 , buffersize : u32 , rettargetinfo : *mut *mut super::super::Credentials:: CREDENTIAL_TARGET_INFORMATIONW , retactualsize : *mut u32 ) -> super::super::super::Foundation:: NTSTATUS );
    CredUnmarshalTargetInfo(buffer, buffersize, ::core::mem::transmute(rettargetinfo.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(retactualsize.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn DecryptMessage(phcontext: *const super::super::Credentials::SecHandle, pmessage: *const SecBufferDesc, messageseqno: u32, pfqop: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
    ::windows::core::link ! ( "secur32.dll""system" fn DecryptMessage ( phcontext : *const super::super::Credentials:: SecHandle , pmessage : *const SecBufferDesc , messageseqno : u32 , pfqop : *mut u32 ) -> :: windows::core::HRESULT );
    DecryptMessage(phcontext, pmessage, messageseqno, ::core::mem::transmute(pfqop.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn DeleteSecurityContext(phcontext: *const super::super::Credentials::SecHandle) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn DeleteSecurityContext ( phcontext : *const super::super::Credentials:: SecHandle ) -> :: windows::core::HRESULT );
    DeleteSecurityContext(phcontext).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn DeleteSecurityPackageA<P0>(pszpackagename: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn DeleteSecurityPackageA ( pszpackagename : :: windows::core::PCSTR ) -> :: windows::core::HRESULT );
    DeleteSecurityPackageA(pszpackagename.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn DeleteSecurityPackageW<P0>(pszpackagename: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn DeleteSecurityPackageW ( pszpackagename : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    DeleteSecurityPackageW(pszpackagename.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn EncryptMessage(phcontext: *const super::super::Credentials::SecHandle, fqop: u32, pmessage: *const SecBufferDesc, messageseqno: u32) -> ::windows::core::HRESULT {
    ::windows::core::link ! ( "secur32.dll""system" fn EncryptMessage ( phcontext : *const super::super::Credentials:: SecHandle , fqop : u32 , pmessage : *const SecBufferDesc , messageseqno : u32 ) -> :: windows::core::HRESULT );
    EncryptMessage(phcontext, fqop, pmessage, messageseqno)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn EnumerateSecurityPackagesA(pcpackages: *mut u32, pppackageinfo: *mut *mut SecPkgInfoA) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn EnumerateSecurityPackagesA ( pcpackages : *mut u32 , pppackageinfo : *mut *mut SecPkgInfoA ) -> :: windows::core::HRESULT );
    EnumerateSecurityPackagesA(pcpackages, pppackageinfo).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn EnumerateSecurityPackagesW(pcpackages: *mut u32, pppackageinfo: *mut *mut SecPkgInfoW) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn EnumerateSecurityPackagesW ( pcpackages : *mut u32 , pppackageinfo : *mut *mut SecPkgInfoW ) -> :: windows::core::HRESULT );
    EnumerateSecurityPackagesW(pcpackages, pppackageinfo).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn ExportSecurityContext(phcontext: *const super::super::Credentials::SecHandle, fflags: EXPORT_SECURITY_CONTEXT_FLAGS, ppackedcontext: *mut SecBuffer, ptoken: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn ExportSecurityContext ( phcontext : *const super::super::Credentials:: SecHandle , fflags : EXPORT_SECURITY_CONTEXT_FLAGS , ppackedcontext : *mut SecBuffer , ptoken : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    ExportSecurityContext(phcontext, fflags, ppackedcontext, ptoken).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn FreeContextBuffer(pvcontextbuffer: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn FreeContextBuffer ( pvcontextbuffer : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    FreeContextBuffer(pvcontextbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn FreeCredentialsHandle(phcredential: *const super::super::Credentials::SecHandle) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn FreeCredentialsHandle ( phcredential : *const super::super::Credentials:: SecHandle ) -> :: windows::core::HRESULT );
    FreeCredentialsHandle(phcredential).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerObjectNameA(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: ::windows::core::PSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "secur32.dll""system" fn GetComputerObjectNameA ( nameformat : EXTENDED_NAME_FORMAT , lpnamebuffer : :: windows::core::PSTR , nsize : *mut u32 ) -> super::super::super::Foundation:: BOOLEAN );
    GetComputerObjectNameA(nameformat, ::core::mem::transmute(lpnamebuffer), nsize)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerObjectNameW(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "secur32.dll""system" fn GetComputerObjectNameW ( nameformat : EXTENDED_NAME_FORMAT , lpnamebuffer : :: windows::core::PWSTR , nsize : *mut u32 ) -> super::super::super::Foundation:: BOOLEAN );
    GetComputerObjectNameW(nameformat, ::core::mem::transmute(lpnamebuffer), nsize)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserNameExA(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: ::windows::core::PSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "secur32.dll""system" fn GetUserNameExA ( nameformat : EXTENDED_NAME_FORMAT , lpnamebuffer : :: windows::core::PSTR , nsize : *mut u32 ) -> super::super::super::Foundation:: BOOLEAN );
    GetUserNameExA(nameformat, ::core::mem::transmute(lpnamebuffer), nsize)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserNameExW(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "secur32.dll""system" fn GetUserNameExW ( nameformat : EXTENDED_NAME_FORMAT , lpnamebuffer : :: windows::core::PWSTR , nsize : *mut u32 ) -> super::super::super::Foundation:: BOOLEAN );
    GetUserNameExW(nameformat, ::core::mem::transmute(lpnamebuffer), nsize)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn ImpersonateSecurityContext(phcontext: *const super::super::Credentials::SecHandle) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn ImpersonateSecurityContext ( phcontext : *const super::super::Credentials:: SecHandle ) -> :: windows::core::HRESULT );
    ImpersonateSecurityContext(phcontext).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn ImportSecurityContextA<P0>(pszpackage: P0, ppackedcontext: *const SecBuffer, token: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Credentials::SecHandle>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn ImportSecurityContextA ( pszpackage : :: windows::core::PCSTR , ppackedcontext : *const SecBuffer , token : *const ::core::ffi::c_void , phcontext : *mut super::super::Credentials:: SecHandle ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    ImportSecurityContextA(pszpackage.into().abi(), ppackedcontext, token, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn ImportSecurityContextW<P0>(pszpackage: P0, ppackedcontext: *const SecBuffer, token: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Credentials::SecHandle>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn ImportSecurityContextW ( pszpackage : :: windows::core::PCWSTR , ppackedcontext : *const SecBuffer , token : *const ::core::ffi::c_void , phcontext : *mut super::super::Credentials:: SecHandle ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    ImportSecurityContextW(pszpackage.into().abi(), ppackedcontext, token, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
#[inline]
pub unsafe fn InitSecurityInterfaceA() -> *mut SecurityFunctionTableA {
    ::windows::core::link ! ( "secur32.dll""system" fn InitSecurityInterfaceA ( ) -> *mut SecurityFunctionTableA );
    InitSecurityInterfaceA()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
#[inline]
pub unsafe fn InitSecurityInterfaceW() -> *mut SecurityFunctionTableW {
    ::windows::core::link ! ( "secur32.dll""system" fn InitSecurityInterfaceW ( ) -> *mut SecurityFunctionTableW );
    InitSecurityInterfaceW()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn InitializeSecurityContextA(phcredential: ::core::option::Option<*const super::super::Credentials::SecHandle>, phcontext: ::core::option::Option<*const super::super::Credentials::SecHandle>, psztargetname: ::core::option::Option<*const i8>, fcontextreq: ISC_REQ_FLAGS, reserved1: u32, targetdatarep: u32, pinput: ::core::option::Option<*const SecBufferDesc>, reserved2: u32, phnewcontext: ::core::option::Option<*mut super::super::Credentials::SecHandle>, poutput: ::core::option::Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: ::core::option::Option<*mut i64>) -> ::windows::core::HRESULT {
    ::windows::core::link ! ( "secur32.dll""system" fn InitializeSecurityContextA ( phcredential : *const super::super::Credentials:: SecHandle , phcontext : *const super::super::Credentials:: SecHandle , psztargetname : *const i8 , fcontextreq : ISC_REQ_FLAGS , reserved1 : u32 , targetdatarep : u32 , pinput : *const SecBufferDesc , reserved2 : u32 , phnewcontext : *mut super::super::Credentials:: SecHandle , poutput : *mut SecBufferDesc , pfcontextattr : *mut u32 , ptsexpiry : *mut i64 ) -> :: windows::core::HRESULT );
    InitializeSecurityContextA(
        ::core::mem::transmute(phcredential.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(phcontext.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(psztargetname.unwrap_or(::std::ptr::null())),
        fcontextreq,
        reserved1,
        targetdatarep,
        ::core::mem::transmute(pinput.unwrap_or(::std::ptr::null())),
        reserved2,
        ::core::mem::transmute(phnewcontext.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(poutput.unwrap_or(::std::ptr::null_mut())),
        pfcontextattr,
        ::core::mem::transmute(ptsexpiry.unwrap_or(::std::ptr::null_mut())),
    )
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn InitializeSecurityContextW(phcredential: ::core::option::Option<*const super::super::Credentials::SecHandle>, phcontext: ::core::option::Option<*const super::super::Credentials::SecHandle>, psztargetname: ::core::option::Option<*const u16>, fcontextreq: ISC_REQ_FLAGS, reserved1: u32, targetdatarep: u32, pinput: ::core::option::Option<*const SecBufferDesc>, reserved2: u32, phnewcontext: ::core::option::Option<*mut super::super::Credentials::SecHandle>, poutput: ::core::option::Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: ::core::option::Option<*mut i64>) -> ::windows::core::HRESULT {
    ::windows::core::link ! ( "secur32.dll""system" fn InitializeSecurityContextW ( phcredential : *const super::super::Credentials:: SecHandle , phcontext : *const super::super::Credentials:: SecHandle , psztargetname : *const u16 , fcontextreq : ISC_REQ_FLAGS , reserved1 : u32 , targetdatarep : u32 , pinput : *const SecBufferDesc , reserved2 : u32 , phnewcontext : *mut super::super::Credentials:: SecHandle , poutput : *mut SecBufferDesc , pfcontextattr : *mut u32 , ptsexpiry : *mut i64 ) -> :: windows::core::HRESULT );
    InitializeSecurityContextW(
        ::core::mem::transmute(phcredential.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(phcontext.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(psztargetname.unwrap_or(::std::ptr::null())),
        fcontextreq,
        reserved1,
        targetdatarep,
        ::core::mem::transmute(pinput.unwrap_or(::std::ptr::null())),
        reserved2,
        ::core::mem::transmute(phnewcontext.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(poutput.unwrap_or(::std::ptr::null_mut())),
        pfcontextattr,
        ::core::mem::transmute(ptsexpiry.unwrap_or(::std::ptr::null_mut())),
    )
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaAddAccountRights<P0, P1>(policyhandle: P0, accountsid: P1, userrights: &[super::super::super::Foundation::UNICODE_STRING]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
    P1: ::std::convert::Into<super::super::super::Foundation::PSID>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaAddAccountRights ( policyhandle : LSA_HANDLE , accountsid : super::super::super::Foundation:: PSID , userrights : *const super::super::super::Foundation:: UNICODE_STRING , countofrights : u32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaAddAccountRights(policyhandle.into(), accountsid.into(), ::core::mem::transmute(userrights.as_ptr()), userrights.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaCallAuthenticationPackage<P0>(lsahandle: P0, authenticationpackage: u32, protocolsubmitbuffer: *const ::core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: ::core::option::Option<*mut *mut ::core::ffi::c_void>, returnbufferlength: ::core::option::Option<*mut u32>, protocolstatus: ::core::option::Option<*mut i32>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn LsaCallAuthenticationPackage ( lsahandle : super::super::super::Foundation:: HANDLE , authenticationpackage : u32 , protocolsubmitbuffer : *const ::core::ffi::c_void , submitbufferlength : u32 , protocolreturnbuffer : *mut *mut ::core::ffi::c_void , returnbufferlength : *mut u32 , protocolstatus : *mut i32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaCallAuthenticationPackage(lsahandle.into(), authenticationpackage, protocolsubmitbuffer, submitbufferlength, ::core::mem::transmute(protocolreturnbuffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(returnbufferlength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(protocolstatus.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaClose<P0>(objecthandle: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaClose ( objecthandle : LSA_HANDLE ) -> super::super::super::Foundation:: NTSTATUS );
    LsaClose(objecthandle.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaConnectUntrusted(lsahandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn LsaConnectUntrusted ( lsahandle : *mut super::super::super::Foundation:: HANDLE ) -> super::super::super::Foundation:: NTSTATUS );
    LsaConnectUntrusted(lsahandle).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaCreateTrustedDomainEx<P0>(policyhandle: P0, trusteddomaininformation: *const TRUSTED_DOMAIN_INFORMATION_EX, authenticationinformation: *const TRUSTED_DOMAIN_AUTH_INFORMATION, desiredaccess: u32, trusteddomainhandle: *mut LSA_HANDLE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaCreateTrustedDomainEx ( policyhandle : LSA_HANDLE , trusteddomaininformation : *const TRUSTED_DOMAIN_INFORMATION_EX , authenticationinformation : *const TRUSTED_DOMAIN_AUTH_INFORMATION , desiredaccess : u32 , trusteddomainhandle : *mut LSA_HANDLE ) -> super::super::super::Foundation:: NTSTATUS );
    LsaCreateTrustedDomainEx(policyhandle.into(), trusteddomaininformation, authenticationinformation, desiredaccess, trusteddomainhandle).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaDeleteTrustedDomain<P0, P1>(policyhandle: P0, trusteddomainsid: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
    P1: ::std::convert::Into<super::super::super::Foundation::PSID>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaDeleteTrustedDomain ( policyhandle : LSA_HANDLE , trusteddomainsid : super::super::super::Foundation:: PSID ) -> super::super::super::Foundation:: NTSTATUS );
    LsaDeleteTrustedDomain(policyhandle.into(), trusteddomainsid.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaDeregisterLogonProcess<P0>(lsahandle: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn LsaDeregisterLogonProcess ( lsahandle : super::super::super::Foundation:: HANDLE ) -> super::super::super::Foundation:: NTSTATUS );
    LsaDeregisterLogonProcess(lsahandle.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaEnumerateAccountRights<P0, P1>(policyhandle: P0, accountsid: P1, userrights: *mut *mut super::super::super::Foundation::UNICODE_STRING, countofrights: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
    P1: ::std::convert::Into<super::super::super::Foundation::PSID>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaEnumerateAccountRights ( policyhandle : LSA_HANDLE , accountsid : super::super::super::Foundation:: PSID , userrights : *mut *mut super::super::super::Foundation:: UNICODE_STRING , countofrights : *mut u32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaEnumerateAccountRights(policyhandle.into(), accountsid.into(), userrights, countofrights).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaEnumerateAccountsWithUserRight<P0>(policyhandle: P0, userright: ::core::option::Option<*const super::super::super::Foundation::UNICODE_STRING>, buffer: *mut *mut ::core::ffi::c_void, countreturned: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaEnumerateAccountsWithUserRight ( policyhandle : LSA_HANDLE , userright : *const super::super::super::Foundation:: UNICODE_STRING , buffer : *mut *mut ::core::ffi::c_void , countreturned : *mut u32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaEnumerateAccountsWithUserRight(policyhandle.into(), ::core::mem::transmute(userright.unwrap_or(::std::ptr::null())), buffer, countreturned).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaEnumerateLogonSessions(logonsessioncount: *mut u32, logonsessionlist: *mut *mut super::super::super::Foundation::LUID) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn LsaEnumerateLogonSessions ( logonsessioncount : *mut u32 , logonsessionlist : *mut *mut super::super::super::Foundation:: LUID ) -> super::super::super::Foundation:: NTSTATUS );
    LsaEnumerateLogonSessions(logonsessioncount, logonsessionlist).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaEnumerateTrustedDomains<P0>(policyhandle: P0, enumerationcontext: *mut u32, buffer: *mut *mut ::core::ffi::c_void, preferedmaximumlength: u32, countreturned: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaEnumerateTrustedDomains ( policyhandle : LSA_HANDLE , enumerationcontext : *mut u32 , buffer : *mut *mut ::core::ffi::c_void , preferedmaximumlength : u32 , countreturned : *mut u32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaEnumerateTrustedDomains(policyhandle.into(), enumerationcontext, buffer, preferedmaximumlength, countreturned).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaEnumerateTrustedDomainsEx<P0>(policyhandle: P0, enumerationcontext: *mut u32, buffer: *mut *mut ::core::ffi::c_void, preferedmaximumlength: u32, countreturned: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaEnumerateTrustedDomainsEx ( policyhandle : LSA_HANDLE , enumerationcontext : *mut u32 , buffer : *mut *mut ::core::ffi::c_void , preferedmaximumlength : u32 , countreturned : *mut u32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaEnumerateTrustedDomainsEx(policyhandle.into(), enumerationcontext, buffer, preferedmaximumlength, countreturned).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaFreeMemory(buffer: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaFreeMemory ( buffer : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: NTSTATUS );
    LsaFreeMemory(::core::mem::transmute(buffer.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaFreeReturnBuffer(buffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn LsaFreeReturnBuffer ( buffer : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: NTSTATUS );
    LsaFreeReturnBuffer(buffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaGetAppliedCAPIDs(systemname: ::core::option::Option<*const super::super::super::Foundation::UNICODE_STRING>, capids: *mut *mut super::super::super::Foundation::PSID, capidcount: *mut u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaGetAppliedCAPIDs ( systemname : *const super::super::super::Foundation:: UNICODE_STRING , capids : *mut *mut super::super::super::Foundation:: PSID , capidcount : *mut u32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaGetAppliedCAPIDs(::core::mem::transmute(systemname.unwrap_or(::std::ptr::null())), capids, capidcount).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaGetLogonSessionData(logonid: *const super::super::super::Foundation::LUID, pplogonsessiondata: *mut *mut SECURITY_LOGON_SESSION_DATA) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn LsaGetLogonSessionData ( logonid : *const super::super::super::Foundation:: LUID , pplogonsessiondata : *mut *mut SECURITY_LOGON_SESSION_DATA ) -> super::super::super::Foundation:: NTSTATUS );
    LsaGetLogonSessionData(logonid, pplogonsessiondata).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn LsaLogonUser<P0>(lsahandle: P0, originname: *const super::super::super::System::Kernel::STRING, logontype: SECURITY_LOGON_TYPE, authenticationpackage: u32, authenticationinformation: *const ::core::ffi::c_void, authenticationinformationlength: u32, localgroups: ::core::option::Option<*const super::super::TOKEN_GROUPS>, sourcecontext: *const super::super::TOKEN_SOURCE, profilebuffer: *mut *mut ::core::ffi::c_void, profilebufferlength: *mut u32, logonid: *mut super::super::super::Foundation::LUID, token: *mut super::super::super::Foundation::HANDLE, quotas: *mut super::super::QUOTA_LIMITS, substatus: *mut i32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn LsaLogonUser ( lsahandle : super::super::super::Foundation:: HANDLE , originname : *const super::super::super::System::Kernel:: STRING , logontype : SECURITY_LOGON_TYPE , authenticationpackage : u32 , authenticationinformation : *const ::core::ffi::c_void , authenticationinformationlength : u32 , localgroups : *const super::super:: TOKEN_GROUPS , sourcecontext : *const super::super:: TOKEN_SOURCE , profilebuffer : *mut *mut ::core::ffi::c_void , profilebufferlength : *mut u32 , logonid : *mut super::super::super::Foundation:: LUID , token : *mut super::super::super::Foundation:: HANDLE , quotas : *mut super::super:: QUOTA_LIMITS , substatus : *mut i32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaLogonUser(lsahandle.into(), originname, logontype, authenticationpackage, authenticationinformation, authenticationinformationlength, ::core::mem::transmute(localgroups.unwrap_or(::std::ptr::null())), sourcecontext, profilebuffer, profilebufferlength, logonid, token, quotas, substatus).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn LsaLookupAuthenticationPackage<P0>(lsahandle: P0, packagename: *const super::super::super::System::Kernel::STRING, authenticationpackage: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn LsaLookupAuthenticationPackage ( lsahandle : super::super::super::Foundation:: HANDLE , packagename : *const super::super::super::System::Kernel:: STRING , authenticationpackage : *mut u32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaLookupAuthenticationPackage(lsahandle.into(), packagename, authenticationpackage).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaLookupNames<P0>(policyhandle: P0, count: u32, names: *const super::super::super::Foundation::UNICODE_STRING, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, sids: *mut *mut LSA_TRANSLATED_SID) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaLookupNames ( policyhandle : LSA_HANDLE , count : u32 , names : *const super::super::super::Foundation:: UNICODE_STRING , referenceddomains : *mut *mut LSA_REFERENCED_DOMAIN_LIST , sids : *mut *mut LSA_TRANSLATED_SID ) -> super::super::super::Foundation:: NTSTATUS );
    LsaLookupNames(policyhandle.into(), count, names, referenceddomains, sids).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaLookupNames2<P0>(policyhandle: P0, flags: u32, count: u32, names: *const super::super::super::Foundation::UNICODE_STRING, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, sids: *mut *mut LSA_TRANSLATED_SID2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaLookupNames2 ( policyhandle : LSA_HANDLE , flags : u32 , count : u32 , names : *const super::super::super::Foundation:: UNICODE_STRING , referenceddomains : *mut *mut LSA_REFERENCED_DOMAIN_LIST , sids : *mut *mut LSA_TRANSLATED_SID2 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaLookupNames2(policyhandle.into(), flags, count, names, referenceddomains, sids).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaLookupSids<P0>(policyhandle: P0, count: u32, sids: *const super::super::super::Foundation::PSID, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, names: *mut *mut LSA_TRANSLATED_NAME) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaLookupSids ( policyhandle : LSA_HANDLE , count : u32 , sids : *const super::super::super::Foundation:: PSID , referenceddomains : *mut *mut LSA_REFERENCED_DOMAIN_LIST , names : *mut *mut LSA_TRANSLATED_NAME ) -> super::super::super::Foundation:: NTSTATUS );
    LsaLookupSids(policyhandle.into(), count, sids, referenceddomains, names).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaLookupSids2<P0>(policyhandle: P0, lookupoptions: u32, count: u32, sids: *const super::super::super::Foundation::PSID, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, names: *mut *mut LSA_TRANSLATED_NAME) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaLookupSids2 ( policyhandle : LSA_HANDLE , lookupoptions : u32 , count : u32 , sids : *const super::super::super::Foundation:: PSID , referenceddomains : *mut *mut LSA_REFERENCED_DOMAIN_LIST , names : *mut *mut LSA_TRANSLATED_NAME ) -> super::super::super::Foundation:: NTSTATUS );
    LsaLookupSids2(policyhandle.into(), lookupoptions, count, sids, referenceddomains, names).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaNtStatusToWinError<P0>(status: P0) -> u32
where
    P0: ::std::convert::Into<super::super::super::Foundation::NTSTATUS>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaNtStatusToWinError ( status : super::super::super::Foundation:: NTSTATUS ) -> u32 );
    LsaNtStatusToWinError(status.into())
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn LsaOpenPolicy(systemname: ::core::option::Option<*const super::super::super::Foundation::UNICODE_STRING>, objectattributes: *const super::super::super::System::WindowsProgramming::OBJECT_ATTRIBUTES, desiredaccess: u32, policyhandle: *mut LSA_HANDLE) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaOpenPolicy ( systemname : *const super::super::super::Foundation:: UNICODE_STRING , objectattributes : *const super::super::super::System::WindowsProgramming:: OBJECT_ATTRIBUTES , desiredaccess : u32 , policyhandle : *mut LSA_HANDLE ) -> super::super::super::Foundation:: NTSTATUS );
    LsaOpenPolicy(::core::mem::transmute(systemname.unwrap_or(::std::ptr::null())), objectattributes, desiredaccess, policyhandle).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaOpenTrustedDomainByName<P0>(policyhandle: P0, trusteddomainname: *const super::super::super::Foundation::UNICODE_STRING, desiredaccess: u32, trusteddomainhandle: *mut LSA_HANDLE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaOpenTrustedDomainByName ( policyhandle : LSA_HANDLE , trusteddomainname : *const super::super::super::Foundation:: UNICODE_STRING , desiredaccess : u32 , trusteddomainhandle : *mut LSA_HANDLE ) -> super::super::super::Foundation:: NTSTATUS );
    LsaOpenTrustedDomainByName(policyhandle.into(), trusteddomainname, desiredaccess, trusteddomainhandle).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaQueryCAPs(capids: ::core::option::Option<&[super::super::super::Foundation::PSID]>, caps: *mut *mut CENTRAL_ACCESS_POLICY, capcount: *mut u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaQueryCAPs ( capids : *const super::super::super::Foundation:: PSID , capidcount : u32 , caps : *mut *mut CENTRAL_ACCESS_POLICY , capcount : *mut u32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaQueryCAPs(::core::mem::transmute(capids.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), capids.as_deref().map_or(0, |slice| slice.len() as _), caps, capcount).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaQueryDomainInformationPolicy<P0>(policyhandle: P0, informationclass: POLICY_DOMAIN_INFORMATION_CLASS, buffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaQueryDomainInformationPolicy ( policyhandle : LSA_HANDLE , informationclass : POLICY_DOMAIN_INFORMATION_CLASS , buffer : *mut *mut ::core::ffi::c_void ) -> super::super::super::Foundation:: NTSTATUS );
    LsaQueryDomainInformationPolicy(policyhandle.into(), informationclass, buffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaQueryForestTrustInformation<P0>(policyhandle: P0, trusteddomainname: *const super::super::super::Foundation::UNICODE_STRING, foresttrustinfo: *mut *mut LSA_FOREST_TRUST_INFORMATION) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaQueryForestTrustInformation ( policyhandle : LSA_HANDLE , trusteddomainname : *const super::super::super::Foundation:: UNICODE_STRING , foresttrustinfo : *mut *mut LSA_FOREST_TRUST_INFORMATION ) -> super::super::super::Foundation:: NTSTATUS );
    LsaQueryForestTrustInformation(policyhandle.into(), trusteddomainname, foresttrustinfo).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaQueryInformationPolicy<P0>(policyhandle: P0, informationclass: POLICY_INFORMATION_CLASS, buffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaQueryInformationPolicy ( policyhandle : LSA_HANDLE , informationclass : POLICY_INFORMATION_CLASS , buffer : *mut *mut ::core::ffi::c_void ) -> super::super::super::Foundation:: NTSTATUS );
    LsaQueryInformationPolicy(policyhandle.into(), informationclass, buffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaQueryTrustedDomainInfo<P0, P1>(policyhandle: P0, trusteddomainsid: P1, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
    P1: ::std::convert::Into<super::super::super::Foundation::PSID>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaQueryTrustedDomainInfo ( policyhandle : LSA_HANDLE , trusteddomainsid : super::super::super::Foundation:: PSID , informationclass : TRUSTED_INFORMATION_CLASS , buffer : *mut *mut ::core::ffi::c_void ) -> super::super::super::Foundation:: NTSTATUS );
    LsaQueryTrustedDomainInfo(policyhandle.into(), trusteddomainsid.into(), informationclass, buffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaQueryTrustedDomainInfoByName<P0>(policyhandle: P0, trusteddomainname: *const super::super::super::Foundation::UNICODE_STRING, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaQueryTrustedDomainInfoByName ( policyhandle : LSA_HANDLE , trusteddomainname : *const super::super::super::Foundation:: UNICODE_STRING , informationclass : TRUSTED_INFORMATION_CLASS , buffer : *mut *mut ::core::ffi::c_void ) -> super::super::super::Foundation:: NTSTATUS );
    LsaQueryTrustedDomainInfoByName(policyhandle.into(), trusteddomainname, informationclass, buffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn LsaRegisterLogonProcess(logonprocessname: *const super::super::super::System::Kernel::STRING, lsahandle: *mut super::super::super::Foundation::HANDLE, securitymode: *mut u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn LsaRegisterLogonProcess ( logonprocessname : *const super::super::super::System::Kernel:: STRING , lsahandle : *mut super::super::super::Foundation:: HANDLE , securitymode : *mut u32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaRegisterLogonProcess(logonprocessname, lsahandle, securitymode).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaRegisterPolicyChangeNotification<P0>(informationclass: POLICY_NOTIFICATION_INFORMATION_CLASS, notificationeventhandle: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn LsaRegisterPolicyChangeNotification ( informationclass : POLICY_NOTIFICATION_INFORMATION_CLASS , notificationeventhandle : super::super::super::Foundation:: HANDLE ) -> super::super::super::Foundation:: NTSTATUS );
    LsaRegisterPolicyChangeNotification(informationclass, notificationeventhandle.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaRemoveAccountRights<P0, P1, P2>(policyhandle: P0, accountsid: P1, allrights: P2, userrights: ::core::option::Option<&[super::super::super::Foundation::UNICODE_STRING]>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
    P1: ::std::convert::Into<super::super::super::Foundation::PSID>,
    P2: ::std::convert::Into<super::super::super::Foundation::BOOLEAN>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaRemoveAccountRights ( policyhandle : LSA_HANDLE , accountsid : super::super::super::Foundation:: PSID , allrights : super::super::super::Foundation:: BOOLEAN , userrights : *const super::super::super::Foundation:: UNICODE_STRING , countofrights : u32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaRemoveAccountRights(policyhandle.into(), accountsid.into(), allrights.into(), ::core::mem::transmute(userrights.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), userrights.as_deref().map_or(0, |slice| slice.len() as _)).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaRetrievePrivateData<P0>(policyhandle: P0, keyname: *const super::super::super::Foundation::UNICODE_STRING, privatedata: *mut *mut super::super::super::Foundation::UNICODE_STRING) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaRetrievePrivateData ( policyhandle : LSA_HANDLE , keyname : *const super::super::super::Foundation:: UNICODE_STRING , privatedata : *mut *mut super::super::super::Foundation:: UNICODE_STRING ) -> super::super::super::Foundation:: NTSTATUS );
    LsaRetrievePrivateData(policyhandle.into(), keyname, privatedata).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaSetCAPs(capdns: ::core::option::Option<&[super::super::super::Foundation::UNICODE_STRING]>, flags: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaSetCAPs ( capdns : *const super::super::super::Foundation:: UNICODE_STRING , capdncount : u32 , flags : u32 ) -> super::super::super::Foundation:: NTSTATUS );
    LsaSetCAPs(::core::mem::transmute(capdns.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), capdns.as_deref().map_or(0, |slice| slice.len() as _), flags).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaSetDomainInformationPolicy<P0>(policyhandle: P0, informationclass: POLICY_DOMAIN_INFORMATION_CLASS, buffer: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaSetDomainInformationPolicy ( policyhandle : LSA_HANDLE , informationclass : POLICY_DOMAIN_INFORMATION_CLASS , buffer : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: NTSTATUS );
    LsaSetDomainInformationPolicy(policyhandle.into(), informationclass, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaSetForestTrustInformation<P0, P1>(policyhandle: P0, trusteddomainname: *const super::super::super::Foundation::UNICODE_STRING, foresttrustinfo: *const LSA_FOREST_TRUST_INFORMATION, checkonly: P1, collisioninfo: *mut *mut LSA_FOREST_TRUST_COLLISION_INFORMATION) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
    P1: ::std::convert::Into<super::super::super::Foundation::BOOLEAN>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaSetForestTrustInformation ( policyhandle : LSA_HANDLE , trusteddomainname : *const super::super::super::Foundation:: UNICODE_STRING , foresttrustinfo : *const LSA_FOREST_TRUST_INFORMATION , checkonly : super::super::super::Foundation:: BOOLEAN , collisioninfo : *mut *mut LSA_FOREST_TRUST_COLLISION_INFORMATION ) -> super::super::super::Foundation:: NTSTATUS );
    LsaSetForestTrustInformation(policyhandle.into(), trusteddomainname, foresttrustinfo, checkonly.into(), collisioninfo).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaSetInformationPolicy<P0>(policyhandle: P0, informationclass: POLICY_INFORMATION_CLASS, buffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaSetInformationPolicy ( policyhandle : LSA_HANDLE , informationclass : POLICY_INFORMATION_CLASS , buffer : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: NTSTATUS );
    LsaSetInformationPolicy(policyhandle.into(), informationclass, buffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaSetTrustedDomainInfoByName<P0>(policyhandle: P0, trusteddomainname: *const super::super::super::Foundation::UNICODE_STRING, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaSetTrustedDomainInfoByName ( policyhandle : LSA_HANDLE , trusteddomainname : *const super::super::super::Foundation:: UNICODE_STRING , informationclass : TRUSTED_INFORMATION_CLASS , buffer : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: NTSTATUS );
    LsaSetTrustedDomainInfoByName(policyhandle.into(), trusteddomainname, informationclass, buffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaSetTrustedDomainInformation<P0, P1>(policyhandle: P0, trusteddomainsid: P1, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
    P1: ::std::convert::Into<super::super::super::Foundation::PSID>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaSetTrustedDomainInformation ( policyhandle : LSA_HANDLE , trusteddomainsid : super::super::super::Foundation:: PSID , informationclass : TRUSTED_INFORMATION_CLASS , buffer : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: NTSTATUS );
    LsaSetTrustedDomainInformation(policyhandle.into(), trusteddomainsid.into(), informationclass, buffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaStorePrivateData<P0>(policyhandle: P0, keyname: *const super::super::super::Foundation::UNICODE_STRING, privatedata: ::core::option::Option<*const super::super::super::Foundation::UNICODE_STRING>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<LSA_HANDLE>,
{
    ::windows::core::link ! ( "advapi32.dll""system" fn LsaStorePrivateData ( policyhandle : LSA_HANDLE , keyname : *const super::super::super::Foundation:: UNICODE_STRING , privatedata : *const super::super::super::Foundation:: UNICODE_STRING ) -> super::super::super::Foundation:: NTSTATUS );
    LsaStorePrivateData(policyhandle.into(), keyname, ::core::mem::transmute(privatedata.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsaUnregisterPolicyChangeNotification<P0>(informationclass: POLICY_NOTIFICATION_INFORMATION_CLASS, notificationeventhandle: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn LsaUnregisterPolicyChangeNotification ( informationclass : POLICY_NOTIFICATION_INFORMATION_CLASS , notificationeventhandle : super::super::super::Foundation:: HANDLE ) -> super::super::super::Foundation:: NTSTATUS );
    LsaUnregisterPolicyChangeNotification(informationclass, notificationeventhandle.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn MakeSignature(phcontext: *const super::super::Credentials::SecHandle, fqop: u32, pmessage: *const SecBufferDesc, messageseqno: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn MakeSignature ( phcontext : *const super::super::Credentials:: SecHandle , fqop : u32 , pmessage : *const SecBufferDesc , messageseqno : u32 ) -> :: windows::core::HRESULT );
    MakeSignature(phcontext, fqop, pmessage, messageseqno).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryContextAttributesA(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn QueryContextAttributesA ( phcontext : *const super::super::Credentials:: SecHandle , ulattribute : SECPKG_ATTR , pbuffer : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    QueryContextAttributesA(phcontext, ulattribute, pbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryContextAttributesExA(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "sspicli.dll""system" fn QueryContextAttributesExA ( phcontext : *const super::super::Credentials:: SecHandle , ulattribute : SECPKG_ATTR , pbuffer : *mut ::core::ffi::c_void , cbbuffer : u32 ) -> :: windows::core::HRESULT );
    QueryContextAttributesExA(phcontext, ulattribute, pbuffer, cbbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryContextAttributesExW(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "sspicli.dll""system" fn QueryContextAttributesExW ( phcontext : *const super::super::Credentials:: SecHandle , ulattribute : SECPKG_ATTR , pbuffer : *mut ::core::ffi::c_void , cbbuffer : u32 ) -> :: windows::core::HRESULT );
    QueryContextAttributesExW(phcontext, ulattribute, pbuffer, cbbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryContextAttributesW(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn QueryContextAttributesW ( phcontext : *const super::super::Credentials:: SecHandle , ulattribute : SECPKG_ATTR , pbuffer : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    QueryContextAttributesW(phcontext, ulattribute, pbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryCredentialsAttributesA(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn QueryCredentialsAttributesA ( phcredential : *const super::super::Credentials:: SecHandle , ulattribute : u32 , pbuffer : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    QueryCredentialsAttributesA(phcredential, ulattribute, pbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryCredentialsAttributesExA(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "sspicli.dll""system" fn QueryCredentialsAttributesExA ( phcredential : *const super::super::Credentials:: SecHandle , ulattribute : u32 , pbuffer : *mut ::core::ffi::c_void , cbbuffer : u32 ) -> :: windows::core::HRESULT );
    QueryCredentialsAttributesExA(phcredential, ulattribute, pbuffer, cbbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryCredentialsAttributesExW(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "sspicli.dll""system" fn QueryCredentialsAttributesExW ( phcredential : *const super::super::Credentials:: SecHandle , ulattribute : u32 , pbuffer : *mut ::core::ffi::c_void , cbbuffer : u32 ) -> :: windows::core::HRESULT );
    QueryCredentialsAttributesExW(phcredential, ulattribute, pbuffer, cbbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryCredentialsAttributesW(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn QueryCredentialsAttributesW ( phcredential : *const super::super::Credentials:: SecHandle , ulattribute : u32 , pbuffer : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    QueryCredentialsAttributesW(phcredential, ulattribute, pbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QuerySecurityContextToken(phcontext: *const super::super::Credentials::SecHandle, token: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn QuerySecurityContextToken ( phcontext : *const super::super::Credentials:: SecHandle , token : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    QuerySecurityContextToken(phcontext, token).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn QuerySecurityPackageInfoA<P0>(pszpackagename: P0) -> ::windows::core::Result<*mut SecPkgInfoA>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn QuerySecurityPackageInfoA ( pszpackagename : :: windows::core::PCSTR , pppackageinfo : *mut *mut SecPkgInfoA ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    QuerySecurityPackageInfoA(pszpackagename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn QuerySecurityPackageInfoW<P0>(pszpackagename: P0) -> ::windows::core::Result<*mut SecPkgInfoW>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn QuerySecurityPackageInfoW ( pszpackagename : :: windows::core::PCWSTR , pppackageinfo : *mut *mut SecPkgInfoW ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    QuerySecurityPackageInfoW(pszpackagename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn RevertSecurityContext(phcontext: *const super::super::Credentials::SecHandle) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn RevertSecurityContext ( phcontext : *const super::super::Credentials:: SecHandle ) -> :: windows::core::HRESULT );
    RevertSecurityContext(phcontext).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLAcquireGenuineTicket<P0, P1, P2>(ppticketblob: *mut *mut ::core::ffi::c_void, pcbticketblob: *mut u32, pwsztemplateid: P0, pwszserverurl: P1, pwszclienttoken: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slcext.dll""system" fn SLAcquireGenuineTicket ( ppticketblob : *mut *mut ::core::ffi::c_void , pcbticketblob : *mut u32 , pwsztemplateid : :: windows::core::PCWSTR , pwszserverurl : :: windows::core::PCWSTR , pwszclienttoken : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    SLAcquireGenuineTicket(ppticketblob, pcbticketblob, pwsztemplateid.into().abi(), pwszserverurl.into().abi(), pwszclienttoken.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLActivateProduct<P0>(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::core::GUID, cbappspecificdata: u32, pvappspecificdata: ::core::option::Option<*const ::core::ffi::c_void>, pactivationinfo: ::core::option::Option<*const SL_ACTIVATION_INFO_HEADER>, pwszproxyserver: P0, wproxyport: u16) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slcext.dll""system" fn SLActivateProduct ( hslc : *const ::core::ffi::c_void , pproductskuid : *const :: windows::core::GUID , cbappspecificdata : u32 , pvappspecificdata : *const ::core::ffi::c_void , pactivationinfo : *const SL_ACTIVATION_INFO_HEADER , pwszproxyserver : :: windows::core::PCWSTR , wproxyport : u16 ) -> :: windows::core::HRESULT );
    SLActivateProduct(hslc, pproductskuid, cbappspecificdata, ::core::mem::transmute(pvappspecificdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pactivationinfo.unwrap_or(::std::ptr::null())), pwszproxyserver.into().abi(), wproxyport).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLClose(hslc: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "slc.dll""system" fn SLClose ( hslc : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SLClose(hslc).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLConsumeRight<P0>(hslc: *const ::core::ffi::c_void, pappid: *const ::windows::core::GUID, pproductskuid: ::core::option::Option<*const ::windows::core::GUID>, pwszrightname: P0, pvreserved: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLConsumeRight ( hslc : *const ::core::ffi::c_void , pappid : *const :: windows::core::GUID , pproductskuid : *const :: windows::core::GUID , pwszrightname : :: windows::core::PCWSTR , pvreserved : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SLConsumeRight(hslc, pappid, ::core::mem::transmute(pproductskuid.unwrap_or(::std::ptr::null())), pwszrightname.into().abi(), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLDepositOfflineConfirmationId<P0, P1>(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::core::GUID, pwszinstallationid: P0, pwszconfirmationid: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLDepositOfflineConfirmationId ( hslc : *const ::core::ffi::c_void , pproductskuid : *const :: windows::core::GUID , pwszinstallationid : :: windows::core::PCWSTR , pwszconfirmationid : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    SLDepositOfflineConfirmationId(hslc, pproductskuid, pwszinstallationid.into().abi(), pwszconfirmationid.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLDepositOfflineConfirmationIdEx<P0, P1>(hslc: *const ::core::ffi::c_void, pproductskuid: ::core::option::Option<*const ::windows::core::GUID>, pactivationinfo: ::core::option::Option<*const SL_ACTIVATION_INFO_HEADER>, pwszinstallationid: P0, pwszconfirmationid: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLDepositOfflineConfirmationIdEx ( hslc : *const ::core::ffi::c_void , pproductskuid : *const :: windows::core::GUID , pactivationinfo : *const SL_ACTIVATION_INFO_HEADER , pwszinstallationid : :: windows::core::PCWSTR , pwszconfirmationid : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    SLDepositOfflineConfirmationIdEx(hslc, ::core::mem::transmute(pproductskuid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pactivationinfo.unwrap_or(::std::ptr::null())), pwszinstallationid.into().abi(), pwszconfirmationid.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLFireEvent<P0>(hslc: *const ::core::ffi::c_void, pwszeventid: P0, papplicationid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLFireEvent ( hslc : *const ::core::ffi::c_void , pwszeventid : :: windows::core::PCWSTR , papplicationid : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    SLFireEvent(hslc, pwszeventid.into().abi(), papplicationid).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGenerateOfflineInstallationId(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::core::link ! ( "slc.dll""system" fn SLGenerateOfflineInstallationId ( hslc : *const ::core::ffi::c_void , pproductskuid : *const :: windows::core::GUID , ppwszinstallationid : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SLGenerateOfflineInstallationId(hslc, pproductskuid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGenerateOfflineInstallationIdEx(hslc: *const ::core::ffi::c_void, pproductskuid: ::core::option::Option<*const ::windows::core::GUID>, pactivationinfo: ::core::option::Option<*const SL_ACTIVATION_INFO_HEADER>) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::core::link ! ( "slc.dll""system" fn SLGenerateOfflineInstallationIdEx ( hslc : *const ::core::ffi::c_void , pproductskuid : *const :: windows::core::GUID , pactivationinfo : *const SL_ACTIVATION_INFO_HEADER , ppwszinstallationid : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SLGenerateOfflineInstallationIdEx(hslc, ::core::mem::transmute(pproductskuid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pactivationinfo.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetApplicationInformation<P0>(hslc: *const ::core::ffi::c_void, papplicationid: *const ::windows::core::GUID, pwszvaluename: P0, pedatatype: ::core::option::Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetApplicationInformation ( hslc : *const ::core::ffi::c_void , papplicationid : *const :: windows::core::GUID , pwszvaluename : :: windows::core::PCWSTR , pedatatype : *mut SLDATATYPE , pcbvalue : *mut u32 , ppbvalue : *mut *mut u8 ) -> :: windows::core::HRESULT );
    SLGetApplicationInformation(hslc, papplicationid, pwszvaluename.into().abi(), ::core::mem::transmute(pedatatype.unwrap_or(::std::ptr::null_mut())), pcbvalue, ppbvalue).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetGenuineInformation<P0>(pqueryid: *const ::windows::core::GUID, pwszvaluename: P0, pedatatype: ::core::option::Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetGenuineInformation ( pqueryid : *const :: windows::core::GUID , pwszvaluename : :: windows::core::PCWSTR , pedatatype : *mut SLDATATYPE , pcbvalue : *mut u32 , ppbvalue : *mut *mut u8 ) -> :: windows::core::HRESULT );
    SLGetGenuineInformation(pqueryid, pwszvaluename.into().abi(), ::core::mem::transmute(pedatatype.unwrap_or(::std::ptr::null_mut())), pcbvalue, ppbvalue).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetInstalledProductKeyIds(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::core::GUID, pnproductkeyids: *mut u32, ppproductkeyids: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "slc.dll""system" fn SLGetInstalledProductKeyIds ( hslc : *const ::core::ffi::c_void , pproductskuid : *const :: windows::core::GUID , pnproductkeyids : *mut u32 , ppproductkeyids : *mut *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    SLGetInstalledProductKeyIds(hslc, pproductskuid, pnproductkeyids, ppproductkeyids).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetLicense(hslc: *const ::core::ffi::c_void, plicensefileid: *const ::windows::core::GUID, pcblicensefile: *mut u32, ppblicensefile: *mut *mut u8) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "slc.dll""system" fn SLGetLicense ( hslc : *const ::core::ffi::c_void , plicensefileid : *const :: windows::core::GUID , pcblicensefile : *mut u32 , ppblicensefile : *mut *mut u8 ) -> :: windows::core::HRESULT );
    SLGetLicense(hslc, plicensefileid, pcblicensefile, ppblicensefile).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetLicenseFileId(hslc: *const ::core::ffi::c_void, pblicenseblob: &[u8]) -> ::windows::core::Result<::windows::core::GUID> {
    ::windows::core::link ! ( "slc.dll""system" fn SLGetLicenseFileId ( hslc : *const ::core::ffi::c_void , cblicenseblob : u32 , pblicenseblob : *const u8 , plicensefileid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SLGetLicenseFileId(hslc, pblicenseblob.len() as _, ::core::mem::transmute(pblicenseblob.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetLicenseInformation<P0>(hslc: *const ::core::ffi::c_void, psllicenseid: *const ::windows::core::GUID, pwszvaluename: P0, pedatatype: ::core::option::Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetLicenseInformation ( hslc : *const ::core::ffi::c_void , psllicenseid : *const :: windows::core::GUID , pwszvaluename : :: windows::core::PCWSTR , pedatatype : *mut SLDATATYPE , pcbvalue : *mut u32 , ppbvalue : *mut *mut u8 ) -> :: windows::core::HRESULT );
    SLGetLicenseInformation(hslc, psllicenseid, pwszvaluename.into().abi(), ::core::mem::transmute(pedatatype.unwrap_or(::std::ptr::null_mut())), pcbvalue, ppbvalue).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetLicensingStatusInformation<P0>(hslc: *const ::core::ffi::c_void, pappid: ::core::option::Option<*const ::windows::core::GUID>, pproductskuid: ::core::option::Option<*const ::windows::core::GUID>, pwszrightname: P0, pnstatuscount: *mut u32, pplicensingstatus: *mut *mut SL_LICENSING_STATUS) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetLicensingStatusInformation ( hslc : *const ::core::ffi::c_void , pappid : *const :: windows::core::GUID , pproductskuid : *const :: windows::core::GUID , pwszrightname : :: windows::core::PCWSTR , pnstatuscount : *mut u32 , pplicensingstatus : *mut *mut SL_LICENSING_STATUS ) -> :: windows::core::HRESULT );
    SLGetLicensingStatusInformation(hslc, ::core::mem::transmute(pappid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pproductskuid.unwrap_or(::std::ptr::null())), pwszrightname.into().abi(), pnstatuscount, pplicensingstatus).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetPKeyId<P0, P1>(hslc: *const ::core::ffi::c_void, pwszpkeyalgorithm: P0, pwszpkeystring: P1, pbpkeyspecificdata: ::core::option::Option<&[u8]>) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetPKeyId ( hslc : *const ::core::ffi::c_void , pwszpkeyalgorithm : :: windows::core::PCWSTR , pwszpkeystring : :: windows::core::PCWSTR , cbpkeyspecificdata : u32 , pbpkeyspecificdata : *const u8 , ppkeyid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SLGetPKeyId(hslc, pwszpkeyalgorithm.into().abi(), pwszpkeystring.into().abi(), pbpkeyspecificdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pbpkeyspecificdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetPKeyInformation<P0>(hslc: *const ::core::ffi::c_void, ppkeyid: *const ::windows::core::GUID, pwszvaluename: P0, pedatatype: ::core::option::Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetPKeyInformation ( hslc : *const ::core::ffi::c_void , ppkeyid : *const :: windows::core::GUID , pwszvaluename : :: windows::core::PCWSTR , pedatatype : *mut SLDATATYPE , pcbvalue : *mut u32 , ppbvalue : *mut *mut u8 ) -> :: windows::core::HRESULT );
    SLGetPKeyInformation(hslc, ppkeyid, pwszvaluename.into().abi(), ::core::mem::transmute(pedatatype.unwrap_or(::std::ptr::null_mut())), pcbvalue, ppbvalue).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetPolicyInformation<P0>(hslc: *const ::core::ffi::c_void, pwszvaluename: P0, pedatatype: ::core::option::Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetPolicyInformation ( hslc : *const ::core::ffi::c_void , pwszvaluename : :: windows::core::PCWSTR , pedatatype : *mut SLDATATYPE , pcbvalue : *mut u32 , ppbvalue : *mut *mut u8 ) -> :: windows::core::HRESULT );
    SLGetPolicyInformation(hslc, pwszvaluename.into().abi(), ::core::mem::transmute(pedatatype.unwrap_or(::std::ptr::null_mut())), pcbvalue, ppbvalue).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetPolicyInformationDWORD<P0>(hslc: *const ::core::ffi::c_void, pwszvaluename: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetPolicyInformationDWORD ( hslc : *const ::core::ffi::c_void , pwszvaluename : :: windows::core::PCWSTR , pdwvalue : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SLGetPolicyInformationDWORD(hslc, pwszvaluename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetProductSkuInformation<P0>(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::core::GUID, pwszvaluename: P0, pedatatype: ::core::option::Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetProductSkuInformation ( hslc : *const ::core::ffi::c_void , pproductskuid : *const :: windows::core::GUID , pwszvaluename : :: windows::core::PCWSTR , pedatatype : *mut SLDATATYPE , pcbvalue : *mut u32 , ppbvalue : *mut *mut u8 ) -> :: windows::core::HRESULT );
    SLGetProductSkuInformation(hslc, pproductskuid, pwszvaluename.into().abi(), ::core::mem::transmute(pedatatype.unwrap_or(::std::ptr::null_mut())), pcbvalue, ppbvalue).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetReferralInformation<P0>(hslc: *const ::core::ffi::c_void, ereferraltype: SLREFERRALTYPE, pskuorappid: *const ::windows::core::GUID, pwszvaluename: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slcext.dll""system" fn SLGetReferralInformation ( hslc : *const ::core::ffi::c_void , ereferraltype : SLREFERRALTYPE , pskuorappid : *const :: windows::core::GUID , pwszvaluename : :: windows::core::PCWSTR , ppwszvalue : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SLGetReferralInformation(hslc, ereferraltype, pskuorappid, pwszvaluename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetSLIDList(hslc: *const ::core::ffi::c_void, equeryidtype: SLIDTYPE, pqueryid: ::core::option::Option<*const ::windows::core::GUID>, ereturnidtype: SLIDTYPE, pnreturnids: *mut u32, ppreturnids: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "slc.dll""system" fn SLGetSLIDList ( hslc : *const ::core::ffi::c_void , equeryidtype : SLIDTYPE , pqueryid : *const :: windows::core::GUID , ereturnidtype : SLIDTYPE , pnreturnids : *mut u32 , ppreturnids : *mut *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    SLGetSLIDList(hslc, equeryidtype, ::core::mem::transmute(pqueryid.unwrap_or(::std::ptr::null())), ereturnidtype, pnreturnids, ppreturnids).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetServerStatus<P0, P1, P2>(pwszserverurl: P0, pwszacquisitiontype: P1, pwszproxyserver: P2, wproxyport: u16) -> ::windows::core::Result<::windows::core::HRESULT>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slcext.dll""system" fn SLGetServerStatus ( pwszserverurl : :: windows::core::PCWSTR , pwszacquisitiontype : :: windows::core::PCWSTR , pwszproxyserver : :: windows::core::PCWSTR , wproxyport : u16 , phrstatus : *mut :: windows::core::HRESULT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SLGetServerStatus(pwszserverurl.into().abi(), pwszacquisitiontype.into().abi(), pwszproxyserver.into().abi(), wproxyport, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetServiceInformation<P0>(hslc: *const ::core::ffi::c_void, pwszvaluename: P0, pedatatype: ::core::option::Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetServiceInformation ( hslc : *const ::core::ffi::c_void , pwszvaluename : :: windows::core::PCWSTR , pedatatype : *mut SLDATATYPE , pcbvalue : *mut u32 , ppbvalue : *mut *mut u8 ) -> :: windows::core::HRESULT );
    SLGetServiceInformation(hslc, pwszvaluename.into().abi(), ::core::mem::transmute(pedatatype.unwrap_or(::std::ptr::null_mut())), pcbvalue, ppbvalue).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetWindowsInformation<P0>(pwszvaluename: P0, pedatatype: ::core::option::Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetWindowsInformation ( pwszvaluename : :: windows::core::PCWSTR , pedatatype : *mut SLDATATYPE , pcbvalue : *mut u32 , ppbvalue : *mut *mut u8 ) -> :: windows::core::HRESULT );
    SLGetWindowsInformation(pwszvaluename.into().abi(), ::core::mem::transmute(pedatatype.unwrap_or(::std::ptr::null_mut())), pcbvalue, ppbvalue).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLGetWindowsInformationDWORD<P0>(pwszvaluename: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLGetWindowsInformationDWORD ( pwszvaluename : :: windows::core::PCWSTR , pdwvalue : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SLGetWindowsInformationDWORD(pwszvaluename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLInstallLicense(hslc: *const ::core::ffi::c_void, pblicenseblob: &[u8]) -> ::windows::core::Result<::windows::core::GUID> {
    ::windows::core::link ! ( "slc.dll""system" fn SLInstallLicense ( hslc : *const ::core::ffi::c_void , cblicenseblob : u32 , pblicenseblob : *const u8 , plicensefileid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SLInstallLicense(hslc, pblicenseblob.len() as _, ::core::mem::transmute(pblicenseblob.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLInstallProofOfPurchase<P0, P1>(hslc: *const ::core::ffi::c_void, pwszpkeyalgorithm: P0, pwszpkeystring: P1, pbpkeyspecificdata: ::core::option::Option<&[u8]>) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLInstallProofOfPurchase ( hslc : *const ::core::ffi::c_void , pwszpkeyalgorithm : :: windows::core::PCWSTR , pwszpkeystring : :: windows::core::PCWSTR , cbpkeyspecificdata : u32 , pbpkeyspecificdata : *const u8 , ppkeyid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SLInstallProofOfPurchase(hslc, pwszpkeyalgorithm.into().abi(), pwszpkeystring.into().abi(), pbpkeyspecificdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pbpkeyspecificdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLIsGenuineLocal(pappid: *const ::windows::core::GUID, pgenuinestate: *mut SL_GENUINE_STATE, puioptions: ::core::option::Option<*mut SL_NONGENUINE_UI_OPTIONS>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "slwga.dll""system" fn SLIsGenuineLocal ( pappid : *const :: windows::core::GUID , pgenuinestate : *mut SL_GENUINE_STATE , puioptions : *mut SL_NONGENUINE_UI_OPTIONS ) -> :: windows::core::HRESULT );
    SLIsGenuineLocal(pappid, pgenuinestate, ::core::mem::transmute(puioptions.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLOpen(phslc: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "slc.dll""system" fn SLOpen ( phslc : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SLOpen(phslc).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLQueryLicenseValueFromApp<P0>(valuename: P0, valuetype: ::core::option::Option<*mut u32>, databuffer: ::core::option::Option<*mut ::core::ffi::c_void>, datasize: u32, resultdatasize: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-core-slapi-l1-1-0.dll""system" fn SLQueryLicenseValueFromApp ( valuename : :: windows::core::PCWSTR , valuetype : *mut u32 , databuffer : *mut ::core::ffi::c_void , datasize : u32 , resultdatasize : *mut u32 ) -> :: windows::core::HRESULT );
    SLQueryLicenseValueFromApp(valuename.into().abi(), ::core::mem::transmute(valuetype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(databuffer.unwrap_or(::std::ptr::null_mut())), datasize, resultdatasize).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SLRegisterEvent<P0, P1>(hslc: ::core::option::Option<*const ::core::ffi::c_void>, pwszeventid: P0, papplicationid: *const ::windows::core::GUID, hevent: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLRegisterEvent ( hslc : *const ::core::ffi::c_void , pwszeventid : :: windows::core::PCWSTR , papplicationid : *const :: windows::core::GUID , hevent : super::super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    SLRegisterEvent(::core::mem::transmute(hslc.unwrap_or(::std::ptr::null())), pwszeventid.into().abi(), papplicationid, hevent.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLSetCurrentProductKey(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::core::GUID, pproductkeyid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "slc.dll""system" fn SLSetCurrentProductKey ( hslc : *const ::core::ffi::c_void , pproductskuid : *const :: windows::core::GUID , pproductkeyid : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    SLSetCurrentProductKey(hslc, pproductskuid, pproductkeyid).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLSetGenuineInformation<P0>(pqueryid: *const ::windows::core::GUID, pwszvaluename: P0, edatatype: SLDATATYPE, pbvalue: ::core::option::Option<&[u8]>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLSetGenuineInformation ( pqueryid : *const :: windows::core::GUID , pwszvaluename : :: windows::core::PCWSTR , edatatype : SLDATATYPE , cbvalue : u32 , pbvalue : *const u8 ) -> :: windows::core::HRESULT );
    SLSetGenuineInformation(pqueryid, pwszvaluename.into().abi(), edatatype, pbvalue.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pbvalue.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLUninstallLicense(hslc: *const ::core::ffi::c_void, plicensefileid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "slc.dll""system" fn SLUninstallLicense ( hslc : *const ::core::ffi::c_void , plicensefileid : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    SLUninstallLicense(hslc, plicensefileid).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SLUninstallProofOfPurchase(hslc: *const ::core::ffi::c_void, ppkeyid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "slc.dll""system" fn SLUninstallProofOfPurchase ( hslc : *const ::core::ffi::c_void , ppkeyid : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    SLUninstallProofOfPurchase(hslc, ppkeyid).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SLUnregisterEvent<P0, P1>(hslc: ::core::option::Option<*const ::core::ffi::c_void>, pwszeventid: P0, papplicationid: *const ::windows::core::GUID, hevent: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "slc.dll""system" fn SLUnregisterEvent ( hslc : *const ::core::ffi::c_void , pwszeventid : :: windows::core::PCWSTR , papplicationid : *const :: windows::core::GUID , hevent : super::super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    SLUnregisterEvent(::core::mem::transmute(hslc.unwrap_or(::std::ptr::null())), pwszeventid.into().abi(), papplicationid, hevent.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SaslAcceptSecurityContext(phcredential: ::core::option::Option<*const super::super::Credentials::SecHandle>, phcontext: ::core::option::Option<*const super::super::Credentials::SecHandle>, pinput: ::core::option::Option<*const SecBufferDesc>, fcontextreq: ASC_REQ_FLAGS, targetdatarep: u32, phnewcontext: ::core::option::Option<*mut super::super::Credentials::SecHandle>, poutput: ::core::option::Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: ::core::option::Option<*mut i64>) -> ::windows::core::HRESULT {
    ::windows::core::link ! ( "secur32.dll""system" fn SaslAcceptSecurityContext ( phcredential : *const super::super::Credentials:: SecHandle , phcontext : *const super::super::Credentials:: SecHandle , pinput : *const SecBufferDesc , fcontextreq : ASC_REQ_FLAGS , targetdatarep : u32 , phnewcontext : *mut super::super::Credentials:: SecHandle , poutput : *mut SecBufferDesc , pfcontextattr : *mut u32 , ptsexpiry : *mut i64 ) -> :: windows::core::HRESULT );
    SaslAcceptSecurityContext(::core::mem::transmute(phcredential.unwrap_or(::std::ptr::null())), ::core::mem::transmute(phcontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pinput.unwrap_or(::std::ptr::null())), fcontextreq, targetdatarep, ::core::mem::transmute(phnewcontext.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(poutput.unwrap_or(::std::ptr::null_mut())), pfcontextattr, ::core::mem::transmute(ptsexpiry.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SaslEnumerateProfilesA(profilelist: *mut ::windows::core::PSTR, profilecount: *mut u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SaslEnumerateProfilesA ( profilelist : *mut :: windows::core::PSTR , profilecount : *mut u32 ) -> :: windows::core::HRESULT );
    SaslEnumerateProfilesA(profilelist, profilecount).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SaslEnumerateProfilesW(profilelist: *mut ::windows::core::PWSTR, profilecount: *mut u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SaslEnumerateProfilesW ( profilelist : *mut :: windows::core::PWSTR , profilecount : *mut u32 ) -> :: windows::core::HRESULT );
    SaslEnumerateProfilesW(profilelist, profilecount).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SaslGetContextOption(contexthandle: *const super::super::Credentials::SecHandle, option: u32, value: *mut ::core::ffi::c_void, size: u32, needed: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SaslGetContextOption ( contexthandle : *const super::super::Credentials:: SecHandle , option : u32 , value : *mut ::core::ffi::c_void , size : u32 , needed : *mut u32 ) -> :: windows::core::HRESULT );
    SaslGetContextOption(contexthandle, option, value, size, ::core::mem::transmute(needed.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SaslGetProfilePackageA<P0>(profilename: P0) -> ::windows::core::Result<*mut SecPkgInfoA>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn SaslGetProfilePackageA ( profilename : :: windows::core::PCSTR , packageinfo : *mut *mut SecPkgInfoA ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SaslGetProfilePackageA(profilename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SaslGetProfilePackageW<P0>(profilename: P0) -> ::windows::core::Result<*mut SecPkgInfoW>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn SaslGetProfilePackageW ( profilename : :: windows::core::PCWSTR , packageinfo : *mut *mut SecPkgInfoW ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SaslGetProfilePackageW(profilename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SaslIdentifyPackageA(pinput: *const SecBufferDesc) -> ::windows::core::Result<*mut SecPkgInfoA> {
    ::windows::core::link ! ( "secur32.dll""system" fn SaslIdentifyPackageA ( pinput : *const SecBufferDesc , packageinfo : *mut *mut SecPkgInfoA ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SaslIdentifyPackageA(pinput, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SaslIdentifyPackageW(pinput: *const SecBufferDesc) -> ::windows::core::Result<*mut SecPkgInfoW> {
    ::windows::core::link ! ( "secur32.dll""system" fn SaslIdentifyPackageW ( pinput : *const SecBufferDesc , packageinfo : *mut *mut SecPkgInfoW ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SaslIdentifyPackageW(pinput, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SaslInitializeSecurityContextA<P0>(phcredential: ::core::option::Option<*const super::super::Credentials::SecHandle>, phcontext: ::core::option::Option<*const super::super::Credentials::SecHandle>, psztargetname: P0, fcontextreq: ISC_REQ_FLAGS, reserved1: u32, targetdatarep: u32, pinput: ::core::option::Option<*const SecBufferDesc>, reserved2: u32, phnewcontext: ::core::option::Option<*mut super::super::Credentials::SecHandle>, poutput: ::core::option::Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: ::core::option::Option<*mut i64>) -> ::windows::core::HRESULT
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn SaslInitializeSecurityContextA ( phcredential : *const super::super::Credentials:: SecHandle , phcontext : *const super::super::Credentials:: SecHandle , psztargetname : :: windows::core::PCSTR , fcontextreq : ISC_REQ_FLAGS , reserved1 : u32 , targetdatarep : u32 , pinput : *const SecBufferDesc , reserved2 : u32 , phnewcontext : *mut super::super::Credentials:: SecHandle , poutput : *mut SecBufferDesc , pfcontextattr : *mut u32 , ptsexpiry : *mut i64 ) -> :: windows::core::HRESULT );
    SaslInitializeSecurityContextA(
        ::core::mem::transmute(phcredential.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(phcontext.unwrap_or(::std::ptr::null())),
        psztargetname.into().abi(),
        fcontextreq,
        reserved1,
        targetdatarep,
        ::core::mem::transmute(pinput.unwrap_or(::std::ptr::null())),
        reserved2,
        ::core::mem::transmute(phnewcontext.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(poutput.unwrap_or(::std::ptr::null_mut())),
        pfcontextattr,
        ::core::mem::transmute(ptsexpiry.unwrap_or(::std::ptr::null_mut())),
    )
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SaslInitializeSecurityContextW<P0>(phcredential: ::core::option::Option<*const super::super::Credentials::SecHandle>, phcontext: ::core::option::Option<*const super::super::Credentials::SecHandle>, psztargetname: P0, fcontextreq: ISC_REQ_FLAGS, reserved1: u32, targetdatarep: u32, pinput: ::core::option::Option<*const SecBufferDesc>, reserved2: u32, phnewcontext: ::core::option::Option<*mut super::super::Credentials::SecHandle>, poutput: ::core::option::Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: ::core::option::Option<*mut i64>) -> ::windows::core::HRESULT
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn SaslInitializeSecurityContextW ( phcredential : *const super::super::Credentials:: SecHandle , phcontext : *const super::super::Credentials:: SecHandle , psztargetname : :: windows::core::PCWSTR , fcontextreq : ISC_REQ_FLAGS , reserved1 : u32 , targetdatarep : u32 , pinput : *const SecBufferDesc , reserved2 : u32 , phnewcontext : *mut super::super::Credentials:: SecHandle , poutput : *mut SecBufferDesc , pfcontextattr : *mut u32 , ptsexpiry : *mut i64 ) -> :: windows::core::HRESULT );
    SaslInitializeSecurityContextW(
        ::core::mem::transmute(phcredential.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(phcontext.unwrap_or(::std::ptr::null())),
        psztargetname.into().abi(),
        fcontextreq,
        reserved1,
        targetdatarep,
        ::core::mem::transmute(pinput.unwrap_or(::std::ptr::null())),
        reserved2,
        ::core::mem::transmute(phnewcontext.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(poutput.unwrap_or(::std::ptr::null_mut())),
        pfcontextattr,
        ::core::mem::transmute(ptsexpiry.unwrap_or(::std::ptr::null_mut())),
    )
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SaslSetContextOption(contexthandle: *const super::super::Credentials::SecHandle, option: u32, value: *const ::core::ffi::c_void, size: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SaslSetContextOption ( contexthandle : *const super::super::Credentials:: SecHandle , option : u32 , value : *const ::core::ffi::c_void , size : u32 ) -> :: windows::core::HRESULT );
    SaslSetContextOption(contexthandle, option, value, size).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendSAS<P0>(asuser: P0)
where
    P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "sas.dll""system" fn SendSAS ( asuser : super::super::super::Foundation:: BOOL ) -> ( ) );
    SendSAS(asuser.into())
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SetContextAttributesA(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SetContextAttributesA ( phcontext : *const super::super::Credentials:: SecHandle , ulattribute : SECPKG_ATTR , pbuffer : *const ::core::ffi::c_void , cbbuffer : u32 ) -> :: windows::core::HRESULT );
    SetContextAttributesA(phcontext, ulattribute, pbuffer, cbbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SetContextAttributesW(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SetContextAttributesW ( phcontext : *const super::super::Credentials:: SecHandle , ulattribute : SECPKG_ATTR , pbuffer : *const ::core::ffi::c_void , cbbuffer : u32 ) -> :: windows::core::HRESULT );
    SetContextAttributesW(phcontext, ulattribute, pbuffer, cbbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SetCredentialsAttributesA(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SetCredentialsAttributesA ( phcredential : *const super::super::Credentials:: SecHandle , ulattribute : u32 , pbuffer : *const ::core::ffi::c_void , cbbuffer : u32 ) -> :: windows::core::HRESULT );
    SetCredentialsAttributesA(phcredential, ulattribute, pbuffer, cbbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SetCredentialsAttributesW(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SetCredentialsAttributesW ( phcredential : *const super::super::Credentials:: SecHandle , ulattribute : u32 , pbuffer : *const ::core::ffi::c_void , cbbuffer : u32 ) -> :: windows::core::HRESULT );
    SetCredentialsAttributesW(phcredential, ulattribute, pbuffer, cbbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SslCrackCertificate(pbcertificate: *mut u8, cbcertificate: u32, dwflags: u32, ppcertificate: *mut *mut X509Certificate) -> super::super::super::Foundation::BOOL {
    ::windows::core::link ! ( "schannel.dll""system" fn SslCrackCertificate ( pbcertificate : *mut u8 , cbcertificate : u32 , dwflags : u32 , ppcertificate : *mut *mut X509Certificate ) -> super::super::super::Foundation:: BOOL );
    SslCrackCertificate(pbcertificate, cbcertificate, dwflags, ppcertificate)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SslEmptyCacheA<P0>(psztargetname: P0, dwflags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "schannel.dll""system" fn SslEmptyCacheA ( psztargetname : :: windows::core::PCSTR , dwflags : u32 ) -> super::super::super::Foundation:: BOOL );
    SslEmptyCacheA(psztargetname.into().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SslEmptyCacheW<P0>(psztargetname: P0, dwflags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "schannel.dll""system" fn SslEmptyCacheW ( psztargetname : :: windows::core::PCWSTR , dwflags : u32 ) -> super::super::super::Foundation:: BOOL );
    SslEmptyCacheW(psztargetname.into().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SslFreeCertificate(pcertificate: *mut X509Certificate) {
    ::windows::core::link ! ( "schannel.dll""system" fn SslFreeCertificate ( pcertificate : *mut X509Certificate ) -> ( ) );
    SslFreeCertificate(pcertificate)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SslGenerateRandomBits(prandomdata: *mut u8, crandomdata: i32) {
    ::windows::core::link ! ( "schannel.dll""system" fn SslGenerateRandomBits ( prandomdata : *mut u8 , crandomdata : i32 ) -> ( ) );
    SslGenerateRandomBits(prandomdata, crandomdata)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SslGetExtensions(clienthello: &[u8], genericextensions: &mut [SCH_EXTENSION_DATA], bytestoread: *mut u32, flags: SchGetExtensionsOptions) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "schannel.dll""system" fn SslGetExtensions ( clienthello : *const u8 , clienthellobytesize : u32 , genericextensions : *mut SCH_EXTENSION_DATA , genericextensionscount : u8 , bytestoread : *mut u32 , flags : SchGetExtensionsOptions ) -> :: windows::core::HRESULT );
    SslGetExtensions(::core::mem::transmute(clienthello.as_ptr()), clienthello.len() as _, ::core::mem::transmute(genericextensions.as_ptr()), genericextensions.len() as _, bytestoread, flags).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SslGetMaximumKeySize(reserved: u32) -> u32 {
    ::windows::core::link ! ( "schannel.dll""system" fn SslGetMaximumKeySize ( reserved : u32 ) -> u32 );
    SslGetMaximumKeySize(reserved)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SslGetServerIdentity(clienthello: &[u8], serveridentity: *mut *mut u8, serveridentitysize: *mut u32, flags: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "schannel.dll""system" fn SslGetServerIdentity ( clienthello : *const u8 , clienthellosize : u32 , serveridentity : *mut *mut u8 , serveridentitysize : *mut u32 , flags : u32 ) -> :: windows::core::HRESULT );
    SslGetServerIdentity(::core::mem::transmute(clienthello.as_ptr()), clienthello.len() as _, serveridentity, serveridentitysize, flags).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SspiCompareAuthIdentities(authidentity1: ::core::option::Option<*const ::core::ffi::c_void>, authidentity2: ::core::option::Option<*const ::core::ffi::c_void>, samesupplieduser: ::core::option::Option<*mut super::super::super::Foundation::BOOLEAN>, samesuppliedidentity: ::core::option::Option<*mut super::super::super::Foundation::BOOLEAN>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiCompareAuthIdentities ( authidentity1 : *const ::core::ffi::c_void , authidentity2 : *const ::core::ffi::c_void , samesupplieduser : *mut super::super::super::Foundation:: BOOLEAN , samesuppliedidentity : *mut super::super::super::Foundation:: BOOLEAN ) -> :: windows::core::HRESULT );
    SspiCompareAuthIdentities(::core::mem::transmute(authidentity1.unwrap_or(::std::ptr::null())), ::core::mem::transmute(authidentity2.unwrap_or(::std::ptr::null())), ::core::mem::transmute(samesupplieduser.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(samesuppliedidentity.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiCopyAuthIdentity(authdata: *const ::core::ffi::c_void, authdatacopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiCopyAuthIdentity ( authdata : *const ::core::ffi::c_void , authdatacopy : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SspiCopyAuthIdentity(authdata, authdatacopy).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiDecryptAuthIdentity(encryptedauthdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiDecryptAuthIdentity ( encryptedauthdata : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SspiDecryptAuthIdentity(encryptedauthdata).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiDecryptAuthIdentityEx(options: u32, encryptedauthdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "sspicli.dll""system" fn SspiDecryptAuthIdentityEx ( options : u32 , encryptedauthdata : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SspiDecryptAuthIdentityEx(options, encryptedauthdata).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiEncodeAuthIdentityAsStrings(pauthidentity: *const ::core::ffi::c_void, ppszusername: *mut ::windows::core::PWSTR, ppszdomainname: *mut ::windows::core::PWSTR, ppszpackedcredentialsstring: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiEncodeAuthIdentityAsStrings ( pauthidentity : *const ::core::ffi::c_void , ppszusername : *mut :: windows::core::PWSTR , ppszdomainname : *mut :: windows::core::PWSTR , ppszpackedcredentialsstring : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    SspiEncodeAuthIdentityAsStrings(pauthidentity, ppszusername, ppszdomainname, ::core::mem::transmute(ppszpackedcredentialsstring.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiEncodeStringsAsAuthIdentity<P0, P1, P2>(pszusername: P0, pszdomainname: P1, pszpackedcredentialsstring: P2, ppauthidentity: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn SspiEncodeStringsAsAuthIdentity ( pszusername : :: windows::core::PCWSTR , pszdomainname : :: windows::core::PCWSTR , pszpackedcredentialsstring : :: windows::core::PCWSTR , ppauthidentity : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SspiEncodeStringsAsAuthIdentity(pszusername.into().abi(), pszdomainname.into().abi(), pszpackedcredentialsstring.into().abi(), ppauthidentity).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiEncryptAuthIdentity(authdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiEncryptAuthIdentity ( authdata : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SspiEncryptAuthIdentity(authdata).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiEncryptAuthIdentityEx(options: u32, authdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "sspicli.dll""system" fn SspiEncryptAuthIdentityEx ( options : u32 , authdata : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SspiEncryptAuthIdentityEx(options, authdata).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiExcludePackage<P0>(authidentity: ::core::option::Option<*const ::core::ffi::c_void>, pszpackagename: P0, ppnewauthidentity: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn SspiExcludePackage ( authidentity : *const ::core::ffi::c_void , pszpackagename : :: windows::core::PCWSTR , ppnewauthidentity : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SspiExcludePackage(::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), pszpackagename.into().abi(), ppnewauthidentity).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiFreeAuthIdentity(authdata: ::core::option::Option<*const ::core::ffi::c_void>) {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiFreeAuthIdentity ( authdata : *const ::core::ffi::c_void ) -> ( ) );
    SspiFreeAuthIdentity(::core::mem::transmute(authdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiGetTargetHostName<P0>(psztargetname: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn SspiGetTargetHostName ( psztargetname : :: windows::core::PCWSTR , pszhostname : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SspiGetTargetHostName(psztargetname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SspiIsAuthIdentityEncrypted(encryptedauthdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiIsAuthIdentityEncrypted ( encryptedauthdata : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOLEAN );
    SspiIsAuthIdentityEncrypted(encryptedauthdata)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SspiIsPromptingNeeded(errororntstatus: u32) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "credui.dll""system" fn SspiIsPromptingNeeded ( errororntstatus : u32 ) -> super::super::super::Foundation:: BOOLEAN );
    SspiIsPromptingNeeded(errororntstatus)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiLocalFree(databuffer: ::core::option::Option<*const ::core::ffi::c_void>) {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiLocalFree ( databuffer : *const ::core::ffi::c_void ) -> ( ) );
    SspiLocalFree(::core::mem::transmute(databuffer.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiMarshalAuthIdentity(authidentity: *const ::core::ffi::c_void, authidentitylength: *mut u32, authidentitybytearray: *mut *mut i8) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiMarshalAuthIdentity ( authidentity : *const ::core::ffi::c_void , authidentitylength : *mut u32 , authidentitybytearray : *mut *mut i8 ) -> :: windows::core::HRESULT );
    SspiMarshalAuthIdentity(authidentity, authidentitylength, authidentitybytearray).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiPrepareForCredRead<P0>(authidentity: *const ::core::ffi::c_void, psztargetname: P0, pcredmancredentialtype: *mut u32, ppszcredmantargetname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn SspiPrepareForCredRead ( authidentity : *const ::core::ffi::c_void , psztargetname : :: windows::core::PCWSTR , pcredmancredentialtype : *mut u32 , ppszcredmantargetname : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    SspiPrepareForCredRead(authidentity, psztargetname.into().abi(), pcredmancredentialtype, ppszcredmantargetname).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiPrepareForCredWrite<P0>(authidentity: *const ::core::ffi::c_void, psztargetname: P0, pcredmancredentialtype: *mut u32, ppszcredmantargetname: *mut ::windows::core::PWSTR, ppszcredmanusername: *mut ::windows::core::PWSTR, ppcredentialblob: *mut *mut u8, pcredentialblobsize: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn SspiPrepareForCredWrite ( authidentity : *const ::core::ffi::c_void , psztargetname : :: windows::core::PCWSTR , pcredmancredentialtype : *mut u32 , ppszcredmantargetname : *mut :: windows::core::PWSTR , ppszcredmanusername : *mut :: windows::core::PWSTR , ppcredentialblob : *mut *mut u8 , pcredentialblobsize : *mut u32 ) -> :: windows::core::HRESULT );
    SspiPrepareForCredWrite(authidentity, psztargetname.into().abi(), pcredmancredentialtype, ppszcredmantargetname, ppszcredmanusername, ppcredentialblob, pcredentialblobsize).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiPromptForCredentialsA<P0, P1>(psztargetname: P0, puiinfo: ::core::option::Option<*const ::core::ffi::c_void>, dwautherror: u32, pszpackage: P1, pinputauthidentity: ::core::option::Option<*const ::core::ffi::c_void>, ppauthidentity: *mut *mut ::core::ffi::c_void, pfsave: ::core::option::Option<*mut i32>, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "credui.dll""system" fn SspiPromptForCredentialsA ( psztargetname : :: windows::core::PCSTR , puiinfo : *const ::core::ffi::c_void , dwautherror : u32 , pszpackage : :: windows::core::PCSTR , pinputauthidentity : *const ::core::ffi::c_void , ppauthidentity : *mut *mut ::core::ffi::c_void , pfsave : *mut i32 , dwflags : u32 ) -> u32 );
    SspiPromptForCredentialsA(psztargetname.into().abi(), ::core::mem::transmute(puiinfo.unwrap_or(::std::ptr::null())), dwautherror, pszpackage.into().abi(), ::core::mem::transmute(pinputauthidentity.unwrap_or(::std::ptr::null())), ppauthidentity, ::core::mem::transmute(pfsave.unwrap_or(::std::ptr::null_mut())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiPromptForCredentialsW<P0, P1>(psztargetname: P0, puiinfo: ::core::option::Option<*const ::core::ffi::c_void>, dwautherror: u32, pszpackage: P1, pinputauthidentity: ::core::option::Option<*const ::core::ffi::c_void>, ppauthidentity: *mut *mut ::core::ffi::c_void, pfsave: ::core::option::Option<*mut i32>, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "credui.dll""system" fn SspiPromptForCredentialsW ( psztargetname : :: windows::core::PCWSTR , puiinfo : *const ::core::ffi::c_void , dwautherror : u32 , pszpackage : :: windows::core::PCWSTR , pinputauthidentity : *const ::core::ffi::c_void , ppauthidentity : *mut *mut ::core::ffi::c_void , pfsave : *mut i32 , dwflags : u32 ) -> u32 );
    SspiPromptForCredentialsW(psztargetname.into().abi(), ::core::mem::transmute(puiinfo.unwrap_or(::std::ptr::null())), dwautherror, pszpackage.into().abi(), ::core::mem::transmute(pinputauthidentity.unwrap_or(::std::ptr::null())), ppauthidentity, ::core::mem::transmute(pfsave.unwrap_or(::std::ptr::null_mut())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiUnmarshalAuthIdentity(authidentitybytearray: &[u8], ppauthidentity: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiUnmarshalAuthIdentity ( authidentitylength : u32 , authidentitybytearray : :: windows::core::PCSTR , ppauthidentity : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SspiUnmarshalAuthIdentity(authidentitybytearray.len() as _, ::core::mem::transmute(authidentitybytearray.as_ptr()), ppauthidentity).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiValidateAuthIdentity(authdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiValidateAuthIdentity ( authdata : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SspiValidateAuthIdentity(authdata).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn SspiZeroAuthIdentity(authdata: ::core::option::Option<*const ::core::ffi::c_void>) {
    ::windows::core::link ! ( "secur32.dll""system" fn SspiZeroAuthIdentity ( authdata : *const ::core::ffi::c_void ) -> ( ) );
    SspiZeroAuthIdentity(::core::mem::transmute(authdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemFunction036(randombuffer: *mut ::core::ffi::c_void, randombufferlength: u32) -> super::super::super::Foundation::BOOLEAN {
    ::windows::core::link ! ( "advapi32.dll""system" fn SystemFunction036 ( randombuffer : *mut ::core::ffi::c_void , randombufferlength : u32 ) -> super::super::super::Foundation:: BOOLEAN );
    SystemFunction036(randombuffer, randombufferlength)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemFunction040(memory: *mut ::core::ffi::c_void, memorysize: u32, optionflags: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "advapi32.dll""system" fn SystemFunction040 ( memory : *mut ::core::ffi::c_void , memorysize : u32 , optionflags : u32 ) -> super::super::super::Foundation:: NTSTATUS );
    SystemFunction040(memory, memorysize, optionflags).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemFunction041(memory: *mut ::core::ffi::c_void, memorysize: u32, optionflags: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "advapi32.dll""system" fn SystemFunction041 ( memory : *mut ::core::ffi::c_void , memorysize : u32 , optionflags : u32 ) -> super::super::super::Foundation:: NTSTATUS );
    SystemFunction041(memory, memorysize, optionflags).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn TokenBindingDeleteAllBindings() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "tokenbinding.dll""system" fn TokenBindingDeleteAllBindings ( ) -> :: windows::core::HRESULT );
    TokenBindingDeleteAllBindings().ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn TokenBindingDeleteBinding<P0>(targeturl: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tokenbinding.dll""system" fn TokenBindingDeleteBinding ( targeturl : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    TokenBindingDeleteBinding(targeturl.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn TokenBindingGenerateBinding<P0>(keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, targeturl: P0, bindingtype: TOKENBINDING_TYPE, tlsekm: *const ::core::ffi::c_void, tlsekmsize: u32, extensionformat: TOKENBINDING_EXTENSION_FORMAT, extensiondata: *const ::core::ffi::c_void, tokenbinding: *mut *mut ::core::ffi::c_void, tokenbindingsize: *mut u32, resultdata: ::core::option::Option<*mut *mut TOKENBINDING_RESULT_DATA>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tokenbinding.dll""system" fn TokenBindingGenerateBinding ( keytype : TOKENBINDING_KEY_PARAMETERS_TYPE , targeturl : :: windows::core::PCWSTR , bindingtype : TOKENBINDING_TYPE , tlsekm : *const ::core::ffi::c_void , tlsekmsize : u32 , extensionformat : TOKENBINDING_EXTENSION_FORMAT , extensiondata : *const ::core::ffi::c_void , tokenbinding : *mut *mut ::core::ffi::c_void , tokenbindingsize : *mut u32 , resultdata : *mut *mut TOKENBINDING_RESULT_DATA ) -> :: windows::core::HRESULT );
    TokenBindingGenerateBinding(keytype, targeturl.into().abi(), bindingtype, tlsekm, tlsekmsize, extensionformat, extensiondata, tokenbinding, tokenbindingsize, ::core::mem::transmute(resultdata.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn TokenBindingGenerateID(keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, publickey: *const ::core::ffi::c_void, publickeysize: u32) -> ::windows::core::Result<*mut TOKENBINDING_RESULT_DATA> {
    ::windows::core::link ! ( "tokenbinding.dll""system" fn TokenBindingGenerateID ( keytype : TOKENBINDING_KEY_PARAMETERS_TYPE , publickey : *const ::core::ffi::c_void , publickeysize : u32 , resultdata : *mut *mut TOKENBINDING_RESULT_DATA ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    TokenBindingGenerateID(keytype, publickey, publickeysize, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn TokenBindingGenerateIDForUri<P0>(keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, targeturi: P0) -> ::windows::core::Result<*mut TOKENBINDING_RESULT_DATA>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tokenbinding.dll""system" fn TokenBindingGenerateIDForUri ( keytype : TOKENBINDING_KEY_PARAMETERS_TYPE , targeturi : :: windows::core::PCWSTR , resultdata : *mut *mut TOKENBINDING_RESULT_DATA ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    TokenBindingGenerateIDForUri(keytype, targeturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn TokenBindingGenerateMessage(tokenbindings: *const *const ::core::ffi::c_void, tokenbindingssize: *const u32, tokenbindingscount: u32, tokenbindingmessage: *mut *mut ::core::ffi::c_void, tokenbindingmessagesize: *mut u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "tokenbinding.dll""system" fn TokenBindingGenerateMessage ( tokenbindings : *const *const ::core::ffi::c_void , tokenbindingssize : *const u32 , tokenbindingscount : u32 , tokenbindingmessage : *mut *mut ::core::ffi::c_void , tokenbindingmessagesize : *mut u32 ) -> :: windows::core::HRESULT );
    TokenBindingGenerateMessage(tokenbindings, tokenbindingssize, tokenbindingscount, tokenbindingmessage, tokenbindingmessagesize).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn TokenBindingGetHighestSupportedVersion(majorversion: *mut u8, minorversion: *mut u8) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "tokenbinding.dll""system" fn TokenBindingGetHighestSupportedVersion ( majorversion : *mut u8 , minorversion : *mut u8 ) -> :: windows::core::HRESULT );
    TokenBindingGetHighestSupportedVersion(majorversion, minorversion).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn TokenBindingGetKeyTypesClient() -> ::windows::core::Result<*mut TOKENBINDING_KEY_TYPES> {
    ::windows::core::link ! ( "tokenbinding.dll""system" fn TokenBindingGetKeyTypesClient ( keytypes : *mut *mut TOKENBINDING_KEY_TYPES ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    TokenBindingGetKeyTypesClient(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn TokenBindingGetKeyTypesServer() -> ::windows::core::Result<*mut TOKENBINDING_KEY_TYPES> {
    ::windows::core::link ! ( "tokenbinding.dll""system" fn TokenBindingGetKeyTypesServer ( keytypes : *mut *mut TOKENBINDING_KEY_TYPES ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    TokenBindingGetKeyTypesServer(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[inline]
pub unsafe fn TokenBindingVerifyMessage(tokenbindingmessage: *const ::core::ffi::c_void, tokenbindingmessagesize: u32, keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, tlsekm: *const ::core::ffi::c_void, tlsekmsize: u32) -> ::windows::core::Result<*mut TOKENBINDING_RESULT_LIST> {
    ::windows::core::link ! ( "tokenbinding.dll""system" fn TokenBindingVerifyMessage ( tokenbindingmessage : *const ::core::ffi::c_void , tokenbindingmessagesize : u32 , keytype : TOKENBINDING_KEY_PARAMETERS_TYPE , tlsekm : *const ::core::ffi::c_void , tlsekmsize : u32 , resultlist : *mut *mut TOKENBINDING_RESULT_LIST ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    TokenBindingVerifyMessage(tokenbindingmessage, tokenbindingmessagesize, keytype, tlsekm, tlsekmsize, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateNameA<P0>(lpaccountname: P0, accountnameformat: EXTENDED_NAME_FORMAT, desirednameformat: EXTENDED_NAME_FORMAT, lptranslatedname: ::windows::core::PSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn TranslateNameA ( lpaccountname : :: windows::core::PCSTR , accountnameformat : EXTENDED_NAME_FORMAT , desirednameformat : EXTENDED_NAME_FORMAT , lptranslatedname : :: windows::core::PSTR , nsize : *mut u32 ) -> super::super::super::Foundation:: BOOLEAN );
    TranslateNameA(lpaccountname.into().abi(), accountnameformat, desirednameformat, ::core::mem::transmute(lptranslatedname), nsize)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateNameW<P0>(lpaccountname: P0, accountnameformat: EXTENDED_NAME_FORMAT, desirednameformat: EXTENDED_NAME_FORMAT, lptranslatedname: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "secur32.dll""system" fn TranslateNameW ( lpaccountname : :: windows::core::PCWSTR , accountnameformat : EXTENDED_NAME_FORMAT , desirednameformat : EXTENDED_NAME_FORMAT , lptranslatedname : :: windows::core::PWSTR , nsize : *mut u32 ) -> super::super::super::Foundation:: BOOLEAN );
    TranslateNameW(lpaccountname.into().abi(), accountnameformat, desirednameformat, ::core::mem::transmute(lptranslatedname), nsize)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn VerifySignature(phcontext: *const super::super::Credentials::SecHandle, pmessage: *const SecBufferDesc, messageseqno: u32) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "secur32.dll""system" fn VerifySignature ( phcontext : *const super::super::Credentials:: SecHandle , pmessage : *const SecBufferDesc , messageseqno : u32 , pfqop : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VerifySignature(phcontext, pmessage, messageseqno, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
pub struct ICcgDomainAuthCredentials(::windows::core::IUnknown);
impl ICcgDomainAuthCredentials {
    pub unsafe fn GetPasswordCredentials<P0>(&self, plugininput: P0, domainname: *mut ::windows::core::PWSTR, username: *mut ::windows::core::PWSTR, password: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetPasswordCredentials)(::windows::core::Vtable::as_raw(self), plugininput.into().abi(), domainname, username, password).ok()
    }
}
::windows::core::interface_hierarchy!(ICcgDomainAuthCredentials, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICcgDomainAuthCredentials {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ICcgDomainAuthCredentials {
    type Vtable = ICcgDomainAuthCredentials_Vtbl;
}
unsafe impl ::windows::core::Interface for ICcgDomainAuthCredentials {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ecda518_2010_4437_8bc3_46e752b7b172);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICcgDomainAuthCredentials_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPasswordCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plugininput: ::windows::core::PCWSTR, domainname: *mut ::windows::core::PWSTR, username: *mut ::windows::core::PWSTR, password: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ACCOUNT_ADJUST_PRIVILEGES: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ACCOUNT_ADJUST_QUOTAS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ACCOUNT_ADJUST_SYSTEM_ACCESS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ACCOUNT_VIEW: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_ALLOCATED_MEMORY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_ALLOW_CONTEXT_REPLAY: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_ALLOW_NON_USER_LOGONS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_CALL_LEVEL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_CONNECTION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_DATAGRAM: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_DELEGATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_EXTENDED_ERROR: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_FRAGMENT_ONLY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_IDENTIFY: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_INTEGRITY: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_LICENSING: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_MESSAGES: u64 = 4294967296u64;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_MUTUAL_AUTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_NO_ADDITIONAL_TOKEN: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_NO_TOKEN: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_NULL_SESSION: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_REPLAY_DETECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_SEQUENCE_DETECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_SESSION_TICKET: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_STREAM: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_THIRD_LEG_FAILED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_USED_DCE_STYLE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_RET_USE_SESSION_KEY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUDIT_ENUMERATE_USERS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUDIT_QUERY_MISC_POLICY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUDIT_QUERY_SYSTEM_POLICY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUDIT_QUERY_USER_POLICY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUDIT_SET_MISC_POLICY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUDIT_SET_SYSTEM_POLICY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUDIT_SET_USER_POLICY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_ALLOW_ENC_TKT_IN_SKEY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_ALLOW_FORWARDABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_ALLOW_NOADDRESS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_ALLOW_POSTDATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_ALLOW_PROXIABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_ALLOW_RENEWABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_ALLOW_S4U_DELEGATE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_ALLOW_VALIDATE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_OK_AS_DELEGATE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_PREAUTH_REQUIRED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_TRANSITIVE_TRUST: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AUTH_REQ_VALIDATE_CLIENT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountLogon: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69979850_797a_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountLogon_CredentialValidation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce923f_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountLogon_KerbCredentialValidation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9242_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountLogon_Kerberos: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9240_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountLogon_Others: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9241_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountManagement: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6997984e_797a_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountManagement_ApplicationGroup: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9239_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountManagement_ComputerAccount: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9236_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountManagement_DistributionGroup: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9238_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountManagement_Others: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce923a_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountManagement_SecurityGroup: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9237_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_AccountManagement_UserAccount: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9235_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_DSAccess_DSAccess: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce923b_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_DetailedTracking: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6997984c_797a_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_DetailedTracking_DpapiActivity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce922d_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_DetailedTracking_PnpActivity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9248_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_DetailedTracking_ProcessCreation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce922b_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_DetailedTracking_ProcessTermination: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce922c_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_DetailedTracking_RpcCall: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce922e_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_DetailedTracking_TokenRightAdjusted: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce924a_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_DirectoryServiceAccess: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6997984f_797a_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_DsAccess_AdAuditChanges: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce923c_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Ds_DetailedReplication: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce923e_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Ds_Replication: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce923d_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69979849_797a_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon_AccountLockout: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9217_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon_Claims: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9247_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon_Groups: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9249_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon_IPSecMainMode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9218_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon_IPSecQuickMode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9219_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon_IPSecUserMode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce921a_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon_Logoff: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9216_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon_Logon: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9215_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon_NPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9243_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon_Others: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce921c_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_Logon_SpecialLogon: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce921b_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6997984a_797a_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_ApplicationGenerated: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9222_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_CbacStaging: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9246_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_CertificationServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9221_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_DetailedFileShare: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9244_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_FileSystem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce921d_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_FirewallConnection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9226_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_FirewallPacketDrops: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9225_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_Handle: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9223_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_Kernel: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce921f_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_Other: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9227_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_Registry: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce921e_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_RemovableStorage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9245_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_Sam: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9220_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_ObjectAccess_Share: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9224_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_PolicyChange: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6997984d_797a_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_PolicyChange_AuditPolicy: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce922f_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_PolicyChange_AuthenticationPolicy: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9230_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_PolicyChange_AuthorizationPolicy: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9231_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_PolicyChange_MpsscvRulePolicy: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9232_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_PolicyChange_Others: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9234_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_PolicyChange_WfpIPSecPolicy: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9233_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_PrivilegeUse: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6997984b_797a_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_PrivilegeUse_NonSensitive: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9229_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_PrivilegeUse_Others: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce922a_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_PrivilegeUse_Sensitive: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9228_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_System: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69979848_797a_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_System_IPSecDriverEvents: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9213_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_System_Integrity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9212_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_System_Others: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9214_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_System_SecurityStateChange: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9210_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Audit_System_SecuritySubsystemExtension: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cce9211_69ae_11d9_bed3_505054503030);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CENTRAL_ACCESS_POLICY_STAGED_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CLEAR_BLOCK_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CLOUDAP_NAME: ::windows::core::PCWSTR = ::windows::w!("CloudAP");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CLOUDAP_NAME_W: ::windows::core::PCWSTR = ::windows::w!("CloudAP");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CREDP_FLAGS_CLEAR_PASSWORD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CREDP_FLAGS_DONT_CACHE_TI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CREDP_FLAGS_IN_PROCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CREDP_FLAGS_TRUSTED_CALLER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CREDP_FLAGS_USER_ENCRYPTED_PASSWORD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CREDP_FLAGS_USE_MIDL_HEAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CREDP_FLAGS_VALIDATE_PROXY_TARGET: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CRED_MARSHALED_TI_SIZE_SIZE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CYPHER_BLOCK_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ClOUDAP_NAME_A: ::windows::core::PCSTR = ::windows::s!("CloudAP");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DEFAULT_TLS_SSP_NAME: ::windows::core::PCWSTR = ::windows::w!("Default TLS SSP");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DEFAULT_TLS_SSP_NAME_A: ::windows::core::PCSTR = ::windows::s!("Default TLS SSP");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DEFAULT_TLS_SSP_NAME_W: ::windows::core::PCWSTR = ::windows::w!("Default TLS SSP");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DOMAIN_NO_LM_OWF_CHANGE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DS_UNKNOWN_ADDRESS_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ENABLE_TLS_CLIENT_EARLY_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const E_RM_UNKNOWN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073415165i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const FACILITY_SL_ITF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ID_CAP_SLAPI: ::windows::core::PCWSTR = ::windows::w!("slapiQueryLicenseValue");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_ALLOCATED_MEMORY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_CALL_LEVEL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_CONFIDENTIALITY_ONLY: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_CONNECTION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_DATAGRAM: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_DEFERRED_CRED_VALIDATION: u64 = 8589934592u64;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_DELEGATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_EXTENDED_ERROR: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_FORWARD_CREDENTIALS: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_FRAGMENT_ONLY: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_IDENTIFY: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_INTEGRITY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_INTERMEDIATE_RETURN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_MANUAL_CRED_VALIDATION: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_MESSAGES: u64 = 4294967296u64;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_MUTUAL_AUTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_NO_ADDITIONAL_TOKEN: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_NULL_SESSION: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_REAUTHENTICATION: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_REPLAY_DETECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_RESERVED1: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_SEQUENCE_DETECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_STREAM: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_USED_COLLECTED_CREDS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_USED_DCE_STYLE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_USED_HTTP_STYLE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_USED_SUPPLIED_CREDS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_RET_USE_SESSION_KEY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISSP_LEVEL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISSP_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KDC_PROXY_SETTINGS_FLAGS_FORCEPROXY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KDC_PROXY_SETTINGS_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERBEROS_REVISION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERBEROS_VERSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_DUPLICATES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_FAIL_IF_NT_AUTH_POLICY_REQUIRED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_IDENTIFY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_CRC32: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_DES_MAC: i32 = -133i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_DES_MAC_MD5: i32 = -134i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_HMAC_MD5: i32 = -138i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128_Ki: i32 = -150i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256_Ki: i32 = -151i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_KRB_DES_MAC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_KRB_DES_MAC_K: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_LM: i32 = -130i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_MD25: i32 = -135i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_MD4: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_MD5: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_MD5_DES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_MD5_HMAC: i32 = -137i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_RC4_MD5: i32 = -136i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_REAL_CRC32: i32 = -132i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_SHA1: i32 = -131i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CHECKSUM_SHA1_NEW: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_CLOUD_KERBEROS_DEBUG_DATA_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_DECRYPT_FLAG_DEFAULT_KEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96_PLAIN: i32 = -148i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96_PLAIN: i32 = -149i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DES3_CBC_MD5: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DES3_CBC_SHA1: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DES3_CBC_SHA1_KD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DES_CBC_MD5_NT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DES_EDE3_CBC_ENV: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DES_PLAIN: i32 = -132i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DSA_SHA1_CMS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DSA_SIGN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_PKCS7_PUB: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC2_CBC_ENV: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_HMAC_NT_EXP: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_HMAC_OLD: i32 = -133i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_HMAC_OLD_EXP: i32 = -135i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_LM: i32 = -130i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_PLAIN: i32 = -140i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_PLAIN2: i32 = -129i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_PLAIN_EXP: i32 = -141i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_PLAIN_OLD: i32 = -134i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_PLAIN_OLD_EXP: i32 = -136i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_SHA: i32 = -131i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RSA_ENV: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RSA_ES_OEAP_ENV: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RSA_MD5_CMS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RSA_PRIV: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RSA_PUB: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RSA_PUB_MD5: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RSA_PUB_SHA1: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RSA_SHA1_CMS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_LOGON_FLAG_ALLOW_EXPIRED_TICKET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_LOGON_FLAG_REDIRECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_PURGE_ALL_TICKETS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE_FLAG_DAC_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_REFRESH_POLICY_KDC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_REFRESH_POLICY_KERBEROS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_REFRESH_SCCRED_GETTGT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_REFRESH_SCCRED_RELEASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_RETRIEVE_TICKET_AS_KERB_CRED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_RETRIEVE_TICKET_CACHE_TICKET: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_RETRIEVE_TICKET_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_RETRIEVE_TICKET_DONT_USE_CACHE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_RETRIEVE_TICKET_MAX_LIFETIME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_RETRIEVE_TICKET_USE_CACHE_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_RETRIEVE_TICKET_USE_CREDHANDLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_RETRIEVE_TICKET_WITH_SEC_CRED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_S4U2PROXY_CACHE_ENTRY_INFO_FLAG_NEGATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_S4U2PROXY_CRED_FLAG_NEGATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_S4U_LOGON_FLAG_IDENTIFY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_SETPASS_USE_CREDHANDLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_SETPASS_USE_LOGONID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_cname_in_pa_data: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_enc_pa_rep: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_name_canonicalize: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TRANSFER_CRED_CLEANUP_CREDENTIALS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TRANSFER_CRED_WITH_TICKETS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_USE_DEFAULT_TICKET_FLAGS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_WRAP_NO_ENCRYPT: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERN_CONTEXT_CERT_INFO_V1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_ANONYMOUS_STRING: ::windows::core::PCWSTR = ::windows::w!("ANONYMOUS");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_ENTERPRISE_PRINCIPAL: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_ENT_PRINCIPAL_AND_ID: i32 = -130i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_MS_BRANCH_ID: i32 = -133i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_MS_PRINCIPAL: i32 = -128i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_MS_PRINCIPAL_AND_ID: i32 = -129i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_PRINCIPAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_PRINCIPAL_AND_ID: i32 = -131i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_SRV_HST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_SRV_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_SRV_INST_AND_ID: i32 = -132i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_SRV_XHST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_UID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_WELLKNOWN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_NT_X500_PRINCIPAL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KRB_WELLKNOWN_STRING: ::windows::core::PCWSTR = ::windows::w!("WELLKNOWN");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LCRED_CRED_EXISTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LCRED_STATUS_NOCRED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LCRED_STATUS_UNKNOWN_ISSUER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_GRACE_LOGON: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_LM_V2: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_MANAGED_SERVICE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_NO_ELEVATION: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_NO_OPTIMIZED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_NTLMV2_ENABLED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_NTLM_V2: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_NT_V2: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_OPTIMIZED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_PKINIT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_WINLOGON: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOOKUP_TRANSLATE_NAMES: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOOKUP_VIEW_LOCAL_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSASETCAPS_RELOAD_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSASETCAPS_VALID_FLAG_MASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_ADT_LEGACY_SECURITY_SOURCE_NAME: ::windows::core::PCWSTR = ::windows::w!("Security");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_ADT_SECURITY_SOURCE_NAME: ::windows::core::PCWSTR = ::windows::w!("Microsoft-Windows-Security-Auditing");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_AP_NAME_CALL_PACKAGE: ::windows::core::PCSTR = ::windows::s!("LsaApCallPackage\u{0}");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_AP_NAME_CALL_PACKAGE_PASSTHROUGH: ::windows::core::PCSTR = ::windows::s!("LsaApCallPackagePassthrough\u{0}");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_AP_NAME_CALL_PACKAGE_UNTRUSTED: ::windows::core::PCSTR = ::windows::s!("LsaApCallPackageUntrusted\u{0}");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_AP_NAME_INITIALIZE_PACKAGE: ::windows::core::PCSTR = ::windows::s!("LsaApInitializePackage\u{0}");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_AP_NAME_LOGON_TERMINATED: ::windows::core::PCSTR = ::windows::s!("LsaApLogonTerminated\u{0}");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_AP_NAME_LOGON_USER: ::windows::core::PCSTR = ::windows::s!("LsaApLogonUser\u{0}");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_AP_NAME_LOGON_USER_EX: ::windows::core::PCSTR = ::windows::s!("LsaApLogonUserEx\u{0}");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_AP_NAME_LOGON_USER_EX2: ::windows::core::PCSTR = ::windows::s!("LsaApLogonUserEx2\u{0}");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_CALL_LICENSE_SERVER: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_FOREST_TRUST_RECORD_TYPE_UNRECOGNIZED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_FTRECORD_DISABLED_REASONS: i32 = 65535i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_GLOBAL_SECRET_PREFIX: ::windows::core::PCWSTR = ::windows::w!("G$");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_GLOBAL_SECRET_PREFIX_LENGTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_LOCAL_SECRET_PREFIX: ::windows::core::PCWSTR = ::windows::w!("L$");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_LOCAL_SECRET_PREFIX_LENGTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_LOOKUP_DISALLOW_CONNECTED_ACCOUNT_INTERNET_SID: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_LOOKUP_ISOLATED_AS_LOCAL: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_LOOKUP_PREFER_INTERNET_NAMES: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_MACHINE_SECRET_PREFIX: ::windows::core::PCWSTR = ::windows::w!("M$");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_MAXIMUM_ENUMERATION_LENGTH: u32 = 32000u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_MAXIMUM_SID_COUNT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_MODE_INDIVIDUAL_ACCOUNTS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_MODE_LOG_FULL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_MODE_MANDATORY_ACCESS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_MODE_PASSWORD_PROTECTED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_NB_DISABLED_ADMIN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_NB_DISABLED_CONFLICT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_QUERY_CLIENT_PRELOGON_SESSION_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_SECRET_MAXIMUM_COUNT: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_SECRET_MAXIMUM_LENGTH: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_SID_DISABLED_ADMIN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_SID_DISABLED_CONFLICT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_TLN_DISABLED_ADMIN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_TLN_DISABLED_CONFLICT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LSA_TLN_DISABLED_NEW: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MAXIMUM_CAPES_PER_CAP: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MAX_CRED_SIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MAX_PROTOCOL_ID_SIZE: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MAX_RECORDS_IN_FOREST_TRUST_INFO: u32 = 4000u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MAX_USER_RECORDS: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MICROSOFT_KERBEROS_NAME: ::windows::core::PCWSTR = ::windows::w!("Kerberos");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MICROSOFT_KERBEROS_NAME_A: ::windows::core::PCSTR = ::windows::s!("Kerberos");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MICROSOFT_KERBEROS_NAME_W: ::windows::core::PCWSTR = ::windows::w!("Kerberos");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_ALLOW_FORCE_GUEST: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_ALLOW_MSVCHAPV2: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_AV_FLAG_FORCE_GUEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_AV_FLAG_MIC_HANDSHAKE_MESSAGES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_AV_FLAG_UNVERIFIED_TARGET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CHALLENGE_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CHECK_LOGONHOURS_FOR_S4U: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CLEARTEXT_PASSWORD_SUPPLIED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CREDENTIAL_KEY_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_CREDKEY_PRESENT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_REMOVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_SHA_PRESENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_VERSION_ARSO: u32 = 4294901763u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_VERSION_INVALID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_VERSION_IUM: u32 = 4294901761u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_VERSION_REMOTE: u32 = 4294901762u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_VERSION_RESERVED_1: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_VERSION_V2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_VERSION_V3: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_DISABLE_PERSONAL_FALLBACK: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_INTERNET_DOMAIN: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_LANMAN_SESSION_KEY_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_MAX_AVL_SIZE: u32 = 64000u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_MAX_NTLM3_LIFE: u32 = 1800u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_MNS_LOGON: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_NTLM3_OWF_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_NTLM3_RESPONSE_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_OWF_PASSWORD_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_PACKAGE_NAME: ::windows::core::PCSTR = ::windows::s!("MICROSOFT_AUTHENTICATION_PACKAGE_V1_0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_PACKAGE_NAMEW: ::windows::core::PCWSTR = ::windows::w!("MICROSOFT_AUTHENTICATION_PACKAGE_V1_0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_S4U2SELF: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SHA_PASSWORD_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_EX: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_IIS: u32 = 132u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_RAS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_SHIFT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_FLAGS: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_KEY: ::windows::core::PCSTR = ::windows::s!("SYSTEM\\CurrentControlSet\\Control\\Lsa\\MSV1_0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTHENTICATION_VALUE: ::windows::core::PCSTR = ::windows::s!("Auth");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTH_ACCOUNT_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTH_ACCOUNT_EXPIRY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTH_ACCOUNT_TYPE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTH_LOCKOUT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTH_LOGON_HOURS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTH_PASSWORD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTH_PASSWORD_EXPIRY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_SUBAUTH_WORKSTATIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_USER_SESSION_KEY_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_USE_CLIENT_CHALLENGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_USE_DOMAIN_FOR_ROUTING_ONLY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_VALIDATION_KICKOFF_TIME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_VALIDATION_LOGOFF_TIME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_VALIDATION_LOGON_DOMAIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_VALIDATION_LOGON_SERVER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_VALIDATION_SESSION_KEY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_VALIDATION_USER_FLAGS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_VALIDATION_USER_ID: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NEGOSSP_NAME: ::windows::core::PCWSTR = ::windows::w!("Negotiate");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NEGOSSP_NAME_A: ::windows::core::PCSTR = ::windows::s!("Negotiate");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NEGOSSP_NAME_W: ::windows::core::PCWSTR = ::windows::w!("Negotiate");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NEGOTIATE_ALLOW_NTLM: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NEGOTIATE_MAX_PREFIX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NEGOTIATE_NEG_NTLM: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NGC_DATA_FLAG_IS_CLOUD_TRUST_CRED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NGC_DATA_FLAG_IS_SMARTCARD_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NGC_DATA_FLAG_KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NGC_DATA_FLAG_KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFIER_FLAG_NEW_THREAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFIER_FLAG_ONE_SHOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFIER_FLAG_SECONDS: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFIER_TYPE_HANDLE_WAIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFIER_TYPE_IMMEDIATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFIER_TYPE_INTERVAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFIER_TYPE_NOTIFY_EVENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFIER_TYPE_STATE_CHANGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFY_CLASS_DOMAIN_CHANGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFY_CLASS_PACKAGE_CHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFY_CLASS_REGISTRY_CHANGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NOTIFY_CLASS_ROLE_CHANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NO_LONG_NAMES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NTLMSP_NAME: ::windows::core::PCWSTR = ::windows::w!("NTLM");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NTLMSP_NAME_A: ::windows::core::PCSTR = ::windows::s!("NTLM");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PCT1SP_NAME: ::windows::core::PCWSTR = ::windows::w!("Microsoft PCT 1.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PCT1SP_NAME_A: ::windows::core::PCSTR = ::windows::s!("Microsoft PCT 1.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PCT1SP_NAME_W: ::windows::core::PCWSTR = ::windows::w!("Microsoft PCT 1.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PER_USER_AUDIT_FAILURE_EXCLUDE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PER_USER_AUDIT_FAILURE_INCLUDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PER_USER_AUDIT_NONE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PER_USER_AUDIT_SUCCESS_EXCLUDE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PER_USER_AUDIT_SUCCESS_INCLUDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PER_USER_POLICY_UNCHANGED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PKU2U_PACKAGE_NAME: ::windows::core::PCWSTR = ::windows::w!("pku2u");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PKU2U_PACKAGE_NAME_A: ::windows::core::PCSTR = ::windows::s!("pku2u");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PKU2U_PACKAGE_NAME_W: ::windows::core::PCWSTR = ::windows::w!("pku2u");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_AUDIT_EVENT_FAILURE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_AUDIT_EVENT_NONE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_AUDIT_EVENT_SUCCESS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_AUDIT_EVENT_UNCHANGED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_AUDIT_LOG_ADMIN: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_CREATE_ACCOUNT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_CREATE_PRIVILEGE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_CREATE_SECRET: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_GET_PRIVATE_INFORMATION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_KERBEROS_VALIDATE_CLIENT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_LOOKUP_NAMES: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_NOTIFICATION: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_QOS_ALLOW_LOCAL_ROOT_CERT_STORE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_QOS_DHCP_SERVER_ALLOWED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_QOS_INBOUND_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_QOS_INBOUND_INTEGRITY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_QOS_OUTBOUND_CONFIDENTIALITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_QOS_OUTBOUND_INTEGRITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_QOS_RAS_SERVER_ALLOWED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_QOS_SCHANNEL_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_SERVER_ADMIN: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_SET_AUDIT_REQUIREMENTS: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_SET_DEFAULT_QUOTA_LIMITS: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_TRUST_ADMIN: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_VIEW_AUDIT_INFORMATION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const POLICY_VIEW_LOCAL_INFORMATION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_ARSO_LOGON: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_AUTH_ID: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_CACHED_INTERACTIVE_LOGON: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_CACHED_LOGON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_CLEAR_PASSWORD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_DO_NOT_SPLIT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_ENCRYPTED_CREDGUARD_PASSWORD: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_ENTERPRISE_INTERNET_USER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_EX: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_INTERACTIVE_FIDO_LOGON: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_INTERACTIVE_NGC_LOGON: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_INTERACTIVE_SMARTCARD_LOGON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_INTERNET_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_LOGON_LUA: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_LOGON_NO_TCB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_LOGON_PACKAGE_SHIFT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_OWF_PASSWORD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_PACKAGE_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_PACKED_CREDS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_PROTECTED_USER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_REFRESH_NEEDED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_RESTRICTED_TS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_SUPPLEMENTAL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_TRANSFER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PRIMARY_CRED_UPDATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const RCRED_CRED_EXISTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const RCRED_STATUS_NOCRED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const RCRED_STATUS_UNKNOWN_ISSUER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const RTL_ENCRYPT_MEMORY_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const RTL_ENCRYPT_OPTION_CROSS_PROCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const RTL_ENCRYPT_OPTION_FOR_SYSTEM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const RTL_ENCRYPT_OPTION_SAME_LOGON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SAM_CREDENTIAL_UPDATE_FREE_ROUTINE: ::windows::core::PCSTR = ::windows::s!("CredentialUpdateFree");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SAM_CREDENTIAL_UPDATE_NOTIFY_ROUTINE: ::windows::core::PCSTR = ::windows::s!("CredentialUpdateNotify");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SAM_CREDENTIAL_UPDATE_REGISTER_MAPPED_ENTRYPOINTS_ROUTINE: ::windows::core::PCSTR = ::windows::s!("RegisterMappedEntrypoints");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SAM_CREDENTIAL_UPDATE_REGISTER_ROUTINE: ::windows::core::PCSTR = ::windows::s!("CredentialUpdateRegister");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SAM_DAYS_PER_WEEK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SAM_INIT_NOTIFICATION_ROUTINE: ::windows::core::PCSTR = ::windows::s!("InitializeChangeNotify");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SAM_PASSWORD_CHANGE_NOTIFY_ROUTINE: ::windows::core::PCSTR = ::windows::s!("PasswordChangeNotify");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SAM_PASSWORD_FILTER_ROUTINE: ::windows::core::PCSTR = ::windows::s!("PasswordFilter");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SASL_OPTION_AUTHZ_PROCESSING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SASL_OPTION_AUTHZ_STRING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SASL_OPTION_RECV_SIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SASL_OPTION_SEND_SIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCHANNEL_ALERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCHANNEL_CRED_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCHANNEL_NAME: ::windows::core::PCWSTR = ::windows::w!("Schannel");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCHANNEL_NAME_A: ::windows::core::PCSTR = ::windows::s!("Schannel");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCHANNEL_NAME_W: ::windows::core::PCWSTR = ::windows::w!("Schannel");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCHANNEL_RENEGOTIATE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCHANNEL_SECRET_PRIVKEY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCHANNEL_SECRET_TYPE_CAPI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCHANNEL_SESSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCHANNEL_SHUTDOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_ALLOW_NULL_ENCRYPTION: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CREDENTIALS_VERSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_CACHE_ONLY_URL_RETRIEVAL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_CERT_CONTEXT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_DEFERRED_CRED_VALIDATION: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_DISABLE_RECONNECTS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_FORMAT_CERT_CONTEXT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_FORMAT_CERT_HASH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_FORMAT_CERT_HASH_STORE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_MAX_STORE_NAME_SIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_MAX_SUPPORTED_ALGS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_MAX_SUPPORTED_ALPN_IDS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_MAX_SUPPORTED_CERTS: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_MAX_SUPPORTED_CHAINING_MODES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_MAX_SUPPORTED_CRYPTO_SETTINGS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_MAX_SUPPORTED_PARAMETERS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_MEMORY_STORE_CERT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_RESTRICTED_ROOTS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_REVOCATION_CHECK_CACHE_ONLY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_SNI_CREDENTIAL: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_SNI_ENABLE_OCSP: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_V2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_V3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_X509_CAPI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_X509_CERTCHAIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_MACHINE_CERT_HASH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_MAX_EXT_SUBSCRIPTIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_USE_DTLS_ONLY: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_ALERT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_APPLICATION_PROTOCOLS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_ATTRMASK: u32 = 4026531840u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_CHANGE_PASS_RESPONSE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_CHANNEL_BINDINGS: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_DTLS_MTU: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_EMPTY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_EXTRA: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_FLAGS: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_KERNEL_MAP: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_MECHLIST: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_MECHLIST_SIGNATURE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_MISSING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_NEGOTIATION_INFO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_PADDING: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_PKG_PARAMS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_PRESHARED_KEY: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_PRESHARED_KEY_IDENTITY: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_READONLY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_READONLY_WITH_CHECKSUM: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_RESERVED: u32 = 1610612736u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_SEND_GENERIC_TLS_EXTENSION: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_SRTP_MASTER_KEY_IDENTIFIER: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_SRTP_PROTECTION_PROFILES: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_STREAM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_STREAM_HEADER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_STREAM_TRAILER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_SUBSCRIBE_GENERIC_TLS_EXTENSION: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_TARGET: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_TARGET_HOST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_TOKEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_TOKEN_BINDING: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_TRAFFIC_SECRETS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_UNMAPPED: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECBUFFER_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKGCONTEXT_CIPHERINFO_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKGCONTEXT_CONNECTION_INFO_EX_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ANSI_ATTRIBUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_APPLICATION_PROTOCOL: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_AUTHENTICATION_ID: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CC_POLICY_RESULT: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CERT_CHECK_RESULT: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CERT_CHECK_RESULT_INPROC: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CIPHER_INFO: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CIPHER_STRENGTHS: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CLIENT_CERT_POLICY: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CONNECTION_INFO_EX: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CONTEXT_DELETED: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CREDENTIAL_NAME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_ISSUER_LIST: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_IS_LOOPBACK: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_KEYING_MATERIAL: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_KEYING_MATERIAL_INPROC: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_KEYING_MATERIAL_TOKEN_BINDING: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_LOCAL_CERT_INFO: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_LOGOFF_TIME: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_MAPPED_CRED_ATTR: u32 = 92u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_NEGOTIATED_TLS_EXTENSIONS: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_KERBEROS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_NTLM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_NEGO_KEYS: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_NEGO_PKG_INFO: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_NEGO_STATUS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_PROMPTING_NEEDED: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_PROTO_INFO: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_REMOTE_CERTIFICATES: u32 = 95u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_REMOTE_CERT_CHAIN: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_REMOTE_CRED: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_SASL_CONTEXT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_SESSION_TICKET_KEYS: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_SRTP_PARAMETERS: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_SUPPORTED_ALGS: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_SUPPORTED_PROTOCOLS: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_TARGET: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_THUNK_ALL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_TOKEN_BINDING: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_UI_INFO: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_USER_FLAGS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_USE_NCRYPT: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_USE_VALIDATED: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALLFLAGS_APPCONTAINER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALLFLAGS_APPCONTAINER_AUTHCAPABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALLFLAGS_APPCONTAINER_UPNCAPABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALLFLAGS_FORCE_SUPPLIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_ANSI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_ASYNC_UPDATE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_BUFFER_MARSHAL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_CLEANUP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_CLOUDAP_CONNECT: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_IN_PROC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_IS_TCB: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_KERNEL_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_NEGO: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_NEGO_EXTENDER: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_NETWORK_ONLY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_CLEANUP_CREDENTIALS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_OPTIMISTIC_LOGON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_TO_SSO_SESSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_PROCESS_TERM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_RECURSIVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_SYSTEM_PROC: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_THREAD_TERM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_UNLOCK: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_URGENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_WINLOGON: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_WOWA32: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_WOWCLIENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CALL_WOWX86: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CLIENT_PROCESS_TERMINATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CLIENT_THREAD_TERMINATED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CREDENTIAL_ATTRIBUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CREDENTIAL_FLAGS_CALLER_HAS_TCB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CREDENTIAL_FLAGS_CREDMAN_CRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CREDENTIAL_VERSION: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_ATTR_CERT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_ATTR_KDC_PROXY_SETTINGS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_ATTR_NAMES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_ATTR_PAC_BYPASS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_ATTR_SSI_PROVIDER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_AUTOLOGON_RESTRICTED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_PROCESS_POLICY_ONLY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_RESERVED: u32 = 4026531840u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_ACCEPT_WIN32_NAME: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_APPCONTAINER_CHECKS: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_APPCONTAINER_PASSTHROUGH: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_APPLY_LOOPBACK: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_ASCII_BUFFERS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_CLIENT_ONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_CONNECTION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_CREDENTIAL_ISOLATION_ENABLED: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_DATAGRAM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_DELEGATION: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_EXTENDED_ERROR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_FRAGMENT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_GSS_COMPATIBLE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_IMPERSONATION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_INTEGRITY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_LOGON: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_MULTI_REQUIRED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_MUTUAL_AUTH: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_NEGOTIABLE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_NEGOTIABLE2: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_NEGO_EXTENDER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_PRIVACY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_READONLY_WITH_CHECKSUM: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_RESTRICTED_TOKENS: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_STREAM: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_FLAG_TOKEN_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ID_NONE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_INTERFACE_VERSION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_INTERFACE_VERSION_10: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_INTERFACE_VERSION_2: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_INTERFACE_VERSION_3: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_INTERFACE_VERSION_4: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_INTERFACE_VERSION_5: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_INTERFACE_VERSION_6: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_INTERFACE_VERSION_7: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_INTERFACE_VERSION_8: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_INTERFACE_VERSION_9: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_LSAMODEINIT_NAME: ::windows::core::PCSTR = ::windows::s!("SpLsaModeInitialize");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_MAX_OID_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_NEGOTIATION_COMPLETE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_NEGOTIATION_DIRECT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_NEGOTIATION_IN_PROGRESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_NEGOTIATION_OPTIMISTIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_NEGOTIATION_TRY_MULTICRED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_OPTIONS_PERMANENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_PRIMARY_CRED_EX_FLAGS_EX_DELEGATION_TOKEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_STATE_CRED_ISOLATION_ENABLED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_STATE_DOMAIN_CONTROLLER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_STATE_ENCRYPTION_PERMITTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_STATE_RESERVED_1: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_STATE_STANDALONE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_STATE_STRONG_ENCRYPTION_PERMITTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_STATE_WORKSTATION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_SURROGATE_LOGON_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_UNICODE_ATTRIBUTE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_USERMODEINIT_NAME: ::windows::core::PCSTR = ::windows::s!("SpUserModeInitialize");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECQOP_WRAP_NO_ENCRYPT: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECQOP_WRAP_OOB_DATA: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECRET_QUERY_VALUE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECRET_SET_VALUE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_ENTRYPOINT: ::windows::core::PCWSTR = ::windows::w!("INITSECURITYINTERFACEA");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_ENTRYPOINT16: ::windows::core::PCSTR = ::windows::s!("INITSECURITYINTERFACEA");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_ENTRYPOINT_ANSI: ::windows::core::PCWSTR = ::windows::w!("InitSecurityInterfaceW");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_ENTRYPOINT_ANSIA: ::windows::core::PCSTR = ::windows::s!("InitSecurityInterfaceA");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_ENTRYPOINT_ANSIW: ::windows::core::PCSTR = ::windows::s!("InitSecurityInterfaceW");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_NATIVE_DREP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_NETWORK_DREP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_5: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_FOR_SYSTEM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_SAME_LOGON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_SAME_PROCESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_ID_PROVIDER: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_NULL_DOMAIN: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_NULL_USER: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_PROCESS_ENCRYPTED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_RESERVED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_LOAD: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_SAVE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_NO_CHECKBOX: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_SAVE_CRED_BY_CALLER: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_SAVE_CRED_CHECKED: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_USE_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SYSTEM_ENCRYPTED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SYSTEM_PROTECTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_USER_PROTECTED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_MARSHALLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_VERSION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_VERSION_2: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SESSION_TICKET_INFO_V0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SESSION_TICKET_INFO_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_ADT_OBJECT_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_ADT_PARAMETERS_SELF_RELATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_ADT_PARAMETERS_SEND_TO_LSA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_ADT_PARAMETER_EXTENSIBLE_AUDIT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_ADT_PARAMETER_GENERIC_AUDIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_ADT_PARAMETER_WRITE_SYNCHRONOUS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_ADT_POLICY_AUDIT_EVENT_TYPE_EX_BEGIN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_BATCH_LOGON_NAME: ::windows::core::PCWSTR = ::windows::w!("SeBatchLogonRight");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_DENY_BATCH_LOGON_NAME: ::windows::core::PCWSTR = ::windows::w!("SeDenyBatchLogonRight");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_DENY_INTERACTIVE_LOGON_NAME: ::windows::core::PCWSTR = ::windows::w!("SeDenyInteractiveLogonRight");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_DENY_NETWORK_LOGON_NAME: ::windows::core::PCWSTR = ::windows::w!("SeDenyNetworkLogonRight");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_DENY_REMOTE_INTERACTIVE_LOGON_NAME: ::windows::core::PCWSTR = ::windows::w!("SeDenyRemoteInteractiveLogonRight");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_DENY_SERVICE_LOGON_NAME: ::windows::core::PCWSTR = ::windows::w!("SeDenyServiceLogonRight");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_INTERACTIVE_LOGON_NAME: ::windows::core::PCWSTR = ::windows::w!("SeInteractiveLogonRight");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_MAX_AUDIT_PARAMETERS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_MAX_GENERIC_AUDIT_PARAMETERS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_NETWORK_LOGON_NAME: ::windows::core::PCWSTR = ::windows::w!("SeNetworkLogonRight");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_REMOTE_INTERACTIVE_LOGON_NAME: ::windows::core::PCWSTR = ::windows::w!("SeRemoteInteractiveLogonRight");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SE_SERVICE_LOGON_NAME: ::windows::core::PCWSTR = ::windows::w!("SeServiceLogonRight");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_CLIENTAPI_ZONE: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_DEFAULT_MIGRATION_ENCRYPTOR_URI: ::windows::core::PCWSTR = ::windows::w!("msft:spp/migrationencryptor/tokenact/1.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_EVENT_LICENSING_STATE_CHANGED: ::windows::core::PCWSTR = ::windows::w!("msft:rm/event/licensingstatechanged");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_EVENT_POLICY_CHANGED: ::windows::core::PCWSTR = ::windows::w!("msft:rm/event/policychanged");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_EVENT_USER_NOTIFICATION: ::windows::core::PCWSTR = ::windows::w!("msft:rm/event/usernotification");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_ACTIVATION_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422296i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_APPLICATION_POLICIES_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418126i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_APPLICATION_POLICIES_NOT_LOADED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418125i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_AUTHN_CANT_VERIFY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418118i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_AUTHN_CHALLENGE_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418119i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_AUTHN_MISMATCHED_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418120i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_AUTHN_WRONG_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418121i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_BASE_SKU_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418155i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_BIOS_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417707i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_BLOCKED_PRODUCT_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418159i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_ACTCONFIG_ID_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430519i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_BINDING_MAPPING_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430522i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_BINDING_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430523i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_BUSINESS_RULE_INPUT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428736i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_DATABASE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430509i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_DIGITALMARKER_BINDING_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430446i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_DIGITALMARKER_INVALID_BINDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430447i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_DMAK_EXTENSION_LIMIT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430495i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_DMAK_LIMIT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430496i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_DYNAMICALLY_BLOCKED_PRODUCT_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430432i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_FAILED_TO_DELETE_PRODUCTKEY_BINDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428649i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_FAILED_TO_DELETE_PRODUCT_KEY_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428644i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_FAILED_TO_INSERT_PRODUCTKEY_BINDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428650i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_FAILED_TO_INSERT_PRODUCT_KEY_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428646i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_FAILED_TO_INSERT_PRODUCT_KEY_RECORD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428608i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_FAILED_TO_PROCESS_PRODUCT_KEY_BINDINGS_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428648i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_FAILED_TO_UPDATE_PRODUCTKEY_BINDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428651i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_FAILED_TO_UPDATE_PRODUCT_KEY_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428645i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_FAILED_TO_UPDATE_PRODUCT_KEY_RECORD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428607i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_GENERAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430448i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_INVALID_ACTCONFIG_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430515i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_INVALID_ARGUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430508i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_INVALID_BINDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430526i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_INVALID_BINDING_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430511i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430517i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_DATA_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430518i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430524i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_KEY_CHAR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430512i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_KEY_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430513i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_KEY_LENGTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430514i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_MAXIMUM_UNLOCK_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430520i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_MSCH_RESPONSE_NOT_AVAILABLE_VGA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429505i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_NETWORK_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430510i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_NO_RULES_TO_ACTIVATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430449i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_NULL_VALUE_FOR_PROPERTY_NAME_OR_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428656i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_OEM_SLP_COA0: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430506i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_OVERRIDE_REQUEST_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430493i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_PRODUCT_KEY_BEING_USED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428624i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_PRODUCT_KEY_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430525i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_PRODUCT_KEY_BLOCKED_IPLOCATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430505i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_PRODUCT_KEY_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430527i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_REISSUANCE_LIMIT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430494i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_RESPONSE_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430507i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_SYSTEM_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430516i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_TIMEBASED_ACTIVATION_AFTER_END_DATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430479i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_TIMEBASED_ACTIVATION_BEFORE_START_DATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430480i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_TIMEBASED_ACTIVATION_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430478i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_TIMEBASED_PRODUCT_KEY_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430477i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_UNKNOWN_PRODUCT_KEY_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428636i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_UNKNOWN_PROPERTY_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428654i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_UNKNOWN_PROPERTY_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428655i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CHPA_UNSUPPORTED_PRODUCT_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073430521i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CIDIID_INVALID_CHECK_DIGITS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418163i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CIDIID_INVALID_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418196i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CIDIID_INVALID_DATA_LENGTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418193i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CIDIID_INVALID_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418195i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CIDIID_MISMATCHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418191i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CIDIID_MISMATCHED_PKEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418114i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CIDIID_NOT_BOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418113i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CIDIID_NOT_DEPOSITED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418192i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_CIDIID_VERSION_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418194i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_DATATYPE_MISMATCHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418210i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_DECRYPTION_LICENSES_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418212i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_DEPENDENT_PROPERTY_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418138i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_DOWNLEVEL_SETUP_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417708i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_DUPLICATE_POLICY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418158i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_EDITION_MISMATCHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417712i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_ENGINE_DETECTED_EXPLOIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429327i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_EUL_CONSUMPTION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422315i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_EUL_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418188i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_EVALUATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422333i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_EVENT_ALREADY_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418213i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_EVENT_NOT_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418214i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_EXTERNAL_SIGNATURE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418234i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_GRACE_TIME_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418231i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_HEALTH_CHECK_FAILED_MUI_FILES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429330i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_HEALTH_CHECK_FAILED_NEUTRAL_FILES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429331i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_HWID_CHANGED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417711i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_HWID_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422309i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_IA_ID_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073414909i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_IA_INVALID_VIRTUALIZATION_PLATFORM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073414911i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_IA_MACHINE_NOT_BOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073414908i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_IA_PARENT_PARTITION_NOT_ACTIVATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073414910i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_IA_THROTTLE_LIMIT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073414912i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418239i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_AD_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429329i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_BINDING_BLOB: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418190i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_CLIENT_TOKEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429720i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_CONTEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422335i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_CONTEXT_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422300i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_EVENT_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418215i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_FILE_HASH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429343i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_GUID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422330i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_HASH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422299i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418209i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_LICENSE_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429336i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_LICENSE_STATE_BREACH_GRACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429871i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_LICENSE_STATE_BREACH_GRACE_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429870i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_OEM_OR_VOLUME_BINDING_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429337i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_OFFLINE_BLOB: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429719i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_OSVERSION_TEMPLATEID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429717i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_OS_FOR_PRODUCT_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429503i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_PACKAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418208i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_PACKAGE_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418144i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_PKEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418224i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_PRODUCT_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418160i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_PRODUCT_KEY_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418115i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_RSDP_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429328i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_RULESET_RULE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422301i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_RUNNING_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418199i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_TEMPLATE_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429770i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_TOKEN_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429332i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_USE_OF_ADD_ON_PKEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147164122i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_INVALID_XML_BLOB: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429766i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_IP_LOCATION_FALIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429335i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_ISSUANCE_LICENSE_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418142i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_LICENSE_AUTHORIZATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418206i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_LICENSE_DECRYPTION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418205i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_LICENSE_FILE_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418223i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_LICENSE_INVALID_ADDON_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422310i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_LICENSE_MANAGEMENT_DATA_DUPLICATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418156i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_LICENSE_MANAGEMENT_DATA_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418161i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_LICENSE_NOT_BOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418112i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_LICENSE_SERVER_URL_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418216i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_LICENSE_SIGNATURE_VERIFICATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418211i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_LUA_ACCESSDENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418203i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_MISMATCHED_APPID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418230i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_MISMATCHED_KEY_TYPES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429340i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_MISMATCHED_PID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418235i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_MISMATCHED_PKEY_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418236i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_MISMATCHED_PRODUCT_SKU: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418135i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_MISMATCHED_SECURITY_PROCESSOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418226i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_MISSING_OVERRIDE_ONLY_ATTRIBUTE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418157i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NONGENUINE_GRACE_TIME_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418140i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NONGENUINE_GRACE_TIME_EXPIRED_2: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418137i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NON_GENUINE_STATUS_LAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073428992i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NOTIFICATION_BREACH_DETECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429199i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NOTIFICATION_GRACE_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429198i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NOTIFICATION_OTHER_REASONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429197i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NOT_ACTIVATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422331i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NOT_EVALUATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422332i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NOT_GENUINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417728i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418218i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NO_PID_CONFIG_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418229i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_NO_PRODUCT_KEY_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417709i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_OEM_KEY_EDITION_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417710i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_OFFLINE_GENUINE_BLOB_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429715i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_OFFLINE_GENUINE_BLOB_REVOKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429716i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_OFFLINE_VALIDATION_BLOB_PARAM_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429718i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_OPERATION_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418134i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_OUT_OF_TOLERANCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418225i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PKEY_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422311i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PKEY_INVALID_ALGORITHM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422312i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PKEY_INVALID_CONFIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422314i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PKEY_INVALID_KEYCHANGE1: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422308i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PKEY_INVALID_KEYCHANGE2: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422307i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PKEY_INVALID_KEYCHANGE3: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422306i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PKEY_INVALID_UNIQUEID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422313i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PKEY_INVALID_UPGRADE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418143i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PKEY_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418220i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PLUGIN_INVALID_MANIFEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418127i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PLUGIN_NOT_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418122i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_POLICY_CACHE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418200i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_POLICY_OTHERINFO_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422304i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PRODUCT_KEY_INSTALLATION_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418189i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PRODUCT_SKU_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418219i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PRODUCT_UNIQUENESS_GROUP_ID_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422303i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PROXY_KEY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418202i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PROXY_POLICY_NOT_UPDATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418169i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_PUBLISHING_LICENSE_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418217i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_RAC_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418233i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_RIGHT_NOT_CONSUMED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418238i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_RIGHT_NOT_GRANTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418221i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SECURE_STORE_ID_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422302i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SERVICE_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418117i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SERVICE_STOPPING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418123i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_BAD_TOKEN_EXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163899i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_BAD_TOKEN_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163900i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_DUPLICATE_TOKEN_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163898i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_FILE_READ_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163895i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_FILE_WRITE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163894i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_INVALID_FD_TABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163902i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_INVALID_FILE_POSITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163893i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_INVALID_FS_HEADER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163891i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_INVALID_FS_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163903i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_INVALID_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163901i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_INVALID_TOKEN_DATA_HASH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163896i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_INVALID_TOKEN_DESCRIPTOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163890i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_NO_ACTIVE_TRANSACTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163892i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SFS_TOKEN_SIZE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147163897i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SLP_BAD_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418151i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SLP_INVALID_MARKER_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418116i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SLP_MISSING_ACPI_SLIC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418153i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SLP_MISSING_SLP_MARKER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418152i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SLP_NOT_SIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418198i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SLP_OEM_CERT_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418141i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SOFTMOD_EXPLOIT_DETECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429333i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SPC_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418232i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_AUTHORIZATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434619i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_BUSINESS_TOKEN_ENTRY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434608i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_CLIENT_CLOCK_OUT_OF_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434607i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_GENERAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434368i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_INVALID_BINDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434618i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_INVALID_LICENSE_STRUCTURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434620i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_INVALID_PAYLOAD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434616i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_INVALID_PRODUCT_KEY_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434622i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_INVALID_PUBLISH_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434623i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_INVALID_RIGHTS_ACCOUNT_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434621i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_INVALID_SECURITY_PROCESSOR_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434615i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_SRV_SERVER_PONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073434617i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_NOT_AUTHORIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422290i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_NOT_PRS_SIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422292i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422295i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_WRONG_EDITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422294i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_WRONG_PID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422293i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_WRONG_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422291i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TAMPER_DETECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418201i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TAMPER_RECOVERY_REQUIRES_ACTIVATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073414656i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_CERT_CNG_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417453i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_CERT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417467i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_CHALLENGE_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417471i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_CHALLENGE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417463i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_CRITERIA_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417457i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_FAILED_GRANT_PARSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417460i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_GRANT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417468i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_INVALID_BLOB: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417465i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_INVALID_CERTIFICATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417462i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_INVALID_CERT_CHAIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417469i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_INVALID_SKU_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417466i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_INVALID_SMARTCARD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417461i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_INVALID_THUMBPRINT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417459i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_SILENT_ACTIVATION_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417470i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_SOFT_CERT_DISALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417455i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_SOFT_CERT_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417454i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_TAMPERED_CERT_CHAIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417464i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_THUMBPRINT_CERT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417458i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TKA_TPID_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073417456i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKEN_STORE_INVALID_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422334i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_ALREADY_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422326i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_CANT_ACQUIRE_MUTEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422317i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_CANT_CREATE_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422324i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_CANT_CREATE_MUTEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422318i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_CANT_PARSE_PROPERTIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422321i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_CANT_READ_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422322i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_CANT_WRITE_TO_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422323i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_INVALID_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422319i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422327i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_NO_ID_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422325i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_NO_PROPERTIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422328i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_NO_TOKEN_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422316i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_PROPERTY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422320i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_TOKSTO_TOKEN_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422329i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_USE_LICENSE_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418237i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VALIDATION_BLOB_PARAM_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429721i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VALIDATION_BLOCKED_PRODUCT_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429342i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VALIDATION_INVALID_PRODUCT_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073429339i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VALIDITY_PERIOD_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073415161i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VALIDITY_TIME_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418207i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VALUE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418222i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_AD_AO_NAME_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418110i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_AD_AO_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418111i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_AD_SCHEMA_VERSION_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418109i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_BINDING_SERVICE_NOT_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418183i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_BINDING_SERVICE_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418124i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_INFO_PRODUCT_USER_RIGHT: ::windows::core::HRESULT = ::windows::core::HRESULT(1074065472i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_INVALID_TIMESTAMP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418132i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_KEY_MANAGEMENT_SERVICE_ID_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418174i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_KEY_MANAGEMENT_SERVICE_NOT_ACTIVATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418175i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_KEY_MANAGEMENT_SERVICE_VM_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418133i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_MACHINE_NOT_BOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418154i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_NOT_ENOUGH_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418184i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_VL_NOT_WINDOWS_SLP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418187i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_WINDOWS_INVALID_LICENSE_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073418204i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_E_WINDOWS_VERSION_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073422297i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_ACTIVE_PLUGINS: ::windows::core::PCWSTR = ::windows::w!("ActivePlugins");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_AUTHOR: ::windows::core::PCWSTR = ::windows::w!("Author");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_BIOS_OA2_MINOR_VERSION: ::windows::core::PCWSTR = ::windows::w!("BiosOA2MinorVersion");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_BIOS_PKEY: ::windows::core::PCWSTR = ::windows::w!("BiosProductKey");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_BIOS_PKEY_DESCRIPTION: ::windows::core::PCWSTR = ::windows::w!("BiosProductKeyDescription");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_BIOS_PKEY_PKPN: ::windows::core::PCWSTR = ::windows::w!("BiosProductKeyPkPn");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_BIOS_SLIC_STATE: ::windows::core::PCWSTR = ::windows::w!("BiosSlicState");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_CHANNEL: ::windows::core::PCWSTR = ::windows::w!("Channel");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_DESCRIPTION: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_DIGITAL_PID: ::windows::core::PCWSTR = ::windows::w!("DigitalPID");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_DIGITAL_PID2: ::windows::core::PCWSTR = ::windows::w!("DigitalPID2");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_IS_KMS: ::windows::core::PCWSTR = ::windows::w!("IsKeyManagementService");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_IS_PRS: ::windows::core::PCWSTR = ::windows::w!("IsPRS");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_KMS_CURRENT_COUNT: ::windows::core::PCWSTR = ::windows::w!("KeyManagementServiceCurrentCount");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_KMS_FAILED_REQUESTS: ::windows::core::PCWSTR = ::windows::w!("KeyManagementServiceFailedRequests");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_KMS_LICENSED_REQUESTS: ::windows::core::PCWSTR = ::windows::w!("KeyManagementServiceLicensedRequests");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_KMS_NON_GENUINE_GRACE_REQUESTS: ::windows::core::PCWSTR = ::windows::w!("KeyManagementServiceNonGenuineGraceRequests");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_KMS_NOTIFICATION_REQUESTS: ::windows::core::PCWSTR = ::windows::w!("KeyManagementServiceNotificationRequests");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_KMS_OOB_GRACE_REQUESTS: ::windows::core::PCWSTR = ::windows::w!("KeyManagementServiceOOBGraceRequests");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_KMS_OOT_GRACE_REQUESTS: ::windows::core::PCWSTR = ::windows::w!("KeyManagementServiceOOTGraceRequests");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_KMS_REQUIRED_CLIENT_COUNT: ::windows::core::PCWSTR = ::windows::w!("KeyManagementServiceRequiredClientCount");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_KMS_TOTAL_REQUESTS: ::windows::core::PCWSTR = ::windows::w!("KeyManagementServiceTotalRequests");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_KMS_UNLICENSED_REQUESTS: ::windows::core::PCWSTR = ::windows::w!("KeyManagementServiceUnlicensedRequests");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_LICENSE_TYPE: ::windows::core::PCWSTR = ::windows::w!("LicenseType");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_LICENSOR_URL: ::windows::core::PCWSTR = ::windows::w!("LicensorUrl");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_NAME: ::windows::core::PCWSTR = ::windows::w!("Name");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_PARTIAL_PRODUCT_KEY: ::windows::core::PCWSTR = ::windows::w!("PartialProductKey");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_PRODUCT_KEY_ACTIVATION_URL: ::windows::core::PCWSTR = ::windows::w!("PKCURL");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_PRODUCT_SKU_ID: ::windows::core::PCWSTR = ::windows::w!("ProductSkuId");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_RIGHT_ACCOUNT_ACTIVATION_URL: ::windows::core::PCWSTR = ::windows::w!("RACURL");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_SECURE_PROCESSOR_ACTIVATION_URL: ::windows::core::PCWSTR = ::windows::w!("SPCURL");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_SECURE_STORE_ID: ::windows::core::PCWSTR = ::windows::w!("SecureStoreId");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_SYSTEM_STATE: ::windows::core::PCWSTR = ::windows::w!("SystemState");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_USE_LICENSE_ACTIVATION_URL: ::windows::core::PCWSTR = ::windows::w!("EULURL");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INFO_KEY_VERSION: ::windows::core::PCWSTR = ::windows::w!("Version");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_INTERNAL_ZONE: u32 = 57344u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_I_NONGENUINE_GRACE_PERIOD: ::windows::core::HRESULT = ::windows::core::HRESULT(1074065509i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_I_NONGENUINE_GRACE_PERIOD_2: ::windows::core::HRESULT = ::windows::core::HRESULT(1074065512i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_I_OOB_GRACE_PERIOD: ::windows::core::HRESULT = ::windows::core::HRESULT(1074065420i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_I_OOT_GRACE_PERIOD: ::windows::core::HRESULT = ::windows::core::HRESULT(1074065421i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_I_PERPETUAL_OOB_GRACE_PERIOD: ::windows::core::HRESULT = ::windows::core::HRESULT(1074068485i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_I_STORE_BASED_ACTIVATION: ::windows::core::HRESULT = ::windows::core::HRESULT(1074066433i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_I_TIMEBASED_EXTENDED_GRACE_PERIOD: ::windows::core::HRESULT = ::windows::core::HRESULT(1074068486i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_I_TIMEBASED_VALIDITY_PERIOD: ::windows::core::HRESULT = ::windows::core::HRESULT(1074068484i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_MDOLLAR_ZONE: u32 = 40960u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_MSCH_ZONE: u32 = 49152u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PKEY_DETECT: ::windows::core::PCWSTR = ::windows::w!("msft:rm/algorithm/pkey/detect");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PKEY_MS2005: ::windows::core::PCWSTR = ::windows::w!("msft:rm/algorithm/pkey/2005");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PKEY_MS2009: ::windows::core::PCWSTR = ::windows::w!("msft:rm/algorithm/pkey/2009");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_POLICY_EVALUATION_MODE_ENABLED: ::windows::core::PCWSTR = ::windows::w!("Security-SPP-EvaluationModeEnabled");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PROP_ACTIVATION_VALIDATION_IN_PROGRESS: ::windows::core::PCWSTR = ::windows::w!("SL_ACTIVATION_VALIDATION_IN_PROGRESS");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PROP_BRT_COMMIT: ::windows::core::PCWSTR = ::windows::w!("SL_BRT_COMMIT");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PROP_BRT_DATA: ::windows::core::PCWSTR = ::windows::w!("SL_BRT_DATA");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PROP_GENUINE_RESULT: ::windows::core::PCWSTR = ::windows::w!("SL_GENUINE_RESULT");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PROP_GET_GENUINE_AUTHZ: ::windows::core::PCWSTR = ::windows::w!("SL_GET_GENUINE_AUTHZ");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PROP_GET_GENUINE_SERVER_AUTHZ: ::windows::core::PCWSTR = ::windows::w!("SL_GET_GENUINE_SERVER_AUTHZ");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PROP_LAST_ACT_ATTEMPT_HRESULT: ::windows::core::PCWSTR = ::windows::w!("SL_LAST_ACT_ATTEMPT_HRESULT");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PROP_LAST_ACT_ATTEMPT_SERVER_FLAGS: ::windows::core::PCWSTR = ::windows::w!("SL_LAST_ACT_ATTEMPT_SERVER_FLAGS");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PROP_LAST_ACT_ATTEMPT_TIME: ::windows::core::PCWSTR = ::windows::w!("SL_LAST_ACT_ATTEMPT_TIME");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_PROP_NONGENUINE_GRACE_FLAG: ::windows::core::PCWSTR = ::windows::w!("SL_NONGENUINE_GRACE_FLAG");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REARM_REBOOT_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_CHECK_DIGITS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313776i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313778i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_DATA_LENGTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313777i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313779i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_DIGITALMARKER_BINDING_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313708i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_DIGITALMARKER_INVALID_BINDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313709i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_DMAK_EXTENSION_LIMIT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313792i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_DMAK_LIMIT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313793i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_DMAK_OVERRIDE_LIMIT_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313706i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_FREE_OFFER_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143312896i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_ACTCONFIG_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313802i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_ARGUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313795i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_BINDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313818i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_BINDING_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313798i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313804i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_DATA_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313805i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313816i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_KEY_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313800i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_KEY_LENGTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313801i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_MAXIMUM_UNLOCK_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313807i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_NO_RULES_TO_ACTIVATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313720i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OEM_SLP_COA0: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313789i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_DEVICE_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310909i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_DEVICE_THROTTLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310914i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_DONOR_HWID_NO_ENTITLEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310920i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_GENERIC_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310919i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_GP_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310913i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_HARDWARE_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310912i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_LICENSE_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310910i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_LICENSE_THROTTLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310915i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_NOT_ADMIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310917i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_NO_ASSOCIATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310918i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_USER_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310911i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_USER_THROTTLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143310916i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_PRODUCT_KEY_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313817i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_PRODUCT_KEY_BLOCKED_IPLOCATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313717i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_PRODUCT_KEY_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313819i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_ROT_OVERRIDE_LIMIT_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313707i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_ACTIVATION_AFTER_END_DATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313768i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_ACTIVATION_BEFORE_START_DATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313769i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_ACTIVATION_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313767i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_PRODUCT_KEY_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313766i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_MDOLLAR_UNSUPPORTED_PRODUCT_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2143313812i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_API_BAD_GET_INFO_QUERY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426414i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_API_HANDLE_NOT_COMMITED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426303i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_API_INVALID_ALGORITHM_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426423i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_API_INVALID_HANDLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426388i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_API_INVALID_KEY_LENGTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426347i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_API_INVALID_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426432i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_API_NO_AES_PROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426317i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_API_TOO_MANY_LOADED_ENVIRONMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426420i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_HASH_FINALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425911i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_BLOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425905i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_BLOCKLENGTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425918i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_CIPHER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425917i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_CIPHERMODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425916i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425904i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_KEYLENGTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425919i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_PADDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425903i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_SIGNATURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425906i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_SIGNATURELENGTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425907i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_KEY_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425910i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_KEY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425909i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_NOT_BLOCK_ALIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425908i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_ATTRIBUTEID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425912i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_HASHID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425913i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_KEYID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425914i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_PROVIDERID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425915i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_GENERAL_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426175i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_KM_CACHE_IDENTICAL: ::windows::core::HRESULT = ::windows::core::HRESULT(1074058753i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_KM_CACHE_POLICY_CHANGED: ::windows::core::HRESULT = ::windows::core::HRESULT(1074058754i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_KM_CACHE_TAMPER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425151i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_KM_CACHE_TAMPER_RESTORE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425150i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_PROXY_SOFT_TAMPER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073424638i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TAMPER_MODULE_AUTHENTICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425407i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TAMPER_SECURITY_PROCESSOR_PATCHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425406i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TIMER_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425654i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TIMER_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425652i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TIMER_NAME_SIZE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425651i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TIMER_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425653i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TIMER_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425647i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TRUSTED_TIME_OK: ::windows::core::HRESULT = ::windows::core::HRESULT(1074057999i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_ACCESS_DENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425644i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_ATTRIBUTE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425645i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_ATTRIBUTE_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425646i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_DATA_SIZE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425656i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_KEY_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425659i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_KEY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425660i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_KEY_SIZE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425658i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425648i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_FULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425650i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_INVALID_HW_BINDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425655i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_MAX_REARM_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425657i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_NAMESPACE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425642i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_NAMESPACE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425643i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_REARMED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425662i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_RECREATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425661i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425663i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_BREADCRUMB_GENERATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425640i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_BREADCRUMB_LOAD_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425641i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_DATA_BREADCRUMB_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425637i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_DATA_VERSION_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425636i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_INVALID_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425639i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_NO_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073425638i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_STATUS_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426171i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_STATUS_DEBUGGER_DETECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147167989i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_STATUS_GENERIC_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426173i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_STATUS_INSUFFICIENT_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426169i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_STATUS_INVALIDARG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426172i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_STATUS_INVALIDDATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426168i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_STATUS_INVALID_SPAPI_CALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426167i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_STATUS_INVALID_SPAPI_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426166i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_STATUS_NO_MORE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073426164i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_STATUS_PUSHKEY_CONFLICT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073424639i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REMAPPING_SP_STATUS_SYSTEM_TIME_SKEWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147167998i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_SERVER_ZONE: u32 = 45056u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_SYSTEM_STATE_REBOOT_POLICY_FOUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_SYSTEM_STATE_TAMPERED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SPP_MIGRATION_GATHER_ACTIVATED_WINDOWS_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SPP_MIGRATION_GATHER_ALL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SPP_MIGRATION_GATHER_MIGRATABLE_APPS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_ACCEPT_CREDENTIALS_NAME: ::windows::core::PCSTR = ::windows::s!("SpAcceptCredentials\u{0}");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_ALL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_DTLS1_0_CLIENT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_DTLS1_0_SERVER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_DTLS1_2_CLIENT: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_DTLS1_2_SERVER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_DTLS_CLIENT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_DTLS_SERVER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_PCT1_CLIENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_PCT1_SERVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_SSL2_CLIENT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_SSL2_SERVER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_SSL3_CLIENT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_SSL3_SERVER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_0_CLIENT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_0_SERVER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_1_CLIENT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_1_SERVER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_2_CLIENT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_2_SERVER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_3PLUS_CLIENT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_3PLUS_SERVER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_3_CLIENT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_3_SERVER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_CLIENT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_TLS1_SERVER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_UNI_CLIENT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SP_PROT_UNI_SERVER: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSL2SP_NAME: ::windows::core::PCWSTR = ::windows::w!("Microsoft SSL 2.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSL2SP_NAME_A: ::windows::core::PCSTR = ::windows::s!("Microsoft SSL 2.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSL2SP_NAME_W: ::windows::core::PCWSTR = ::windows::w!("Microsoft SSL 2.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSL3SP_NAME: ::windows::core::PCWSTR = ::windows::w!("Microsoft SSL 3.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSL3SP_NAME_A: ::windows::core::PCSTR = ::windows::s!("Microsoft SSL 3.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSL3SP_NAME_W: ::windows::core::PCWSTR = ::windows::w!("Microsoft SSL 3.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSL_CRACK_CERTIFICATE_NAME: ::windows::core::PCWSTR = ::windows::w!("SslCrackCertificate");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSL_FREE_CERTIFICATE_NAME: ::windows::core::PCWSTR = ::windows::w!("SslFreeCertificate");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSL_SESSION_RECONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSPIPFC_CREDPROV_DO_NOT_LOAD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSPIPFC_CREDPROV_DO_NOT_SAVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSPIPFC_NO_CHECKBOX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSPIPFC_SAVE_CRED_BY_CALLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSPIPFC_USE_CREDUIBROKER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SZ_ALG_MAX_SIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1SP_NAME: ::windows::core::PCWSTR = ::windows::w!("Microsoft TLS 1.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1SP_NAME_A: ::windows::core::PCSTR = ::windows::s!("Microsoft TLS 1.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1SP_NAME_W: ::windows::core::PCWSTR = ::windows::w!("Microsoft TLS 1.0");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_ACCESS_DENIED: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_BAD_CERTIFICATE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_BAD_RECORD_MAC: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_CERTIFICATE_EXPIRED: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_CERTIFICATE_REVOKED: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_CERTIFICATE_UNKNOWN: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_CLOSE_NOTIFY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_DECODE_ERROR: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_DECOMPRESSION_FAIL: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_DECRYPTION_FAILED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_DECRYPT_ERROR: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_EXPORT_RESTRICTION: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_HANDSHAKE_FAILURE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_ILLEGAL_PARAMETER: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_INSUFFIENT_SECURITY: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_INTERNAL_ERROR: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_NO_APP_PROTOCOL: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_NO_RENEGOTIATION: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_PROTOCOL_VERSION: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_RECORD_OVERFLOW: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_UNEXPECTED_MESSAGE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_UNKNOWN_CA: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_UNKNOWN_PSK_IDENTITY: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_UNSUPPORTED_CERT: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_UNSUPPORTED_EXT: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_USER_CANCELED: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS_PARAMS_OPTIONAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUSTED_QUERY_AUTH: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUSTED_QUERY_CONTROLLERS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUSTED_QUERY_DOMAIN_NAME: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUSTED_QUERY_POSIX: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUSTED_SET_AUTH: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUSTED_SET_CONTROLLERS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUSTED_SET_POSIX: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTES_USER: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTES_VALID: u32 = 4278386687u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION_ENABLE_TGT_DELEGATION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION_NO_TGT_DELEGATION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_PIM_TRUST: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_QUARANTINED_DOMAIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_TREE_PARENT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_TREE_ROOT: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_TRUST_USES_AES_KEYS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_TRUST_USES_RC4_ENCRYPTION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const UNDERSTANDS_LONG_NAMES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const UNISP_NAME: ::windows::core::PCWSTR = ::windows::w!("Microsoft Unified Security Protocol Provider");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const UNISP_NAME_A: ::windows::core::PCSTR = ::windows::s!("Microsoft Unified Security Protocol Provider");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const UNISP_NAME_W: ::windows::core::PCWSTR = ::windows::w!("Microsoft Unified Security Protocol Provider");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const UNISP_RPC_ID: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_ACCOUNT_AUTO_LOCKED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_ACCOUNT_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_ALL_PARAMETERS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_DONT_EXPIRE_PASSWORD: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_DONT_REQUIRE_PREAUTH: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_ENCRYPTED_TEXT_PASSWORD_ALLOWED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_HOME_DIRECTORY_REQUIRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_INTERDOMAIN_TRUST_ACCOUNT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_MNS_LOGON_ACCOUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_NORMAL_ACCOUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_NOT_DELEGATED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_NO_AUTH_DATA_REQUIRED: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_PARTIAL_SECRETS_ACCOUNT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_PASSWORD_EXPIRED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_PASSWORD_NOT_REQUIRED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_SERVER_TRUST_ACCOUNT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_SMARTCARD_REQUIRED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_TEMP_DUPLICATE_ACCOUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_TRUSTED_FOR_DELEGATION: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_USE_AES_KEYS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_USE_DES_KEY_ONLY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const USER_WORKSTATION_TRUST_ACCOUNT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const WDIGEST_SP_NAME: ::windows::core::PCWSTR = ::windows::w!("WDigest");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const WDIGEST_SP_NAME_A: ::windows::core::PCSTR = ::windows::s!("WDigest");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const WDIGEST_SP_NAME_W: ::windows::core::PCWSTR = ::windows::w!("WDigest");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const WINDOWS_SLID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55c92734_d682_4d71_983e_d6ec3f16059f);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const _FACILITY_WINDOWS_STORE: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ASC_REQ_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_DELEGATE: ASC_REQ_FLAGS = ASC_REQ_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_MUTUAL_AUTH: ASC_REQ_FLAGS = ASC_REQ_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_REPLAY_DETECT: ASC_REQ_FLAGS = ASC_REQ_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_SEQUENCE_DETECT: ASC_REQ_FLAGS = ASC_REQ_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_CONFIDENTIALITY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_USE_SESSION_KEY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_SESSION_TICKET: ASC_REQ_FLAGS = ASC_REQ_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_ALLOCATE_MEMORY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_USE_DCE_STYLE: ASC_REQ_FLAGS = ASC_REQ_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_DATAGRAM: ASC_REQ_FLAGS = ASC_REQ_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_CONNECTION: ASC_REQ_FLAGS = ASC_REQ_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_CALL_LEVEL: ASC_REQ_FLAGS = ASC_REQ_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_FRAGMENT_SUPPLIED: ASC_REQ_FLAGS = ASC_REQ_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_EXTENDED_ERROR: ASC_REQ_FLAGS = ASC_REQ_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_STREAM: ASC_REQ_FLAGS = ASC_REQ_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_INTEGRITY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_LICENSING: ASC_REQ_FLAGS = ASC_REQ_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_IDENTIFY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_ALLOW_NULL_SESSION: ASC_REQ_FLAGS = ASC_REQ_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_ALLOW_NON_USER_LOGONS: ASC_REQ_FLAGS = ASC_REQ_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_ALLOW_CONTEXT_REPLAY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_FRAGMENT_TO_FIT: ASC_REQ_FLAGS = ASC_REQ_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_NO_TOKEN: ASC_REQ_FLAGS = ASC_REQ_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_PROXY_BINDINGS: ASC_REQ_FLAGS = ASC_REQ_FLAGS(67108864u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_ALLOW_MISSING_BINDINGS: ASC_REQ_FLAGS = ASC_REQ_FLAGS(268435456u32);
impl ::core::marker::Copy for ASC_REQ_FLAGS {}
impl ::core::clone::Clone for ASC_REQ_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ASC_REQ_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ASC_REQ_HIGH_FLAGS(pub u64);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ASC_REQ_MESSAGES: ASC_REQ_HIGH_FLAGS = ASC_REQ_HIGH_FLAGS(4294967296u64);
impl ::core::marker::Copy for ASC_REQ_HIGH_FLAGS {}
impl ::core::clone::Clone for ASC_REQ_HIGH_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ASC_REQ_HIGH_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRED_FETCH(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CredFetchDefault: CRED_FETCH = CRED_FETCH(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CredFetchDPAPI: CRED_FETCH = CRED_FETCH(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CredFetchForced: CRED_FETCH = CRED_FETCH(2i32);
impl ::core::marker::Copy for CRED_FETCH {}
impl ::core::clone::Clone for CRED_FETCH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CRED_FETCH {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOMAIN_PASSWORD_PROPERTIES(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DOMAIN_PASSWORD_COMPLEX: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DOMAIN_PASSWORD_NO_ANON_CHANGE: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DOMAIN_PASSWORD_NO_CLEAR_CHANGE: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(4u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DOMAIN_LOCKOUT_ADMINS: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(8u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DOMAIN_PASSWORD_STORE_CLEARTEXT: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(16u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DOMAIN_REFUSE_PASSWORD_CHANGE: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(32u32);
impl ::core::marker::Copy for DOMAIN_PASSWORD_PROPERTIES {}
impl ::core::clone::Clone for DOMAIN_PASSWORD_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOMAIN_PASSWORD_PROPERTIES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXPORT_SECURITY_CONTEXT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CONTEXT_EXPORT_RESET_NEW: EXPORT_SECURITY_CONTEXT_FLAGS = EXPORT_SECURITY_CONTEXT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CONTEXT_EXPORT_DELETE_OLD: EXPORT_SECURITY_CONTEXT_FLAGS = EXPORT_SECURITY_CONTEXT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CONTEXT_EXPORT_TO_KERNEL: EXPORT_SECURITY_CONTEXT_FLAGS = EXPORT_SECURITY_CONTEXT_FLAGS(4u32);
impl ::core::marker::Copy for EXPORT_SECURITY_CONTEXT_FLAGS {}
impl ::core::clone::Clone for EXPORT_SECURITY_CONTEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EXPORT_SECURITY_CONTEXT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXTENDED_NAME_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameUnknown: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameFullyQualifiedDN: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameSamCompatible: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameDisplay: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameUniqueId: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameCanonical: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameUserPrincipal: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(8i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameCanonicalEx: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(9i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameServicePrincipal: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(10i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameDnsDomain: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(12i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameGivenName: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(13i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NameSurname: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(14i32);
impl ::core::marker::Copy for EXTENDED_NAME_FORMAT {}
impl ::core::clone::Clone for EXTENDED_NAME_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EXTENDED_NAME_FORMAT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ISC_REQ_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_DELEGATE: ISC_REQ_FLAGS = ISC_REQ_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_MUTUAL_AUTH: ISC_REQ_FLAGS = ISC_REQ_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_REPLAY_DETECT: ISC_REQ_FLAGS = ISC_REQ_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_SEQUENCE_DETECT: ISC_REQ_FLAGS = ISC_REQ_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_CONFIDENTIALITY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_USE_SESSION_KEY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_PROMPT_FOR_CREDS: ISC_REQ_FLAGS = ISC_REQ_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_USE_SUPPLIED_CREDS: ISC_REQ_FLAGS = ISC_REQ_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_ALLOCATE_MEMORY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_USE_DCE_STYLE: ISC_REQ_FLAGS = ISC_REQ_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_DATAGRAM: ISC_REQ_FLAGS = ISC_REQ_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_CONNECTION: ISC_REQ_FLAGS = ISC_REQ_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_CALL_LEVEL: ISC_REQ_FLAGS = ISC_REQ_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_FRAGMENT_SUPPLIED: ISC_REQ_FLAGS = ISC_REQ_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_EXTENDED_ERROR: ISC_REQ_FLAGS = ISC_REQ_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_STREAM: ISC_REQ_FLAGS = ISC_REQ_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_INTEGRITY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_IDENTIFY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_NULL_SESSION: ISC_REQ_FLAGS = ISC_REQ_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_MANUAL_CRED_VALIDATION: ISC_REQ_FLAGS = ISC_REQ_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_RESERVED1: ISC_REQ_FLAGS = ISC_REQ_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_FRAGMENT_TO_FIT: ISC_REQ_FLAGS = ISC_REQ_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_FORWARD_CREDENTIALS: ISC_REQ_FLAGS = ISC_REQ_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_NO_INTEGRITY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_USE_HTTP_STYLE: ISC_REQ_FLAGS = ISC_REQ_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_UNVERIFIED_TARGET_NAME: ISC_REQ_FLAGS = ISC_REQ_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_CONFIDENTIALITY_ONLY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(1073741824u32);
impl ::core::marker::Copy for ISC_REQ_FLAGS {}
impl ::core::clone::Clone for ISC_REQ_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ISC_REQ_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ISC_REQ_HIGH_FLAGS(pub u64);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_MESSAGES: ISC_REQ_HIGH_FLAGS = ISC_REQ_HIGH_FLAGS(4294967296u64);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ISC_REQ_DEFERRED_CRED_VALIDATION: ISC_REQ_HIGH_FLAGS = ISC_REQ_HIGH_FLAGS(8589934592u64);
impl ::core::marker::Copy for ISC_REQ_HIGH_FLAGS {}
impl ::core::clone::Clone for ISC_REQ_HIGH_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ISC_REQ_HIGH_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KERB_ADDRESS_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DS_INET_ADDRESS: KERB_ADDRESS_TYPE = KERB_ADDRESS_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DS_NETBIOS_ADDRESS: KERB_ADDRESS_TYPE = KERB_ADDRESS_TYPE(2u32);
impl ::core::marker::Copy for KERB_ADDRESS_TYPE {}
impl ::core::clone::Clone for KERB_ADDRESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_ADDRESS_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KERB_CERTIFICATE_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CertHashInfo: KERB_CERTIFICATE_INFO_TYPE = KERB_CERTIFICATE_INFO_TYPE(1i32);
impl ::core::marker::Copy for KERB_CERTIFICATE_INFO_TYPE {}
impl ::core::clone::Clone for KERB_CERTIFICATE_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_CERTIFICATE_INFO_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KERB_CRYPTO_KEY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DES_CBC_CRC: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DES_CBC_MD4: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_DES_CBC_MD5: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_NULL: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_HMAC_NT: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_ETYPE_RC4_MD4: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(-128i32);
impl ::core::marker::Copy for KERB_CRYPTO_KEY_TYPE {}
impl ::core::clone::Clone for KERB_CRYPTO_KEY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_CRYPTO_KEY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KERB_LOGON_SUBMIT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbInteractiveLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbSmartCardLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbWorkstationUnlockLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbSmartCardUnlockLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbProxyLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbTicketLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbTicketUnlockLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbS4ULogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbCertificateLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbCertificateS4ULogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbCertificateUnlockLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbNoElevationLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(83i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbLuidLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(84i32);
impl ::core::marker::Copy for KERB_LOGON_SUBMIT_TYPE {}
impl ::core::clone::Clone for KERB_LOGON_SUBMIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_LOGON_SUBMIT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KERB_PROFILE_BUFFER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbInteractiveProfile: KERB_PROFILE_BUFFER_TYPE = KERB_PROFILE_BUFFER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbSmartCardProfile: KERB_PROFILE_BUFFER_TYPE = KERB_PROFILE_BUFFER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbTicketProfile: KERB_PROFILE_BUFFER_TYPE = KERB_PROFILE_BUFFER_TYPE(6i32);
impl ::core::marker::Copy for KERB_PROFILE_BUFFER_TYPE {}
impl ::core::clone::Clone for KERB_PROFILE_BUFFER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_PROFILE_BUFFER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KERB_PROTOCOL_MESSAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbDebugRequestMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbQueryTicketCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbChangeMachinePasswordMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbVerifyPacMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbRetrieveTicketMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbUpdateAddressesMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbPurgeTicketCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbChangePasswordMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbRetrieveEncodedTicketMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbDecryptDataMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbAddBindingCacheEntryMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbSetPasswordMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbSetPasswordExMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbVerifyCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbQueryTicketCacheExMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbPurgeTicketCacheExMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbRefreshSmartcardCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbAddExtraCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbQuerySupplementalCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbTransferCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbQueryTicketCacheEx2Message: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbSubmitTicketMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbAddExtraCredentialsExMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbQueryKdcProxyCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbPurgeKdcProxyCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbQueryTicketCacheEx3Message: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbCleanupMachinePkinitCredsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbAddBindingCacheEntryExMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbQueryBindingCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbPurgeBindingCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbPinKdcMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbUnpinAllKdcsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbQueryDomainExtendedPoliciesMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbQueryS4U2ProxyCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbRetrieveKeyTabMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbRefreshPolicyMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(35i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KerbPrintCloudKerberosDebugMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(36i32);
impl ::core::marker::Copy for KERB_PROTOCOL_MESSAGE_TYPE {}
impl ::core::clone::Clone for KERB_PROTOCOL_MESSAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_PROTOCOL_MESSAGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KERB_REQUEST_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_REQUEST_ADD_CREDENTIAL: KERB_REQUEST_FLAGS = KERB_REQUEST_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_REQUEST_REPLACE_CREDENTIAL: KERB_REQUEST_FLAGS = KERB_REQUEST_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_REQUEST_REMOVE_CREDENTIAL: KERB_REQUEST_FLAGS = KERB_REQUEST_FLAGS(4u32);
impl ::core::marker::Copy for KERB_REQUEST_FLAGS {}
impl ::core::clone::Clone for KERB_REQUEST_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_REQUEST_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KERB_TICKET_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_forwardable: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(1073741824u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_forwarded: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_hw_authent: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_initial: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_invalid: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_may_postdate: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(67108864u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_ok_as_delegate: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_postdated: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_pre_authent: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_proxiable: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_proxy: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(134217728u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_renewable: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_reserved: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KERB_TICKET_FLAGS_reserved1: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(1u32);
impl ::core::marker::Copy for KERB_TICKET_FLAGS {}
impl ::core::clone::Clone for KERB_TICKET_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_TICKET_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KSEC_CONTEXT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KSecPaged: KSEC_CONTEXT_TYPE = KSEC_CONTEXT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const KSecNonPaged: KSEC_CONTEXT_TYPE = KSEC_CONTEXT_TYPE(1i32);
impl ::core::marker::Copy for KSEC_CONTEXT_TYPE {}
impl ::core::clone::Clone for KSEC_CONTEXT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KSEC_CONTEXT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LSA_AUTH_INFORMATION_AUTH_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_AUTH_TYPE_NONE: LSA_AUTH_INFORMATION_AUTH_TYPE = LSA_AUTH_INFORMATION_AUTH_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_AUTH_TYPE_NT4OWF: LSA_AUTH_INFORMATION_AUTH_TYPE = LSA_AUTH_INFORMATION_AUTH_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_AUTH_TYPE_CLEAR: LSA_AUTH_INFORMATION_AUTH_TYPE = LSA_AUTH_INFORMATION_AUTH_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_AUTH_TYPE_VERSION: LSA_AUTH_INFORMATION_AUTH_TYPE = LSA_AUTH_INFORMATION_AUTH_TYPE(3u32);
impl ::core::marker::Copy for LSA_AUTH_INFORMATION_AUTH_TYPE {}
impl ::core::clone::Clone for LSA_AUTH_INFORMATION_AUTH_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LSA_AUTH_INFORMATION_AUTH_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LSA_FOREST_TRUST_COLLISION_RECORD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CollisionTdo: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE = LSA_FOREST_TRUST_COLLISION_RECORD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CollisionXref: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE = LSA_FOREST_TRUST_COLLISION_RECORD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const CollisionOther: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE = LSA_FOREST_TRUST_COLLISION_RECORD_TYPE(2i32);
impl ::core::marker::Copy for LSA_FOREST_TRUST_COLLISION_RECORD_TYPE {}
impl ::core::clone::Clone for LSA_FOREST_TRUST_COLLISION_RECORD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LSA_FOREST_TRUST_COLLISION_RECORD_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LSA_FOREST_TRUST_RECORD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ForestTrustTopLevelName: LSA_FOREST_TRUST_RECORD_TYPE = LSA_FOREST_TRUST_RECORD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ForestTrustTopLevelNameEx: LSA_FOREST_TRUST_RECORD_TYPE = LSA_FOREST_TRUST_RECORD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ForestTrustDomainInfo: LSA_FOREST_TRUST_RECORD_TYPE = LSA_FOREST_TRUST_RECORD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ForestTrustRecordTypeLast: LSA_FOREST_TRUST_RECORD_TYPE = LSA_FOREST_TRUST_RECORD_TYPE(2i32);
impl ::core::marker::Copy for LSA_FOREST_TRUST_RECORD_TYPE {}
impl ::core::clone::Clone for LSA_FOREST_TRUST_RECORD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LSA_FOREST_TRUST_RECORD_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LSA_LOOKUP_DOMAIN_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AccountDomainInformation: LSA_LOOKUP_DOMAIN_INFO_CLASS = LSA_LOOKUP_DOMAIN_INFO_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DnsDomainInformation: LSA_LOOKUP_DOMAIN_INFO_CLASS = LSA_LOOKUP_DOMAIN_INFO_CLASS(12i32);
impl ::core::marker::Copy for LSA_LOOKUP_DOMAIN_INFO_CLASS {}
impl ::core::clone::Clone for LSA_LOOKUP_DOMAIN_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LSA_LOOKUP_DOMAIN_INFO_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LSA_TOKEN_INFORMATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LsaTokenInformationNull: LSA_TOKEN_INFORMATION_TYPE = LSA_TOKEN_INFORMATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LsaTokenInformationV1: LSA_TOKEN_INFORMATION_TYPE = LSA_TOKEN_INFORMATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LsaTokenInformationV2: LSA_TOKEN_INFORMATION_TYPE = LSA_TOKEN_INFORMATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LsaTokenInformationV3: LSA_TOKEN_INFORMATION_TYPE = LSA_TOKEN_INFORMATION_TYPE(3i32);
impl ::core::marker::Copy for LSA_TOKEN_INFORMATION_TYPE {}
impl ::core::clone::Clone for LSA_TOKEN_INFORMATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LSA_TOKEN_INFORMATION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSV1_0(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_PASSTHRU: MSV1_0 = MSV1_0(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_GUEST_LOGON: MSV1_0 = MSV1_0(2u32);
impl ::core::marker::Copy for MSV1_0 {}
impl ::core::clone::Clone for MSV1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSV1_0_AVID(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsvAvEOL: MSV1_0_AVID = MSV1_0_AVID(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsvAvNbComputerName: MSV1_0_AVID = MSV1_0_AVID(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsvAvNbDomainName: MSV1_0_AVID = MSV1_0_AVID(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsvAvDnsComputerName: MSV1_0_AVID = MSV1_0_AVID(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsvAvDnsDomainName: MSV1_0_AVID = MSV1_0_AVID(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsvAvDnsTreeName: MSV1_0_AVID = MSV1_0_AVID(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsvAvFlags: MSV1_0_AVID = MSV1_0_AVID(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsvAvTimestamp: MSV1_0_AVID = MSV1_0_AVID(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsvAvRestrictions: MSV1_0_AVID = MSV1_0_AVID(8i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsvAvTargetName: MSV1_0_AVID = MSV1_0_AVID(9i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsvAvChannelBindings: MSV1_0_AVID = MSV1_0_AVID(10i32);
impl ::core::marker::Copy for MSV1_0_AVID {}
impl ::core::clone::Clone for MSV1_0_AVID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_AVID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSV1_0_CREDENTIAL_KEY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const InvalidCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = MSV1_0_CREDENTIAL_KEY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DeprecatedIUMCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = MSV1_0_CREDENTIAL_KEY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const DomainUserCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = MSV1_0_CREDENTIAL_KEY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LocalUserCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = MSV1_0_CREDENTIAL_KEY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const ExternallySuppliedCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = MSV1_0_CREDENTIAL_KEY_TYPE(4i32);
impl ::core::marker::Copy for MSV1_0_CREDENTIAL_KEY_TYPE {}
impl ::core::clone::Clone for MSV1_0_CREDENTIAL_KEY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_CREDENTIAL_KEY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSV1_0_LOGON_SUBMIT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0InteractiveLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0Lm20Logon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0NetworkLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0SubAuthLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0WorkstationUnlockLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0S4ULogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0VirtualLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(82i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0NoElevationLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(83i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0LuidLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(84i32);
impl ::core::marker::Copy for MSV1_0_LOGON_SUBMIT_TYPE {}
impl ::core::clone::Clone for MSV1_0_LOGON_SUBMIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_LOGON_SUBMIT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSV1_0_PROFILE_BUFFER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0InteractiveProfile: MSV1_0_PROFILE_BUFFER_TYPE = MSV1_0_PROFILE_BUFFER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0Lm20LogonProfile: MSV1_0_PROFILE_BUFFER_TYPE = MSV1_0_PROFILE_BUFFER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0SmartCardProfile: MSV1_0_PROFILE_BUFFER_TYPE = MSV1_0_PROFILE_BUFFER_TYPE(4i32);
impl ::core::marker::Copy for MSV1_0_PROFILE_BUFFER_TYPE {}
impl ::core::clone::Clone for MSV1_0_PROFILE_BUFFER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_PROFILE_BUFFER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSV1_0_PROTOCOL_MESSAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0Lm20ChallengeRequest: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0Lm20GetChallengeResponse: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0EnumerateUsers: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0GetUserInfo: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0ReLogonUsers: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0ChangePassword: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0ChangeCachedPassword: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0GenericPassthrough: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0CacheLogon: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0SubAuth: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0DeriveCredential: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0CacheLookup: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0SetProcessOption: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0ConfigLocalAliases: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0ClearCachedCredentials: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0LookupToken: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0ValidateAuth: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0CacheLookupEx: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0GetCredentialKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0SetThreadOption: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0DecryptDpapiMasterKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0GetStrongCredentialKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0TransferCred: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0ProvisionTbal: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MsV1_0DeleteTbalSecrets: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(24i32);
impl ::core::marker::Copy for MSV1_0_PROTOCOL_MESSAGE_TYPE {}
impl ::core::clone::Clone for MSV1_0_PROTOCOL_MESSAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_PROTOCOL_MESSAGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CLEARTEXT_PASSWORD_ALLOWED: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_UPDATE_LOGON_STATISTICS: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(4u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_RETURN_USER_PARAMETERS: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(8u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_DONT_TRY_GUEST_ACCOUNT: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(16u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_ALLOW_SERVER_TRUST_ACCOUNT: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(32u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_RETURN_PASSWORD_EXPIRY: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(64u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_ALLOW_WORKSTATION_TRUST_ACCOUNT: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(2048u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_TRY_GUEST_ACCOUNT_ONLY: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(256u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_RETURN_PROFILE_PATH: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(512u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_TRY_SPECIFIED_DOMAIN_ONLY: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(1024u32);
impl ::core::marker::Copy for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {}
impl ::core::clone::Clone for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSV_SUB_AUTHENTICATION_FILTER(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_GUEST: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_NOENCRYPTION: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_CACHED_ACCOUNT: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(4u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_USED_LM_PASSWORD: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(8u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_EXTRA_SIDS: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(32u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_SUBAUTH_SESSION_KEY: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(64u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_SERVER_TRUST_ACCOUNT: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(128u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_PROFILE_PATH_RETURNED: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(1024u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const LOGON_RESOURCE_GROUPS: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(512u32);
impl ::core::marker::Copy for MSV_SUB_AUTHENTICATION_FILTER {}
impl ::core::clone::Clone for MSV_SUB_AUTHENTICATION_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV_SUB_AUTHENTICATION_FILTER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_LM_PRESENT: MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS = MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_NT_PRESENT: MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS = MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const MSV1_0_CRED_VERSION: MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS = MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS(0u32);
impl ::core::marker::Copy for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {}
impl ::core::clone::Clone for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NEGOTIATE_MESSAGES(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NegEnumPackagePrefixes: NEGOTIATE_MESSAGES = NEGOTIATE_MESSAGES(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NegGetCallerName: NEGOTIATE_MESSAGES = NEGOTIATE_MESSAGES(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NegTransferCredentials: NEGOTIATE_MESSAGES = NEGOTIATE_MESSAGES(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NegMsgReserved1: NEGOTIATE_MESSAGES = NEGOTIATE_MESSAGES(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NegCallPackageMax: NEGOTIATE_MESSAGES = NEGOTIATE_MESSAGES(4i32);
impl ::core::marker::Copy for NEGOTIATE_MESSAGES {}
impl ::core::clone::Clone for NEGOTIATE_MESSAGES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NEGOTIATE_MESSAGES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETLOGON_LOGON_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NetlogonInteractiveInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NetlogonNetworkInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NetlogonServiceInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NetlogonGenericInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NetlogonInteractiveTransitiveInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NetlogonNetworkTransitiveInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const NetlogonServiceTransitiveInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(7i32);
impl ::core::marker::Copy for NETLOGON_LOGON_INFO_CLASS {}
impl ::core::clone::Clone for NETLOGON_LOGON_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NETLOGON_LOGON_INFO_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PKU2U_LOGON_SUBMIT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Pku2uCertificateS4ULogon: PKU2U_LOGON_SUBMIT_TYPE = PKU2U_LOGON_SUBMIT_TYPE(14i32);
impl ::core::marker::Copy for PKU2U_LOGON_SUBMIT_TYPE {}
impl ::core::clone::Clone for PKU2U_LOGON_SUBMIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PKU2U_LOGON_SUBMIT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POLICY_AUDIT_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AuditCategorySystem: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AuditCategoryLogon: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AuditCategoryObjectAccess: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AuditCategoryPrivilegeUse: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AuditCategoryDetailedTracking: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AuditCategoryPolicyChange: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AuditCategoryAccountManagement: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AuditCategoryDirectoryServiceAccess: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const AuditCategoryAccountLogon: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(8i32);
impl ::core::marker::Copy for POLICY_AUDIT_EVENT_TYPE {}
impl ::core::clone::Clone for POLICY_AUDIT_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_AUDIT_EVENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POLICY_DOMAIN_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyDomainEfsInformation: POLICY_DOMAIN_INFORMATION_CLASS = POLICY_DOMAIN_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyDomainKerberosTicketInformation: POLICY_DOMAIN_INFORMATION_CLASS = POLICY_DOMAIN_INFORMATION_CLASS(3i32);
impl ::core::marker::Copy for POLICY_DOMAIN_INFORMATION_CLASS {}
impl ::core::clone::Clone for POLICY_DOMAIN_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_DOMAIN_INFORMATION_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POLICY_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyAuditLogInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyAuditEventsInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyPrimaryDomainInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyPdAccountInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyAccountDomainInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyLsaServerRoleInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyReplicaSourceInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyDefaultQuotaInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(8i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyModificationInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(9i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyAuditFullSetInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(10i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyAuditFullQueryInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(11i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyDnsDomainInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(12i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyDnsDomainInformationInt: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(13i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyLocalAccountDomainInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(14i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyMachineAccountInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(15i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyLastEntry: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(16i32);
impl ::core::marker::Copy for POLICY_INFORMATION_CLASS {}
impl ::core::clone::Clone for POLICY_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_INFORMATION_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POLICY_LSA_SERVER_ROLE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyServerRoleBackup: POLICY_LSA_SERVER_ROLE = POLICY_LSA_SERVER_ROLE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyServerRolePrimary: POLICY_LSA_SERVER_ROLE = POLICY_LSA_SERVER_ROLE(3i32);
impl ::core::marker::Copy for POLICY_LSA_SERVER_ROLE {}
impl ::core::clone::Clone for POLICY_LSA_SERVER_ROLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_LSA_SERVER_ROLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POLICY_NOTIFICATION_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyNotifyAuditEventsInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyNotifyAccountDomainInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyNotifyServerRoleInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyNotifyDnsDomainInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyNotifyDomainEfsInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyNotifyDomainKerberosTicketInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyNotifyMachineAccountPasswordInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyNotifyGlobalSaclInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(8i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const PolicyNotifyMax: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(9i32);
impl ::core::marker::Copy for POLICY_NOTIFICATION_INFORMATION_CLASS {}
impl ::core::clone::Clone for POLICY_NOTIFICATION_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_NOTIFICATION_INFORMATION_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SASL_AUTHZID_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Sasl_AuthZIDForbidden: SASL_AUTHZID_STATE = SASL_AUTHZID_STATE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const Sasl_AuthZIDProcessed: SASL_AUTHZID_STATE = SASL_AUTHZID_STATE(1i32);
impl ::core::marker::Copy for SASL_AUTHZID_STATE {}
impl ::core::clone::Clone for SASL_AUTHZID_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SASL_AUTHZID_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCHANNEL_ALERT_TOKEN_ALERT_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_WARNING: SCHANNEL_ALERT_TOKEN_ALERT_TYPE = SCHANNEL_ALERT_TOKEN_ALERT_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TLS1_ALERT_FATAL: SCHANNEL_ALERT_TOKEN_ALERT_TYPE = SCHANNEL_ALERT_TOKEN_ALERT_TYPE(2u32);
impl ::core::marker::Copy for SCHANNEL_ALERT_TOKEN_ALERT_TYPE {}
impl ::core::clone::Clone for SCHANNEL_ALERT_TOKEN_ALERT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCHANNEL_ALERT_TOKEN_ALERT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCHANNEL_CRED_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_AUTO_CRED_VALIDATION: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_CACHE_ONLY_URL_RETRIEVAL_ON_CREATE: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_DISABLE_RECONNECTS: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_IGNORE_NO_REVOCATION_CHECK: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_IGNORE_REVOCATION_OFFLINE: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_MANUAL_CRED_VALIDATION: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_NO_DEFAULT_CREDS: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_NO_SERVERNAME_CHECK: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_NO_SYSTEM_MAPPER: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_REVOCATION_CHECK_CHAIN: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_REVOCATION_CHECK_END_CERT: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_CRED_USE_DEFAULT_CREDS: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_SEND_AUX_RECORD: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_SEND_ROOT_CERT: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_USE_STRONG_CRYPTO: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_USE_PRESHAREDKEY_ONLY: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(8388608u32);
impl ::core::marker::Copy for SCHANNEL_CRED_FLAGS {}
impl ::core::clone::Clone for SCHANNEL_CRED_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCHANNEL_CRED_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCHANNEL_SESSION_TOKEN_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSL_SESSION_ENABLE_RECONNECTS: SCHANNEL_SESSION_TOKEN_FLAGS = SCHANNEL_SESSION_TOKEN_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SSL_SESSION_DISABLE_RECONNECTS: SCHANNEL_SESSION_TOKEN_FLAGS = SCHANNEL_SESSION_TOKEN_FLAGS(2u32);
impl ::core::marker::Copy for SCHANNEL_SESSION_TOKEN_FLAGS {}
impl ::core::clone::Clone for SCHANNEL_SESSION_TOKEN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCHANNEL_SESSION_TOKEN_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECPKG_ATTR(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_C_ACCESS_TOKEN: SECPKG_ATTR = SECPKG_ATTR(2147483666u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_C_FULL_ACCESS_TOKEN: SECPKG_ATTR = SECPKG_ATTR(2147483778u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CERT_TRUST_STATUS: SECPKG_ATTR = SECPKG_ATTR(2147483780u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CREDS: SECPKG_ATTR = SECPKG_ATTR(2147483776u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CREDS_2: SECPKG_ATTR = SECPKG_ATTR(2147483782u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_NEGOTIATION_PACKAGE: SECPKG_ATTR = SECPKG_ATTR(2147483777u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_PACKAGE_INFO: SECPKG_ATTR = SECPKG_ATTR(10u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_SERVER_AUTH_FLAGS: SECPKG_ATTR = SECPKG_ATTR(2147483779u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_SIZES: SECPKG_ATTR = SECPKG_ATTR(0u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_SUBJECT_SECURITY_ATTRIBUTES: SECPKG_ATTR = SECPKG_ATTR(124u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_APP_DATA: SECPKG_ATTR = SECPKG_ATTR(94u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_EAP_PRF_INFO: SECPKG_ATTR = SECPKG_ATTR(101u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_EARLY_START: SECPKG_ATTR = SECPKG_ATTR(105u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_DTLS_MTU: SECPKG_ATTR = SECPKG_ATTR(34u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_KEYING_MATERIAL_INFO: SECPKG_ATTR = SECPKG_ATTR(106u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_ACCESS_TOKEN: SECPKG_ATTR = SECPKG_ATTR(18u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_AUTHORITY: SECPKG_ATTR = SECPKG_ATTR(6u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CLIENT_SPECIFIED_TARGET: SECPKG_ATTR = SECPKG_ATTR(27u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_CONNECTION_INFO: SECPKG_ATTR = SECPKG_ATTR(90u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_DCE_INFO: SECPKG_ATTR = SECPKG_ATTR(3u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_ENDPOINT_BINDINGS: SECPKG_ATTR = SECPKG_ATTR(26u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_EAP_KEY_BLOCK: SECPKG_ATTR = SECPKG_ATTR(91u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_FLAGS: SECPKG_ATTR = SECPKG_ATTR(14u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_ISSUER_LIST_EX: SECPKG_ATTR = SECPKG_ATTR(89u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_KEY_INFO: SECPKG_ATTR = SECPKG_ATTR(5u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_LAST_CLIENT_TOKEN_STATUS: SECPKG_ATTR = SECPKG_ATTR(30u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_LIFESPAN: SECPKG_ATTR = SECPKG_ATTR(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_LOCAL_CERT_CONTEXT: SECPKG_ATTR = SECPKG_ATTR(84u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_LOCAL_CRED: SECPKG_ATTR = SECPKG_ATTR(82u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_NAMES: SECPKG_ATTR = SECPKG_ATTR(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_NATIVE_NAMES: SECPKG_ATTR = SECPKG_ATTR(13u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_NEGOTIATION_INFO: SECPKG_ATTR = SECPKG_ATTR(12u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_PASSWORD_EXPIRY: SECPKG_ATTR = SECPKG_ATTR(8u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_REMOTE_CERT_CONTEXT: SECPKG_ATTR = SECPKG_ATTR(83u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_ROOT_STORE: SECPKG_ATTR = SECPKG_ATTR(85u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_SESSION_KEY: SECPKG_ATTR = SECPKG_ATTR(9u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_SESSION_INFO: SECPKG_ATTR = SECPKG_ATTR(93u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_STREAM_SIZES: SECPKG_ATTR = SECPKG_ATTR(4u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_SUPPORTED_SIGNATURES: SECPKG_ATTR = SECPKG_ATTR(102u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_TARGET_INFORMATION: SECPKG_ATTR = SECPKG_ATTR(17u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_ATTR_UNIQUE_BINDINGS: SECPKG_ATTR = SECPKG_ATTR(25u32);
impl ::core::marker::Copy for SECPKG_ATTR {}
impl ::core::clone::Clone for SECPKG_ATTR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_ATTR {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECPKG_ATTR_LCT_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgAttrLastClientTokenYes: SECPKG_ATTR_LCT_STATUS = SECPKG_ATTR_LCT_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgAttrLastClientTokenNo: SECPKG_ATTR_LCT_STATUS = SECPKG_ATTR_LCT_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgAttrLastClientTokenMaybe: SECPKG_ATTR_LCT_STATUS = SECPKG_ATTR_LCT_STATUS(2i32);
impl ::core::marker::Copy for SECPKG_ATTR_LCT_STATUS {}
impl ::core::clone::Clone for SECPKG_ATTR_LCT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_ATTR_LCT_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECPKG_CALL_PACKAGE_MESSAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgCallPackageMinMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = SECPKG_CALL_PACKAGE_MESSAGE_TYPE(1024i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgCallPackagePinDcMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = SECPKG_CALL_PACKAGE_MESSAGE_TYPE(1024i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgCallPackageUnpinAllDcsMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = SECPKG_CALL_PACKAGE_MESSAGE_TYPE(1025i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgCallPackageTransferCredMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = SECPKG_CALL_PACKAGE_MESSAGE_TYPE(1026i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgCallPackageMaxMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = SECPKG_CALL_PACKAGE_MESSAGE_TYPE(1026i32);
impl ::core::marker::Copy for SECPKG_CALL_PACKAGE_MESSAGE_TYPE {}
impl ::core::clone::Clone for SECPKG_CALL_PACKAGE_MESSAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_CALL_PACKAGE_MESSAGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECPKG_CRED(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_INBOUND: SECPKG_CRED = SECPKG_CRED(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_CRED_OUTBOUND: SECPKG_CRED = SECPKG_CRED(2u32);
impl ::core::marker::Copy for SECPKG_CRED {}
impl ::core::clone::Clone for SECPKG_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_CRED {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECPKG_CRED_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgCredClass_None: SECPKG_CRED_CLASS = SECPKG_CRED_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgCredClass_Ephemeral: SECPKG_CRED_CLASS = SECPKG_CRED_CLASS(10i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgCredClass_PersistedGeneric: SECPKG_CRED_CLASS = SECPKG_CRED_CLASS(20i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgCredClass_PersistedSpecific: SECPKG_CRED_CLASS = SECPKG_CRED_CLASS(30i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecPkgCredClass_Explicit: SECPKG_CRED_CLASS = SECPKG_CRED_CLASS(40i32);
impl ::core::marker::Copy for SECPKG_CRED_CLASS {}
impl ::core::clone::Clone for SECPKG_CRED_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_CRED_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECPKG_EXTENDED_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecpkgGssInfo: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecpkgContextThunks: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecpkgMutualAuthLevel: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecpkgWowClientDll: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecpkgExtraOids: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecpkgMaxInfo: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecpkgNego2Info: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(7i32);
impl ::core::marker::Copy for SECPKG_EXTENDED_INFORMATION_CLASS {}
impl ::core::clone::Clone for SECPKG_EXTENDED_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_EXTENDED_INFORMATION_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECPKG_NAME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecNameSamCompatible: SECPKG_NAME_TYPE = SECPKG_NAME_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecNameAlternateId: SECPKG_NAME_TYPE = SECPKG_NAME_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecNameFlat: SECPKG_NAME_TYPE = SECPKG_NAME_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecNameDN: SECPKG_NAME_TYPE = SECPKG_NAME_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecNameSPN: SECPKG_NAME_TYPE = SECPKG_NAME_TYPE(4i32);
impl ::core::marker::Copy for SECPKG_NAME_TYPE {}
impl ::core::clone::Clone for SECPKG_NAME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_NAME_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECPKG_PACKAGE_CHANGE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_PACKAGE_CHANGE_LOAD: SECPKG_PACKAGE_CHANGE_TYPE = SECPKG_PACKAGE_CHANGE_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_PACKAGE_CHANGE_UNLOAD: SECPKG_PACKAGE_CHANGE_TYPE = SECPKG_PACKAGE_CHANGE_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_PACKAGE_CHANGE_SELECT: SECPKG_PACKAGE_CHANGE_TYPE = SECPKG_PACKAGE_CHANGE_TYPE(2u32);
impl ::core::marker::Copy for SECPKG_PACKAGE_CHANGE_TYPE {}
impl ::core::clone::Clone for SECPKG_PACKAGE_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_PACKAGE_CHANGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECPKG_SESSIONINFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecSessionPrimaryCred: SECPKG_SESSIONINFO_TYPE = SECPKG_SESSIONINFO_TYPE(0i32);
impl ::core::marker::Copy for SECPKG_SESSIONINFO_TYPE {}
impl ::core::clone::Clone for SECPKG_SESSIONINFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_SESSIONINFO_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECURITY_LOGON_TYPE(pub i32);
impl SECURITY_LOGON_TYPE {
    pub const UndefinedLogonType: Self = Self(0i32);
    pub const Interactive: Self = Self(2i32);
    pub const Network: Self = Self(3i32);
    pub const Batch: Self = Self(4i32);
    pub const Service: Self = Self(5i32);
    pub const Proxy: Self = Self(6i32);
    pub const Unlock: Self = Self(7i32);
    pub const NetworkCleartext: Self = Self(8i32);
    pub const NewCredentials: Self = Self(9i32);
    pub const RemoteInteractive: Self = Self(10i32);
    pub const CachedInteractive: Self = Self(11i32);
    pub const CachedRemoteInteractive: Self = Self(12i32);
    pub const CachedUnlock: Self = Self(13i32);
}
impl ::core::marker::Copy for SECURITY_LOGON_TYPE {}
impl ::core::clone::Clone for SECURITY_LOGON_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECURITY_LOGON_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECURITY_PACKAGE_OPTIONS_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_OPTIONS_TYPE_UNKNOWN: SECURITY_PACKAGE_OPTIONS_TYPE = SECURITY_PACKAGE_OPTIONS_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_OPTIONS_TYPE_LSA: SECURITY_PACKAGE_OPTIONS_TYPE = SECURITY_PACKAGE_OPTIONS_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SECPKG_OPTIONS_TYPE_SSPI: SECURITY_PACKAGE_OPTIONS_TYPE = SECURITY_PACKAGE_OPTIONS_TYPE(2u32);
impl ::core::marker::Copy for SECURITY_PACKAGE_OPTIONS_TYPE {}
impl ::core::clone::Clone for SECURITY_PACKAGE_OPTIONS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECURITY_PACKAGE_OPTIONS_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecApplicationProtocolNegotiationExt_None: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecApplicationProtocolNegotiationExt_NPN: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecApplicationProtocolNegotiationExt_ALPN: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT(2i32);
impl ::core::marker::Copy for SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT {}
impl ::core::clone::Clone for SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecApplicationProtocolNegotiationStatus_None: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS = SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecApplicationProtocolNegotiationStatus_Success: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS = SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecApplicationProtocolNegotiationStatus_SelectedClientOnly: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS = SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS(2i32);
impl ::core::marker::Copy for SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS {}
impl ::core::clone::Clone for SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SEC_TRAFFIC_SECRET_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecTrafficSecret_None: SEC_TRAFFIC_SECRET_TYPE = SEC_TRAFFIC_SECRET_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecTrafficSecret_Client: SEC_TRAFFIC_SECRET_TYPE = SEC_TRAFFIC_SECRET_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecTrafficSecret_Server: SEC_TRAFFIC_SECRET_TYPE = SEC_TRAFFIC_SECRET_TYPE(2i32);
impl ::core::marker::Copy for SEC_TRAFFIC_SECRET_TYPE {}
impl ::core::clone::Clone for SEC_TRAFFIC_SECRET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_TRAFFIC_SECRET_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SE_ADT_PARAMETER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeNone: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeString: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeFileSpec: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeUlong: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeSid: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeLogonId: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeNoLogonId: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeAccessMask: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypePrivs: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeObjectTypes: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeHexUlong: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypePtr: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeTime: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeGuid: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeLuid: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeHexInt64: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeStringList: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeSidList: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeDuration: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeUserAccountControl: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeNoUac: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeMessage: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeDateTime: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeSockAddr: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeSD: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeLogonHours: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeLogonIdNoSid: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeUlongNoConv: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeSockAddrNoPort: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeAccessReason: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeStagingReason: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeResourceAttribute: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeClaims: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeLogonIdAsSid: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeMultiSzString: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SeAdtParmTypeLogonIdEx: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(35i32);
impl ::core::marker::Copy for SE_ADT_PARAMETER_TYPE {}
impl ::core::clone::Clone for SE_ADT_PARAMETER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SE_ADT_PARAMETER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SLDATATYPE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_DATA_NONE: SLDATATYPE = SLDATATYPE(0u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_DATA_SZ: SLDATATYPE = SLDATATYPE(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_DATA_DWORD: SLDATATYPE = SLDATATYPE(4u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_DATA_BINARY: SLDATATYPE = SLDATATYPE(3u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_DATA_MULTI_SZ: SLDATATYPE = SLDATATYPE(7u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_DATA_SUM: SLDATATYPE = SLDATATYPE(100u32);
impl ::core::marker::Copy for SLDATATYPE {}
impl ::core::clone::Clone for SLDATATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SLDATATYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SLIDTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_ID_APPLICATION: SLIDTYPE = SLIDTYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_ID_PRODUCT_SKU: SLIDTYPE = SLIDTYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_ID_LICENSE_FILE: SLIDTYPE = SLIDTYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_ID_LICENSE: SLIDTYPE = SLIDTYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_ID_PKEY: SLIDTYPE = SLIDTYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_ID_ALL_LICENSES: SLIDTYPE = SLIDTYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_ID_ALL_LICENSE_FILES: SLIDTYPE = SLIDTYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_ID_STORE_TOKEN: SLIDTYPE = SLIDTYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_ID_LAST: SLIDTYPE = SLIDTYPE(8i32);
impl ::core::marker::Copy for SLIDTYPE {}
impl ::core::clone::Clone for SLIDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SLIDTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SLLICENSINGSTATUS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_LICENSING_STATUS_UNLICENSED: SLLICENSINGSTATUS = SLLICENSINGSTATUS(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_LICENSING_STATUS_LICENSED: SLLICENSINGSTATUS = SLLICENSINGSTATUS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_LICENSING_STATUS_IN_GRACE_PERIOD: SLLICENSINGSTATUS = SLLICENSINGSTATUS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_LICENSING_STATUS_NOTIFICATION: SLLICENSINGSTATUS = SLLICENSINGSTATUS(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_LICENSING_STATUS_LAST: SLLICENSINGSTATUS = SLLICENSINGSTATUS(4i32);
impl ::core::marker::Copy for SLLICENSINGSTATUS {}
impl ::core::clone::Clone for SLLICENSINGSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SLLICENSINGSTATUS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SLREFERRALTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REFERRALTYPE_SKUID: SLREFERRALTYPE = SLREFERRALTYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REFERRALTYPE_APPID: SLREFERRALTYPE = SLREFERRALTYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REFERRALTYPE_OVERRIDE_SKUID: SLREFERRALTYPE = SLREFERRALTYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REFERRALTYPE_OVERRIDE_APPID: SLREFERRALTYPE = SLREFERRALTYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_REFERRALTYPE_BEST_MATCH: SLREFERRALTYPE = SLREFERRALTYPE(4i32);
impl ::core::marker::Copy for SLREFERRALTYPE {}
impl ::core::clone::Clone for SLREFERRALTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SLREFERRALTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SL_ACTIVATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_ACTIVATION_TYPE_DEFAULT: SL_ACTIVATION_TYPE = SL_ACTIVATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_ACTIVATION_TYPE_ACTIVE_DIRECTORY: SL_ACTIVATION_TYPE = SL_ACTIVATION_TYPE(1i32);
impl ::core::marker::Copy for SL_ACTIVATION_TYPE {}
impl ::core::clone::Clone for SL_ACTIVATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SL_ACTIVATION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SL_GENUINE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_GEN_STATE_IS_GENUINE: SL_GENUINE_STATE = SL_GENUINE_STATE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_GEN_STATE_INVALID_LICENSE: SL_GENUINE_STATE = SL_GENUINE_STATE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_GEN_STATE_TAMPERED: SL_GENUINE_STATE = SL_GENUINE_STATE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_GEN_STATE_OFFLINE: SL_GENUINE_STATE = SL_GENUINE_STATE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SL_GEN_STATE_LAST: SL_GENUINE_STATE = SL_GENUINE_STATE(4i32);
impl ::core::marker::Copy for SL_GENUINE_STATE {}
impl ::core::clone::Clone for SL_GENUINE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SL_GENUINE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SchGetExtensionsOptions(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_EXTENSIONS_OPTIONS_NONE: SchGetExtensionsOptions = SchGetExtensionsOptions(0u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SCH_NO_RECORD_HEADER: SchGetExtensionsOptions = SchGetExtensionsOptions(1u32);
impl ::core::marker::Copy for SchGetExtensionsOptions {}
impl ::core::clone::Clone for SchGetExtensionsOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SchGetExtensionsOptions {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SecDelegationType(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecFull: SecDelegationType = SecDelegationType(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecService: SecDelegationType = SecDelegationType(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecTree: SecDelegationType = SecDelegationType(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecDirectory: SecDelegationType = SecDelegationType(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const SecObject: SecDelegationType = SecDelegationType(4i32);
impl ::core::marker::Copy for SecDelegationType {}
impl ::core::clone::Clone for SecDelegationType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecDelegationType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOKENBINDING_EXTENSION_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TOKENBINDING_EXTENSION_FORMAT_UNDEFINED: TOKENBINDING_EXTENSION_FORMAT = TOKENBINDING_EXTENSION_FORMAT(0i32);
impl ::core::marker::Copy for TOKENBINDING_EXTENSION_FORMAT {}
impl ::core::clone::Clone for TOKENBINDING_EXTENSION_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOKENBINDING_EXTENSION_FORMAT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOKENBINDING_KEY_PARAMETERS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TOKENBINDING_KEY_PARAMETERS_TYPE_RSA2048_PKCS: TOKENBINDING_KEY_PARAMETERS_TYPE = TOKENBINDING_KEY_PARAMETERS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TOKENBINDING_KEY_PARAMETERS_TYPE_RSA2048_PSS: TOKENBINDING_KEY_PARAMETERS_TYPE = TOKENBINDING_KEY_PARAMETERS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TOKENBINDING_KEY_PARAMETERS_TYPE_ECDSAP256: TOKENBINDING_KEY_PARAMETERS_TYPE = TOKENBINDING_KEY_PARAMETERS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TOKENBINDING_KEY_PARAMETERS_TYPE_ANYEXISTING: TOKENBINDING_KEY_PARAMETERS_TYPE = TOKENBINDING_KEY_PARAMETERS_TYPE(255i32);
impl ::core::marker::Copy for TOKENBINDING_KEY_PARAMETERS_TYPE {}
impl ::core::clone::Clone for TOKENBINDING_KEY_PARAMETERS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOKENBINDING_KEY_PARAMETERS_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOKENBINDING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TOKENBINDING_TYPE_PROVIDED: TOKENBINDING_TYPE = TOKENBINDING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TOKENBINDING_TYPE_REFERRED: TOKENBINDING_TYPE = TOKENBINDING_TYPE(1i32);
impl ::core::marker::Copy for TOKENBINDING_TYPE {}
impl ::core::clone::Clone for TOKENBINDING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOKENBINDING_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRUSTED_DOMAIN_TRUST_ATTRIBUTES(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_NON_TRANSITIVE: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_UPLEVEL_ONLY: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_FILTER_SIDS: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(4u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_FOREST_TRANSITIVE: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(8u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(16u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_TREAT_AS_EXTERNAL: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(64u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_ATTRIBUTE_WITHIN_FOREST: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(32u32);
impl ::core::marker::Copy for TRUSTED_DOMAIN_TRUST_ATTRIBUTES {}
impl ::core::clone::Clone for TRUSTED_DOMAIN_TRUST_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRUSTED_DOMAIN_TRUST_ATTRIBUTES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRUSTED_DOMAIN_TRUST_DIRECTION(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_DIRECTION_DISABLED: TRUSTED_DOMAIN_TRUST_DIRECTION = TRUSTED_DOMAIN_TRUST_DIRECTION(0u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_DIRECTION_INBOUND: TRUSTED_DOMAIN_TRUST_DIRECTION = TRUSTED_DOMAIN_TRUST_DIRECTION(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_DIRECTION_OUTBOUND: TRUSTED_DOMAIN_TRUST_DIRECTION = TRUSTED_DOMAIN_TRUST_DIRECTION(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_DIRECTION_BIDIRECTIONAL: TRUSTED_DOMAIN_TRUST_DIRECTION = TRUSTED_DOMAIN_TRUST_DIRECTION(3u32);
impl ::core::marker::Copy for TRUSTED_DOMAIN_TRUST_DIRECTION {}
impl ::core::clone::Clone for TRUSTED_DOMAIN_TRUST_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRUSTED_DOMAIN_TRUST_DIRECTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRUSTED_DOMAIN_TRUST_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_TYPE_DOWNLEVEL: TRUSTED_DOMAIN_TRUST_TYPE = TRUSTED_DOMAIN_TRUST_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_TYPE_UPLEVEL: TRUSTED_DOMAIN_TRUST_TYPE = TRUSTED_DOMAIN_TRUST_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_TYPE_MIT: TRUSTED_DOMAIN_TRUST_TYPE = TRUSTED_DOMAIN_TRUST_TYPE(3u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TRUST_TYPE_DCE: TRUSTED_DOMAIN_TRUST_TYPE = TRUSTED_DOMAIN_TRUST_TYPE(4u32);
impl ::core::marker::Copy for TRUSTED_DOMAIN_TRUST_TYPE {}
impl ::core::clone::Clone for TRUSTED_DOMAIN_TRUST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRUSTED_DOMAIN_TRUST_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRUSTED_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedDomainNameInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedControllersInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedPosixOffsetInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedPasswordInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedDomainInformationBasic: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedDomainInformationEx: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedDomainAuthInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(7i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedDomainFullInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(8i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedDomainAuthInformationInternal: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(9i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedDomainFullInformationInternal: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(10i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedDomainInformationEx2Internal: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(11i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedDomainFullInformation2Internal: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(12i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TrustedDomainSupportedEncryptionTypes: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(13i32);
impl ::core::marker::Copy for TRUSTED_INFORMATION_CLASS {}
impl ::core::clone::Clone for TRUSTED_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRUSTED_INFORMATION_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct eTlsHashAlgorithm(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TlsHashAlgorithm_None: eTlsHashAlgorithm = eTlsHashAlgorithm(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TlsHashAlgorithm_Md5: eTlsHashAlgorithm = eTlsHashAlgorithm(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TlsHashAlgorithm_Sha1: eTlsHashAlgorithm = eTlsHashAlgorithm(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TlsHashAlgorithm_Sha224: eTlsHashAlgorithm = eTlsHashAlgorithm(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TlsHashAlgorithm_Sha256: eTlsHashAlgorithm = eTlsHashAlgorithm(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TlsHashAlgorithm_Sha384: eTlsHashAlgorithm = eTlsHashAlgorithm(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TlsHashAlgorithm_Sha512: eTlsHashAlgorithm = eTlsHashAlgorithm(6i32);
impl ::core::marker::Copy for eTlsHashAlgorithm {}
impl ::core::clone::Clone for eTlsHashAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for eTlsHashAlgorithm {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct eTlsSignatureAlgorithm(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TlsSignatureAlgorithm_Anonymous: eTlsSignatureAlgorithm = eTlsSignatureAlgorithm(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TlsSignatureAlgorithm_Rsa: eTlsSignatureAlgorithm = eTlsSignatureAlgorithm(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TlsSignatureAlgorithm_Dsa: eTlsSignatureAlgorithm = eTlsSignatureAlgorithm(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub const TlsSignatureAlgorithm_Ecdsa: eTlsSignatureAlgorithm = eTlsSignatureAlgorithm(3i32);
impl ::core::marker::Copy for eTlsSignatureAlgorithm {}
impl ::core::clone::Clone for eTlsSignatureAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for eTlsSignatureAlgorithm {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct AUDIT_POLICY_INFORMATION {
    pub AuditSubCategoryGuid: ::windows::core::GUID,
    pub AuditingInformation: u32,
    pub AuditCategoryGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for AUDIT_POLICY_INFORMATION {}
impl ::core::clone::Clone for AUDIT_POLICY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AUDIT_POLICY_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CENTRAL_ACCESS_POLICY {
    pub CAPID: super::super::super::Foundation::PSID,
    pub Name: super::super::super::Foundation::UNICODE_STRING,
    pub Description: super::super::super::Foundation::UNICODE_STRING,
    pub ChangeId: super::super::super::Foundation::UNICODE_STRING,
    pub Flags: u32,
    pub CAPECount: u32,
    pub CAPEs: *mut *mut CENTRAL_ACCESS_POLICY_ENTRY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CENTRAL_ACCESS_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CENTRAL_ACCESS_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CENTRAL_ACCESS_POLICY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CENTRAL_ACCESS_POLICY_ENTRY {
    pub Name: super::super::super::Foundation::UNICODE_STRING,
    pub Description: super::super::super::Foundation::UNICODE_STRING,
    pub ChangeId: super::super::super::Foundation::UNICODE_STRING,
    pub LengthAppliesTo: u32,
    pub AppliesTo: *mut u8,
    pub LengthSD: u32,
    pub SD: super::super::PSECURITY_DESCRIPTOR,
    pub LengthStagedSD: u32,
    pub StagedSD: super::super::PSECURITY_DESCRIPTOR,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CENTRAL_ACCESS_POLICY_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CENTRAL_ACCESS_POLICY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CENTRAL_ACCESS_POLICY_ENTRY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLEAR_BLOCK {
    pub data: [super::super::super::Foundation::CHAR; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLEAR_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLEAR_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLEAR_BLOCK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct DOMAIN_PASSWORD_INFORMATION {
    pub MinPasswordLength: u16,
    pub PasswordHistoryLength: u16,
    pub PasswordProperties: DOMAIN_PASSWORD_PROPERTIES,
    pub MaxPasswordAge: i64,
    pub MinPasswordAge: i64,
}
impl ::core::marker::Copy for DOMAIN_PASSWORD_INFORMATION {}
impl ::core::clone::Clone for DOMAIN_PASSWORD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOMAIN_PASSWORD_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub struct ENCRYPTED_CREDENTIALW {
    pub Cred: super::super::Credentials::CREDENTIALW,
    pub ClearCredentialBlobSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::marker::Copy for ENCRYPTED_CREDENTIALW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::clone::Clone for ENCRYPTED_CREDENTIALW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
unsafe impl ::windows::core::Abi for ENCRYPTED_CREDENTIALW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KDC_PROXY_CACHE_ENTRY_DATA {
    pub SinceLastUsed: u64,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub ProxyServerName: super::super::super::Foundation::UNICODE_STRING,
    pub ProxyServerVdir: super::super::super::Foundation::UNICODE_STRING,
    pub ProxyServerPort: u16,
    pub LogonId: super::super::super::Foundation::LUID,
    pub CredUserName: super::super::super::Foundation::UNICODE_STRING,
    pub CredDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub GlobalCache: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KDC_PROXY_CACHE_ENTRY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KDC_PROXY_CACHE_ENTRY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KDC_PROXY_CACHE_ENTRY_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub RealmName: super::super::super::Foundation::UNICODE_STRING,
    pub KdcAddress: super::super::super::Foundation::UNICODE_STRING,
    pub AddressType: KERB_ADDRESS_TYPE,
    pub DcFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub RealmName: super::super::super::Foundation::UNICODE_STRING,
    pub KdcAddress: super::super::super::Foundation::UNICODE_STRING,
    pub AddressType: KERB_ADDRESS_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_ADD_CREDENTIALS_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub Password: super::super::super::Foundation::UNICODE_STRING,
    pub LogonId: super::super::super::Foundation::LUID,
    pub Flags: KERB_REQUEST_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_ADD_CREDENTIALS_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_ADD_CREDENTIALS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_ADD_CREDENTIALS_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_ADD_CREDENTIALS_REQUEST_EX {
    pub Credentials: KERB_ADD_CREDENTIALS_REQUEST,
    pub PrincipalNameCount: u32,
    pub PrincipalNames: [super::super::super::Foundation::UNICODE_STRING; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_ADD_CREDENTIALS_REQUEST_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_ADD_CREDENTIALS_REQUEST_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_ADD_CREDENTIALS_REQUEST_EX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_AUTH_DATA {
    pub Type: u32,
    pub Length: u32,
    pub Data: *mut u8,
}
impl ::core::marker::Copy for KERB_AUTH_DATA {}
impl ::core::clone::Clone for KERB_AUTH_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_AUTH_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_BINDING_CACHE_ENTRY_DATA {
    pub DiscoveryTime: u64,
    pub RealmName: super::super::super::Foundation::UNICODE_STRING,
    pub KdcAddress: super::super::super::Foundation::UNICODE_STRING,
    pub AddressType: KERB_ADDRESS_TYPE,
    pub Flags: u32,
    pub DcFlags: u32,
    pub CacheFlags: u32,
    pub KdcName: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_BINDING_CACHE_ENTRY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_BINDING_CACHE_ENTRY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_BINDING_CACHE_ENTRY_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_CERTIFICATE_HASHINFO {
    pub StoreNameLength: u16,
    pub HashLength: u16,
}
impl ::core::marker::Copy for KERB_CERTIFICATE_HASHINFO {}
impl ::core::clone::Clone for KERB_CERTIFICATE_HASHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_CERTIFICATE_HASHINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_CERTIFICATE_INFO {
    pub CertInfoSize: u32,
    pub InfoType: u32,
}
impl ::core::marker::Copy for KERB_CERTIFICATE_INFO {}
impl ::core::clone::Clone for KERB_CERTIFICATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_CERTIFICATE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_CERTIFICATE_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub Pin: super::super::super::Foundation::UNICODE_STRING,
    pub Flags: u32,
    pub CspDataLength: u32,
    pub CspData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_CERTIFICATE_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_CERTIFICATE_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_CERTIFICATE_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_CERTIFICATE_S4U_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub UserPrincipalName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub CertificateLength: u32,
    pub Certificate: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_CERTIFICATE_S4U_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_CERTIFICATE_S4U_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_CERTIFICATE_S4U_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_CERTIFICATE_UNLOCK_LOGON {
    pub Logon: KERB_CERTIFICATE_LOGON,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_CERTIFICATE_UNLOCK_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_CERTIFICATE_UNLOCK_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_CERTIFICATE_UNLOCK_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_CHANGEPASSWORD_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub AccountName: super::super::super::Foundation::UNICODE_STRING,
    pub OldPassword: super::super::super::Foundation::UNICODE_STRING,
    pub NewPassword: super::super::super::Foundation::UNICODE_STRING,
    pub Impersonating: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_CHANGEPASSWORD_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_CHANGEPASSWORD_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_CHANGEPASSWORD_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_CLOUD_KERBEROS_DEBUG_DATA_V0 {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for KERB_CLOUD_KERBEROS_DEBUG_DATA_V0 {}
impl ::core::clone::Clone for KERB_CLOUD_KERBEROS_DEBUG_DATA_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_CLOUD_KERBEROS_DEBUG_DATA_V0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_CLOUD_KERBEROS_DEBUG_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_CLOUD_KERBEROS_DEBUG_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_CLOUD_KERBEROS_DEBUG_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_CLOUD_KERBEROS_DEBUG_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Version: u32,
    pub Length: u32,
    pub Data: [u32; 1],
}
impl ::core::marker::Copy for KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {}
impl ::core::clone::Clone for KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_CRYPTO_KEY {
    pub KeyType: KERB_CRYPTO_KEY_TYPE,
    pub Length: u32,
    pub Value: *mut u8,
}
impl ::core::marker::Copy for KERB_CRYPTO_KEY {}
impl ::core::clone::Clone for KERB_CRYPTO_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_CRYPTO_KEY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_CRYPTO_KEY32 {
    pub KeyType: i32,
    pub Length: u32,
    pub Offset: u32,
}
impl ::core::marker::Copy for KERB_CRYPTO_KEY32 {}
impl ::core::clone::Clone for KERB_CRYPTO_KEY32 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_CRYPTO_KEY32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_DECRYPT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
    pub CryptoType: i32,
    pub KeyUsage: i32,
    pub Key: KERB_CRYPTO_KEY,
    pub EncryptedDataSize: u32,
    pub InitialVectorSize: u32,
    pub InitialVector: *mut u8,
    pub EncryptedData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_DECRYPT_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_DECRYPT_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_DECRYPT_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_DECRYPT_RESPONSE {
    pub DecryptedData: [u8; 1],
}
impl ::core::marker::Copy for KERB_DECRYPT_RESPONSE {}
impl ::core::clone::Clone for KERB_DECRYPT_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_DECRYPT_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_EXTERNAL_NAME {
    pub NameType: i16,
    pub NameCount: u16,
    pub Names: [super::super::super::Foundation::UNICODE_STRING; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_EXTERNAL_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_EXTERNAL_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_EXTERNAL_NAME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_EXTERNAL_TICKET {
    pub ServiceName: *mut KERB_EXTERNAL_NAME,
    pub TargetName: *mut KERB_EXTERNAL_NAME,
    pub ClientName: *mut KERB_EXTERNAL_NAME,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub TargetDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub AltTargetDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub SessionKey: KERB_CRYPTO_KEY,
    pub TicketFlags: KERB_TICKET_FLAGS,
    pub Flags: u32,
    pub KeyExpirationTime: i64,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewUntil: i64,
    pub TimeSkew: i64,
    pub EncodedTicketSize: u32,
    pub EncodedTicket: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_EXTERNAL_TICKET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_EXTERNAL_TICKET {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_EXTERNAL_TICKET {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_INTERACTIVE_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub Password: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_INTERACTIVE_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_INTERACTIVE_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_INTERACTIVE_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_INTERACTIVE_PROFILE {
    pub MessageType: KERB_PROFILE_BUFFER_TYPE,
    pub LogonCount: u16,
    pub BadPasswordCount: u16,
    pub LogonTime: i64,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub LogonScript: super::super::super::Foundation::UNICODE_STRING,
    pub HomeDirectory: super::super::super::Foundation::UNICODE_STRING,
    pub FullName: super::super::super::Foundation::UNICODE_STRING,
    pub ProfilePath: super::super::super::Foundation::UNICODE_STRING,
    pub HomeDirectoryDrive: super::super::super::Foundation::UNICODE_STRING,
    pub LogonServer: super::super::super::Foundation::UNICODE_STRING,
    pub UserFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_INTERACTIVE_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_INTERACTIVE_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_INTERACTIVE_PROFILE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_INTERACTIVE_UNLOCK_LOGON {
    pub Logon: KERB_INTERACTIVE_LOGON,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_INTERACTIVE_UNLOCK_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_INTERACTIVE_UNLOCK_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_INTERACTIVE_UNLOCK_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_NET_ADDRESS {
    pub Family: u32,
    pub Length: u32,
    pub Address: ::windows::core::PSTR,
}
impl ::core::marker::Copy for KERB_NET_ADDRESS {}
impl ::core::clone::Clone for KERB_NET_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_NET_ADDRESS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_NET_ADDRESSES {
    pub Number: u32,
    pub Addresses: [KERB_NET_ADDRESS; 1],
}
impl ::core::marker::Copy for KERB_NET_ADDRESSES {}
impl ::core::clone::Clone for KERB_NET_ADDRESSES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_NET_ADDRESSES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_PURGE_BINDING_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
}
impl ::core::marker::Copy for KERB_PURGE_BINDING_CACHE_REQUEST {}
impl ::core::clone::Clone for KERB_PURGE_BINDING_CACHE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_PURGE_BINDING_CACHE_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_PURGE_KDC_PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_PURGE_KDC_PROXY_CACHE_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_PURGE_KDC_PROXY_CACHE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_PURGE_KDC_PROXY_CACHE_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfPurged: u32,
}
impl ::core::marker::Copy for KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {}
impl ::core::clone::Clone for KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_PURGE_TKT_CACHE_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
    pub TicketTemplate: KERB_TICKET_CACHE_INFO_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_PURGE_TKT_CACHE_EX_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_PURGE_TKT_CACHE_EX_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_PURGE_TKT_CACHE_EX_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_PURGE_TKT_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub ServerName: super::super::super::Foundation::UNICODE_STRING,
    pub RealmName: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_PURGE_TKT_CACHE_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_PURGE_TKT_CACHE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_PURGE_TKT_CACHE_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_QUERY_BINDING_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
}
impl ::core::marker::Copy for KERB_QUERY_BINDING_CACHE_REQUEST {}
impl ::core::clone::Clone for KERB_QUERY_BINDING_CACHE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_QUERY_BINDING_CACHE_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_QUERY_BINDING_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfEntries: u32,
    pub Entries: *mut KERB_BINDING_CACHE_ENTRY_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_QUERY_BINDING_CACHE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_QUERY_BINDING_CACHE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_QUERY_BINDING_CACHE_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub ExtendedPolicies: u32,
    pub DsFlags: u32,
}
impl ::core::marker::Copy for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {}
impl ::core::clone::Clone for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_QUERY_KDC_PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_QUERY_KDC_PROXY_CACHE_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_QUERY_KDC_PROXY_CACHE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_QUERY_KDC_PROXY_CACHE_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfEntries: u32,
    pub Entries: *mut KDC_PROXY_CACHE_ENTRY_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_QUERY_S4U2PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_QUERY_S4U2PROXY_CACHE_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_QUERY_S4U2PROXY_CACHE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_QUERY_S4U2PROXY_CACHE_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfCreds: u32,
    pub Creds: *mut KERB_S4U2PROXY_CRED,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_QUERY_TKT_CACHE_EX2_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX3; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_QUERY_TKT_CACHE_EX3_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_QUERY_TKT_CACHE_EX_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_QUERY_TKT_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_QUERY_TKT_CACHE_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_QUERY_TKT_CACHE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_QUERY_TKT_CACHE_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_QUERY_TKT_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_QUERY_TKT_CACHE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_QUERY_TKT_CACHE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_QUERY_TKT_CACHE_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_REFRESH_POLICY_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
}
impl ::core::marker::Copy for KERB_REFRESH_POLICY_REQUEST {}
impl ::core::clone::Clone for KERB_REFRESH_POLICY_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_REFRESH_POLICY_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_REFRESH_POLICY_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
}
impl ::core::marker::Copy for KERB_REFRESH_POLICY_RESPONSE {}
impl ::core::clone::Clone for KERB_REFRESH_POLICY_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_REFRESH_POLICY_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_REFRESH_SCCRED_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CredentialBlob: super::super::super::Foundation::UNICODE_STRING,
    pub LogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_REFRESH_SCCRED_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_REFRESH_SCCRED_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_REFRESH_SCCRED_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_RETRIEVE_KEY_TAB_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub Password: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_RETRIEVE_KEY_TAB_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_RETRIEVE_KEY_TAB_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_RETRIEVE_KEY_TAB_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_RETRIEVE_KEY_TAB_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub KeyTabLength: u32,
    pub KeyTab: *mut u8,
}
impl ::core::marker::Copy for KERB_RETRIEVE_KEY_TAB_RESPONSE {}
impl ::core::clone::Clone for KERB_RETRIEVE_KEY_TAB_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_RETRIEVE_KEY_TAB_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub struct KERB_RETRIEVE_TKT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub TargetName: super::super::super::Foundation::UNICODE_STRING,
    pub TicketFlags: u32,
    pub CacheOptions: u32,
    pub EncryptionType: KERB_CRYPTO_KEY_TYPE,
    pub CredentialsHandle: super::super::Credentials::SecHandle,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::marker::Copy for KERB_RETRIEVE_TKT_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::clone::Clone for KERB_RETRIEVE_TKT_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
unsafe impl ::windows::core::Abi for KERB_RETRIEVE_TKT_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_RETRIEVE_TKT_RESPONSE {
    pub Ticket: KERB_EXTERNAL_TICKET,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_RETRIEVE_TKT_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_RETRIEVE_TKT_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_RETRIEVE_TKT_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_S4U2PROXY_CACHE_ENTRY_INFO {
    pub ServerName: super::super::super::Foundation::UNICODE_STRING,
    pub Flags: u32,
    pub LastStatus: super::super::super::Foundation::NTSTATUS,
    pub Expiry: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_S4U2PROXY_CACHE_ENTRY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_S4U2PROXY_CACHE_ENTRY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_S4U2PROXY_CACHE_ENTRY_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_S4U2PROXY_CRED {
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub Flags: u32,
    pub LastStatus: super::super::super::Foundation::NTSTATUS,
    pub Expiry: i64,
    pub CountOfEntries: u32,
    pub Entries: *mut KERB_S4U2PROXY_CACHE_ENTRY_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_S4U2PROXY_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_S4U2PROXY_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_S4U2PROXY_CRED {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_S4U_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub ClientUpn: super::super::super::Foundation::UNICODE_STRING,
    pub ClientRealm: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_S4U_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_S4U_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_S4U_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub struct KERB_SETPASSWORD_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub CredentialsHandle: super::super::Credentials::SecHandle,
    pub Flags: u32,
    pub AccountRealm: super::super::super::Foundation::UNICODE_STRING,
    pub AccountName: super::super::super::Foundation::UNICODE_STRING,
    pub Password: super::super::super::Foundation::UNICODE_STRING,
    pub ClientRealm: super::super::super::Foundation::UNICODE_STRING,
    pub ClientName: super::super::super::Foundation::UNICODE_STRING,
    pub Impersonating: super::super::super::Foundation::BOOLEAN,
    pub KdcAddress: super::super::super::Foundation::UNICODE_STRING,
    pub KdcAddressType: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::marker::Copy for KERB_SETPASSWORD_EX_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::clone::Clone for KERB_SETPASSWORD_EX_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
unsafe impl ::windows::core::Abi for KERB_SETPASSWORD_EX_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub struct KERB_SETPASSWORD_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub CredentialsHandle: super::super::Credentials::SecHandle,
    pub Flags: u32,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub AccountName: super::super::super::Foundation::UNICODE_STRING,
    pub Password: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::marker::Copy for KERB_SETPASSWORD_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::clone::Clone for KERB_SETPASSWORD_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
unsafe impl ::windows::core::Abi for KERB_SETPASSWORD_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_SMART_CARD_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Pin: super::super::super::Foundation::UNICODE_STRING,
    pub CspDataLength: u32,
    pub CspData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_SMART_CARD_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_SMART_CARD_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_SMART_CARD_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_SMART_CARD_PROFILE {
    pub Profile: KERB_INTERACTIVE_PROFILE,
    pub CertificateSize: u32,
    pub CertificateData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_SMART_CARD_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_SMART_CARD_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_SMART_CARD_PROFILE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_SMART_CARD_UNLOCK_LOGON {
    pub Logon: KERB_SMART_CARD_LOGON,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_SMART_CARD_UNLOCK_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_SMART_CARD_UNLOCK_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_SMART_CARD_UNLOCK_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_SUBMIT_TKT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
    pub Key: KERB_CRYPTO_KEY32,
    pub KerbCredSize: u32,
    pub KerbCredOffset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_SUBMIT_TKT_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_SUBMIT_TKT_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_SUBMIT_TKT_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_TICKET_CACHE_INFO {
    pub ServerName: super::super::super::Foundation::UNICODE_STRING,
    pub RealmName: super::super::super::Foundation::UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: KERB_TICKET_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_TICKET_CACHE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_TICKET_CACHE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_TICKET_CACHE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_TICKET_CACHE_INFO_EX {
    pub ClientName: super::super::super::Foundation::UNICODE_STRING,
    pub ClientRealm: super::super::super::Foundation::UNICODE_STRING,
    pub ServerName: super::super::super::Foundation::UNICODE_STRING,
    pub ServerRealm: super::super::super::Foundation::UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_TICKET_CACHE_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_TICKET_CACHE_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_TICKET_CACHE_INFO_EX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_TICKET_CACHE_INFO_EX2 {
    pub ClientName: super::super::super::Foundation::UNICODE_STRING,
    pub ClientRealm: super::super::super::Foundation::UNICODE_STRING,
    pub ServerName: super::super::super::Foundation::UNICODE_STRING,
    pub ServerRealm: super::super::super::Foundation::UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: u32,
    pub SessionKeyType: u32,
    pub BranchId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_TICKET_CACHE_INFO_EX2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_TICKET_CACHE_INFO_EX2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_TICKET_CACHE_INFO_EX2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_TICKET_CACHE_INFO_EX3 {
    pub ClientName: super::super::super::Foundation::UNICODE_STRING,
    pub ClientRealm: super::super::super::Foundation::UNICODE_STRING,
    pub ServerName: super::super::super::Foundation::UNICODE_STRING,
    pub ServerRealm: super::super::super::Foundation::UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: u32,
    pub SessionKeyType: u32,
    pub BranchId: u32,
    pub CacheFlags: u32,
    pub KdcCalled: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_TICKET_CACHE_INFO_EX3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_TICKET_CACHE_INFO_EX3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_TICKET_CACHE_INFO_EX3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct KERB_TICKET_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub ServiceTicketLength: u32,
    pub TicketGrantingTicketLength: u32,
    pub ServiceTicket: *mut u8,
    pub TicketGrantingTicket: *mut u8,
}
impl ::core::marker::Copy for KERB_TICKET_LOGON {}
impl ::core::clone::Clone for KERB_TICKET_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for KERB_TICKET_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_TICKET_PROFILE {
    pub Profile: KERB_INTERACTIVE_PROFILE,
    pub SessionKey: KERB_CRYPTO_KEY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_TICKET_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_TICKET_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_TICKET_PROFILE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_TICKET_UNLOCK_LOGON {
    pub Logon: KERB_TICKET_LOGON,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_TICKET_UNLOCK_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_TICKET_UNLOCK_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_TICKET_UNLOCK_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KERB_TRANSFER_CRED_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub OriginLogonId: super::super::super::Foundation::LUID,
    pub DestinationLogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KERB_TRANSFER_CRED_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KERB_TRANSFER_CRED_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KERB_TRANSFER_CRED_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
pub struct KSEC_LIST_ENTRY {
    pub List: super::super::super::System::Kernel::LIST_ENTRY,
    pub RefCount: i32,
    pub Signature: u32,
    pub OwningList: *mut ::core::ffi::c_void,
    pub Reserved: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for KSEC_LIST_ENTRY {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for KSEC_LIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
unsafe impl ::windows::core::Abi for KSEC_LIST_ENTRY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct LOGON_HOURS {
    pub UnitsPerWeek: u16,
    pub LogonHours: *mut u8,
}
impl ::core::marker::Copy for LOGON_HOURS {}
impl ::core::clone::Clone for LOGON_HOURS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LOGON_HOURS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct LSA_AUTH_INFORMATION {
    pub LastUpdateTime: i64,
    pub AuthType: LSA_AUTH_INFORMATION_AUTH_TYPE,
    pub AuthInfoLength: u32,
    pub AuthInfo: *mut u8,
}
impl ::core::marker::Copy for LSA_AUTH_INFORMATION {}
impl ::core::clone::Clone for LSA_AUTH_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LSA_AUTH_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct LSA_DISPATCH_TABLE {
    pub CreateLogonSession: PLSA_CREATE_LOGON_SESSION,
    pub DeleteLogonSession: PLSA_DELETE_LOGON_SESSION,
    pub AddCredential: PLSA_ADD_CREDENTIAL,
    pub GetCredentials: PLSA_GET_CREDENTIALS,
    pub DeleteCredential: PLSA_DELETE_CREDENTIAL,
    pub AllocateLsaHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeLsaHeap: PLSA_FREE_LSA_HEAP,
    pub AllocateClientBuffer: PLSA_ALLOCATE_CLIENT_BUFFER,
    pub FreeClientBuffer: PLSA_FREE_CLIENT_BUFFER,
    pub CopyToClientBuffer: PLSA_COPY_TO_CLIENT_BUFFER,
    pub CopyFromClientBuffer: PLSA_COPY_FROM_CLIENT_BUFFER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for LSA_DISPATCH_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for LSA_DISPATCH_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for LSA_DISPATCH_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_ENUMERATION_INFORMATION {
    pub Sid: super::super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_ENUMERATION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_ENUMERATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_ENUMERATION_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct LSA_FOREST_TRUST_BINARY_DATA {
    pub Length: u32,
    pub Buffer: *mut u8,
}
impl ::core::marker::Copy for LSA_FOREST_TRUST_BINARY_DATA {}
impl ::core::clone::Clone for LSA_FOREST_TRUST_BINARY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LSA_FOREST_TRUST_BINARY_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_FOREST_TRUST_COLLISION_INFORMATION {
    pub RecordCount: u32,
    pub Entries: *mut *mut LSA_FOREST_TRUST_COLLISION_RECORD,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_FOREST_TRUST_COLLISION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_FOREST_TRUST_COLLISION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_FOREST_TRUST_COLLISION_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_FOREST_TRUST_COLLISION_RECORD {
    pub Index: u32,
    pub Type: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE,
    pub Flags: u32,
    pub Name: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_FOREST_TRUST_COLLISION_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_FOREST_TRUST_COLLISION_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_FOREST_TRUST_COLLISION_RECORD {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_FOREST_TRUST_DOMAIN_INFO {
    pub Sid: super::super::super::Foundation::PSID,
    pub DnsName: super::super::super::Foundation::UNICODE_STRING,
    pub NetbiosName: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_FOREST_TRUST_DOMAIN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_FOREST_TRUST_DOMAIN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_FOREST_TRUST_DOMAIN_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_FOREST_TRUST_INFORMATION {
    pub RecordCount: u32,
    pub Entries: *mut *mut LSA_FOREST_TRUST_RECORD,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_FOREST_TRUST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_FOREST_TRUST_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_FOREST_TRUST_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_FOREST_TRUST_RECORD {
    pub Flags: u32,
    pub ForestTrustType: LSA_FOREST_TRUST_RECORD_TYPE,
    pub Time: i64,
    pub ForestTrustData: LSA_FOREST_TRUST_RECORD_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_FOREST_TRUST_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_FOREST_TRUST_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_FOREST_TRUST_RECORD {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union LSA_FOREST_TRUST_RECORD_0 {
    pub TopLevelName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainInfo: LSA_FOREST_TRUST_DOMAIN_INFO,
    pub Data: LSA_FOREST_TRUST_BINARY_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_FOREST_TRUST_RECORD_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_FOREST_TRUST_RECORD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_FOREST_TRUST_RECORD_0 {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LSA_HANDLE(pub isize);
impl LSA_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for LSA_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for LSA_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for LSA_HANDLE {}
impl ::core::fmt::Debug for LSA_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LSA_HANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<LSA_HANDLE>> for LSA_HANDLE {
    fn from(optional: ::core::option::Option<LSA_HANDLE>) -> LSA_HANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for LSA_HANDLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct LSA_LAST_INTER_LOGON_INFO {
    pub LastSuccessfulLogon: i64,
    pub LastFailedLogon: i64,
    pub FailedAttemptCountSinceLastSuccessfulLogon: u32,
}
impl ::core::marker::Copy for LSA_LAST_INTER_LOGON_INFO {}
impl ::core::clone::Clone for LSA_LAST_INTER_LOGON_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LSA_LAST_INTER_LOGON_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_REFERENCED_DOMAIN_LIST {
    pub Entries: u32,
    pub Domains: *mut LSA_TRUST_INFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_REFERENCED_DOMAIN_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_REFERENCED_DOMAIN_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_REFERENCED_DOMAIN_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
pub struct LSA_SECPKG_FUNCTION_TABLE {
    pub CreateLogonSession: PLSA_CREATE_LOGON_SESSION,
    pub DeleteLogonSession: PLSA_DELETE_LOGON_SESSION,
    pub AddCredential: PLSA_ADD_CREDENTIAL,
    pub GetCredentials: PLSA_GET_CREDENTIALS,
    pub DeleteCredential: PLSA_DELETE_CREDENTIAL,
    pub AllocateLsaHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeLsaHeap: PLSA_FREE_LSA_HEAP,
    pub AllocateClientBuffer: PLSA_ALLOCATE_CLIENT_BUFFER,
    pub FreeClientBuffer: PLSA_FREE_CLIENT_BUFFER,
    pub CopyToClientBuffer: PLSA_COPY_TO_CLIENT_BUFFER,
    pub CopyFromClientBuffer: PLSA_COPY_FROM_CLIENT_BUFFER,
    pub ImpersonateClient: PLSA_IMPERSONATE_CLIENT,
    pub UnloadPackage: PLSA_UNLOAD_PACKAGE,
    pub DuplicateHandle: PLSA_DUPLICATE_HANDLE,
    pub SaveSupplementalCredentials: PLSA_SAVE_SUPPLEMENTAL_CREDENTIALS,
    pub CreateThread: PLSA_CREATE_THREAD,
    pub GetClientInfo: PLSA_GET_CLIENT_INFO,
    pub RegisterNotification: PLSA_REGISTER_NOTIFICATION,
    pub CancelNotification: PLSA_CANCEL_NOTIFICATION,
    pub MapBuffer: PLSA_MAP_BUFFER,
    pub CreateToken: PLSA_CREATE_TOKEN,
    pub AuditLogon: PLSA_AUDIT_LOGON,
    pub CallPackage: PLSA_CALL_PACKAGE,
    pub FreeReturnBuffer: PLSA_FREE_LSA_HEAP,
    pub GetCallInfo: PLSA_GET_CALL_INFO,
    pub CallPackageEx: PLSA_CALL_PACKAGEEX,
    pub CreateSharedMemory: PLSA_CREATE_SHARED_MEMORY,
    pub AllocateSharedMemory: PLSA_ALLOCATE_SHARED_MEMORY,
    pub FreeSharedMemory: PLSA_FREE_SHARED_MEMORY,
    pub DeleteSharedMemory: PLSA_DELETE_SHARED_MEMORY,
    pub OpenSamUser: PLSA_OPEN_SAM_USER,
    pub GetUserCredentials: PLSA_GET_USER_CREDENTIALS,
    pub GetUserAuthData: PLSA_GET_USER_AUTH_DATA,
    pub CloseSamUser: PLSA_CLOSE_SAM_USER,
    pub ConvertAuthDataToToken: PLSA_CONVERT_AUTH_DATA_TO_TOKEN,
    pub ClientCallback: PLSA_CLIENT_CALLBACK,
    pub UpdateCredentials: PLSA_UPDATE_PRIMARY_CREDENTIALS,
    pub GetAuthDataForUser: PLSA_GET_AUTH_DATA_FOR_USER,
    pub CrackSingleName: PLSA_CRACK_SINGLE_NAME,
    pub AuditAccountLogon: PLSA_AUDIT_ACCOUNT_LOGON,
    pub CallPackagePassthrough: PLSA_CALL_PACKAGE_PASSTHROUGH,
    pub CrediRead: CredReadFn,
    pub CrediReadDomainCredentials: CredReadDomainCredentialsFn,
    pub CrediFreeCredentials: CredFreeCredentialsFn,
    pub LsaProtectMemory: PLSA_PROTECT_MEMORY,
    pub LsaUnprotectMemory: PLSA_PROTECT_MEMORY,
    pub OpenTokenByLogonId: PLSA_OPEN_TOKEN_BY_LOGON_ID,
    pub ExpandAuthDataForDomain: PLSA_EXPAND_AUTH_DATA_FOR_DOMAIN,
    pub AllocatePrivateHeap: PLSA_ALLOCATE_PRIVATE_HEAP,
    pub FreePrivateHeap: PLSA_FREE_PRIVATE_HEAP,
    pub CreateTokenEx: PLSA_CREATE_TOKEN_EX,
    pub CrediWrite: CredWriteFn,
    pub CrediUnmarshalandDecodeString: CrediUnmarshalandDecodeStringFn,
    pub DummyFunction6: PLSA_PROTECT_MEMORY,
    pub GetExtendedCallFlags: PLSA_GET_EXTENDED_CALL_FLAGS,
    pub DuplicateTokenHandle: PLSA_DUPLICATE_HANDLE,
    pub GetServiceAccountPassword: PLSA_GET_SERVICE_ACCOUNT_PASSWORD,
    pub DummyFunction7: PLSA_PROTECT_MEMORY,
    pub AuditLogonEx: PLSA_AUDIT_LOGON_EX,
    pub CheckProtectedUserByToken: PLSA_CHECK_PROTECTED_USER_BY_TOKEN,
    pub QueryClientRequest: PLSA_QUERY_CLIENT_REQUEST,
    pub GetAppModeInfo: PLSA_GET_APP_MODE_INFO,
    pub SetAppModeInfo: PLSA_SET_APP_MODE_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
impl ::core::marker::Copy for LSA_SECPKG_FUNCTION_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
impl ::core::clone::Clone for LSA_SECPKG_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
unsafe impl ::windows::core::Abi for LSA_SECPKG_FUNCTION_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_TOKEN_INFORMATION_NULL {
    pub ExpirationTime: i64,
    pub Groups: *mut super::super::TOKEN_GROUPS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_TOKEN_INFORMATION_NULL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_TOKEN_INFORMATION_NULL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_TOKEN_INFORMATION_NULL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_TOKEN_INFORMATION_V1 {
    pub ExpirationTime: i64,
    pub User: super::super::TOKEN_USER,
    pub Groups: *mut super::super::TOKEN_GROUPS,
    pub PrimaryGroup: super::super::TOKEN_PRIMARY_GROUP,
    pub Privileges: *mut super::super::TOKEN_PRIVILEGES,
    pub Owner: super::super::TOKEN_OWNER,
    pub DefaultDacl: super::super::TOKEN_DEFAULT_DACL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_TOKEN_INFORMATION_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_TOKEN_INFORMATION_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_TOKEN_INFORMATION_V1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_TOKEN_INFORMATION_V3 {
    pub ExpirationTime: i64,
    pub User: super::super::TOKEN_USER,
    pub Groups: *mut super::super::TOKEN_GROUPS,
    pub PrimaryGroup: super::super::TOKEN_PRIMARY_GROUP,
    pub Privileges: *mut super::super::TOKEN_PRIVILEGES,
    pub Owner: super::super::TOKEN_OWNER,
    pub DefaultDacl: super::super::TOKEN_DEFAULT_DACL,
    pub UserClaims: super::super::TOKEN_USER_CLAIMS,
    pub DeviceClaims: super::super::TOKEN_DEVICE_CLAIMS,
    pub DeviceGroups: *mut super::super::TOKEN_GROUPS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_TOKEN_INFORMATION_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_TOKEN_INFORMATION_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_TOKEN_INFORMATION_V3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_TRANSLATED_NAME {
    pub Use: super::super::SID_NAME_USE,
    pub Name: super::super::super::Foundation::UNICODE_STRING,
    pub DomainIndex: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_TRANSLATED_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_TRANSLATED_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_TRANSLATED_NAME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct LSA_TRANSLATED_SID {
    pub Use: super::super::SID_NAME_USE,
    pub RelativeId: u32,
    pub DomainIndex: i32,
}
impl ::core::marker::Copy for LSA_TRANSLATED_SID {}
impl ::core::clone::Clone for LSA_TRANSLATED_SID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LSA_TRANSLATED_SID {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_TRANSLATED_SID2 {
    pub Use: super::super::SID_NAME_USE,
    pub Sid: super::super::super::Foundation::PSID,
    pub DomainIndex: i32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_TRANSLATED_SID2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_TRANSLATED_SID2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_TRANSLATED_SID2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LSA_TRUST_INFORMATION {
    pub Name: super::super::super::Foundation::UNICODE_STRING,
    pub Sid: super::super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LSA_TRUST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LSA_TRUST_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LSA_TRUST_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct MSV1_0_AV_PAIR {
    pub AvId: u16,
    pub AvLen: u16,
}
impl ::core::marker::Copy for MSV1_0_AV_PAIR {}
impl ::core::clone::Clone for MSV1_0_AV_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_AV_PAIR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_CHANGEPASSWORD_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub AccountName: super::super::super::Foundation::UNICODE_STRING,
    pub OldPassword: super::super::super::Foundation::UNICODE_STRING,
    pub NewPassword: super::super::super::Foundation::UNICODE_STRING,
    pub Impersonating: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_CHANGEPASSWORD_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_CHANGEPASSWORD_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSV1_0_CHANGEPASSWORD_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_CHANGEPASSWORD_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub PasswordInfoValid: super::super::super::Foundation::BOOLEAN,
    pub DomainPasswordInfo: DOMAIN_PASSWORD_INFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_CHANGEPASSWORD_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_CHANGEPASSWORD_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSV1_0_CHANGEPASSWORD_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct MSV1_0_CREDENTIAL_KEY {
    pub Data: [u8; 20],
}
impl ::core::marker::Copy for MSV1_0_CREDENTIAL_KEY {}
impl ::core::clone::Clone for MSV1_0_CREDENTIAL_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_CREDENTIAL_KEY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_INTERACTIVE_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub Password: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_INTERACTIVE_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_INTERACTIVE_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSV1_0_INTERACTIVE_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_INTERACTIVE_PROFILE {
    pub MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    pub LogonCount: u16,
    pub BadPasswordCount: u16,
    pub LogonTime: i64,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub LogonScript: super::super::super::Foundation::UNICODE_STRING,
    pub HomeDirectory: super::super::super::Foundation::UNICODE_STRING,
    pub FullName: super::super::super::Foundation::UNICODE_STRING,
    pub ProfilePath: super::super::super::Foundation::UNICODE_STRING,
    pub HomeDirectoryDrive: super::super::super::Foundation::UNICODE_STRING,
    pub LogonServer: super::super::super::Foundation::UNICODE_STRING,
    pub UserFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_INTERACTIVE_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_INTERACTIVE_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSV1_0_INTERACTIVE_PROFILE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub EncryptedCredsSize: u32,
    pub EncryptedCreds: [u8; 1],
}
impl ::core::marker::Copy for MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {}
impl ::core::clone::Clone for MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct MSV1_0_LM20_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub Workstation: super::super::super::Foundation::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub CaseSensitiveChallengeResponse: super::super::super::System::Kernel::STRING,
    pub CaseInsensitiveChallengeResponse: super::super::super::System::Kernel::STRING,
    pub ParameterControl: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MSV1_0_LM20_LOGON {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MSV1_0_LM20_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for MSV1_0_LM20_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_LM20_LOGON_PROFILE {
    pub MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    pub KickOffTime: i64,
    pub LogoffTime: i64,
    pub UserFlags: MSV_SUB_AUTHENTICATION_FILTER,
    pub UserSessionKey: [u8; 16],
    pub LogonDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub LanmanSessionKey: [u8; 8],
    pub LogonServer: super::super::super::Foundation::UNICODE_STRING,
    pub UserParameters: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_LM20_LOGON_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_LM20_LOGON_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSV1_0_LM20_LOGON_PROFILE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct MSV1_0_NTLM3_RESPONSE {
    pub Response: [u8; 16],
    pub RespType: u8,
    pub HiRespType: u8,
    pub Flags: u16,
    pub MsgWord: u32,
    pub TimeStamp: u64,
    pub ChallengeFromClient: [u8; 8],
    pub AvPairsOff: u32,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for MSV1_0_NTLM3_RESPONSE {}
impl ::core::clone::Clone for MSV1_0_NTLM3_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_NTLM3_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_PASSTHROUGH_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub PackageName: super::super::super::Foundation::UNICODE_STRING,
    pub DataLength: u32,
    pub LogonData: *mut u8,
    pub Pad: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_PASSTHROUGH_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_PASSTHROUGH_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSV1_0_PASSTHROUGH_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct MSV1_0_PASSTHROUGH_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub Pad: u32,
    pub DataLength: u32,
    pub ValidationData: *mut u8,
}
impl ::core::marker::Copy for MSV1_0_PASSTHROUGH_RESPONSE {}
impl ::core::clone::Clone for MSV1_0_PASSTHROUGH_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_PASSTHROUGH_RESPONSE {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub Flags: u32,
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
    pub CredentialKeyType: MSV1_0_CREDENTIAL_KEY_TYPE,
    pub EncryptedCredsSize: u32,
    pub EncryptedCreds: [u8; 1],
}
impl ::core::marker::Copy for MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {}
impl ::core::clone::Clone for MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSV1_0_S4U_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub UserPrincipalName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSV1_0_S4U_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSV1_0_S4U_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSV1_0_S4U_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct MSV1_0_SUBAUTH_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub Workstation: super::super::super::Foundation::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub AuthenticationInfo1: super::super::super::System::Kernel::STRING,
    pub AuthenticationInfo2: super::super::super::System::Kernel::STRING,
    pub ParameterControl: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL,
    pub SubAuthPackageId: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MSV1_0_SUBAUTH_LOGON {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MSV1_0_SUBAUTH_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for MSV1_0_SUBAUTH_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct MSV1_0_SUBAUTH_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub SubAuthPackageId: u32,
    pub SubAuthInfoLength: u32,
    pub SubAuthSubmitBuffer: *mut u8,
}
impl ::core::marker::Copy for MSV1_0_SUBAUTH_REQUEST {}
impl ::core::clone::Clone for MSV1_0_SUBAUTH_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_SUBAUTH_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct MSV1_0_SUBAUTH_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub SubAuthInfoLength: u32,
    pub SubAuthReturnBuffer: *mut u8,
}
impl ::core::marker::Copy for MSV1_0_SUBAUTH_RESPONSE {}
impl ::core::clone::Clone for MSV1_0_SUBAUTH_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_SUBAUTH_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub Flags: MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS,
    pub LmPassword: [u8; 16],
    pub NtPassword: [u8; 16],
}
impl ::core::marker::Copy for MSV1_0_SUPPLEMENTAL_CREDENTIAL {}
impl ::core::clone::Clone for MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    pub Version: u32,
    pub Flags: u32,
    pub NtPassword: [u8; 16],
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
}
impl ::core::marker::Copy for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {}
impl ::core::clone::Clone for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    pub Version: u32,
    pub Flags: u32,
    pub CredentialKeyType: MSV1_0_CREDENTIAL_KEY_TYPE,
    pub NtPassword: [u8; 16],
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
    pub ShaPassword: [u8; 20],
}
impl ::core::marker::Copy for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {}
impl ::core::clone::Clone for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_PasswordManagement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
pub struct MSV1_0_VALIDATION_INFO {
    pub LogoffTime: i64,
    pub KickoffTime: i64,
    pub LogonServer: super::super::super::Foundation::UNICODE_STRING,
    pub LogonDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub SessionKey: USER_SESSION_KEY,
    pub Authoritative: super::super::super::Foundation::BOOLEAN,
    pub UserFlags: u32,
    pub WhichFields: u32,
    pub UserId: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::marker::Copy for MSV1_0_VALIDATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::clone::Clone for MSV1_0_VALIDATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
unsafe impl ::windows::core::Abi for MSV1_0_VALIDATION_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NEGOTIATE_CALLER_NAME_REQUEST {
    pub MessageType: u32,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NEGOTIATE_CALLER_NAME_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NEGOTIATE_CALLER_NAME_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NEGOTIATE_CALLER_NAME_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct NEGOTIATE_CALLER_NAME_RESPONSE {
    pub MessageType: u32,
    pub CallerName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for NEGOTIATE_CALLER_NAME_RESPONSE {}
impl ::core::clone::Clone for NEGOTIATE_CALLER_NAME_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NEGOTIATE_CALLER_NAME_RESPONSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct NEGOTIATE_PACKAGE_PREFIX {
    pub PackageId: usize,
    pub PackageDataA: *mut ::core::ffi::c_void,
    pub PackageDataW: *mut ::core::ffi::c_void,
    pub PrefixLen: usize,
    pub Prefix: [u8; 32],
}
impl ::core::marker::Copy for NEGOTIATE_PACKAGE_PREFIX {}
impl ::core::clone::Clone for NEGOTIATE_PACKAGE_PREFIX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NEGOTIATE_PACKAGE_PREFIX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct NEGOTIATE_PACKAGE_PREFIXES {
    pub MessageType: u32,
    pub PrefixCount: u32,
    pub Offset: u32,
    pub Pad: u32,
}
impl ::core::marker::Copy for NEGOTIATE_PACKAGE_PREFIXES {}
impl ::core::clone::Clone for NEGOTIATE_PACKAGE_PREFIXES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NEGOTIATE_PACKAGE_PREFIXES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NETLOGON_GENERIC_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub PackageName: super::super::super::Foundation::UNICODE_STRING,
    pub DataLength: u32,
    pub LogonData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETLOGON_GENERIC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETLOGON_GENERIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NETLOGON_GENERIC_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_PasswordManagement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
pub struct NETLOGON_INTERACTIVE_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub LmOwfPassword: super::super::super::System::PasswordManagement::LM_OWF_PASSWORD,
    pub NtOwfPassword: super::super::super::System::PasswordManagement::LM_OWF_PASSWORD,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::marker::Copy for NETLOGON_INTERACTIVE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::clone::Clone for NETLOGON_INTERACTIVE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
unsafe impl ::windows::core::Abi for NETLOGON_INTERACTIVE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NETLOGON_LOGON_IDENTITY_INFO {
    pub LogonDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub ParameterControl: u32,
    pub LogonId: i64,
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub Workstation: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETLOGON_LOGON_IDENTITY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETLOGON_LOGON_IDENTITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NETLOGON_LOGON_IDENTITY_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct NETLOGON_NETWORK_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub LmChallenge: CLEAR_BLOCK,
    pub NtChallengeResponse: super::super::super::System::Kernel::STRING,
    pub LmChallengeResponse: super::super::super::System::Kernel::STRING,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for NETLOGON_NETWORK_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for NETLOGON_NETWORK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for NETLOGON_NETWORK_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_PasswordManagement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
pub struct NETLOGON_SERVICE_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub LmOwfPassword: super::super::super::System::PasswordManagement::LM_OWF_PASSWORD,
    pub NtOwfPassword: super::super::super::System::PasswordManagement::LM_OWF_PASSWORD,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::marker::Copy for NETLOGON_SERVICE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::clone::Clone for NETLOGON_SERVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
unsafe impl ::windows::core::Abi for NETLOGON_SERVICE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PKU2U_CERTIFICATE_S4U_LOGON {
    pub MessageType: PKU2U_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub UserPrincipalName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub CertificateLength: u32,
    pub Certificate: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PKU2U_CERTIFICATE_S4U_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PKU2U_CERTIFICATE_S4U_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PKU2U_CERTIFICATE_S4U_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct PKU2U_CERT_BLOB {
    pub CertOffset: u32,
    pub CertLength: u16,
}
impl ::core::marker::Copy for PKU2U_CERT_BLOB {}
impl ::core::clone::Clone for PKU2U_CERT_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PKU2U_CERT_BLOB {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct PKU2U_CREDUI_CONTEXT {
    pub Version: u64,
    pub cbHeaderLength: u16,
    pub cbStructureLength: u32,
    pub CertArrayCount: u16,
    pub CertArrayOffset: u32,
}
impl ::core::marker::Copy for PKU2U_CREDUI_CONTEXT {}
impl ::core::clone::Clone for PKU2U_CREDUI_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PKU2U_CREDUI_CONTEXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_ACCOUNT_DOMAIN_INFO {
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainSid: super::super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_ACCOUNT_DOMAIN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_ACCOUNT_DOMAIN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICY_ACCOUNT_DOMAIN_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct POLICY_AUDIT_CATEGORIES_INFO {
    pub MaximumCategoryCount: u32,
    pub SubCategoriesInfo: *mut POLICY_AUDIT_SUBCATEGORIES_INFO,
}
impl ::core::marker::Copy for POLICY_AUDIT_CATEGORIES_INFO {}
impl ::core::clone::Clone for POLICY_AUDIT_CATEGORIES_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_AUDIT_CATEGORIES_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_AUDIT_EVENTS_INFO {
    pub AuditingMode: super::super::super::Foundation::BOOLEAN,
    pub EventAuditingOptions: *mut u32,
    pub MaximumAuditEventCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_AUDIT_EVENTS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_AUDIT_EVENTS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICY_AUDIT_EVENTS_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_AUDIT_FULL_QUERY_INFO {
    pub ShutDownOnFull: super::super::super::Foundation::BOOLEAN,
    pub LogIsFull: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_AUDIT_FULL_QUERY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_AUDIT_FULL_QUERY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICY_AUDIT_FULL_QUERY_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_AUDIT_FULL_SET_INFO {
    pub ShutDownOnFull: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_AUDIT_FULL_SET_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_AUDIT_FULL_SET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICY_AUDIT_FULL_SET_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_AUDIT_LOG_INFO {
    pub AuditLogPercentFull: u32,
    pub MaximumLogSize: u32,
    pub AuditRetentionPeriod: i64,
    pub AuditLogFullShutdownInProgress: super::super::super::Foundation::BOOLEAN,
    pub TimeToShutdown: i64,
    pub NextAuditRecordId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_AUDIT_LOG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_AUDIT_LOG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICY_AUDIT_LOG_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_AUDIT_SID_ARRAY {
    pub UsersCount: u32,
    pub UserSidArray: *mut super::super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_AUDIT_SID_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_AUDIT_SID_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICY_AUDIT_SID_ARRAY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct POLICY_AUDIT_SUBCATEGORIES_INFO {
    pub MaximumSubCategoryCount: u32,
    pub EventAuditingOptions: *mut u32,
}
impl ::core::marker::Copy for POLICY_AUDIT_SUBCATEGORIES_INFO {}
impl ::core::clone::Clone for POLICY_AUDIT_SUBCATEGORIES_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_AUDIT_SUBCATEGORIES_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct POLICY_DEFAULT_QUOTA_INFO {
    pub QuotaLimits: super::super::QUOTA_LIMITS,
}
impl ::core::marker::Copy for POLICY_DEFAULT_QUOTA_INFO {}
impl ::core::clone::Clone for POLICY_DEFAULT_QUOTA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_DEFAULT_QUOTA_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_DNS_DOMAIN_INFO {
    pub Name: super::super::super::Foundation::UNICODE_STRING,
    pub DnsDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub DnsForestName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainGuid: ::windows::core::GUID,
    pub Sid: super::super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_DNS_DOMAIN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_DNS_DOMAIN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICY_DNS_DOMAIN_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct POLICY_DOMAIN_EFS_INFO {
    pub InfoLength: u32,
    pub EfsBlob: *mut u8,
}
impl ::core::marker::Copy for POLICY_DOMAIN_EFS_INFO {}
impl ::core::clone::Clone for POLICY_DOMAIN_EFS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_DOMAIN_EFS_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct POLICY_DOMAIN_KERBEROS_TICKET_INFO {
    pub AuthenticationOptions: u32,
    pub MaxServiceTicketAge: i64,
    pub MaxTicketAge: i64,
    pub MaxRenewAge: i64,
    pub MaxClockSkew: i64,
    pub Reserved: i64,
}
impl ::core::marker::Copy for POLICY_DOMAIN_KERBEROS_TICKET_INFO {}
impl ::core::clone::Clone for POLICY_DOMAIN_KERBEROS_TICKET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_DOMAIN_KERBEROS_TICKET_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct POLICY_LSA_SERVER_ROLE_INFO {
    pub LsaServerRole: POLICY_LSA_SERVER_ROLE,
}
impl ::core::marker::Copy for POLICY_LSA_SERVER_ROLE_INFO {}
impl ::core::clone::Clone for POLICY_LSA_SERVER_ROLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_LSA_SERVER_ROLE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_MACHINE_ACCT_INFO {
    pub Rid: u32,
    pub Sid: super::super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_MACHINE_ACCT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_MACHINE_ACCT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICY_MACHINE_ACCT_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct POLICY_MODIFICATION_INFO {
    pub ModifiedId: i64,
    pub DatabaseCreationTime: i64,
}
impl ::core::marker::Copy for POLICY_MODIFICATION_INFO {}
impl ::core::clone::Clone for POLICY_MODIFICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POLICY_MODIFICATION_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_PD_ACCOUNT_INFO {
    pub Name: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_PD_ACCOUNT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_PD_ACCOUNT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICY_PD_ACCOUNT_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_PRIMARY_DOMAIN_INFO {
    pub Name: super::super::super::Foundation::UNICODE_STRING,
    pub Sid: super::super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_PRIMARY_DOMAIN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_PRIMARY_DOMAIN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICY_PRIMARY_DOMAIN_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_REPLICA_SOURCE_INFO {
    pub ReplicaSource: super::super::super::Foundation::UNICODE_STRING,
    pub ReplicaAccountName: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_REPLICA_SOURCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_REPLICA_SOURCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICY_REPLICA_SOURCE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct PctPublicKey {
    pub Type: u32,
    pub cbKey: u32,
    pub pKey: [u8; 1],
}
impl ::core::marker::Copy for PctPublicKey {}
impl ::core::clone::Clone for PctPublicKey {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PctPublicKey {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SAM_REGISTER_MAPPING_ELEMENT {
    pub Original: ::windows::core::PSTR,
    pub Mapped: ::windows::core::PSTR,
    pub Continuable: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SAM_REGISTER_MAPPING_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SAM_REGISTER_MAPPING_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAM_REGISTER_MAPPING_ELEMENT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SAM_REGISTER_MAPPING_LIST {
    pub Count: u32,
    pub Elements: *mut SAM_REGISTER_MAPPING_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SAM_REGISTER_MAPPING_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SAM_REGISTER_MAPPING_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAM_REGISTER_MAPPING_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SAM_REGISTER_MAPPING_TABLE {
    pub Count: u32,
    pub Lists: *mut SAM_REGISTER_MAPPING_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SAM_REGISTER_MAPPING_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SAM_REGISTER_MAPPING_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAM_REGISTER_MAPPING_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SCHANNEL_ALERT_TOKEN {
    pub dwTokenType: u32,
    pub dwAlertType: SCHANNEL_ALERT_TOKEN_ALERT_TYPE,
    pub dwAlertNumber: u32,
}
impl ::core::marker::Copy for SCHANNEL_ALERT_TOKEN {}
impl ::core::clone::Clone for SCHANNEL_ALERT_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCHANNEL_ALERT_TOKEN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SCHANNEL_CERT_HASH {
    pub dwLength: u32,
    pub dwFlags: u32,
    pub hProv: usize,
    pub ShaHash: [u8; 20],
}
impl ::core::marker::Copy for SCHANNEL_CERT_HASH {}
impl ::core::clone::Clone for SCHANNEL_CERT_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCHANNEL_CERT_HASH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SCHANNEL_CERT_HASH_STORE {
    pub dwLength: u32,
    pub dwFlags: u32,
    pub hProv: usize,
    pub ShaHash: [u8; 20],
    pub pwszStoreName: [u16; 128],
}
impl ::core::marker::Copy for SCHANNEL_CERT_HASH_STORE {}
impl ::core::clone::Clone for SCHANNEL_CERT_HASH_STORE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCHANNEL_CERT_HASH_STORE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SCHANNEL_CLIENT_SIGNATURE {
    pub cbLength: u32,
    pub aiHash: u32,
    pub cbHash: u32,
    pub HashValue: [u8; 36],
    pub CertThumbprint: [u8; 20],
}
impl ::core::marker::Copy for SCHANNEL_CLIENT_SIGNATURE {}
impl ::core::clone::Clone for SCHANNEL_CLIENT_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCHANNEL_CLIENT_SIGNATURE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SCHANNEL_CRED {
    pub dwVersion: u32,
    pub cCreds: u32,
    pub paCred: *mut *mut super::super::Cryptography::CERT_CONTEXT,
    pub hRootStore: super::super::Cryptography::HCERTSTORE,
    pub cMappers: u32,
    pub aphMappers: *mut *mut _HMAPPER,
    pub cSupportedAlgs: u32,
    pub palgSupportedAlgs: *mut u32,
    pub grbitEnabledProtocols: u32,
    pub dwMinimumCipherStrength: u32,
    pub dwMaximumCipherStrength: u32,
    pub dwSessionLifespan: u32,
    pub dwFlags: SCHANNEL_CRED_FLAGS,
    pub dwCredFormat: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for SCHANNEL_CRED {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for SCHANNEL_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for SCHANNEL_CRED {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SCHANNEL_SESSION_TOKEN {
    pub dwTokenType: u32,
    pub dwFlags: SCHANNEL_SESSION_TOKEN_FLAGS,
}
impl ::core::marker::Copy for SCHANNEL_SESSION_TOKEN {}
impl ::core::clone::Clone for SCHANNEL_SESSION_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCHANNEL_SESSION_TOKEN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SCH_CRED {
    pub dwVersion: u32,
    pub cCreds: u32,
    pub paSecret: *mut *mut ::core::ffi::c_void,
    pub paPublic: *mut *mut ::core::ffi::c_void,
    pub cMappers: u32,
    pub aphMappers: *mut *mut _HMAPPER,
}
impl ::core::marker::Copy for SCH_CRED {}
impl ::core::clone::Clone for SCH_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCH_CRED {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SCH_CRED_PUBLIC_CERTCHAIN {
    pub dwType: u32,
    pub cbCertChain: u32,
    pub pCertChain: *mut u8,
}
impl ::core::marker::Copy for SCH_CRED_PUBLIC_CERTCHAIN {}
impl ::core::clone::Clone for SCH_CRED_PUBLIC_CERTCHAIN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCH_CRED_PUBLIC_CERTCHAIN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SCH_CRED_SECRET_CAPI {
    pub dwType: u32,
    pub hProv: usize,
}
impl ::core::marker::Copy for SCH_CRED_SECRET_CAPI {}
impl ::core::clone::Clone for SCH_CRED_SECRET_CAPI {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCH_CRED_SECRET_CAPI {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SCH_CRED_SECRET_PRIVKEY {
    pub dwType: u32,
    pub pPrivateKey: *mut u8,
    pub cbPrivateKey: u32,
    pub pszPassword: ::windows::core::PSTR,
}
impl ::core::marker::Copy for SCH_CRED_SECRET_PRIVKEY {}
impl ::core::clone::Clone for SCH_CRED_SECRET_PRIVKEY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCH_CRED_SECRET_PRIVKEY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SCH_EXTENSION_DATA {
    pub ExtensionType: u16,
    pub pExtData: *const u8,
    pub cbExtData: u32,
}
impl ::core::marker::Copy for SCH_EXTENSION_DATA {}
impl ::core::clone::Clone for SCH_EXTENSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCH_EXTENSION_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_APP_MODE_INFO {
    pub UserFunction: u32,
    pub Argument1: usize,
    pub Argument2: usize,
    pub UserData: SecBuffer,
    pub ReturnToLsa: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_APP_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_APP_MODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_APP_MODE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_BYTE_VECTOR {
    pub ByteArrayOffset: u32,
    pub ByteArrayLength: u16,
}
impl ::core::marker::Copy for SECPKG_BYTE_VECTOR {}
impl ::core::clone::Clone for SECPKG_BYTE_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_BYTE_VECTOR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_CALL_INFO {
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub Attributes: u32,
    pub CallCount: u32,
    pub MechOid: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SECPKG_CALL_INFO {}
impl ::core::clone::Clone for SECPKG_CALL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_CALL_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_CALL_PACKAGE_PIN_DC_REQUEST {
    pub MessageType: u32,
    pub Flags: u32,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub DcName: super::super::super::Foundation::UNICODE_STRING,
    pub DcFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_CALL_PACKAGE_PIN_DC_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_CALL_PACKAGE_PIN_DC_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_CALL_PACKAGE_PIN_DC_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST {
    pub MessageType: u32,
    pub OriginLogonId: super::super::super::Foundation::LUID,
    pub DestinationLogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST {
    pub MessageType: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST {}
impl ::core::clone::Clone for SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_CLIENT_INFO {
    pub LogonId: super::super::super::Foundation::LUID,
    pub ProcessID: u32,
    pub ThreadID: u32,
    pub HasTcbPrivilege: super::super::super::Foundation::BOOLEAN,
    pub Impersonating: super::super::super::Foundation::BOOLEAN,
    pub Restricted: super::super::super::Foundation::BOOLEAN,
    pub ClientFlags: u8,
    pub ImpersonationLevel: super::super::SECURITY_IMPERSONATION_LEVEL,
    pub ClientToken: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_CLIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_CLIENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_CLIENT_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_CONTEXT_THUNKS {
    pub InfoLevelCount: u32,
    pub Levels: [u32; 1],
}
impl ::core::marker::Copy for SECPKG_CONTEXT_THUNKS {}
impl ::core::clone::Clone for SECPKG_CONTEXT_THUNKS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_CONTEXT_THUNKS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_CREDENTIAL {
    pub Version: u64,
    pub cbHeaderLength: u16,
    pub cbStructureLength: u32,
    pub ClientProcess: u32,
    pub ClientThread: u32,
    pub LogonId: super::super::super::Foundation::LUID,
    pub ClientToken: super::super::super::Foundation::HANDLE,
    pub SessionId: u32,
    pub ModifiedId: super::super::super::Foundation::LUID,
    pub fCredentials: u32,
    pub Flags: u32,
    pub PrincipalName: SECPKG_BYTE_VECTOR,
    pub PackageList: SECPKG_BYTE_VECTOR,
    pub MarshaledSuppliedCreds: SECPKG_BYTE_VECTOR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_CREDENTIAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_CREDENTIAL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_DLL_FUNCTIONS {
    pub AllocateHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeHeap: PLSA_FREE_LSA_HEAP,
    pub RegisterCallback: PLSA_REGISTER_CALLBACK,
    pub LocatePackageById: PLSA_LOCATE_PKG_BY_ID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_DLL_FUNCTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_DLL_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_DLL_FUNCTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_EVENT_NOTIFY {
    pub EventClass: u32,
    pub Reserved: u32,
    pub EventDataSize: u32,
    pub EventData: *mut ::core::ffi::c_void,
    pub PackageParameter: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SECPKG_EVENT_NOTIFY {}
impl ::core::clone::Clone for SECPKG_EVENT_NOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_EVENT_NOTIFY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_EVENT_PACKAGE_CHANGE {
    pub ChangeType: SECPKG_PACKAGE_CHANGE_TYPE,
    pub PackageId: usize,
    pub PackageName: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_EVENT_PACKAGE_CHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_EVENT_PACKAGE_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_EVENT_PACKAGE_CHANGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_EVENT_ROLE_CHANGE {
    pub PreviousRole: u32,
    pub NewRole: u32,
}
impl ::core::marker::Copy for SECPKG_EVENT_ROLE_CHANGE {}
impl ::core::clone::Clone for SECPKG_EVENT_ROLE_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_EVENT_ROLE_CHANGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_EXTENDED_INFORMATION {
    pub Class: SECPKG_EXTENDED_INFORMATION_CLASS,
    pub Info: SECPKG_EXTENDED_INFORMATION_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_EXTENDED_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_EXTENDED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_EXTENDED_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union SECPKG_EXTENDED_INFORMATION_0 {
    pub GssInfo: SECPKG_GSS_INFO,
    pub ContextThunks: SECPKG_CONTEXT_THUNKS,
    pub MutualAuthLevel: SECPKG_MUTUAL_AUTH_LEVEL,
    pub WowClientDll: SECPKG_WOW_CLIENT_DLL,
    pub ExtraOids: SECPKG_EXTRA_OIDS,
    pub Nego2Info: SECPKG_NEGO2_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_EXTENDED_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_EXTENDED_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_EXTENDED_INFORMATION_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_EXTRA_OIDS {
    pub OidCount: u32,
    pub Oids: [SECPKG_SERIALIZED_OID; 1],
}
impl ::core::marker::Copy for SECPKG_EXTRA_OIDS {}
impl ::core::clone::Clone for SECPKG_EXTRA_OIDS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_EXTRA_OIDS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
pub struct SECPKG_FUNCTION_TABLE {
    pub InitializePackage: PLSA_AP_INITIALIZE_PACKAGE,
    pub LogonUserA: PLSA_AP_LOGON_USER,
    pub CallPackage: PLSA_AP_CALL_PACKAGE,
    pub LogonTerminated: PLSA_AP_LOGON_TERMINATED,
    pub CallPackageUntrusted: PLSA_AP_CALL_PACKAGE,
    pub CallPackagePassthrough: PLSA_AP_CALL_PACKAGE_PASSTHROUGH,
    pub LogonUserExA: PLSA_AP_LOGON_USER_EX,
    pub LogonUserEx2: PLSA_AP_LOGON_USER_EX2,
    pub Initialize: SpInitializeFn,
    pub Shutdown: SpShutdownFn,
    pub GetInfo: SpGetInfoFn,
    pub AcceptCredentials: SpAcceptCredentialsFn,
    pub AcquireCredentialsHandleA: SpAcquireCredentialsHandleFn,
    pub QueryCredentialsAttributesA: SpQueryCredentialsAttributesFn,
    pub FreeCredentialsHandle: SpFreeCredentialsHandleFn,
    pub SaveCredentials: SpSaveCredentialsFn,
    pub GetCredentials: SpGetCredentialsFn,
    pub DeleteCredentials: SpDeleteCredentialsFn,
    pub InitLsaModeContext: SpInitLsaModeContextFn,
    pub AcceptLsaModeContext: SpAcceptLsaModeContextFn,
    pub DeleteContext: SpDeleteContextFn,
    pub ApplyControlToken: SpApplyControlTokenFn,
    pub GetUserInfo: SpGetUserInfoFn,
    pub GetExtendedInformation: SpGetExtendedInformationFn,
    pub QueryContextAttributesA: SpQueryContextAttributesFn,
    pub AddCredentialsA: SpAddCredentialsFn,
    pub SetExtendedInformation: SpSetExtendedInformationFn,
    pub SetContextAttributesA: SpSetContextAttributesFn,
    pub SetCredentialsAttributesA: SpSetCredentialsAttributesFn,
    pub ChangeAccountPasswordA: SpChangeAccountPasswordFn,
    pub QueryMetaData: SpQueryMetaDataFn,
    pub ExchangeMetaData: SpExchangeMetaDataFn,
    pub GetCredUIContext: SpGetCredUIContextFn,
    pub UpdateCredentials: SpUpdateCredentialsFn,
    pub ValidateTargetInfo: SpValidateTargetInfoFn,
    pub PostLogonUser: LSA_AP_POST_LOGON_USER,
    pub GetRemoteCredGuardLogonBuffer: SpGetRemoteCredGuardLogonBufferFn,
    pub GetRemoteCredGuardSupplementalCreds: SpGetRemoteCredGuardSupplementalCredsFn,
    pub GetTbalSupplementalCreds: SpGetTbalSupplementalCredsFn,
    pub LogonUserEx3: PLSA_AP_LOGON_USER_EX3,
    pub PreLogonUserSurrogate: PLSA_AP_PRE_LOGON_USER_SURROGATE,
    pub PostLogonUserSurrogate: PLSA_AP_POST_LOGON_USER_SURROGATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
impl ::core::marker::Copy for SECPKG_FUNCTION_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
impl ::core::clone::Clone for SECPKG_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
unsafe impl ::windows::core::Abi for SECPKG_FUNCTION_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_GSS_INFO {
    pub EncodedIdLength: u32,
    pub EncodedId: [u8; 4],
}
impl ::core::marker::Copy for SECPKG_GSS_INFO {}
impl ::core::clone::Clone for SECPKG_GSS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_GSS_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct SECPKG_KERNEL_FUNCTIONS {
    pub AllocateHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeHeap: PLSA_FREE_LSA_HEAP,
    pub CreateContextList: PKSEC_CREATE_CONTEXT_LIST,
    pub InsertListEntry: PKSEC_INSERT_LIST_ENTRY,
    pub ReferenceListEntry: PKSEC_REFERENCE_LIST_ENTRY,
    pub DereferenceListEntry: PKSEC_DEREFERENCE_LIST_ENTRY,
    pub SerializeWinntAuthData: PKSEC_SERIALIZE_WINNT_AUTH_DATA,
    pub SerializeSchannelAuthData: PKSEC_SERIALIZE_SCHANNEL_AUTH_DATA,
    pub LocatePackageById: PKSEC_LOCATE_PKG_BY_ID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for SECPKG_KERNEL_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for SECPKG_KERNEL_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for SECPKG_KERNEL_FUNCTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct SECPKG_KERNEL_FUNCTION_TABLE {
    pub Initialize: KspInitPackageFn,
    pub DeleteContext: KspDeleteContextFn,
    pub InitContext: KspInitContextFn,
    pub MapHandle: KspMapHandleFn,
    pub Sign: KspMakeSignatureFn,
    pub Verify: KspVerifySignatureFn,
    pub Seal: KspSealMessageFn,
    pub Unseal: KspUnsealMessageFn,
    pub GetToken: KspGetTokenFn,
    pub QueryAttributes: KspQueryAttributesFn,
    pub CompleteToken: KspCompleteTokenFn,
    pub ExportContext: SpExportSecurityContextFn,
    pub ImportContext: SpImportSecurityContextFn,
    pub SetPackagePagingMode: KspSetPagingModeFn,
    pub SerializeAuthData: KspSerializeAuthDataFn,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for SECPKG_KERNEL_FUNCTION_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for SECPKG_KERNEL_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for SECPKG_KERNEL_FUNCTION_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_MUTUAL_AUTH_LEVEL {
    pub MutualAuthLevel: u32,
}
impl ::core::marker::Copy for SECPKG_MUTUAL_AUTH_LEVEL {}
impl ::core::clone::Clone for SECPKG_MUTUAL_AUTH_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_MUTUAL_AUTH_LEVEL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_NEGO2_INFO {
    pub AuthScheme: [u8; 16],
    pub PackageFlags: u32,
}
impl ::core::marker::Copy for SECPKG_NEGO2_INFO {}
impl ::core::clone::Clone for SECPKG_NEGO2_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_NEGO2_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_PARAMETERS {
    pub Version: u32,
    pub MachineState: u32,
    pub SetupMode: u32,
    pub DomainSid: super::super::super::Foundation::PSID,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub DnsDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_PARAMETERS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_POST_LOGON_USER_INFO {
    pub Flags: u32,
    pub LogonId: super::super::super::Foundation::LUID,
    pub LinkedLogonId: super::super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_POST_LOGON_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_POST_LOGON_USER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_POST_LOGON_USER_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_PRIMARY_CRED {
    pub LogonId: super::super::super::Foundation::LUID,
    pub DownlevelName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub Password: super::super::super::Foundation::UNICODE_STRING,
    pub OldPassword: super::super::super::Foundation::UNICODE_STRING,
    pub UserSid: super::super::super::Foundation::PSID,
    pub Flags: u32,
    pub DnsDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub Upn: super::super::super::Foundation::UNICODE_STRING,
    pub LogonServer: super::super::super::Foundation::UNICODE_STRING,
    pub Spare1: super::super::super::Foundation::UNICODE_STRING,
    pub Spare2: super::super::super::Foundation::UNICODE_STRING,
    pub Spare3: super::super::super::Foundation::UNICODE_STRING,
    pub Spare4: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_PRIMARY_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_PRIMARY_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_PRIMARY_CRED {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_PRIMARY_CRED_EX {
    pub LogonId: super::super::super::Foundation::LUID,
    pub DownlevelName: super::super::super::Foundation::UNICODE_STRING,
    pub DomainName: super::super::super::Foundation::UNICODE_STRING,
    pub Password: super::super::super::Foundation::UNICODE_STRING,
    pub OldPassword: super::super::super::Foundation::UNICODE_STRING,
    pub UserSid: super::super::super::Foundation::PSID,
    pub Flags: u32,
    pub DnsDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub Upn: super::super::super::Foundation::UNICODE_STRING,
    pub LogonServer: super::super::super::Foundation::UNICODE_STRING,
    pub Spare1: super::super::super::Foundation::UNICODE_STRING,
    pub Spare2: super::super::super::Foundation::UNICODE_STRING,
    pub Spare3: super::super::super::Foundation::UNICODE_STRING,
    pub Spare4: super::super::super::Foundation::UNICODE_STRING,
    pub PackageId: usize,
    pub PrevLogonId: super::super::super::Foundation::LUID,
    pub FlagsEx: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_PRIMARY_CRED_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_PRIMARY_CRED_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_PRIMARY_CRED_EX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_REDIRECTED_LOGON_BUFFER {
    pub RedirectedLogonGuid: ::windows::core::GUID,
    pub RedirectedLogonHandle: super::super::super::Foundation::HANDLE,
    pub Init: PLSA_REDIRECTED_LOGON_INIT,
    pub Callback: PLSA_REDIRECTED_LOGON_CALLBACK,
    pub CleanupCallback: PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK,
    pub GetLogonCreds: PLSA_REDIRECTED_LOGON_GET_LOGON_CREDS,
    pub GetSupplementalCreds: PLSA_REDIRECTED_LOGON_GET_SUPP_CREDS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_REDIRECTED_LOGON_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_REDIRECTED_LOGON_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_REDIRECTED_LOGON_BUFFER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_SERIALIZED_OID {
    pub OidLength: u32,
    pub OidAttributes: u32,
    pub OidValue: [u8; 32],
}
impl ::core::marker::Copy for SECPKG_SERIALIZED_OID {}
impl ::core::clone::Clone for SECPKG_SERIALIZED_OID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_SERIALIZED_OID {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_SHORT_VECTOR {
    pub ShortArrayOffset: u32,
    pub ShortArrayCount: u16,
}
impl ::core::marker::Copy for SECPKG_SHORT_VECTOR {}
impl ::core::clone::Clone for SECPKG_SHORT_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_SHORT_VECTOR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_SUPPLEMENTAL_CRED {
    pub PackageName: super::super::super::Foundation::UNICODE_STRING,
    pub CredentialSize: u32,
    pub Credentials: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_SUPPLEMENTAL_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_SUPPLEMENTAL_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_SUPPLEMENTAL_CRED {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_SUPPLEMENTAL_CRED_ARRAY {
    pub CredentialCount: u32,
    pub Credentials: [SECPKG_SUPPLEMENTAL_CRED; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_SUPPLEMENTAL_CRED_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_SUPPLEMENTAL_CRED_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_SUPPLEMENTAL_CRED_ARRAY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_SUPPLIED_CREDENTIAL {
    pub cbHeaderLength: u16,
    pub cbStructureLength: u16,
    pub UserName: SECPKG_SHORT_VECTOR,
    pub DomainName: SECPKG_SHORT_VECTOR,
    pub PackedCredentials: SECPKG_BYTE_VECTOR,
    pub CredFlags: u32,
}
impl ::core::marker::Copy for SECPKG_SUPPLIED_CREDENTIAL {}
impl ::core::clone::Clone for SECPKG_SUPPLIED_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_SUPPLIED_CREDENTIAL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_SURROGATE_LOGON {
    pub Version: u32,
    pub SurrogateLogonID: super::super::super::Foundation::LUID,
    pub EntryCount: u32,
    pub Entries: *mut SECPKG_SURROGATE_LOGON_ENTRY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_SURROGATE_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_SURROGATE_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_SURROGATE_LOGON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECPKG_SURROGATE_LOGON_ENTRY {
    pub Type: ::windows::core::GUID,
    pub Data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SECPKG_SURROGATE_LOGON_ENTRY {}
impl ::core::clone::Clone for SECPKG_SURROGATE_LOGON_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECPKG_SURROGATE_LOGON_ENTRY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_TARGETINFO {
    pub DomainSid: super::super::super::Foundation::PSID,
    pub ComputerName: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_TARGETINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_TARGETINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_TARGETINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_USER_FUNCTION_TABLE {
    pub InstanceInit: SpInstanceInitFn,
    pub InitUserModeContext: SpInitUserModeContextFn,
    pub MakeSignature: SpMakeSignatureFn,
    pub VerifySignature: SpVerifySignatureFn,
    pub SealMessage: SpSealMessageFn,
    pub UnsealMessage: SpUnsealMessageFn,
    pub GetContextToken: SpGetContextTokenFn,
    pub QueryContextAttributesA: SpQueryContextAttributesFn,
    pub CompleteAuthToken: SpCompleteAuthTokenFn,
    pub DeleteUserModeContext: SpDeleteContextFn,
    pub FormatCredentials: SpFormatCredentialsFn,
    pub MarshallSupplementalCreds: SpMarshallSupplementalCredsFn,
    pub ExportContext: SpExportSecurityContextFn,
    pub ImportContext: SpImportSecurityContextFn,
    pub MarshalAttributeData: SpMarshalAttributeDataFn,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_USER_FUNCTION_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_USER_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_USER_FUNCTION_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECPKG_WOW_CLIENT_DLL {
    pub WowClientDllPath: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECPKG_WOW_CLIENT_DLL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECPKG_WOW_CLIENT_DLL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECPKG_WOW_CLIENT_DLL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_LOGON_SESSION_DATA {
    pub Size: u32,
    pub LogonId: super::super::super::Foundation::LUID,
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub LogonDomain: super::super::super::Foundation::UNICODE_STRING,
    pub AuthenticationPackage: super::super::super::Foundation::UNICODE_STRING,
    pub LogonType: u32,
    pub Session: u32,
    pub Sid: super::super::super::Foundation::PSID,
    pub LogonTime: i64,
    pub LogonServer: super::super::super::Foundation::UNICODE_STRING,
    pub DnsDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub Upn: super::super::super::Foundation::UNICODE_STRING,
    pub UserFlags: u32,
    pub LastLogonInfo: LSA_LAST_INTER_LOGON_INFO,
    pub LogonScript: super::super::super::Foundation::UNICODE_STRING,
    pub ProfilePath: super::super::super::Foundation::UNICODE_STRING,
    pub HomeDirectory: super::super::super::Foundation::UNICODE_STRING,
    pub HomeDirectoryDrive: super::super::super::Foundation::UNICODE_STRING,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_LOGON_SESSION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_LOGON_SESSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECURITY_LOGON_SESSION_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SECURITY_PACKAGE_OPTIONS {
    pub Size: u32,
    pub Type: SECURITY_PACKAGE_OPTIONS_TYPE,
    pub Flags: u32,
    pub SignatureSize: u32,
    pub Signature: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SECURITY_PACKAGE_OPTIONS {}
impl ::core::clone::Clone for SECURITY_PACKAGE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECURITY_PACKAGE_OPTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_USER_DATA {
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub LogonDomainName: super::super::super::Foundation::UNICODE_STRING,
    pub LogonServer: super::super::super::Foundation::UNICODE_STRING,
    pub pSid: super::super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_USER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_USER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECURITY_USER_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_APPLICATION_PROTOCOLS {
    pub ProtocolListsSize: u32,
    pub ProtocolLists: [SEC_APPLICATION_PROTOCOL_LIST; 1],
}
impl ::core::marker::Copy for SEC_APPLICATION_PROTOCOLS {}
impl ::core::clone::Clone for SEC_APPLICATION_PROTOCOLS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_APPLICATION_PROTOCOLS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_APPLICATION_PROTOCOL_LIST {
    pub ProtoNegoExt: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT,
    pub ProtocolListSize: u16,
    pub ProtocolList: [u8; 1],
}
impl ::core::marker::Copy for SEC_APPLICATION_PROTOCOL_LIST {}
impl ::core::clone::Clone for SEC_APPLICATION_PROTOCOL_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_APPLICATION_PROTOCOL_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_CHANNEL_BINDINGS {
    pub dwInitiatorAddrType: u32,
    pub cbInitiatorLength: u32,
    pub dwInitiatorOffset: u32,
    pub dwAcceptorAddrType: u32,
    pub cbAcceptorLength: u32,
    pub dwAcceptorOffset: u32,
    pub cbApplicationDataLength: u32,
    pub dwApplicationDataOffset: u32,
}
impl ::core::marker::Copy for SEC_CHANNEL_BINDINGS {}
impl ::core::clone::Clone for SEC_CHANNEL_BINDINGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_CHANNEL_BINDINGS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_DTLS_MTU {
    pub PathMTU: u16,
}
impl ::core::marker::Copy for SEC_DTLS_MTU {}
impl ::core::clone::Clone for SEC_DTLS_MTU {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_DTLS_MTU {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_FLAGS {
    pub Flags: u64,
}
impl ::core::marker::Copy for SEC_FLAGS {}
impl ::core::clone::Clone for SEC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_FLAGS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_NEGOTIATION_INFO {
    pub Size: u32,
    pub NameLength: u32,
    pub Name: *mut u16,
    pub Reserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SEC_NEGOTIATION_INFO {}
impl ::core::clone::Clone for SEC_NEGOTIATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_NEGOTIATION_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_PRESHAREDKEY {
    pub KeySize: u16,
    pub Key: [u8; 1],
}
impl ::core::marker::Copy for SEC_PRESHAREDKEY {}
impl ::core::clone::Clone for SEC_PRESHAREDKEY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_PRESHAREDKEY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_PRESHAREDKEY_IDENTITY {
    pub KeyIdentitySize: u16,
    pub KeyIdentity: [u8; 1],
}
impl ::core::marker::Copy for SEC_PRESHAREDKEY_IDENTITY {}
impl ::core::clone::Clone for SEC_PRESHAREDKEY_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_PRESHAREDKEY_IDENTITY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_SRTP_MASTER_KEY_IDENTIFIER {
    pub MasterKeyIdentifierSize: u8,
    pub MasterKeyIdentifier: [u8; 1],
}
impl ::core::marker::Copy for SEC_SRTP_MASTER_KEY_IDENTIFIER {}
impl ::core::clone::Clone for SEC_SRTP_MASTER_KEY_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_SRTP_MASTER_KEY_IDENTIFIER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_SRTP_PROTECTION_PROFILES {
    pub ProfilesSize: u16,
    pub ProfilesList: [u16; 1],
}
impl ::core::marker::Copy for SEC_SRTP_PROTECTION_PROFILES {}
impl ::core::clone::Clone for SEC_SRTP_PROTECTION_PROFILES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_SRTP_PROTECTION_PROFILES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_TOKEN_BINDING {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub KeyParametersSize: u16,
    pub KeyParameters: [u8; 1],
}
impl ::core::marker::Copy for SEC_TOKEN_BINDING {}
impl ::core::clone::Clone for SEC_TOKEN_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_TOKEN_BINDING {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_TRAFFIC_SECRETS {
    pub SymmetricAlgId: [u16; 64],
    pub ChainingMode: [u16; 64],
    pub HashAlgId: [u16; 64],
    pub KeySize: u16,
    pub IvSize: u16,
    pub MsgSequenceStart: u16,
    pub MsgSequenceEnd: u16,
    pub TrafficSecretType: SEC_TRAFFIC_SECRET_TYPE,
    pub TrafficSecretSize: u16,
    pub TrafficSecret: [u8; 1],
}
impl ::core::marker::Copy for SEC_TRAFFIC_SECRETS {}
impl ::core::clone::Clone for SEC_TRAFFIC_SECRETS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_TRAFFIC_SECRETS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY32 {
    pub User: u32,
    pub UserLength: u32,
    pub Domain: u32,
    pub DomainLength: u32,
    pub Password: u32,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY32 {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY32 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_WINNT_AUTH_IDENTITY32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_EX2 {
    pub Version: u32,
    pub cbHeaderLength: u16,
    pub cbStructureLength: u32,
    pub UserOffset: u32,
    pub UserLength: u16,
    pub DomainOffset: u32,
    pub DomainLength: u16,
    pub PackedCredentialsOffset: u32,
    pub PackedCredentialsLength: u16,
    pub Flags: u32,
    pub PackageListOffset: u32,
    pub PackageListLength: u16,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_EX2 {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_EX2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_WINNT_AUTH_IDENTITY_EX2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_EX32 {
    pub Version: u32,
    pub Length: u32,
    pub User: u32,
    pub UserLength: u32,
    pub Domain: u32,
    pub DomainLength: u32,
    pub Password: u32,
    pub PasswordLength: u32,
    pub Flags: u32,
    pub PackageList: u32,
    pub PackageListLength: u32,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_EX32 {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_EX32 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_WINNT_AUTH_IDENTITY_EX32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_EXA {
    pub Version: u32,
    pub Length: u32,
    pub User: *mut u8,
    pub UserLength: u32,
    pub Domain: *mut u8,
    pub DomainLength: u32,
    pub Password: *mut u8,
    pub PasswordLength: u32,
    pub Flags: u32,
    pub PackageList: *mut u8,
    pub PackageListLength: u32,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_EXA {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_EXA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_WINNT_AUTH_IDENTITY_EXA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_EXW {
    pub Version: u32,
    pub Length: u32,
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
    pub PackageList: *mut u16,
    pub PackageListLength: u32,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_EXW {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_EXW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEC_WINNT_AUTH_IDENTITY_EXW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_System_Rpc\"`*"]
#[cfg(feature = "Win32_System_Rpc")]
pub union SEC_WINNT_AUTH_IDENTITY_INFO {
    pub AuthIdExw: SEC_WINNT_AUTH_IDENTITY_EXW,
    pub AuthIdExa: SEC_WINNT_AUTH_IDENTITY_EXA,
    pub AuthId_a: super::super::super::System::Rpc::SEC_WINNT_AUTH_IDENTITY_A,
    pub AuthId_w: super::super::super::System::Rpc::SEC_WINNT_AUTH_IDENTITY_W,
    pub AuthIdEx2: SEC_WINNT_AUTH_IDENTITY_EX2,
}
#[cfg(feature = "Win32_System_Rpc")]
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_INFO {}
#[cfg(feature = "Win32_System_Rpc")]
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Rpc")]
unsafe impl ::windows::core::Abi for SEC_WINNT_AUTH_IDENTITY_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SEND_GENERIC_TLS_EXTENSION {
    pub ExtensionType: u16,
    pub HandshakeType: u16,
    pub Flags: u32,
    pub BufferSize: u16,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for SEND_GENERIC_TLS_EXTENSION {}
impl ::core::clone::Clone for SEND_GENERIC_TLS_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SEND_GENERIC_TLS_EXTENSION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SE_ADT_ACCESS_REASON {
    pub AccessMask: u32,
    pub AccessReasons: [u32; 32],
    pub ObjectTypeIndex: u32,
    pub AccessGranted: u32,
    pub SecurityDescriptor: super::super::PSECURITY_DESCRIPTOR,
}
impl ::core::marker::Copy for SE_ADT_ACCESS_REASON {}
impl ::core::clone::Clone for SE_ADT_ACCESS_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SE_ADT_ACCESS_REASON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SE_ADT_CLAIMS {
    pub Length: u32,
    pub Claims: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SE_ADT_CLAIMS {}
impl ::core::clone::Clone for SE_ADT_CLAIMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SE_ADT_CLAIMS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SE_ADT_OBJECT_TYPE {
    pub ObjectType: ::windows::core::GUID,
    pub Flags: u16,
    pub Level: u16,
    pub AccessMask: u32,
}
impl ::core::marker::Copy for SE_ADT_OBJECT_TYPE {}
impl ::core::clone::Clone for SE_ADT_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SE_ADT_OBJECT_TYPE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SE_ADT_PARAMETER_ARRAY {
    pub CategoryId: u32,
    pub AuditId: u32,
    pub ParameterCount: u32,
    pub Length: u32,
    pub FlatSubCategoryId: u16,
    pub Type: u16,
    pub Flags: u32,
    pub Parameters: [SE_ADT_PARAMETER_ARRAY_ENTRY; 32],
}
impl ::core::marker::Copy for SE_ADT_PARAMETER_ARRAY {}
impl ::core::clone::Clone for SE_ADT_PARAMETER_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SE_ADT_PARAMETER_ARRAY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SE_ADT_PARAMETER_ARRAY_ENTRY {
    pub Type: SE_ADT_PARAMETER_TYPE,
    pub Length: u32,
    pub Data: [usize; 2],
    pub Address: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SE_ADT_PARAMETER_ARRAY_ENTRY {}
impl ::core::clone::Clone for SE_ADT_PARAMETER_ARRAY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SE_ADT_PARAMETER_ARRAY_ENTRY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SE_ADT_PARAMETER_ARRAY_EX {
    pub CategoryId: u32,
    pub AuditId: u32,
    pub Version: u32,
    pub ParameterCount: u32,
    pub Length: u32,
    pub FlatSubCategoryId: u16,
    pub Type: u16,
    pub Flags: u32,
    pub Parameters: [SE_ADT_PARAMETER_ARRAY_ENTRY; 32],
}
impl ::core::marker::Copy for SE_ADT_PARAMETER_ARRAY_EX {}
impl ::core::clone::Clone for SE_ADT_PARAMETER_ARRAY_EX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SE_ADT_PARAMETER_ARRAY_EX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SL_ACTIVATION_INFO_HEADER {
    pub cbSize: u32,
    pub r#type: SL_ACTIVATION_TYPE,
}
impl ::core::marker::Copy for SL_ACTIVATION_INFO_HEADER {}
impl ::core::clone::Clone for SL_ACTIVATION_INFO_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SL_ACTIVATION_INFO_HEADER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SL_AD_ACTIVATION_INFO {
    pub header: SL_ACTIVATION_INFO_HEADER,
    pub pwszProductKey: ::windows::core::PCWSTR,
    pub pwszActivationObjectName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for SL_AD_ACTIVATION_INFO {}
impl ::core::clone::Clone for SL_AD_ACTIVATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SL_AD_ACTIVATION_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SL_LICENSING_STATUS {
    pub SkuId: ::windows::core::GUID,
    pub eStatus: SLLICENSINGSTATUS,
    pub dwGraceTime: u32,
    pub dwTotalGraceDays: u32,
    pub hrReason: ::windows::core::HRESULT,
    pub qwValidityExpiration: u64,
}
impl ::core::marker::Copy for SL_LICENSING_STATUS {}
impl ::core::clone::Clone for SL_LICENSING_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SL_LICENSING_STATUS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SL_NONGENUINE_UI_OPTIONS {
    pub cbSize: u32,
    pub pComponentId: *const ::windows::core::GUID,
    pub hResultUI: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for SL_NONGENUINE_UI_OPTIONS {}
impl ::core::clone::Clone for SL_NONGENUINE_UI_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SL_NONGENUINE_UI_OPTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SL_SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [*mut ::core::ffi::c_void; 2],
    pub Reserved2: [u32; 3],
}
impl ::core::marker::Copy for SL_SYSTEM_POLICY_INFORMATION {}
impl ::core::clone::Clone for SL_SYSTEM_POLICY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SL_SYSTEM_POLICY_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SR_SECURITY_DESCRIPTOR {
    pub Length: u32,
    pub SecurityDescriptor: *mut u8,
}
impl ::core::marker::Copy for SR_SECURITY_DESCRIPTOR {}
impl ::core::clone::Clone for SR_SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SR_SECURITY_DESCRIPTOR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SSL_CREDENTIAL_CERTIFICATE {
    pub cbPrivateKey: u32,
    pub pPrivateKey: *mut u8,
    pub cbCertificate: u32,
    pub pCertificate: *mut u8,
    pub pszPassword: ::windows::core::PSTR,
}
impl ::core::marker::Copy for SSL_CREDENTIAL_CERTIFICATE {}
impl ::core::clone::Clone for SSL_CREDENTIAL_CERTIFICATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SSL_CREDENTIAL_CERTIFICATE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SUBSCRIBE_GENERIC_TLS_EXTENSION {
    pub Flags: u32,
    pub SubscriptionsCount: u32,
    pub Subscriptions: [TLS_EXTENSION_SUBSCRIPTION; 1],
}
impl ::core::marker::Copy for SUBSCRIBE_GENERIC_TLS_EXTENSION {}
impl ::core::clone::Clone for SUBSCRIBE_GENERIC_TLS_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SUBSCRIBE_GENERIC_TLS_EXTENSION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecBuffer {
    pub cbBuffer: u32,
    pub BufferType: u32,
    pub pvBuffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SecBuffer {}
impl ::core::clone::Clone for SecBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecBuffer {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecBufferDesc {
    pub ulVersion: u32,
    pub cBuffers: u32,
    pub pBuffers: *mut SecBuffer,
}
impl ::core::marker::Copy for SecBufferDesc {}
impl ::core::clone::Clone for SecBufferDesc {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecBufferDesc {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_AccessToken {
    pub AccessToken: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SecPkgContext_AccessToken {}
impl ::core::clone::Clone for SecPkgContext_AccessToken {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_AccessToken {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_ApplicationProtocol {
    pub ProtoNegoStatus: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS,
    pub ProtoNegoExt: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT,
    pub ProtocolIdSize: u8,
    pub ProtocolId: [u8; 255],
}
impl ::core::marker::Copy for SecPkgContext_ApplicationProtocol {}
impl ::core::clone::Clone for SecPkgContext_ApplicationProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_ApplicationProtocol {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_AuthorityA {
    pub sAuthorityName: *mut i8,
}
impl ::core::marker::Copy for SecPkgContext_AuthorityA {}
impl ::core::clone::Clone for SecPkgContext_AuthorityA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_AuthorityA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_AuthorityW {
    pub sAuthorityName: *mut u16,
}
impl ::core::marker::Copy for SecPkgContext_AuthorityW {}
impl ::core::clone::Clone for SecPkgContext_AuthorityW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_AuthorityW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_AuthzID {
    pub AuthzIDLength: u32,
    pub AuthzID: ::windows::core::PSTR,
}
impl ::core::marker::Copy for SecPkgContext_AuthzID {}
impl ::core::clone::Clone for SecPkgContext_AuthzID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_AuthzID {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_Bindings {
    pub BindingsLength: u32,
    pub Bindings: *mut SEC_CHANNEL_BINDINGS,
}
impl ::core::marker::Copy for SecPkgContext_Bindings {}
impl ::core::clone::Clone for SecPkgContext_Bindings {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_Bindings {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_CertInfo {
    pub dwVersion: u32,
    pub cbSubjectName: u32,
    pub pwszSubjectName: ::windows::core::PWSTR,
    pub cbIssuerName: u32,
    pub pwszIssuerName: ::windows::core::PWSTR,
    pub dwKeySize: u32,
}
impl ::core::marker::Copy for SecPkgContext_CertInfo {}
impl ::core::clone::Clone for SecPkgContext_CertInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_CertInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_CertificateValidationResult {
    pub dwChainErrorStatus: u32,
    pub hrVerifyChainStatus: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for SecPkgContext_CertificateValidationResult {}
impl ::core::clone::Clone for SecPkgContext_CertificateValidationResult {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_CertificateValidationResult {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_Certificates {
    pub cCertificates: u32,
    pub cbCertificateChain: u32,
    pub pbCertificateChain: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_Certificates {}
impl ::core::clone::Clone for SecPkgContext_Certificates {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_Certificates {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_CipherInfo {
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub dwCipherSuite: u32,
    pub dwBaseCipherSuite: u32,
    pub szCipherSuite: [u16; 64],
    pub szCipher: [u16; 64],
    pub dwCipherLen: u32,
    pub dwCipherBlockLen: u32,
    pub szHash: [u16; 64],
    pub dwHashLen: u32,
    pub szExchange: [u16; 64],
    pub dwMinExchangeLen: u32,
    pub dwMaxExchangeLen: u32,
    pub szCertificate: [u16; 64],
    pub dwKeyType: u32,
}
impl ::core::marker::Copy for SecPkgContext_CipherInfo {}
impl ::core::clone::Clone for SecPkgContext_CipherInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_CipherInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_ClientCertPolicyResult {
    pub dwPolicyResult: ::windows::core::HRESULT,
    pub guidPolicyId: ::windows::core::GUID,
}
impl ::core::marker::Copy for SecPkgContext_ClientCertPolicyResult {}
impl ::core::clone::Clone for SecPkgContext_ClientCertPolicyResult {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_ClientCertPolicyResult {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_ClientSpecifiedTarget {
    pub sTargetName: *mut u16,
}
impl ::core::marker::Copy for SecPkgContext_ClientSpecifiedTarget {}
impl ::core::clone::Clone for SecPkgContext_ClientSpecifiedTarget {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_ClientSpecifiedTarget {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_ConnectionInfo {
    pub dwProtocol: u32,
    pub aiCipher: u32,
    pub dwCipherStrength: u32,
    pub aiHash: u32,
    pub dwHashStrength: u32,
    pub aiExch: u32,
    pub dwExchStrength: u32,
}
impl ::core::marker::Copy for SecPkgContext_ConnectionInfo {}
impl ::core::clone::Clone for SecPkgContext_ConnectionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_ConnectionInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_ConnectionInfoEx {
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub szCipher: [u16; 64],
    pub dwCipherStrength: u32,
    pub szHash: [u16; 64],
    pub dwHashStrength: u32,
    pub szExchange: [u16; 64],
    pub dwExchStrength: u32,
}
impl ::core::marker::Copy for SecPkgContext_ConnectionInfoEx {}
impl ::core::clone::Clone for SecPkgContext_ConnectionInfoEx {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_ConnectionInfoEx {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_CredInfo {
    pub CredClass: SECPKG_CRED_CLASS,
    pub IsPromptingNeeded: u32,
}
impl ::core::marker::Copy for SecPkgContext_CredInfo {}
impl ::core::clone::Clone for SecPkgContext_CredInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_CredInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_CredentialNameA {
    pub CredentialType: u32,
    pub sCredentialName: *mut i8,
}
impl ::core::marker::Copy for SecPkgContext_CredentialNameA {}
impl ::core::clone::Clone for SecPkgContext_CredentialNameA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_CredentialNameA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_CredentialNameW {
    pub CredentialType: u32,
    pub sCredentialName: *mut u16,
}
impl ::core::marker::Copy for SecPkgContext_CredentialNameW {}
impl ::core::clone::Clone for SecPkgContext_CredentialNameW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_CredentialNameW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_DceInfo {
    pub AuthzSvc: u32,
    pub pPac: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SecPkgContext_DceInfo {}
impl ::core::clone::Clone for SecPkgContext_DceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_DceInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_EapKeyBlock {
    pub rgbKeys: [u8; 128],
    pub rgbIVs: [u8; 64],
}
impl ::core::marker::Copy for SecPkgContext_EapKeyBlock {}
impl ::core::clone::Clone for SecPkgContext_EapKeyBlock {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_EapKeyBlock {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_EapPrfInfo {
    pub dwVersion: u32,
    pub cbPrfData: u32,
    pub pbPrfData: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_EapPrfInfo {}
impl ::core::clone::Clone for SecPkgContext_EapPrfInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_EapPrfInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_EarlyStart {
    pub dwEarlyStartFlags: u32,
}
impl ::core::marker::Copy for SecPkgContext_EarlyStart {}
impl ::core::clone::Clone for SecPkgContext_EarlyStart {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_EarlyStart {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_Flags {
    pub Flags: u32,
}
impl ::core::marker::Copy for SecPkgContext_Flags {}
impl ::core::clone::Clone for SecPkgContext_Flags {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_Flags {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SecPkgContext_IssuerListInfoEx {
    pub aIssuers: *mut super::super::Cryptography::CRYPT_INTEGER_BLOB,
    pub cIssuers: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SecPkgContext_IssuerListInfoEx {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SecPkgContext_IssuerListInfoEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for SecPkgContext_IssuerListInfoEx {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_KeyInfoA {
    pub sSignatureAlgorithmName: *mut i8,
    pub sEncryptAlgorithmName: *mut i8,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
    pub EncryptAlgorithm: u32,
}
impl ::core::marker::Copy for SecPkgContext_KeyInfoA {}
impl ::core::clone::Clone for SecPkgContext_KeyInfoA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_KeyInfoA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_KeyInfoW {
    pub sSignatureAlgorithmName: *mut u16,
    pub sEncryptAlgorithmName: *mut u16,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
    pub EncryptAlgorithm: u32,
}
impl ::core::marker::Copy for SecPkgContext_KeyInfoW {}
impl ::core::clone::Clone for SecPkgContext_KeyInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_KeyInfoW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_KeyingMaterial {
    pub cbKeyingMaterial: u32,
    pub pbKeyingMaterial: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_KeyingMaterial {}
impl ::core::clone::Clone for SecPkgContext_KeyingMaterial {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_KeyingMaterial {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_KeyingMaterialInfo {
    pub cbLabel: u16,
    pub pszLabel: ::windows::core::PSTR,
    pub cbContextValue: u16,
    pub pbContextValue: *mut u8,
    pub cbKeyingMaterial: u32,
}
impl ::core::marker::Copy for SecPkgContext_KeyingMaterialInfo {}
impl ::core::clone::Clone for SecPkgContext_KeyingMaterialInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_KeyingMaterialInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_KeyingMaterial_Inproc {
    pub cbLabel: u16,
    pub pszLabel: ::windows::core::PSTR,
    pub cbContextValue: u16,
    pub pbContextValue: *mut u8,
    pub cbKeyingMaterial: u32,
    pub pbKeyingMaterial: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_KeyingMaterial_Inproc {}
impl ::core::clone::Clone for SecPkgContext_KeyingMaterial_Inproc {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_KeyingMaterial_Inproc {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_LastClientTokenStatus {
    pub LastClientTokenStatus: SECPKG_ATTR_LCT_STATUS,
}
impl ::core::marker::Copy for SecPkgContext_LastClientTokenStatus {}
impl ::core::clone::Clone for SecPkgContext_LastClientTokenStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_LastClientTokenStatus {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_Lifespan {
    pub tsStart: i64,
    pub tsExpiry: i64,
}
impl ::core::marker::Copy for SecPkgContext_Lifespan {}
impl ::core::clone::Clone for SecPkgContext_Lifespan {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_Lifespan {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_LocalCredentialInfo {
    pub cbCertificateChain: u32,
    pub pbCertificateChain: *mut u8,
    pub cCertificates: u32,
    pub fFlags: u32,
    pub dwBits: u32,
}
impl ::core::marker::Copy for SecPkgContext_LocalCredentialInfo {}
impl ::core::clone::Clone for SecPkgContext_LocalCredentialInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_LocalCredentialInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_LogoffTime {
    pub tsLogoffTime: i64,
}
impl ::core::marker::Copy for SecPkgContext_LogoffTime {}
impl ::core::clone::Clone for SecPkgContext_LogoffTime {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_LogoffTime {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_MappedCredAttr {
    pub dwAttribute: u32,
    pub pvBuffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SecPkgContext_MappedCredAttr {}
impl ::core::clone::Clone for SecPkgContext_MappedCredAttr {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_MappedCredAttr {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_NamesA {
    pub sUserName: *mut i8,
}
impl ::core::marker::Copy for SecPkgContext_NamesA {}
impl ::core::clone::Clone for SecPkgContext_NamesA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_NamesA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_NamesW {
    pub sUserName: *mut u16,
}
impl ::core::marker::Copy for SecPkgContext_NamesW {}
impl ::core::clone::Clone for SecPkgContext_NamesW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_NamesW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_NativeNamesA {
    pub sClientName: *mut i8,
    pub sServerName: *mut i8,
}
impl ::core::marker::Copy for SecPkgContext_NativeNamesA {}
impl ::core::clone::Clone for SecPkgContext_NativeNamesA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_NativeNamesA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_NativeNamesW {
    pub sClientName: *mut u16,
    pub sServerName: *mut u16,
}
impl ::core::marker::Copy for SecPkgContext_NativeNamesW {}
impl ::core::clone::Clone for SecPkgContext_NativeNamesW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_NativeNamesW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_NegoKeys {
    pub KeyType: u32,
    pub KeyLength: u16,
    pub KeyValue: *mut u8,
    pub VerifyKeyType: u32,
    pub VerifyKeyLength: u16,
    pub VerifyKeyValue: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_NegoKeys {}
impl ::core::clone::Clone for SecPkgContext_NegoKeys {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_NegoKeys {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_NegoPackageInfo {
    pub PackageMask: u32,
}
impl ::core::marker::Copy for SecPkgContext_NegoPackageInfo {}
impl ::core::clone::Clone for SecPkgContext_NegoPackageInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_NegoPackageInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_NegoStatus {
    pub LastStatus: u32,
}
impl ::core::marker::Copy for SecPkgContext_NegoStatus {}
impl ::core::clone::Clone for SecPkgContext_NegoStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_NegoStatus {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_NegotiatedTlsExtensions {
    pub ExtensionsCount: u32,
    pub Extensions: *mut u16,
}
impl ::core::marker::Copy for SecPkgContext_NegotiatedTlsExtensions {}
impl ::core::clone::Clone for SecPkgContext_NegotiatedTlsExtensions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_NegotiatedTlsExtensions {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_NegotiationInfoA {
    pub PackageInfo: *mut SecPkgInfoA,
    pub NegotiationState: u32,
}
impl ::core::marker::Copy for SecPkgContext_NegotiationInfoA {}
impl ::core::clone::Clone for SecPkgContext_NegotiationInfoA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_NegotiationInfoA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_NegotiationInfoW {
    pub PackageInfo: *mut SecPkgInfoW,
    pub NegotiationState: u32,
}
impl ::core::marker::Copy for SecPkgContext_NegotiationInfoW {}
impl ::core::clone::Clone for SecPkgContext_NegotiationInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_NegotiationInfoW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_PackageInfoA {
    pub PackageInfo: *mut SecPkgInfoA,
}
impl ::core::marker::Copy for SecPkgContext_PackageInfoA {}
impl ::core::clone::Clone for SecPkgContext_PackageInfoA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_PackageInfoA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_PackageInfoW {
    pub PackageInfo: *mut SecPkgInfoW,
}
impl ::core::marker::Copy for SecPkgContext_PackageInfoW {}
impl ::core::clone::Clone for SecPkgContext_PackageInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_PackageInfoW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_PasswordExpiry {
    pub tsPasswordExpires: i64,
}
impl ::core::marker::Copy for SecPkgContext_PasswordExpiry {}
impl ::core::clone::Clone for SecPkgContext_PasswordExpiry {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_PasswordExpiry {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_ProtoInfoA {
    pub sProtocolName: *mut i8,
    pub majorVersion: u32,
    pub minorVersion: u32,
}
impl ::core::marker::Copy for SecPkgContext_ProtoInfoA {}
impl ::core::clone::Clone for SecPkgContext_ProtoInfoA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_ProtoInfoA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_ProtoInfoW {
    pub sProtocolName: *mut u16,
    pub majorVersion: u32,
    pub minorVersion: u32,
}
impl ::core::marker::Copy for SecPkgContext_ProtoInfoW {}
impl ::core::clone::Clone for SecPkgContext_ProtoInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_ProtoInfoW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_RemoteCredentialInfo {
    pub cbCertificateChain: u32,
    pub pbCertificateChain: *mut u8,
    pub cCertificates: u32,
    pub fFlags: u32,
    pub dwBits: u32,
}
impl ::core::marker::Copy for SecPkgContext_RemoteCredentialInfo {}
impl ::core::clone::Clone for SecPkgContext_RemoteCredentialInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_RemoteCredentialInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_SaslContext {
    pub SaslContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SecPkgContext_SaslContext {}
impl ::core::clone::Clone for SecPkgContext_SaslContext {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_SaslContext {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_SessionAppData {
    pub dwFlags: u32,
    pub cbAppData: u32,
    pub pbAppData: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_SessionAppData {}
impl ::core::clone::Clone for SecPkgContext_SessionAppData {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_SessionAppData {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_SessionInfo {
    pub dwFlags: u32,
    pub cbSessionId: u32,
    pub rgbSessionId: [u8; 32],
}
impl ::core::marker::Copy for SecPkgContext_SessionInfo {}
impl ::core::clone::Clone for SecPkgContext_SessionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_SessionInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_SessionKey {
    pub SessionKeyLength: u32,
    pub SessionKey: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_SessionKey {}
impl ::core::clone::Clone for SecPkgContext_SessionKey {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_SessionKey {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_Sizes {
    pub cbMaxToken: u32,
    pub cbMaxSignature: u32,
    pub cbBlockSize: u32,
    pub cbSecurityTrailer: u32,
}
impl ::core::marker::Copy for SecPkgContext_Sizes {}
impl ::core::clone::Clone for SecPkgContext_Sizes {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_Sizes {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_SrtpParameters {
    pub ProtectionProfile: u16,
    pub MasterKeyIdentifierSize: u8,
    pub MasterKeyIdentifier: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_SrtpParameters {}
impl ::core::clone::Clone for SecPkgContext_SrtpParameters {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_SrtpParameters {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_StreamSizes {
    pub cbHeader: u32,
    pub cbTrailer: u32,
    pub cbMaximumMessage: u32,
    pub cBuffers: u32,
    pub cbBlockSize: u32,
}
impl ::core::marker::Copy for SecPkgContext_StreamSizes {}
impl ::core::clone::Clone for SecPkgContext_StreamSizes {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_StreamSizes {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_SubjectAttributes {
    pub AttributeInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SecPkgContext_SubjectAttributes {}
impl ::core::clone::Clone for SecPkgContext_SubjectAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_SubjectAttributes {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_SupportedSignatures {
    pub cSignatureAndHashAlgorithms: u16,
    pub pSignatureAndHashAlgorithms: *mut u16,
}
impl ::core::marker::Copy for SecPkgContext_SupportedSignatures {}
impl ::core::clone::Clone for SecPkgContext_SupportedSignatures {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_SupportedSignatures {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_Target {
    pub TargetLength: u32,
    pub Target: ::windows::core::PSTR,
}
impl ::core::marker::Copy for SecPkgContext_Target {}
impl ::core::clone::Clone for SecPkgContext_Target {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_Target {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_TargetInformation {
    pub MarshalledTargetInfoLength: u32,
    pub MarshalledTargetInfo: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_TargetInformation {}
impl ::core::clone::Clone for SecPkgContext_TargetInformation {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_TargetInformation {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_TokenBinding {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub KeyParametersSize: u16,
    pub KeyParameters: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_TokenBinding {}
impl ::core::clone::Clone for SecPkgContext_TokenBinding {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_TokenBinding {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SecPkgContext_UiInfo {
    pub hParentWindow: super::super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SecPkgContext_UiInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SecPkgContext_UiInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SecPkgContext_UiInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgContext_UserFlags {
    pub UserFlags: u32,
}
impl ::core::marker::Copy for SecPkgContext_UserFlags {}
impl ::core::clone::Clone for SecPkgContext_UserFlags {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_UserFlags {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgCred_CipherStrengths {
    pub dwMinimumCipherStrength: u32,
    pub dwMaximumCipherStrength: u32,
}
impl ::core::marker::Copy for SecPkgCred_CipherStrengths {}
impl ::core::clone::Clone for SecPkgCred_CipherStrengths {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgCred_CipherStrengths {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SecPkgCred_ClientCertPolicy {
    pub dwFlags: u32,
    pub guidPolicyId: ::windows::core::GUID,
    pub dwCertFlags: u32,
    pub dwUrlRetrievalTimeout: u32,
    pub fCheckRevocationFreshnessTime: super::super::super::Foundation::BOOL,
    pub dwRevocationFreshnessTime: u32,
    pub fOmitUsageCheck: super::super::super::Foundation::BOOL,
    pub pwszSslCtlStoreName: ::windows::core::PWSTR,
    pub pwszSslCtlIdentifier: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SecPkgCred_ClientCertPolicy {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SecPkgCred_ClientCertPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SecPkgCred_ClientCertPolicy {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgCred_SessionTicketKey {
    pub TicketInfoVersion: u32,
    pub KeyId: [u8; 16],
    pub KeyingMaterial: [u8; 64],
    pub KeyingMaterialSize: u8,
}
impl ::core::marker::Copy for SecPkgCred_SessionTicketKey {}
impl ::core::clone::Clone for SecPkgCred_SessionTicketKey {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgCred_SessionTicketKey {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgCred_SessionTicketKeys {
    pub cSessionTicketKeys: u32,
    pub pSessionTicketKeys: *mut SecPkgCred_SessionTicketKey,
}
impl ::core::marker::Copy for SecPkgCred_SessionTicketKeys {}
impl ::core::clone::Clone for SecPkgCred_SessionTicketKeys {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgCred_SessionTicketKeys {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgCred_SupportedAlgs {
    pub cSupportedAlgs: u32,
    pub palgSupportedAlgs: *mut u32,
}
impl ::core::marker::Copy for SecPkgCred_SupportedAlgs {}
impl ::core::clone::Clone for SecPkgCred_SupportedAlgs {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgCred_SupportedAlgs {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgCred_SupportedProtocols {
    pub grbitProtocol: u32,
}
impl ::core::marker::Copy for SecPkgCred_SupportedProtocols {}
impl ::core::clone::Clone for SecPkgCred_SupportedProtocols {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgCred_SupportedProtocols {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgCredentials_Cert {
    pub EncodedCertSize: u32,
    pub EncodedCert: *mut u8,
}
impl ::core::marker::Copy for SecPkgCredentials_Cert {}
impl ::core::clone::Clone for SecPkgCredentials_Cert {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgCredentials_Cert {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgCredentials_KdcProxySettingsW {
    pub Version: u32,
    pub Flags: u32,
    pub ProxyServerOffset: u16,
    pub ProxyServerLength: u16,
    pub ClientTlsCredOffset: u16,
    pub ClientTlsCredLength: u16,
}
impl ::core::marker::Copy for SecPkgCredentials_KdcProxySettingsW {}
impl ::core::clone::Clone for SecPkgCredentials_KdcProxySettingsW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgCredentials_KdcProxySettingsW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgCredentials_NamesA {
    pub sUserName: *mut i8,
}
impl ::core::marker::Copy for SecPkgCredentials_NamesA {}
impl ::core::clone::Clone for SecPkgCredentials_NamesA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgCredentials_NamesA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgCredentials_NamesW {
    pub sUserName: *mut u16,
}
impl ::core::marker::Copy for SecPkgCredentials_NamesW {}
impl ::core::clone::Clone for SecPkgCredentials_NamesW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgCredentials_NamesW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgCredentials_SSIProviderA {
    pub sProviderName: *mut i8,
    pub ProviderInfoLength: u32,
    pub ProviderInfo: ::windows::core::PSTR,
}
impl ::core::marker::Copy for SecPkgCredentials_SSIProviderA {}
impl ::core::clone::Clone for SecPkgCredentials_SSIProviderA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgCredentials_SSIProviderA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgCredentials_SSIProviderW {
    pub sProviderName: *mut u16,
    pub ProviderInfoLength: u32,
    pub ProviderInfo: ::windows::core::PSTR,
}
impl ::core::marker::Copy for SecPkgCredentials_SSIProviderW {}
impl ::core::clone::Clone for SecPkgCredentials_SSIProviderW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgCredentials_SSIProviderW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgInfoA {
    pub fCapabilities: u32,
    pub wVersion: u16,
    pub wRPCID: u16,
    pub cbMaxToken: u32,
    pub Name: *mut i8,
    pub Comment: *mut i8,
}
impl ::core::marker::Copy for SecPkgInfoA {}
impl ::core::clone::Clone for SecPkgInfoA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgInfoA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct SecPkgInfoW {
    pub fCapabilities: u32,
    pub wVersion: u16,
    pub wRPCID: u16,
    pub cbMaxToken: u32,
    pub Name: *mut u16,
    pub Comment: *mut u16,
}
impl ::core::marker::Copy for SecPkgInfoW {}
impl ::core::clone::Clone for SecPkgInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecPkgInfoW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub struct SecurityFunctionTableA {
    pub dwVersion: u32,
    pub EnumerateSecurityPackagesA: ENUMERATE_SECURITY_PACKAGES_FN_A,
    pub QueryCredentialsAttributesA: QUERY_CREDENTIALS_ATTRIBUTES_FN_A,
    pub AcquireCredentialsHandleA: ACQUIRE_CREDENTIALS_HANDLE_FN_A,
    pub FreeCredentialsHandle: FREE_CREDENTIALS_HANDLE_FN,
    pub Reserved2: *mut ::core::ffi::c_void,
    pub InitializeSecurityContextA: INITIALIZE_SECURITY_CONTEXT_FN_A,
    pub AcceptSecurityContext: ACCEPT_SECURITY_CONTEXT_FN,
    pub CompleteAuthToken: COMPLETE_AUTH_TOKEN_FN,
    pub DeleteSecurityContext: DELETE_SECURITY_CONTEXT_FN,
    pub ApplyControlToken: APPLY_CONTROL_TOKEN_FN,
    pub QueryContextAttributesA: QUERY_CONTEXT_ATTRIBUTES_FN_A,
    pub ImpersonateSecurityContext: IMPERSONATE_SECURITY_CONTEXT_FN,
    pub RevertSecurityContext: REVERT_SECURITY_CONTEXT_FN,
    pub MakeSignature: MAKE_SIGNATURE_FN,
    pub VerifySignature: VERIFY_SIGNATURE_FN,
    pub FreeContextBuffer: FREE_CONTEXT_BUFFER_FN,
    pub QuerySecurityPackageInfoA: QUERY_SECURITY_PACKAGE_INFO_FN_A,
    pub Reserved3: *mut ::core::ffi::c_void,
    pub Reserved4: *mut ::core::ffi::c_void,
    pub ExportSecurityContext: EXPORT_SECURITY_CONTEXT_FN,
    pub ImportSecurityContextA: IMPORT_SECURITY_CONTEXT_FN_A,
    pub AddCredentialsA: ADD_CREDENTIALS_FN_A,
    pub Reserved8: *mut ::core::ffi::c_void,
    pub QuerySecurityContextToken: QUERY_SECURITY_CONTEXT_TOKEN_FN,
    pub EncryptMessage: ENCRYPT_MESSAGE_FN,
    pub DecryptMessage: DECRYPT_MESSAGE_FN,
    pub SetContextAttributesA: SET_CONTEXT_ATTRIBUTES_FN_A,
    pub SetCredentialsAttributesA: SET_CREDENTIALS_ATTRIBUTES_FN_A,
    pub ChangeAccountPasswordA: CHANGE_PASSWORD_FN_A,
    pub QueryContextAttributesExA: QUERY_CONTEXT_ATTRIBUTES_EX_FN_A,
    pub QueryCredentialsAttributesExA: QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_A,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::marker::Copy for SecurityFunctionTableA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::clone::Clone for SecurityFunctionTableA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
unsafe impl ::windows::core::Abi for SecurityFunctionTableA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub struct SecurityFunctionTableW {
    pub dwVersion: u32,
    pub EnumerateSecurityPackagesW: ENUMERATE_SECURITY_PACKAGES_FN_W,
    pub QueryCredentialsAttributesW: QUERY_CREDENTIALS_ATTRIBUTES_FN_W,
    pub AcquireCredentialsHandleW: ACQUIRE_CREDENTIALS_HANDLE_FN_W,
    pub FreeCredentialsHandle: FREE_CREDENTIALS_HANDLE_FN,
    pub Reserved2: *mut ::core::ffi::c_void,
    pub InitializeSecurityContextW: INITIALIZE_SECURITY_CONTEXT_FN_W,
    pub AcceptSecurityContext: ACCEPT_SECURITY_CONTEXT_FN,
    pub CompleteAuthToken: COMPLETE_AUTH_TOKEN_FN,
    pub DeleteSecurityContext: DELETE_SECURITY_CONTEXT_FN,
    pub ApplyControlToken: APPLY_CONTROL_TOKEN_FN,
    pub QueryContextAttributesW: QUERY_CONTEXT_ATTRIBUTES_FN_W,
    pub ImpersonateSecurityContext: IMPERSONATE_SECURITY_CONTEXT_FN,
    pub RevertSecurityContext: REVERT_SECURITY_CONTEXT_FN,
    pub MakeSignature: MAKE_SIGNATURE_FN,
    pub VerifySignature: VERIFY_SIGNATURE_FN,
    pub FreeContextBuffer: FREE_CONTEXT_BUFFER_FN,
    pub QuerySecurityPackageInfoW: QUERY_SECURITY_PACKAGE_INFO_FN_W,
    pub Reserved3: *mut ::core::ffi::c_void,
    pub Reserved4: *mut ::core::ffi::c_void,
    pub ExportSecurityContext: EXPORT_SECURITY_CONTEXT_FN,
    pub ImportSecurityContextW: IMPORT_SECURITY_CONTEXT_FN_W,
    pub AddCredentialsW: ADD_CREDENTIALS_FN_W,
    pub Reserved8: *mut ::core::ffi::c_void,
    pub QuerySecurityContextToken: QUERY_SECURITY_CONTEXT_TOKEN_FN,
    pub EncryptMessage: ENCRYPT_MESSAGE_FN,
    pub DecryptMessage: DECRYPT_MESSAGE_FN,
    pub SetContextAttributesW: SET_CONTEXT_ATTRIBUTES_FN_W,
    pub SetCredentialsAttributesW: SET_CREDENTIALS_ATTRIBUTES_FN_W,
    pub ChangeAccountPasswordW: CHANGE_PASSWORD_FN_W,
    pub QueryContextAttributesExW: QUERY_CONTEXT_ATTRIBUTES_EX_FN_W,
    pub QueryCredentialsAttributesExW: QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::marker::Copy for SecurityFunctionTableW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::clone::Clone for SecurityFunctionTableW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
unsafe impl ::windows::core::Abi for SecurityFunctionTableW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct TLS_EXTENSION_SUBSCRIPTION {
    pub ExtensionType: u16,
    pub HandshakeType: u16,
}
impl ::core::marker::Copy for TLS_EXTENSION_SUBSCRIPTION {}
impl ::core::clone::Clone for TLS_EXTENSION_SUBSCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TLS_EXTENSION_SUBSCRIPTION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct TOKENBINDING_IDENTIFIER {
    pub keyType: u8,
}
impl ::core::marker::Copy for TOKENBINDING_IDENTIFIER {}
impl ::core::clone::Clone for TOKENBINDING_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOKENBINDING_IDENTIFIER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct TOKENBINDING_KEY_TYPES {
    pub keyCount: u32,
    pub keyType: *mut TOKENBINDING_KEY_PARAMETERS_TYPE,
}
impl ::core::marker::Copy for TOKENBINDING_KEY_TYPES {}
impl ::core::clone::Clone for TOKENBINDING_KEY_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOKENBINDING_KEY_TYPES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct TOKENBINDING_RESULT_DATA {
    pub bindingType: TOKENBINDING_TYPE,
    pub identifierSize: u32,
    pub identifierData: *mut TOKENBINDING_IDENTIFIER,
    pub extensionFormat: TOKENBINDING_EXTENSION_FORMAT,
    pub extensionSize: u32,
    pub extensionData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TOKENBINDING_RESULT_DATA {}
impl ::core::clone::Clone for TOKENBINDING_RESULT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOKENBINDING_RESULT_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct TOKENBINDING_RESULT_LIST {
    pub resultCount: u32,
    pub resultData: *mut TOKENBINDING_RESULT_DATA,
}
impl ::core::marker::Copy for TOKENBINDING_RESULT_LIST {}
impl ::core::clone::Clone for TOKENBINDING_RESULT_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOKENBINDING_RESULT_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRUSTED_CONTROLLERS_INFO {
    pub Entries: u32,
    pub Names: *mut super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRUSTED_CONTROLLERS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRUSTED_CONTROLLERS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRUSTED_CONTROLLERS_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct TRUSTED_DOMAIN_AUTH_INFORMATION {
    pub IncomingAuthInfos: u32,
    pub IncomingAuthenticationInformation: *mut LSA_AUTH_INFORMATION,
    pub IncomingPreviousAuthenticationInformation: *mut LSA_AUTH_INFORMATION,
    pub OutgoingAuthInfos: u32,
    pub OutgoingAuthenticationInformation: *mut LSA_AUTH_INFORMATION,
    pub OutgoingPreviousAuthenticationInformation: *mut LSA_AUTH_INFORMATION,
}
impl ::core::marker::Copy for TRUSTED_DOMAIN_AUTH_INFORMATION {}
impl ::core::clone::Clone for TRUSTED_DOMAIN_AUTH_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRUSTED_DOMAIN_AUTH_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRUSTED_DOMAIN_FULL_INFORMATION {
    pub Information: TRUSTED_DOMAIN_INFORMATION_EX,
    pub PosixOffset: TRUSTED_POSIX_OFFSET_INFO,
    pub AuthInformation: TRUSTED_DOMAIN_AUTH_INFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRUSTED_DOMAIN_FULL_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRUSTED_DOMAIN_FULL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRUSTED_DOMAIN_FULL_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRUSTED_DOMAIN_FULL_INFORMATION2 {
    pub Information: TRUSTED_DOMAIN_INFORMATION_EX2,
    pub PosixOffset: TRUSTED_POSIX_OFFSET_INFO,
    pub AuthInformation: TRUSTED_DOMAIN_AUTH_INFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRUSTED_DOMAIN_FULL_INFORMATION2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRUSTED_DOMAIN_FULL_INFORMATION2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRUSTED_DOMAIN_FULL_INFORMATION2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRUSTED_DOMAIN_INFORMATION_EX {
    pub Name: super::super::super::Foundation::UNICODE_STRING,
    pub FlatName: super::super::super::Foundation::UNICODE_STRING,
    pub Sid: super::super::super::Foundation::PSID,
    pub TrustDirection: TRUSTED_DOMAIN_TRUST_DIRECTION,
    pub TrustType: TRUSTED_DOMAIN_TRUST_TYPE,
    pub TrustAttributes: TRUSTED_DOMAIN_TRUST_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRUSTED_DOMAIN_INFORMATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRUSTED_DOMAIN_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRUSTED_DOMAIN_INFORMATION_EX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRUSTED_DOMAIN_INFORMATION_EX2 {
    pub Name: super::super::super::Foundation::UNICODE_STRING,
    pub FlatName: super::super::super::Foundation::UNICODE_STRING,
    pub Sid: super::super::super::Foundation::PSID,
    pub TrustDirection: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub ForestTrustLength: u32,
    pub ForestTrustInfo: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRUSTED_DOMAIN_INFORMATION_EX2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRUSTED_DOMAIN_INFORMATION_EX2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRUSTED_DOMAIN_INFORMATION_EX2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRUSTED_DOMAIN_NAME_INFO {
    pub Name: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRUSTED_DOMAIN_NAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRUSTED_DOMAIN_NAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRUSTED_DOMAIN_NAME_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {
    pub SupportedEncryptionTypes: u32,
}
impl ::core::marker::Copy for TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {}
impl ::core::clone::Clone for TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRUSTED_PASSWORD_INFO {
    pub Password: super::super::super::Foundation::UNICODE_STRING,
    pub OldPassword: super::super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRUSTED_PASSWORD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRUSTED_PASSWORD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRUSTED_PASSWORD_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub struct TRUSTED_POSIX_OFFSET_INFO {
    pub Offset: u32,
}
impl ::core::marker::Copy for TRUSTED_POSIX_OFFSET_INFO {}
impl ::core::clone::Clone for TRUSTED_POSIX_OFFSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRUSTED_POSIX_OFFSET_INFO {
    type Abi = Self;
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_ALL_INFORMATION {
    pub LastLogon: i64,
    pub LastLogoff: i64,
    pub PasswordLastSet: i64,
    pub AccountExpires: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub UserName: super::super::super::Foundation::UNICODE_STRING,
    pub FullName: super::super::super::Foundation::UNICODE_STRING,
    pub HomeDirectory: super::super::super::Foundation::UNICODE_STRING,
    pub HomeDirectoryDrive: super::super::super::Foundation::UNICODE_STRING,
    pub ScriptPath: super::super::super::Foundation::UNICODE_STRING,
    pub ProfilePath: super::super::super::Foundation::UNICODE_STRING,
    pub AdminComment: super::super::super::Foundation::UNICODE_STRING,
    pub WorkStations: super::super::super::Foundation::UNICODE_STRING,
    pub UserComment: super::super::super::Foundation::UNICODE_STRING,
    pub Parameters: super::super::super::Foundation::UNICODE_STRING,
    pub LmPassword: super::super::super::Foundation::UNICODE_STRING,
    pub NtPassword: super::super::super::Foundation::UNICODE_STRING,
    pub PrivateData: super::super::super::Foundation::UNICODE_STRING,
    pub SecurityDescriptor: SR_SECURITY_DESCRIPTOR,
    pub UserId: u32,
    pub PrimaryGroupId: u32,
    pub UserAccountControl: u32,
    pub WhichFields: u32,
    pub LogonHours: LOGON_HOURS,
    pub BadPasswordCount: u16,
    pub LogonCount: u16,
    pub CountryCode: u16,
    pub CodePage: u16,
    pub LmPasswordPresent: super::super::super::Foundation::BOOLEAN,
    pub NtPasswordPresent: super::super::super::Foundation::BOOLEAN,
    pub PasswordExpired: super::super::super::Foundation::BOOLEAN,
    pub PrivateDataSensitive: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_ALL_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_ALL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_ALL_INFORMATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_PasswordManagement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
pub struct USER_SESSION_KEY {
    pub data: [super::super::super::System::PasswordManagement::CYPHER_BLOCK; 2],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::marker::Copy for USER_SESSION_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::clone::Clone for USER_SESSION_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
unsafe impl ::windows::core::Abi for USER_SESSION_KEY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct X509Certificate {
    pub Version: u32,
    pub SerialNumber: [u32; 4],
    pub SignatureAlgorithm: u32,
    pub ValidFrom: super::super::super::Foundation::FILETIME,
    pub ValidUntil: super::super::super::Foundation::FILETIME,
    pub pszIssuer: ::windows::core::PSTR,
    pub pszSubject: ::windows::core::PSTR,
    pub pPublicKey: *mut PctPublicKey,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for X509Certificate {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for X509Certificate {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for X509Certificate {
    type Abi = Self;
}
#[repr(C)]
pub struct _HMAPPER(pub u8);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type ACCEPT_SECURITY_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut super::super::Credentials::SecHandle, param2: *mut SecBufferDesc, param3: u32, param4: u32, param5: *mut super::super::Credentials::SecHandle, param6: *mut SecBufferDesc, param7: *mut u32, param8: *mut i64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type ACQUIRE_CREDENTIALS_HANDLE_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut i8, param1: *mut i8, param2: u32, param3: *mut ::core::ffi::c_void, param4: *mut ::core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut ::core::ffi::c_void, param7: *mut super::super::Credentials::SecHandle, param8: *mut i64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type ACQUIRE_CREDENTIALS_HANDLE_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut u16, param2: u32, param3: *mut ::core::ffi::c_void, param4: *mut ::core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut ::core::ffi::c_void, param7: *mut super::super::Credentials::SecHandle, param8: *mut i64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type ADD_CREDENTIALS_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut i8, param2: *mut i8, param3: u32, param4: *mut ::core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut ::core::ffi::c_void, param7: *mut i64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type ADD_CREDENTIALS_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut u16, param2: *mut u16, param3: u32, param4: *mut ::core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut ::core::ffi::c_void, param7: *mut i64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type APPLY_CONTROL_TOKEN_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut SecBufferDesc) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CHANGE_PASSWORD_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut i8, param1: *mut i8, param2: *mut i8, param3: *mut i8, param4: *mut i8, param5: super::super::super::Foundation::BOOLEAN, param6: u32, param7: *mut SecBufferDesc) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CHANGE_PASSWORD_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut u16, param2: *mut u16, param3: *mut u16, param4: *mut u16, param5: super::super::super::Foundation::BOOLEAN, param6: u32, param7: *mut SecBufferDesc) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type COMPLETE_AUTH_TOKEN_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut SecBufferDesc) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub type CredFreeCredentialsFn = ::core::option::Option<unsafe extern "system" fn(count: u32, credentials: *mut *mut ENCRYPTED_CREDENTIALW) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub type CredReadDomainCredentialsFn = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, credflags: u32, targetinfo: *const super::super::Credentials::CREDENTIAL_TARGET_INFORMATIONW, flags: u32, count: *mut u32, credential: *mut *mut *mut ENCRYPTED_CREDENTIALW) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub type CredReadFn = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, credflags: u32, targetname: ::windows::core::PCWSTR, r#type: u32, flags: u32, credential: *mut *mut ENCRYPTED_CREDENTIALW) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub type CredWriteFn = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, credflags: u32, credential: *const ENCRYPTED_CREDENTIALW, flags: u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CrediUnmarshalandDecodeStringFn = ::core::option::Option<unsafe extern "system" fn(marshaledstring: ::windows::core::PCWSTR, blob: *mut *mut u8, blobsize: *mut u32, isfailurefatal: *mut u8) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type DECRYPT_MESSAGE_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut SecBufferDesc, param2: u32, param3: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type DELETE_SECURITY_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type ENCRYPT_MESSAGE_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut SecBufferDesc, param3: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type ENUMERATE_SECURITY_PACKAGES_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut *mut SecPkgInfoA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type ENUMERATE_SECURITY_PACKAGES_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut *mut SecPkgInfoW) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type EXPORT_SECURITY_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut SecBuffer, param3: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type FREE_CONTEXT_BUFFER_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type FREE_CREDENTIALS_HANDLE_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type IMPERSONATE_SECURITY_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type IMPORT_SECURITY_CONTEXT_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut i8, param1: *mut SecBuffer, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Credentials::SecHandle) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type IMPORT_SECURITY_CONTEXT_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut SecBuffer, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Credentials::SecHandle) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type INITIALIZE_SECURITY_CONTEXT_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut super::super::Credentials::SecHandle, param2: *mut i8, param3: u32, param4: u32, param5: u32, param6: *mut SecBufferDesc, param7: u32, param8: *mut super::super::Credentials::SecHandle, param9: *mut SecBufferDesc, param10: *mut u32, param11: *mut i64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type INITIALIZE_SECURITY_CONTEXT_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut super::super::Credentials::SecHandle, param2: *mut u16, param3: u32, param4: u32, param5: u32, param6: *mut SecBufferDesc, param7: u32, param8: *mut super::super::Credentials::SecHandle, param9: *mut SecBufferDesc, param10: *mut u32, param11: *mut i64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub type INIT_SECURITY_INTERFACE_A = ::core::option::Option<unsafe extern "system" fn() -> *mut SecurityFunctionTableA>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
pub type INIT_SECURITY_INTERFACE_W = ::core::option::Option<unsafe extern "system" fn() -> *mut SecurityFunctionTableW>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspCompleteTokenFn = ::core::option::Option<unsafe extern "system" fn(contextid: usize, token: *const SecBufferDesc) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspDeleteContextFn = ::core::option::Option<unsafe extern "system" fn(contextid: usize, lsacontextid: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspGetTokenFn = ::core::option::Option<unsafe extern "system" fn(contextid: usize, impersonationtoken: *mut super::super::super::Foundation::HANDLE, rawtoken: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspInitContextFn = ::core::option::Option<unsafe extern "system" fn(contextid: usize, contextdata: *const SecBuffer, newcontextid: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type KspInitPackageFn = ::core::option::Option<unsafe extern "system" fn(functiontable: *const SECPKG_KERNEL_FUNCTIONS) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspMakeSignatureFn = ::core::option::Option<unsafe extern "system" fn(contextid: usize, fqop: u32, message: *const SecBufferDesc, messageseqno: u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspMapHandleFn = ::core::option::Option<unsafe extern "system" fn(contextid: usize, lsacontextid: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspQueryAttributesFn = ::core::option::Option<unsafe extern "system" fn(contextid: usize, attribute: u32, buffer: *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspSealMessageFn = ::core::option::Option<unsafe extern "system" fn(contextid: usize, fqop: u32, message: *const SecBufferDesc, messageseqno: u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspSerializeAuthDataFn = ::core::option::Option<unsafe extern "system" fn(pvauthdata: *const ::core::ffi::c_void, size: *mut u32, serializeddata: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspSetPagingModeFn = ::core::option::Option<unsafe extern "system" fn(pagingmode: super::super::super::Foundation::BOOLEAN) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspUnsealMessageFn = ::core::option::Option<unsafe extern "system" fn(contextid: usize, message: *const SecBufferDesc, messageseqno: u32, pfqop: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type KspVerifySignatureFn = ::core::option::Option<unsafe extern "system" fn(contextid: usize, message: *const SecBufferDesc, messageseqno: u32, pfqop: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LSA_AP_POST_LOGON_USER = ::core::option::Option<unsafe extern "system" fn(postlogonuserinfo: *const SECPKG_POST_LOGON_USER_INFO) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type MAKE_SIGNATURE_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut SecBufferDesc, param3: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PKSEC_CREATE_CONTEXT_LIST = ::core::option::Option<unsafe extern "system" fn(r#type: KSEC_CONTEXT_TYPE) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
pub type PKSEC_DEREFERENCE_LIST_ENTRY = ::core::option::Option<unsafe extern "system" fn(entry: *const KSEC_LIST_ENTRY, delete: *mut u8) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
pub type PKSEC_INSERT_LIST_ENTRY = ::core::option::Option<unsafe extern "system" fn(list: *const ::core::ffi::c_void, entry: *const KSEC_LIST_ENTRY) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PKSEC_LOCATE_PKG_BY_ID = ::core::option::Option<unsafe extern "system" fn(packageid: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PKSEC_REFERENCE_LIST_ENTRY = ::core::option::Option<unsafe extern "system" fn(entry: *const KSEC_LIST_ENTRY, signature: u32, removenoref: super::super::super::Foundation::BOOLEAN) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PKSEC_SERIALIZE_SCHANNEL_AUTH_DATA = ::core::option::Option<unsafe extern "system" fn(pvauthdata: *const ::core::ffi::c_void, size: *mut u32, serializeddata: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PKSEC_SERIALIZE_WINNT_AUTH_DATA = ::core::option::Option<unsafe extern "system" fn(pvauthdata: *const ::core::ffi::c_void, size: *mut u32, serializeddata: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PLSA_ADD_CREDENTIAL = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, authenticationpackage: u32, primarykeyvalue: *const super::super::super::System::Kernel::STRING, credentials: *const super::super::super::System::Kernel::STRING) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_ALLOCATE_CLIENT_BUFFER = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, lengthrequired: u32, clientbaseaddress: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PLSA_ALLOCATE_LSA_HEAP = ::core::option::Option<unsafe extern "system" fn(length: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PLSA_ALLOCATE_PRIVATE_HEAP = ::core::option::Option<unsafe extern "system" fn(length: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PLSA_ALLOCATE_SHARED_MEMORY = ::core::option::Option<unsafe extern "system" fn(sharedmem: *const ::core::ffi::c_void, size: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_CALL_PACKAGE = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, protocolsubmitbuffer: *const ::core::ffi::c_void, clientbufferbase: *const ::core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut ::core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_CALL_PACKAGE_PASSTHROUGH = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, protocolsubmitbuffer: *const ::core::ffi::c_void, clientbufferbase: *const ::core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut ::core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PLSA_AP_INITIALIZE_PACKAGE = ::core::option::Option<unsafe extern "system" fn(authenticationpackageid: u32, lsadispatchtable: *const LSA_DISPATCH_TABLE, database: *const super::super::super::System::Kernel::STRING, confidentiality: *const super::super::super::System::Kernel::STRING, authenticationpackagename: *mut *mut super::super::super::System::Kernel::STRING) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_LOGON_TERMINATED = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_LOGON_USER = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, logontype: SECURITY_LOGON_TYPE, authenticationinformation: *const ::core::ffi::c_void, clientauthenticationbase: *const ::core::ffi::c_void, authenticationinformationlength: u32, profilebuffer: *mut *mut ::core::ffi::c_void, profilebufferlength: *mut u32, logonid: *mut super::super::super::Foundation::LUID, substatus: *mut i32, tokeninformationtype: *mut LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *mut *mut ::core::ffi::c_void, accountname: *mut *mut super::super::super::Foundation::UNICODE_STRING, authenticatingauthority: *mut *mut super::super::super::Foundation::UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_LOGON_USER_EX = ::core::option::Option<
    unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, logontype: SECURITY_LOGON_TYPE, authenticationinformation: *const ::core::ffi::c_void, clientauthenticationbase: *const ::core::ffi::c_void, authenticationinformationlength: u32, profilebuffer: *mut *mut ::core::ffi::c_void, profilebufferlength: *mut u32, logonid: *mut super::super::super::Foundation::LUID, substatus: *mut i32, tokeninformationtype: *mut LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *mut *mut ::core::ffi::c_void, accountname: *mut *mut super::super::super::Foundation::UNICODE_STRING, authenticatingauthority: *mut *mut super::super::super::Foundation::UNICODE_STRING, machinename: *mut *mut super::super::super::Foundation::UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS,
>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_LOGON_USER_EX2 = ::core::option::Option<
    unsafe extern "system" fn(
        clientrequest: *const *const ::core::ffi::c_void,
        logontype: SECURITY_LOGON_TYPE,
        protocolsubmitbuffer: *const ::core::ffi::c_void,
        clientbufferbase: *const ::core::ffi::c_void,
        submitbuffersize: u32,
        profilebuffer: *mut *mut ::core::ffi::c_void,
        profilebuffersize: *mut u32,
        logonid: *mut super::super::super::Foundation::LUID,
        substatus: *mut i32,
        tokeninformationtype: *mut LSA_TOKEN_INFORMATION_TYPE,
        tokeninformation: *mut *mut ::core::ffi::c_void,
        accountname: *mut *mut super::super::super::Foundation::UNICODE_STRING,
        authenticatingauthority: *mut *mut super::super::super::Foundation::UNICODE_STRING,
        machinename: *mut *mut super::super::super::Foundation::UNICODE_STRING,
        primarycredentials: *mut SECPKG_PRIMARY_CRED,
        supplementalcredentials: *mut *mut SECPKG_SUPPLEMENTAL_CRED_ARRAY,
    ) -> super::super::super::Foundation::NTSTATUS,
>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_LOGON_USER_EX3 = ::core::option::Option<
    unsafe extern "system" fn(
        clientrequest: *const *const ::core::ffi::c_void,
        logontype: SECURITY_LOGON_TYPE,
        protocolsubmitbuffer: *const ::core::ffi::c_void,
        clientbufferbase: *const ::core::ffi::c_void,
        submitbuffersize: u32,
        surrogatelogon: *mut SECPKG_SURROGATE_LOGON,
        profilebuffer: *mut *mut ::core::ffi::c_void,
        profilebuffersize: *mut u32,
        logonid: *mut super::super::super::Foundation::LUID,
        substatus: *mut i32,
        tokeninformationtype: *mut LSA_TOKEN_INFORMATION_TYPE,
        tokeninformation: *mut *mut ::core::ffi::c_void,
        accountname: *mut *mut super::super::super::Foundation::UNICODE_STRING,
        authenticatingauthority: *mut *mut super::super::super::Foundation::UNICODE_STRING,
        machinename: *mut *mut super::super::super::Foundation::UNICODE_STRING,
        primarycredentials: *mut SECPKG_PRIMARY_CRED,
        supplementalcredentials: *mut *mut SECPKG_SUPPLEMENTAL_CRED_ARRAY,
    ) -> super::super::super::Foundation::NTSTATUS,
>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_POST_LOGON_USER_SURROGATE = ::core::option::Option<
    unsafe extern "system" fn(
        clientrequest: *const *const ::core::ffi::c_void,
        logontype: SECURITY_LOGON_TYPE,
        protocolsubmitbuffer: *const ::core::ffi::c_void,
        clientbufferbase: *const ::core::ffi::c_void,
        submitbuffersize: u32,
        surrogatelogon: *const SECPKG_SURROGATE_LOGON,
        profilebuffer: *const ::core::ffi::c_void,
        profilebuffersize: u32,
        logonid: *const super::super::super::Foundation::LUID,
        status: super::super::super::Foundation::NTSTATUS,
        substatus: super::super::super::Foundation::NTSTATUS,
        tokeninformationtype: LSA_TOKEN_INFORMATION_TYPE,
        tokeninformation: *const ::core::ffi::c_void,
        accountname: *const super::super::super::Foundation::UNICODE_STRING,
        authenticatingauthority: *const super::super::super::Foundation::UNICODE_STRING,
        machinename: *const super::super::super::Foundation::UNICODE_STRING,
        primarycredentials: *const SECPKG_PRIMARY_CRED,
        supplementalcredentials: *const SECPKG_SUPPLEMENTAL_CRED_ARRAY,
    ) -> super::super::super::Foundation::NTSTATUS,
>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_PRE_LOGON_USER_SURROGATE = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, logontype: SECURITY_LOGON_TYPE, protocolsubmitbuffer: *const ::core::ffi::c_void, clientbufferbase: *const ::core::ffi::c_void, submitbuffersize: u32, surrogatelogon: *mut SECPKG_SURROGATE_LOGON, substatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AUDIT_ACCOUNT_LOGON = ::core::option::Option<unsafe extern "system" fn(auditid: u32, success: super::super::super::Foundation::BOOLEAN, source: *const super::super::super::Foundation::UNICODE_STRING, clientname: *const super::super::super::Foundation::UNICODE_STRING, mappedname: *const super::super::super::Foundation::UNICODE_STRING, status: super::super::super::Foundation::NTSTATUS) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AUDIT_LOGON = ::core::option::Option<unsafe extern "system" fn(status: super::super::super::Foundation::NTSTATUS, substatus: super::super::super::Foundation::NTSTATUS, accountname: *const super::super::super::Foundation::UNICODE_STRING, authenticatingauthority: *const super::super::super::Foundation::UNICODE_STRING, workstationname: *const super::super::super::Foundation::UNICODE_STRING, usersid: super::super::super::Foundation::PSID, logontype: SECURITY_LOGON_TYPE, tokensource: *const super::super::TOKEN_SOURCE, logonid: *const super::super::super::Foundation::LUID) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AUDIT_LOGON_EX = ::core::option::Option<unsafe extern "system" fn(status: super::super::super::Foundation::NTSTATUS, substatus: super::super::super::Foundation::NTSTATUS, accountname: *const super::super::super::Foundation::UNICODE_STRING, authenticatingauthority: *const super::super::super::Foundation::UNICODE_STRING, workstationname: *const super::super::super::Foundation::UNICODE_STRING, usersid: super::super::super::Foundation::PSID, logontype: SECURITY_LOGON_TYPE, impersonationlevel: super::super::SECURITY_IMPERSONATION_LEVEL, tokensource: *const super::super::TOKEN_SOURCE, logonid: *const super::super::super::Foundation::LUID) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CALLBACK_FUNCTION = ::core::option::Option<unsafe extern "system" fn(argument1: usize, argument2: usize, inputbuffer: *mut SecBuffer, outputbuffer: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CALL_PACKAGE = ::core::option::Option<unsafe extern "system" fn(authenticationpackage: *const super::super::super::Foundation::UNICODE_STRING, protocolsubmitbuffer: *const ::core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut ::core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CALL_PACKAGEEX = ::core::option::Option<unsafe extern "system" fn(authenticationpackage: *const super::super::super::Foundation::UNICODE_STRING, clientbufferbase: *const ::core::ffi::c_void, protocolsubmitbuffer: *const ::core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut ::core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CALL_PACKAGE_PASSTHROUGH = ::core::option::Option<unsafe extern "system" fn(authenticationpackage: *const super::super::super::Foundation::UNICODE_STRING, clientbufferbase: *const ::core::ffi::c_void, protocolsubmitbuffer: *const ::core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut ::core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CANCEL_NOTIFICATION = ::core::option::Option<unsafe extern "system" fn(notifyhandle: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CHECK_PROTECTED_USER_BY_TOKEN = ::core::option::Option<unsafe extern "system" fn(usertoken: super::super::super::Foundation::HANDLE, protecteduser: *mut super::super::super::Foundation::BOOLEAN) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CLIENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callback: ::windows::core::PCSTR, argument1: usize, argument2: usize, input: *const SecBuffer, output: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CLOSE_SAM_USER = ::core::option::Option<unsafe extern "system" fn(userhandle: *const ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CONVERT_AUTH_DATA_TO_TOKEN = ::core::option::Option<unsafe extern "system" fn(userauthdata: *const ::core::ffi::c_void, userauthdatasize: u32, impersonationlevel: super::super::SECURITY_IMPERSONATION_LEVEL, tokensource: *const super::super::TOKEN_SOURCE, logontype: SECURITY_LOGON_TYPE, authorityname: *const super::super::super::Foundation::UNICODE_STRING, token: *mut super::super::super::Foundation::HANDLE, logonid: *mut super::super::super::Foundation::LUID, accountname: *mut super::super::super::Foundation::UNICODE_STRING, substatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_COPY_FROM_CLIENT_BUFFER = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, length: u32, buffertocopy: *mut ::core::ffi::c_void, clientbaseaddress: *const ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_COPY_TO_CLIENT_BUFFER = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, length: u32, clientbaseaddress: *mut ::core::ffi::c_void, buffertocopy: *const ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CRACK_SINGLE_NAME = ::core::option::Option<unsafe extern "system" fn(formatoffered: u32, performatgc: super::super::super::Foundation::BOOLEAN, nameinput: *const super::super::super::Foundation::UNICODE_STRING, prefix: *const super::super::super::Foundation::UNICODE_STRING, requestedformat: u32, crackedname: *mut super::super::super::Foundation::UNICODE_STRING, dnsdomainname: *mut super::super::super::Foundation::UNICODE_STRING, substatus: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CREATE_LOGON_SESSION = ::core::option::Option<unsafe extern "system" fn(logonid: *mut super::super::super::Foundation::LUID) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PLSA_CREATE_SHARED_MEMORY = ::core::option::Option<unsafe extern "system" fn(maxsize: u32, initialsize: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
pub type PLSA_CREATE_THREAD = ::core::option::Option<unsafe extern "system" fn(securityattributes: *const super::super::SECURITY_ATTRIBUTES, stacksize: u32, startfunction: super::super::super::System::Threading::LPTHREAD_START_ROUTINE, threadparameter: *const ::core::ffi::c_void, creationflags: u32, threadid: *mut u32) -> super::super::super::Foundation::HANDLE>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CREATE_TOKEN =
    ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, tokensource: *const super::super::TOKEN_SOURCE, logontype: SECURITY_LOGON_TYPE, impersonationlevel: super::super::SECURITY_IMPERSONATION_LEVEL, tokeninformationtype: LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *const ::core::ffi::c_void, tokengroups: *const super::super::TOKEN_GROUPS, accountname: *const super::super::super::Foundation::UNICODE_STRING, authorityname: *const super::super::super::Foundation::UNICODE_STRING, workstation: *const super::super::super::Foundation::UNICODE_STRING, profilepath: *const super::super::super::Foundation::UNICODE_STRING, token: *mut super::super::super::Foundation::HANDLE, substatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_CREATE_TOKEN_EX = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, tokensource: *const super::super::TOKEN_SOURCE, logontype: SECURITY_LOGON_TYPE, impersonationlevel: super::super::SECURITY_IMPERSONATION_LEVEL, tokeninformationtype: LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *const ::core::ffi::c_void, tokengroups: *const super::super::TOKEN_GROUPS, workstation: *const super::super::super::Foundation::UNICODE_STRING, profilepath: *const super::super::super::Foundation::UNICODE_STRING, sessioninformation: *const ::core::ffi::c_void, sessioninformationtype: SECPKG_SESSIONINFO_TYPE, token: *mut super::super::super::Foundation::HANDLE, substatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PLSA_DELETE_CREDENTIAL = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, authenticationpackage: u32, primarykeyvalue: *const super::super::super::System::Kernel::STRING) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_DELETE_LOGON_SESSION = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_DELETE_SHARED_MEMORY = ::core::option::Option<unsafe extern "system" fn(sharedmem: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_DUPLICATE_HANDLE = ::core::option::Option<unsafe extern "system" fn(sourcehandle: super::super::super::Foundation::HANDLE, destionationhandle: *mut super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_EXPAND_AUTH_DATA_FOR_DOMAIN = ::core::option::Option<unsafe extern "system" fn(userauthdata: *const u8, userauthdatasize: u32, reserved: *const ::core::ffi::c_void, expandedauthdata: *mut *mut u8, expandedauthdatasize: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_FREE_CLIENT_BUFFER = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, clientbaseaddress: *const ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PLSA_FREE_LSA_HEAP = ::core::option::Option<unsafe extern "system" fn(base: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PLSA_FREE_PRIVATE_HEAP = ::core::option::Option<unsafe extern "system" fn(base: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PLSA_FREE_SHARED_MEMORY = ::core::option::Option<unsafe extern "system" fn(sharedmem: *const ::core::ffi::c_void, memory: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_GET_APP_MODE_INFO = ::core::option::Option<unsafe extern "system" fn(userfunction: *mut u32, argument1: *mut usize, argument2: *mut usize, userdata: *mut SecBuffer, returntolsa: *mut super::super::super::Foundation::BOOLEAN) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_GET_AUTH_DATA_FOR_USER = ::core::option::Option<unsafe extern "system" fn(name: *const super::super::super::Foundation::UNICODE_STRING, nametype: SECPKG_NAME_TYPE, prefix: *const super::super::super::Foundation::UNICODE_STRING, userauthdata: *mut *mut u8, userauthdatasize: *mut u32, userflatname: *mut super::super::super::Foundation::UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_GET_CALL_INFO = ::core::option::Option<unsafe extern "system" fn(info: *mut SECPKG_CALL_INFO) -> super::super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_GET_CLIENT_INFO = ::core::option::Option<unsafe extern "system" fn(clientinfo: *mut SECPKG_CLIENT_INFO) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PLSA_GET_CREDENTIALS = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, authenticationpackage: u32, querycontext: *mut u32, retrieveallcredentials: super::super::super::Foundation::BOOLEAN, primarykeyvalue: *const super::super::super::System::Kernel::STRING, primarykeylength: *mut u32, credentials: *const super::super::super::System::Kernel::STRING) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_GET_EXTENDED_CALL_FLAGS = ::core::option::Option<unsafe extern "system" fn(flags: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_GET_SERVICE_ACCOUNT_PASSWORD = ::core::option::Option<unsafe extern "system" fn(accountname: *const super::super::super::Foundation::UNICODE_STRING, domainname: *const super::super::super::Foundation::UNICODE_STRING, credfetch: CRED_FETCH, filetimeexpiry: *mut super::super::super::Foundation::FILETIME, currentpassword: *mut super::super::super::Foundation::UNICODE_STRING, previouspassword: *mut super::super::super::Foundation::UNICODE_STRING, filetimecurrpwdvalidforoutbound: *mut super::super::super::Foundation::FILETIME) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_GET_USER_AUTH_DATA = ::core::option::Option<unsafe extern "system" fn(userhandle: *const ::core::ffi::c_void, userauthdata: *mut *mut u8, userauthdatasize: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_GET_USER_CREDENTIALS = ::core::option::Option<unsafe extern "system" fn(userhandle: *const ::core::ffi::c_void, primarycreds: *mut *mut ::core::ffi::c_void, primarycredssize: *mut u32, supplementalcreds: *mut *mut ::core::ffi::c_void, supplementalcredssize: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_IMPERSONATE_CLIENT = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PLSA_LOCATE_PKG_BY_ID = ::core::option::Option<unsafe extern "system" fn(packgeid: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_MAP_BUFFER = ::core::option::Option<unsafe extern "system" fn(inputbuffer: *const SecBuffer, outputbuffer: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_OPEN_SAM_USER = ::core::option::Option<unsafe extern "system" fn(name: *const super::super::super::Foundation::UNICODE_STRING, nametype: SECPKG_NAME_TYPE, prefix: *const super::super::super::Foundation::UNICODE_STRING, allowguest: super::super::super::Foundation::BOOLEAN, reserved: u32, userhandle: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_OPEN_TOKEN_BY_LOGON_ID = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, rettokenhandle: *mut super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PLSA_PROTECT_MEMORY = ::core::option::Option<unsafe extern "system" fn(buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_QUERY_CLIENT_REQUEST = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, querytype: u32, replybuffer: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_REDIRECTED_LOGON_CALLBACK = ::core::option::Option<unsafe extern "system" fn(redirectedlogonhandle: super::super::super::Foundation::HANDLE, buffer: *mut ::core::ffi::c_void, bufferlength: u32, returnbuffer: *mut *mut ::core::ffi::c_void, returnbufferlength: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK = ::core::option::Option<unsafe extern "system" fn(redirectedlogonhandle: super::super::super::Foundation::HANDLE) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_REDIRECTED_LOGON_GET_LOGON_CREDS = ::core::option::Option<unsafe extern "system" fn(redirectedlogonhandle: super::super::super::Foundation::HANDLE, logonbuffer: *mut *mut u8, logonbufferlength: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_REDIRECTED_LOGON_GET_SUPP_CREDS = ::core::option::Option<unsafe extern "system" fn(redirectedlogonhandle: super::super::super::Foundation::HANDLE, supplementalcredentials: *mut *mut SECPKG_SUPPLEMENTAL_CRED_ARRAY) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_REDIRECTED_LOGON_INIT = ::core::option::Option<unsafe extern "system" fn(redirectedlogonhandle: super::super::super::Foundation::HANDLE, packagename: *const super::super::super::Foundation::UNICODE_STRING, sessionid: u32, logonid: *const super::super::super::Foundation::LUID) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_REGISTER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackid: u32, callback: PLSA_CALLBACK_FUNCTION) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
pub type PLSA_REGISTER_NOTIFICATION = ::core::option::Option<unsafe extern "system" fn(startfunction: super::super::super::System::Threading::LPTHREAD_START_ROUTINE, parameter: *const ::core::ffi::c_void, notificationtype: u32, notificationclass: u32, notificationflags: u32, intervalminutes: u32, waitevent: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::HANDLE>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_SAVE_SUPPLEMENTAL_CREDENTIALS = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, supplementalcredsize: u32, supplementalcreds: *const ::core::ffi::c_void, synchronous: super::super::super::Foundation::BOOLEAN) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_SET_APP_MODE_INFO = ::core::option::Option<unsafe extern "system" fn(userfunction: u32, argument1: usize, argument2: usize, userdata: *const SecBuffer, returntolsa: super::super::super::Foundation::BOOLEAN) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_UNLOAD_PACKAGE = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_UPDATE_PRIMARY_CREDENTIALS = ::core::option::Option<unsafe extern "system" fn(primarycredentials: *const SECPKG_PRIMARY_CRED, credentials: *const SECPKG_SUPPLEMENTAL_CRED_ARRAY) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type PSAM_CREDENTIAL_UPDATE_FREE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(p: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSAM_CREDENTIAL_UPDATE_NOTIFY_ROUTINE = ::core::option::Option<unsafe extern "system" fn(clearpassword: *const super::super::super::Foundation::UNICODE_STRING, oldcredentials: *const ::core::ffi::c_void, oldcredentialsize: u32, useraccountcontrol: u32, upn: *const super::super::super::Foundation::UNICODE_STRING, username: *const super::super::super::Foundation::UNICODE_STRING, netbiosdomainname: *const super::super::super::Foundation::UNICODE_STRING, dnsdomainname: *const super::super::super::Foundation::UNICODE_STRING, newcredentials: *mut *mut ::core::ffi::c_void, newcredentialsize: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSAM_CREDENTIAL_UPDATE_REGISTER_MAPPED_ENTRYPOINTS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(table: *mut SAM_REGISTER_MAPPING_TABLE) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSAM_CREDENTIAL_UPDATE_REGISTER_ROUTINE = ::core::option::Option<unsafe extern "system" fn(credentialname: *mut super::super::super::Foundation::UNICODE_STRING) -> super::super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSAM_INIT_NOTIFICATION_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSAM_PASSWORD_FILTER_ROUTINE = ::core::option::Option<unsafe extern "system" fn(accountname: *const super::super::super::Foundation::UNICODE_STRING, fullname: *const super::super::super::Foundation::UNICODE_STRING, password: *const super::super::super::Foundation::UNICODE_STRING, setoperation: super::super::super::Foundation::BOOLEAN) -> super::super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSAM_PASSWORD_NOTIFICATION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(username: *mut super::super::super::Foundation::UNICODE_STRING, relativeid: u32, newpassword: *mut super::super::super::Foundation::UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CONTEXT_ATTRIBUTES_EX_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CONTEXT_ATTRIBUTES_EX_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CONTEXT_ATTRIBUTES_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CONTEXT_ATTRIBUTES_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_SECURITY_CONTEXT_TOKEN_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type QUERY_SECURITY_PACKAGE_INFO_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut i8, param1: *mut *mut SecPkgInfoA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type QUERY_SECURITY_PACKAGE_INFO_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut *mut SecPkgInfoW) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type REVERT_SECURITY_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type SEC_GET_KEY_FN = ::core::option::Option<unsafe extern "system" fn(arg: *mut ::core::ffi::c_void, principal: *mut ::core::ffi::c_void, keyver: u32, key: *mut *mut ::core::ffi::c_void, status: *mut ::windows::core::HRESULT) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type SET_CONTEXT_ATTRIBUTES_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type SET_CONTEXT_ATTRIBUTES_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type SET_CREDENTIALS_ATTRIBUTES_FN_A = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type SET_CREDENTIALS_ATTRIBUTES_FN_W = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SSL_CRACK_CERTIFICATE_FN = ::core::option::Option<unsafe extern "system" fn(pbcertificate: *mut u8, cbcertificate: u32, verifysignature: super::super::super::Foundation::BOOL, ppcertificate: *mut *mut X509Certificate) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SSL_EMPTY_CACHE_FN_A = ::core::option::Option<unsafe extern "system" fn(psztargetname: ::windows::core::PCSTR, dwflags: u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SSL_EMPTY_CACHE_FN_W = ::core::option::Option<unsafe extern "system" fn(psztargetname: ::windows::core::PCWSTR, dwflags: u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SSL_FREE_CERTIFICATE_FN = ::core::option::Option<unsafe extern "system" fn(pcertificate: *mut X509Certificate) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpAcceptCredentialsFn = ::core::option::Option<unsafe extern "system" fn(logontype: SECURITY_LOGON_TYPE, accountname: *const super::super::super::Foundation::UNICODE_STRING, primarycredentials: *const SECPKG_PRIMARY_CRED, supplementalcredentials: *const SECPKG_SUPPLEMENTAL_CRED) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpAcceptLsaModeContextFn = ::core::option::Option<unsafe extern "system" fn(credentialhandle: usize, contexthandle: usize, inputbuffer: *const SecBufferDesc, contextrequirements: u32, targetdatarep: u32, newcontexthandle: *mut usize, outputbuffer: *mut SecBufferDesc, contextattributes: *mut u32, expirationtime: *mut i64, mappedcontext: *mut super::super::super::Foundation::BOOLEAN, contextdata: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpAcquireCredentialsHandleFn = ::core::option::Option<unsafe extern "system" fn(principalname: *const super::super::super::Foundation::UNICODE_STRING, credentialuseflags: u32, logonid: *const super::super::super::Foundation::LUID, authorizationdata: *const ::core::ffi::c_void, getkeyfunciton: *const ::core::ffi::c_void, getkeyargument: *const ::core::ffi::c_void, credentialhandle: *mut usize, expirationtime: *mut i64) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpAddCredentialsFn = ::core::option::Option<unsafe extern "system" fn(credentialhandle: usize, principalname: *const super::super::super::Foundation::UNICODE_STRING, package: *const super::super::super::Foundation::UNICODE_STRING, credentialuseflags: u32, authorizationdata: *const ::core::ffi::c_void, getkeyfunciton: *const ::core::ffi::c_void, getkeyargument: *const ::core::ffi::c_void, expirationtime: *mut i64) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpApplyControlTokenFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, controltoken: *const SecBufferDesc) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpChangeAccountPasswordFn = ::core::option::Option<unsafe extern "system" fn(pdomainname: *const super::super::super::Foundation::UNICODE_STRING, paccountname: *const super::super::super::Foundation::UNICODE_STRING, poldpassword: *const super::super::super::Foundation::UNICODE_STRING, pnewpassword: *const super::super::super::Foundation::UNICODE_STRING, impersonating: super::super::super::Foundation::BOOLEAN, poutput: *mut SecBufferDesc) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpCompleteAuthTokenFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, inputbuffer: *const SecBufferDesc) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpDeleteContextFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpDeleteCredentialsFn = ::core::option::Option<unsafe extern "system" fn(credentialhandle: usize, key: *const SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpExchangeMetaDataFn = ::core::option::Option<unsafe extern "system" fn(credentialhandle: usize, targetname: *const super::super::super::Foundation::UNICODE_STRING, contextrequirements: u32, metadatalength: u32, metadata: *const u8, contexthandle: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpExportSecurityContextFn = ::core::option::Option<unsafe extern "system" fn(phcontext: usize, fflags: u32, ppackedcontext: *mut SecBuffer, ptoken: *mut super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpFormatCredentialsFn = ::core::option::Option<unsafe extern "system" fn(credentials: *const SecBuffer, formattedcredentials: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpFreeCredentialsHandleFn = ::core::option::Option<unsafe extern "system" fn(credentialhandle: usize) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpGetContextTokenFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, impersonationtoken: *mut super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpGetCredUIContextFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, credtype: *const ::windows::core::GUID, flatcreduicontextlength: *mut u32, flatcreduicontext: *mut *mut u8) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpGetCredentialsFn = ::core::option::Option<unsafe extern "system" fn(credentialhandle: usize, credentials: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpGetExtendedInformationFn = ::core::option::Option<unsafe extern "system" fn(class: SECPKG_EXTENDED_INFORMATION_CLASS, ppinformation: *mut *mut SECPKG_EXTENDED_INFORMATION) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpGetInfoFn = ::core::option::Option<unsafe extern "system" fn(packageinfo: *mut SecPkgInfoA) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpGetRemoteCredGuardLogonBufferFn = ::core::option::Option<unsafe extern "system" fn(credhandle: usize, contexthandle: usize, targetname: *const super::super::super::Foundation::UNICODE_STRING, redirectedlogonhandle: *mut super::super::super::Foundation::HANDLE, callback: *mut PLSA_REDIRECTED_LOGON_CALLBACK, cleanupcallback: *mut PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK, logonbuffersize: *mut u32, logonbuffer: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpGetRemoteCredGuardSupplementalCredsFn = ::core::option::Option<unsafe extern "system" fn(credhandle: usize, targetname: *const super::super::super::Foundation::UNICODE_STRING, redirectedlogonhandle: *mut super::super::super::Foundation::HANDLE, callback: *mut PLSA_REDIRECTED_LOGON_CALLBACK, cleanupcallback: *mut PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK, supplementalcredssize: *mut u32, supplementalcreds: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpGetTbalSupplementalCredsFn = ::core::option::Option<unsafe extern "system" fn(logonid: super::super::super::Foundation::LUID, supplementalcredssize: *mut u32, supplementalcreds: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpGetUserInfoFn = ::core::option::Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, flags: u32, userdata: *mut *mut SECURITY_USER_DATA) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpImportSecurityContextFn = ::core::option::Option<unsafe extern "system" fn(ppackedcontext: *const SecBuffer, token: super::super::super::Foundation::HANDLE, phcontext: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpInitLsaModeContextFn = ::core::option::Option<unsafe extern "system" fn(credentialhandle: usize, contexthandle: usize, targetname: *const super::super::super::Foundation::UNICODE_STRING, contextrequirements: u32, targetdatarep: u32, inputbuffers: *const SecBufferDesc, newcontexthandle: *mut usize, outputbuffers: *mut SecBufferDesc, contextattributes: *mut u32, expirationtime: *mut i64, mappedcontext: *mut super::super::super::Foundation::BOOLEAN, contextdata: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpInitUserModeContextFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, packedcontext: *const SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
pub type SpInitializeFn = ::core::option::Option<unsafe extern "system" fn(packageid: usize, parameters: *const SECPKG_PARAMETERS, functiontable: *const LSA_SECPKG_FUNCTION_TABLE) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpInstanceInitFn = ::core::option::Option<unsafe extern "system" fn(version: u32, functiontable: *const SECPKG_DLL_FUNCTIONS, userfunctions: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Credentials\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
pub type SpLsaModeInitializeFn = ::core::option::Option<unsafe extern "system" fn(lsaversion: u32, packageversion: *mut u32, pptables: *mut *mut SECPKG_FUNCTION_TABLE, pctables: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpMakeSignatureFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, qualityofprotection: u32, messagebuffers: *const SecBufferDesc, messagesequencenumber: u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpMarshalAttributeDataFn = ::core::option::Option<unsafe extern "system" fn(attributeinfo: u32, attribute: u32, attributedatasize: u32, attributedata: *const u8, marshaledattributedatasize: *mut u32, marshaledattributedata: *mut *mut u8) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpMarshallSupplementalCredsFn = ::core::option::Option<unsafe extern "system" fn(credentialsize: u32, credentials: *const u8, marshalledcredsize: *mut u32, marshalledcreds: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpQueryContextAttributesFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, contextattribute: u32, buffer: *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpQueryCredentialsAttributesFn = ::core::option::Option<unsafe extern "system" fn(credentialhandle: usize, credentialattribute: u32, buffer: *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpQueryMetaDataFn = ::core::option::Option<unsafe extern "system" fn(credentialhandle: usize, targetname: *const super::super::super::Foundation::UNICODE_STRING, contextrequirements: u32, metadatalength: *mut u32, metadata: *mut *mut u8, contexthandle: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpSaveCredentialsFn = ::core::option::Option<unsafe extern "system" fn(credentialhandle: usize, credentials: *const SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpSealMessageFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, qualityofprotection: u32, messagebuffers: *const SecBufferDesc, messagesequencenumber: u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpSetContextAttributesFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, contextattribute: u32, buffer: *const ::core::ffi::c_void, buffersize: u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpSetCredentialsAttributesFn = ::core::option::Option<unsafe extern "system" fn(credentialhandle: usize, credentialattribute: u32, buffer: *const ::core::ffi::c_void, buffersize: u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpSetExtendedInformationFn = ::core::option::Option<unsafe extern "system" fn(class: SECPKG_EXTENDED_INFORMATION_CLASS, info: *const SECPKG_EXTENDED_INFORMATION) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpShutdownFn = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpUnsealMessageFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, messagebuffers: *const SecBufferDesc, messagesequencenumber: u32, qualityofprotection: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpUpdateCredentialsFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, credtype: *const ::windows::core::GUID, flatcreduicontextlength: u32, flatcreduicontext: *const u8) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpUserModeInitializeFn = ::core::option::Option<unsafe extern "system" fn(lsaversion: u32, packageversion: *mut u32, pptables: *mut *mut SECPKG_USER_FUNCTION_TABLE, pctables: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpValidateTargetInfoFn = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, protocolsubmitbuffer: *const ::core::ffi::c_void, clientbufferbase: *const ::core::ffi::c_void, submitbufferlength: u32, targetinfo: *const SECPKG_TARGETINFO) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SpVerifySignatureFn = ::core::option::Option<unsafe extern "system" fn(contexthandle: usize, messagebuffers: *const SecBufferDesc, messagesequencenumber: u32, qualityofprotection: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type SslGetExtensionsFn = ::core::option::Option<unsafe extern "system" fn(clienthello: *const u8, clienthellobytesize: u32, genericextensions: *mut SCH_EXTENSION_DATA, genericextensionscount: u8, bytestoread: *mut u32, flags: SchGetExtensionsOptions) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`*"]
pub type SslGetServerIdentityFn = ::core::option::Option<unsafe extern "system" fn(clienthello: *const u8, clienthellosize: u32, serveridentity: *mut *mut u8, serveridentitysize: *mut u32, flags: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Credentials\"`*"]
#[cfg(feature = "Win32_Security_Credentials")]
pub type VERIFY_SIGNATURE_FN = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut SecBufferDesc, param2: u32, param3: *mut u32) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
