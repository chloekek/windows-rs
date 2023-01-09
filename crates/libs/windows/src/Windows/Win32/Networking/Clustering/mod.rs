#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn AddClusterGroupDependency(hdependentgroup: *const _HGROUP, hprovidergroup: *const _HGROUP) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn AddClusterGroupDependency ( hdependentgroup : *const _HGROUP , hprovidergroup : *const _HGROUP ) -> u32 );
    AddClusterGroupDependency(hdependentgroup, hprovidergroup)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn AddClusterGroupSetDependency(hdependentgroupset: *const _HGROUPSET, hprovidergroupset: *const _HGROUPSET) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn AddClusterGroupSetDependency ( hdependentgroupset : *const _HGROUPSET , hprovidergroupset : *const _HGROUPSET ) -> u32 );
    AddClusterGroupSetDependency(hdependentgroupset, hprovidergroupset)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn AddClusterGroupToGroupSetDependency(hdependentgroup: *const _HGROUP, hprovidergroupset: *const _HGROUPSET) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn AddClusterGroupToGroupSetDependency ( hdependentgroup : *const _HGROUP , hprovidergroupset : *const _HGROUPSET ) -> u32 );
    AddClusterGroupToGroupSetDependency(hdependentgroup, hprovidergroupset)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddClusterNode<P0>(hcluster: *const _HCLUSTER, lpsznodename: P0, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut _HNODE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn AddClusterNode ( hcluster : *const _HCLUSTER , lpsznodename : :: windows::core::PCWSTR , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void ) -> *mut _HNODE );
    AddClusterNode(hcluster, lpsznodename.into().abi(), pfnprogresscallback, ::core::mem::transmute(pvcallbackarg.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddClusterNodeEx<P0>(hcluster: *const _HCLUSTER, lpsznodename: P0, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut _HNODE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn AddClusterNodeEx ( hcluster : *const _HCLUSTER , lpsznodename : :: windows::core::PCWSTR , dwflags : u32 , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void ) -> *mut _HNODE );
    AddClusterNodeEx(hcluster, lpsznodename.into().abi(), dwflags, pfnprogresscallback, ::core::mem::transmute(pvcallbackarg.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn AddClusterResourceDependency(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn AddClusterResourceDependency ( hresource : *const _HRESOURCE , hdependson : *const _HRESOURCE ) -> u32 );
    AddClusterResourceDependency(hresource, hdependson)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn AddClusterResourceNode(hresource: *const _HRESOURCE, hnode: *const _HNODE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn AddClusterResourceNode ( hresource : *const _HRESOURCE , hnode : *const _HNODE ) -> u32 );
    AddClusterResourceNode(hresource, hnode)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddClusterStorageNode<P0, P1, P2>(hcluster: *const _HCLUSTER, lpsznodename: P0, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: ::core::option::Option<*const ::core::ffi::c_void>, lpszclusterstoragenodedescription: P1, lpszclusterstoragenodelocation: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn AddClusterStorageNode ( hcluster : *const _HCLUSTER , lpsznodename : :: windows::core::PCWSTR , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void , lpszclusterstoragenodedescription : :: windows::core::PCWSTR , lpszclusterstoragenodelocation : :: windows::core::PCWSTR ) -> u32 );
    AddClusterStorageNode(hcluster, lpsznodename.into().abi(), pfnprogresscallback, ::core::mem::transmute(pvcallbackarg.unwrap_or(::std::ptr::null())), lpszclusterstoragenodedescription.into().abi(), lpszclusterstoragenodelocation.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn AddCrossClusterGroupSetDependency<P0, P1>(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: P0, lpremotegroupsetname: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn AddCrossClusterGroupSetDependency ( hdependentgroupset : *const _HGROUPSET , lpremoteclustername : :: windows::core::PCWSTR , lpremotegroupsetname : :: windows::core::PCWSTR ) -> u32 );
    AddCrossClusterGroupSetDependency(hdependentgroupset, lpremoteclustername.into().abi(), lpremotegroupsetname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn AddResourceToClusterSharedVolumes(hresource: *const _HRESOURCE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn AddResourceToClusterSharedVolumes ( hresource : *const _HRESOURCE ) -> u32 );
    AddResourceToClusterSharedVolumes(hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn BackupClusterDatabase<P0>(hcluster: *const _HCLUSTER, lpszpathname: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn BackupClusterDatabase ( hcluster : *const _HCLUSTER , lpszpathname : :: windows::core::PCWSTR ) -> u32 );
    BackupClusterDatabase(hcluster, lpszpathname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CanResourceBeDependent(hresource: *const _HRESOURCE, hresourcedependent: *const _HRESOURCE) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "clusapi.dll""system" fn CanResourceBeDependent ( hresource : *const _HRESOURCE , hresourcedependent : *const _HRESOURCE ) -> super::super::Foundation:: BOOL );
    CanResourceBeDependent(hresource, hresourcedependent)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn CancelClusterGroupOperation(hgroup: *const _HGROUP, dwcancelflags_reserved: u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn CancelClusterGroupOperation ( hgroup : *const _HGROUP , dwcancelflags_reserved : u32 ) -> u32 );
    CancelClusterGroupOperation(hgroup, dwcancelflags_reserved)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ChangeClusterResourceGroup(hresource: *const _HRESOURCE, hgroup: *const _HGROUP) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ChangeClusterResourceGroup ( hresource : *const _HRESOURCE , hgroup : *const _HGROUP ) -> u32 );
    ChangeClusterResourceGroup(hresource, hgroup)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ChangeClusterResourceGroupEx(hresource: *const _HRESOURCE, hgroup: *const _HGROUP, flags: u64) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ChangeClusterResourceGroupEx ( hresource : *const _HRESOURCE , hgroup : *const _HGROUP , flags : u64 ) -> u32 );
    ChangeClusterResourceGroupEx(hresource, hgroup, flags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseCluster(hcluster: *const _HCLUSTER) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "clusapi.dll""system" fn CloseCluster ( hcluster : *const _HCLUSTER ) -> super::super::Foundation:: BOOL );
    CloseCluster(hcluster)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn CloseClusterCryptProvider(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn CloseClusterCryptProvider ( hcluscryptprovider : *const _HCLUSCRYPTPROVIDER ) -> u32 );
    CloseClusterCryptProvider(hcluscryptprovider)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseClusterGroup(hgroup: *const _HGROUP) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "clusapi.dll""system" fn CloseClusterGroup ( hgroup : *const _HGROUP ) -> super::super::Foundation:: BOOL );
    CloseClusterGroup(hgroup)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseClusterGroupSet(hgroupset: *const _HGROUPSET) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "clusapi.dll""system" fn CloseClusterGroupSet ( hgroupset : *const _HGROUPSET ) -> super::super::Foundation:: BOOL );
    CloseClusterGroupSet(hgroupset)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseClusterNetInterface(hnetinterface: *const _HNETINTERFACE) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "clusapi.dll""system" fn CloseClusterNetInterface ( hnetinterface : *const _HNETINTERFACE ) -> super::super::Foundation:: BOOL );
    CloseClusterNetInterface(hnetinterface)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseClusterNetwork(hnetwork: *const _HNETWORK) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "clusapi.dll""system" fn CloseClusterNetwork ( hnetwork : *const _HNETWORK ) -> super::super::Foundation:: BOOL );
    CloseClusterNetwork(hnetwork)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseClusterNode(hnode: *const _HNODE) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "clusapi.dll""system" fn CloseClusterNode ( hnode : *const _HNODE ) -> super::super::Foundation:: BOOL );
    CloseClusterNode(hnode)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseClusterNotifyPort(hchange: *const _HCHANGE) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "clusapi.dll""system" fn CloseClusterNotifyPort ( hchange : *const _HCHANGE ) -> super::super::Foundation:: BOOL );
    CloseClusterNotifyPort(hchange)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseClusterResource(hresource: *const _HRESOURCE) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "clusapi.dll""system" fn CloseClusterResource ( hresource : *const _HRESOURCE ) -> super::super::Foundation:: BOOL );
    CloseClusterResource(hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusAddClusterHealthFault(hcluster: *const _HCLUSTER, failure: *const CLUSTER_HEALTH_FAULT, param2: u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ClusAddClusterHealthFault ( hcluster : *const _HCLUSTER , failure : *const CLUSTER_HEALTH_FAULT , param2 : u32 ) -> u32 );
    ClusAddClusterHealthFault(hcluster, failure, param2)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusGetClusterHealthFaults(hcluster: *const _HCLUSTER, objects: *mut CLUSTER_HEALTH_FAULT_ARRAY, flags: u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ClusGetClusterHealthFaults ( hcluster : *const _HCLUSTER , objects : *mut CLUSTER_HEALTH_FAULT_ARRAY , flags : u32 ) -> u32 );
    ClusGetClusterHealthFaults(hcluster, objects, flags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusRemoveClusterHealthFault<P0>(hcluster: *const _HCLUSTER, id: P0, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ClusRemoveClusterHealthFault ( hcluster : *const _HCLUSTER , id : :: windows::core::PCWSTR , flags : u32 ) -> u32 );
    ClusRemoveClusterHealthFault(hcluster, id.into().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClusWorkerCheckTerminate(lpworker: *mut CLUS_WORKER) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "resutils.dll""system" fn ClusWorkerCheckTerminate ( lpworker : *mut CLUS_WORKER ) -> super::super::Foundation:: BOOL );
    ClusWorkerCheckTerminate(lpworker)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClusWorkerCreate(lpworker: *mut CLUS_WORKER, lpstartaddress: PWORKER_START_ROUTINE, lpparameter: *mut ::core::ffi::c_void) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ClusWorkerCreate ( lpworker : *mut CLUS_WORKER , lpstartaddress : PWORKER_START_ROUTINE , lpparameter : *mut ::core::ffi::c_void ) -> u32 );
    ClusWorkerCreate(lpworker, lpstartaddress, lpparameter)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClusWorkerTerminate(lpworker: *const CLUS_WORKER) {
    ::windows::core::link ! ( "resutils.dll""system" fn ClusWorkerTerminate ( lpworker : *const CLUS_WORKER ) -> ( ) );
    ClusWorkerTerminate(lpworker)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClusWorkerTerminateEx<P0>(clusworker: *mut CLUS_WORKER, timeoutinmilliseconds: u32, waitonly: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ClusWorkerTerminateEx ( clusworker : *mut CLUS_WORKER , timeoutinmilliseconds : u32 , waitonly : super::super::Foundation:: BOOL ) -> u32 );
    ClusWorkerTerminateEx(clusworker, timeoutinmilliseconds, waitonly.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClusWorkersTerminate<P0>(clusworkers: &mut [*mut CLUS_WORKER], timeoutinmilliseconds: u32, waitonly: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ClusWorkersTerminate ( clusworkers : *mut *mut CLUS_WORKER , clusworkerscount : usize , timeoutinmilliseconds : u32 , waitonly : super::super::Foundation:: BOOL ) -> u32 );
    ClusWorkersTerminate(::core::mem::transmute(clusworkers.as_ptr()), clusworkers.len() as _, timeoutinmilliseconds, waitonly.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterAddGroupToAffinityRule<P0>(hcluster: *const _HCLUSTER, rulename: P0, hgroup: *const _HGROUP) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterAddGroupToAffinityRule ( hcluster : *const _HCLUSTER , rulename : :: windows::core::PCWSTR , hgroup : *const _HGROUP ) -> u32 );
    ClusterAddGroupToAffinityRule(hcluster, rulename.into().abi(), hgroup)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterAddGroupToGroupSet(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterAddGroupToGroupSet ( hgroupset : *const _HGROUPSET , hgroup : *const _HGROUP ) -> u32 );
    ClusterAddGroupToGroupSet(hgroupset, hgroup)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterAddGroupToGroupSetWithDomains(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP, faultdomain: u32, updatedomain: u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterAddGroupToGroupSetWithDomains ( hgroupset : *const _HGROUPSET , hgroup : *const _HGROUP , faultdomain : u32 , updatedomain : u32 ) -> u32 );
    ClusterAddGroupToGroupSetWithDomains(hgroupset, hgroup, faultdomain, updatedomain)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterAffinityRuleControl<P0>(hcluster: *const _HCLUSTER, affinityrulename: P0, hhostnode: ::core::option::Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterAffinityRuleControl ( hcluster : *const _HCLUSTER , affinityrulename : :: windows::core::PCWSTR , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , cbinbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , cboutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
    ClusterAffinityRuleControl(hcluster, affinityrulename.into().abi(), ::core::mem::transmute(hhostnode.unwrap_or(::std::ptr::null())), dwcontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), cbinbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), cboutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterClearBackupStateForSharedVolume<P0>(lpszvolumepathname: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ClusterClearBackupStateForSharedVolume ( lpszvolumepathname : :: windows::core::PCWSTR ) -> u32 );
    ClusterClearBackupStateForSharedVolume(lpszvolumepathname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterCloseEnum(henum: *const _HCLUSENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterCloseEnum ( henum : *const _HCLUSENUM ) -> u32 );
    ClusterCloseEnum(henum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterCloseEnumEx(hclusterenum: *const _HCLUSENUMEX) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterCloseEnumEx ( hclusterenum : *const _HCLUSENUMEX ) -> u32 );
    ClusterCloseEnumEx(hclusterenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterControl(hcluster: *const _HCLUSTER, hhostnode: ::core::option::Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterControl ( hcluster : *const _HCLUSTER , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
    ClusterControl(hcluster, ::core::mem::transmute(hhostnode.unwrap_or(::std::ptr::null())), dwcontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), ninbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), noutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterCreateAffinityRule<P0>(hcluster: *const _HCLUSTER, rulename: P0, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterCreateAffinityRule ( hcluster : *const _HCLUSTER , rulename : :: windows::core::PCWSTR , ruletype : CLUS_AFFINITY_RULE_TYPE ) -> u32 );
    ClusterCreateAffinityRule(hcluster, rulename.into().abi(), ruletype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterDecrypt(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pcryptinput: *const u8, cbcryptinput: u32, ppcryptoutput: *mut *mut u8, pcbcryptoutput: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ClusterDecrypt ( hcluscryptprovider : *const _HCLUSCRYPTPROVIDER , pcryptinput : *const u8 , cbcryptinput : u32 , ppcryptoutput : *mut *mut u8 , pcbcryptoutput : *mut u32 ) -> u32 );
    ClusterDecrypt(hcluscryptprovider, pcryptinput, cbcryptinput, ppcryptoutput, pcbcryptoutput)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterEncrypt(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pdata: &[u8], ppdata: *mut *mut u8, pcbdata: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ClusterEncrypt ( hcluscryptprovider : *const _HCLUSCRYPTPROVIDER , pdata : *const u8 , cbdata : u32 , ppdata : *mut *mut u8 , pcbdata : *mut u32 ) -> u32 );
    ClusterEncrypt(hcluscryptprovider, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, ppdata, pcbdata)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterEnum(henum: *const _HCLUSENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterEnum ( henum : *const _HCLUSENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszname : :: windows::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
    ClusterEnum(henum, dwindex, lpdwtype, ::core::mem::transmute(lpszname), lpcchname)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterEnumEx(hclusterenum: *const _HCLUSENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterEnumEx ( hclusterenum : *const _HCLUSENUMEX , dwindex : u32 , pitem : *mut CLUSTER_ENUM_ITEM , cbitem : *mut u32 ) -> u32 );
    ClusterEnumEx(hclusterenum, dwindex, pitem, cbitem)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGetEnumCount(henum: *const _HCLUSENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGetEnumCount ( henum : *const _HCLUSENUM ) -> u32 );
    ClusterGetEnumCount(henum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGetEnumCountEx(hclusterenum: *const _HCLUSENUMEX) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGetEnumCountEx ( hclusterenum : *const _HCLUSENUMEX ) -> u32 );
    ClusterGetEnumCountEx(hclusterenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClusterGetVolumeNameForVolumeMountPoint<P0>(lpszvolumemountpoint: P0, lpszvolumename: ::windows::core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ClusterGetVolumeNameForVolumeMountPoint ( lpszvolumemountpoint : :: windows::core::PCWSTR , lpszvolumename : :: windows::core::PWSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    ClusterGetVolumeNameForVolumeMountPoint(lpszvolumemountpoint.into().abi(), ::core::mem::transmute(lpszvolumename), cchbufferlength)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClusterGetVolumePathName<P0>(lpszfilename: P0, lpszvolumepathname: ::windows::core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ClusterGetVolumePathName ( lpszfilename : :: windows::core::PCWSTR , lpszvolumepathname : :: windows::core::PWSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    ClusterGetVolumePathName(lpszfilename.into().abi(), ::core::mem::transmute(lpszvolumepathname), cchbufferlength)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupCloseEnum(hgroupenum: *const _HGROUPENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupCloseEnum ( hgroupenum : *const _HGROUPENUM ) -> u32 );
    ClusterGroupCloseEnum(hgroupenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupCloseEnumEx(hgroupenumex: *const _HGROUPENUMEX) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupCloseEnumEx ( hgroupenumex : *const _HGROUPENUMEX ) -> u32 );
    ClusterGroupCloseEnumEx(hgroupenumex)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupControl(hgroup: *const _HGROUP, hhostnode: ::core::option::Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupControl ( hgroup : *const _HGROUP , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
    ClusterGroupControl(hgroup, ::core::mem::transmute(hhostnode.unwrap_or(::std::ptr::null())), dwcontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), ninbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), noutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupEnum(hgroupenum: *const _HGROUPENUM, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupEnum ( hgroupenum : *const _HGROUPENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszresourcename : :: windows::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
    ClusterGroupEnum(hgroupenum, dwindex, lpdwtype, ::core::mem::transmute(lpszresourcename), lpcchname)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupEnumEx(hgroupenumex: *const _HGROUPENUMEX, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupEnumEx ( hgroupenumex : *const _HGROUPENUMEX , dwindex : u32 , pitem : *mut CLUSTER_GROUP_ENUM_ITEM , cbitem : *mut u32 ) -> u32 );
    ClusterGroupEnumEx(hgroupenumex, dwindex, pitem, cbitem)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupGetEnumCount(hgroupenum: *const _HGROUPENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupGetEnumCount ( hgroupenum : *const _HGROUPENUM ) -> u32 );
    ClusterGroupGetEnumCount(hgroupenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupGetEnumCountEx(hgroupenumex: *const _HGROUPENUMEX) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupGetEnumCountEx ( hgroupenumex : *const _HGROUPENUMEX ) -> u32 );
    ClusterGroupGetEnumCountEx(hgroupenumex)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupOpenEnum(hgroup: *const _HGROUP, dwtype: u32) -> *mut _HGROUPENUM {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupOpenEnum ( hgroup : *const _HGROUP , dwtype : u32 ) -> *mut _HGROUPENUM );
    ClusterGroupOpenEnum(hgroup, dwtype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupOpenEnumEx<P0, P1>(hcluster: *const _HCLUSTER, lpszproperties: P0, cbproperties: u32, lpszroproperties: P1, cbroproperties: u32, dwflags: u32) -> *mut _HGROUPENUMEX
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupOpenEnumEx ( hcluster : *const _HCLUSTER , lpszproperties : :: windows::core::PCWSTR , cbproperties : u32 , lpszroproperties : :: windows::core::PCWSTR , cbroproperties : u32 , dwflags : u32 ) -> *mut _HGROUPENUMEX );
    ClusterGroupOpenEnumEx(hcluster, lpszproperties.into().abi(), cbproperties, lpszroproperties.into().abi(), cbroproperties, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupSetCloseEnum(hgroupsetenum: *mut _HGROUPSETENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupSetCloseEnum ( hgroupsetenum : *mut _HGROUPSETENUM ) -> u32 );
    ClusterGroupSetCloseEnum(hgroupsetenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupSetControl(hgroupset: *const _HGROUPSET, hhostnode: ::core::option::Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupSetControl ( hgroupset : *const _HGROUPSET , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , cbinbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , cboutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
    ClusterGroupSetControl(hgroupset, ::core::mem::transmute(hhostnode.unwrap_or(::std::ptr::null())), dwcontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), cbinbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), cboutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupSetEnum(hgroupsetenum: *const _HGROUPSETENUM, dwindex: u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupSetEnum ( hgroupsetenum : *const _HGROUPSETENUM , dwindex : u32 , lpszname : :: windows::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
    ClusterGroupSetEnum(hgroupsetenum, dwindex, ::core::mem::transmute(lpszname), lpcchname)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupSetGetEnumCount(hgroupsetenum: *mut _HGROUPSETENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupSetGetEnumCount ( hgroupsetenum : *mut _HGROUPSETENUM ) -> u32 );
    ClusterGroupSetGetEnumCount(hgroupsetenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterGroupSetOpenEnum(hcluster: *mut _HCLUSTER) -> *mut _HGROUPSETENUM {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterGroupSetOpenEnum ( hcluster : *mut _HCLUSTER ) -> *mut _HGROUPSETENUM );
    ClusterGroupSetOpenEnum(hcluster)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClusterIsPathOnSharedVolume<P0>(lpszpathname: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ClusterIsPathOnSharedVolume ( lpszpathname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    ClusterIsPathOnSharedVolume(lpszpathname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNetInterfaceCloseEnum(hnetinterfaceenum: *const _HNETINTERFACEENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNetInterfaceCloseEnum ( hnetinterfaceenum : *const _HNETINTERFACEENUM ) -> u32 );
    ClusterNetInterfaceCloseEnum(hnetinterfaceenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNetInterfaceControl(hnetinterface: *const _HNETINTERFACE, hhostnode: ::core::option::Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNetInterfaceControl ( hnetinterface : *const _HNETINTERFACE , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
    ClusterNetInterfaceControl(hnetinterface, ::core::mem::transmute(hhostnode.unwrap_or(::std::ptr::null())), dwcontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), ninbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), noutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNetInterfaceEnum(hnetinterfaceenum: *const _HNETINTERFACEENUM, dwindex: u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNetInterfaceEnum ( hnetinterfaceenum : *const _HNETINTERFACEENUM , dwindex : u32 , lpszname : :: windows::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
    ClusterNetInterfaceEnum(hnetinterfaceenum, dwindex, ::core::mem::transmute(lpszname), lpcchname)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNetInterfaceOpenEnum<P0, P1>(hcluster: *const _HCLUSTER, lpsznodename: P0, lpsznetworkname: P1) -> *mut _HNETINTERFACEENUM
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNetInterfaceOpenEnum ( hcluster : *const _HCLUSTER , lpsznodename : :: windows::core::PCWSTR , lpsznetworkname : :: windows::core::PCWSTR ) -> *mut _HNETINTERFACEENUM );
    ClusterNetInterfaceOpenEnum(hcluster, lpsznodename.into().abi(), lpsznetworkname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNetworkCloseEnum(hnetworkenum: *const _HNETWORKENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNetworkCloseEnum ( hnetworkenum : *const _HNETWORKENUM ) -> u32 );
    ClusterNetworkCloseEnum(hnetworkenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNetworkControl(hnetwork: *const _HNETWORK, hhostnode: ::core::option::Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNetworkControl ( hnetwork : *const _HNETWORK , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
    ClusterNetworkControl(hnetwork, ::core::mem::transmute(hhostnode.unwrap_or(::std::ptr::null())), dwcontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), ninbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), noutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNetworkEnum(hnetworkenum: *const _HNETWORKENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNetworkEnum ( hnetworkenum : *const _HNETWORKENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszname : :: windows::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
    ClusterNetworkEnum(hnetworkenum, dwindex, lpdwtype, ::core::mem::transmute(lpszname), lpcchname)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNetworkGetEnumCount(hnetworkenum: *const _HNETWORKENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNetworkGetEnumCount ( hnetworkenum : *const _HNETWORKENUM ) -> u32 );
    ClusterNetworkGetEnumCount(hnetworkenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNetworkOpenEnum(hnetwork: *const _HNETWORK, dwtype: u32) -> *mut _HNETWORKENUM {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNetworkOpenEnum ( hnetwork : *const _HNETWORK , dwtype : u32 ) -> *mut _HNETWORKENUM );
    ClusterNetworkOpenEnum(hnetwork, dwtype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNodeCloseEnum(hnodeenum: *const _HNODEENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNodeCloseEnum ( hnodeenum : *const _HNODEENUM ) -> u32 );
    ClusterNodeCloseEnum(hnodeenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNodeCloseEnumEx(hnodeenum: *const _HNODEENUMEX) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNodeCloseEnumEx ( hnodeenum : *const _HNODEENUMEX ) -> u32 );
    ClusterNodeCloseEnumEx(hnodeenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNodeControl(hnode: *const _HNODE, hhostnode: ::core::option::Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNodeControl ( hnode : *const _HNODE , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
    ClusterNodeControl(hnode, ::core::mem::transmute(hhostnode.unwrap_or(::std::ptr::null())), dwcontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), ninbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), noutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNodeEnum(hnodeenum: *const _HNODEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNodeEnum ( hnodeenum : *const _HNODEENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszname : :: windows::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
    ClusterNodeEnum(hnodeenum, dwindex, lpdwtype, ::core::mem::transmute(lpszname), lpcchname)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNodeEnumEx(hnodeenum: *const _HNODEENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNodeEnumEx ( hnodeenum : *const _HNODEENUMEX , dwindex : u32 , pitem : *mut CLUSTER_ENUM_ITEM , cbitem : *mut u32 ) -> u32 );
    ClusterNodeEnumEx(hnodeenum, dwindex, pitem, cbitem)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNodeGetEnumCount(hnodeenum: *const _HNODEENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNodeGetEnumCount ( hnodeenum : *const _HNODEENUM ) -> u32 );
    ClusterNodeGetEnumCount(hnodeenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNodeGetEnumCountEx(hnodeenum: *const _HNODEENUMEX) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNodeGetEnumCountEx ( hnodeenum : *const _HNODEENUMEX ) -> u32 );
    ClusterNodeGetEnumCountEx(hnodeenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNodeOpenEnum(hnode: *const _HNODE, dwtype: u32) -> *mut _HNODEENUM {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNodeOpenEnum ( hnode : *const _HNODE , dwtype : u32 ) -> *mut _HNODEENUM );
    ClusterNodeOpenEnum(hnode, dwtype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNodeOpenEnumEx(hnode: *const _HNODE, dwtype: u32, poptions: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut _HNODEENUMEX {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNodeOpenEnumEx ( hnode : *const _HNODE , dwtype : u32 , poptions : *const ::core::ffi::c_void ) -> *mut _HNODEENUMEX );
    ClusterNodeOpenEnumEx(hnode, dwtype, ::core::mem::transmute(poptions.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterNodeReplacement<P0, P1>(hcluster: *const _HCLUSTER, lpsznodenamecurrent: P0, lpsznodenamenew: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterNodeReplacement ( hcluster : *const _HCLUSTER , lpsznodenamecurrent : :: windows::core::PCWSTR , lpsznodenamenew : :: windows::core::PCWSTR ) -> u32 );
    ClusterNodeReplacement(hcluster, lpsznodenamecurrent.into().abi(), lpsznodenamenew.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterOpenEnum(hcluster: *const _HCLUSTER, dwtype: u32) -> *mut _HCLUSENUM {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterOpenEnum ( hcluster : *const _HCLUSTER , dwtype : u32 ) -> *mut _HCLUSENUM );
    ClusterOpenEnum(hcluster, dwtype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterOpenEnumEx(hcluster: *const _HCLUSTER, dwtype: u32, poptions: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut _HCLUSENUMEX {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterOpenEnumEx ( hcluster : *const _HCLUSTER , dwtype : u32 , poptions : *const ::core::ffi::c_void ) -> *mut _HCLUSENUMEX );
    ClusterOpenEnumEx(hcluster, dwtype, ::core::mem::transmute(poptions.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterPrepareSharedVolumeForBackup<P0>(lpszfilename: P0, lpszvolumepathname: ::windows::core::PWSTR, lpcchvolumepathname: *mut u32, lpszvolumename: ::windows::core::PWSTR, lpcchvolumename: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ClusterPrepareSharedVolumeForBackup ( lpszfilename : :: windows::core::PCWSTR , lpszvolumepathname : :: windows::core::PWSTR , lpcchvolumepathname : *mut u32 , lpszvolumename : :: windows::core::PWSTR , lpcchvolumename : *mut u32 ) -> u32 );
    ClusterPrepareSharedVolumeForBackup(lpszfilename.into().abi(), ::core::mem::transmute(lpszvolumepathname), lpcchvolumepathname, ::core::mem::transmute(lpszvolumename), lpcchvolumename)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegBatchAddCommand<P0>(hregbatch: *const _HREGBATCH, dwcommand: CLUSTER_REG_COMMAND, wzname: P0, dwoptions: u32, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, cbdata: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegBatchAddCommand ( hregbatch : *const _HREGBATCH , dwcommand : CLUSTER_REG_COMMAND , wzname : :: windows::core::PCWSTR , dwoptions : u32 , lpdata : *const ::core::ffi::c_void , cbdata : u32 ) -> i32 );
    ClusterRegBatchAddCommand(hregbatch, dwcommand, wzname.into().abi(), dwoptions, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), cbdata)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegBatchCloseNotification(hbatchnotification: *const _HREGBATCHNOTIFICATION) -> i32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegBatchCloseNotification ( hbatchnotification : *const _HREGBATCHNOTIFICATION ) -> i32 );
    ClusterRegBatchCloseNotification(hbatchnotification)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegBatchReadCommand(hbatchnotification: *const _HREGBATCHNOTIFICATION, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegBatchReadCommand ( hbatchnotification : *const _HREGBATCHNOTIFICATION , pbatchcommand : *mut CLUSTER_BATCH_COMMAND ) -> i32 );
    ClusterRegBatchReadCommand(hbatchnotification, pbatchcommand)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClusterRegCloseBatch<P0>(hregbatch: *const _HREGBATCH, bcommit: P0, failedcommandnumber: ::core::option::Option<*mut i32>) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegCloseBatch ( hregbatch : *const _HREGBATCH , bcommit : super::super::Foundation:: BOOL , failedcommandnumber : *mut i32 ) -> i32 );
    ClusterRegCloseBatch(hregbatch, bcommit.into(), ::core::mem::transmute(failedcommandnumber.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegCloseBatchEx(hregbatch: *const _HREGBATCH, flags: u32, failedcommandnumber: ::core::option::Option<*mut i32>) -> i32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegCloseBatchEx ( hregbatch : *const _HREGBATCH , flags : u32 , failedcommandnumber : *mut i32 ) -> i32 );
    ClusterRegCloseBatchEx(hregbatch, flags, ::core::mem::transmute(failedcommandnumber.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegCloseBatchNotifyPort(hbatchnotifyport: *const _HREGBATCHPORT) -> i32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegCloseBatchNotifyPort ( hbatchnotifyport : *const _HREGBATCHPORT ) -> i32 );
    ClusterRegCloseBatchNotifyPort(hbatchnotifyport)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCloseKey<P0>(hkey: P0) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegCloseKey ( hkey : super::super::System::Registry:: HKEY ) -> i32 );
    ClusterRegCloseKey(hkey.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegCloseReadBatch(hregreadbatch: *const _HREGREADBATCH, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegCloseReadBatch ( hregreadbatch : *const _HREGREADBATCH , phregreadbatchreply : *mut *mut _HREGREADBATCHREPLY ) -> i32 );
    ClusterRegCloseReadBatch(hregreadbatch, phregreadbatchreply)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegCloseReadBatchEx(hregreadbatch: *const _HREGREADBATCH, flags: u32, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegCloseReadBatchEx ( hregreadbatch : *const _HREGREADBATCH , flags : u32 , phregreadbatchreply : *mut *mut _HREGREADBATCHREPLY ) -> i32 );
    ClusterRegCloseReadBatchEx(hregreadbatch, flags, phregreadbatchreply)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegCloseReadBatchReply(hregreadbatchreply: *const _HREGREADBATCHREPLY) -> i32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegCloseReadBatchReply ( hregreadbatchreply : *const _HREGREADBATCHREPLY ) -> i32 );
    ClusterRegCloseReadBatchReply(hregreadbatchreply)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCreateBatch<P0>(hkey: P0, phregbatch: *mut *mut _HREGBATCH) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegCreateBatch ( hkey : super::super::System::Registry:: HKEY , phregbatch : *mut *mut _HREGBATCH ) -> i32 );
    ClusterRegCreateBatch(hkey.into(), phregbatch)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCreateBatchNotifyPort<P0>(hkey: P0, phbatchnotifyport: *mut *mut _HREGBATCHPORT) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegCreateBatchNotifyPort ( hkey : super::super::System::Registry:: HKEY , phbatchnotifyport : *mut *mut _HREGBATCHPORT ) -> i32 );
    ClusterRegCreateBatchNotifyPort(hkey.into(), phbatchnotifyport)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegCreateKey<P0, P1>(hkey: P0, lpszsubkey: P1, dwoptions: u32, samdesired: u32, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: ::core::option::Option<*mut u32>) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegCreateKey ( hkey : super::super::System::Registry:: HKEY , lpszsubkey : :: windows::core::PCWSTR , dwoptions : u32 , samdesired : u32 , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , phkresult : *mut super::super::System::Registry:: HKEY , lpdwdisposition : *mut u32 ) -> i32 );
    ClusterRegCreateKey(hkey.into(), lpszsubkey.into().abi(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), phkresult, ::core::mem::transmute(lpdwdisposition.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCreateReadBatch<P0>(hkey: P0, phregreadbatch: *mut *mut _HREGREADBATCH) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegCreateReadBatch ( hkey : super::super::System::Registry:: HKEY , phregreadbatch : *mut *mut _HREGREADBATCH ) -> i32 );
    ClusterRegCreateReadBatch(hkey.into(), phregreadbatch)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegDeleteKey<P0, P1>(hkey: P0, lpszsubkey: P1) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegDeleteKey ( hkey : super::super::System::Registry:: HKEY , lpszsubkey : :: windows::core::PCWSTR ) -> i32 );
    ClusterRegDeleteKey(hkey.into(), lpszsubkey.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegDeleteValue<P0, P1>(hkey: P0, lpszvaluename: P1) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegDeleteValue ( hkey : super::super::System::Registry:: HKEY , lpszvaluename : :: windows::core::PCWSTR ) -> u32 );
    ClusterRegDeleteValue(hkey.into(), lpszvaluename.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegEnumKey<P0>(hkey: P0, dwindex: u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32, lpftlastwritetime: ::core::option::Option<*mut super::super::Foundation::FILETIME>) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegEnumKey ( hkey : super::super::System::Registry:: HKEY , dwindex : u32 , lpszname : :: windows::core::PWSTR , lpcchname : *mut u32 , lpftlastwritetime : *mut super::super::Foundation:: FILETIME ) -> i32 );
    ClusterRegEnumKey(hkey.into(), dwindex, ::core::mem::transmute(lpszname), lpcchname, ::core::mem::transmute(lpftlastwritetime.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegEnumValue<P0>(hkey: P0, dwindex: u32, lpszvaluename: ::windows::core::PWSTR, lpcchvaluename: *mut u32, lpdwtype: ::core::option::Option<*mut u32>, lpdata: ::core::option::Option<*mut u8>, lpcbdata: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegEnumValue ( hkey : super::super::System::Registry:: HKEY , dwindex : u32 , lpszvaluename : :: windows::core::PWSTR , lpcchvaluename : *mut u32 , lpdwtype : *mut u32 , lpdata : *mut u8 , lpcbdata : *mut u32 ) -> u32 );
    ClusterRegEnumValue(hkey.into(), dwindex, ::core::mem::transmute(lpszvaluename), lpcchvaluename, ::core::mem::transmute(lpdwtype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegGetBatchNotification(hbatchnotify: *const _HREGBATCHPORT, phbatchnotification: *mut *mut _HREGBATCHNOTIFICATION) -> i32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegGetBatchNotification ( hbatchnotify : *const _HREGBATCHPORT , phbatchnotification : *mut *mut _HREGBATCHNOTIFICATION ) -> i32 );
    ClusterRegGetBatchNotification(hbatchnotify, phbatchnotification)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegGetKeySecurity<P0>(hkey: P0, requestedinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegGetKeySecurity ( hkey : super::super::System::Registry:: HKEY , requestedinformation : u32 , psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR , lpcbsecuritydescriptor : *mut u32 ) -> i32 );
    ClusterRegGetKeySecurity(hkey.into(), requestedinformation, psecuritydescriptor, lpcbsecuritydescriptor)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegOpenKey<P0, P1>(hkey: P0, lpszsubkey: P1, samdesired: u32, phkresult: *mut super::super::System::Registry::HKEY) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegOpenKey ( hkey : super::super::System::Registry:: HKEY , lpszsubkey : :: windows::core::PCWSTR , samdesired : u32 , phkresult : *mut super::super::System::Registry:: HKEY ) -> i32 );
    ClusterRegOpenKey(hkey.into(), lpszsubkey.into().abi(), samdesired, phkresult)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegQueryInfoKey<P0>(hkey: P0, lpcsubkeys: *const u32, lpcchmaxsubkeylen: *const u32, lpcvalues: *const u32, lpcchmaxvaluenamelen: *const u32, lpcbmaxvaluelen: *const u32, lpcbsecuritydescriptor: *const u32, lpftlastwritetime: *const super::super::Foundation::FILETIME) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegQueryInfoKey ( hkey : super::super::System::Registry:: HKEY , lpcsubkeys : *const u32 , lpcchmaxsubkeylen : *const u32 , lpcvalues : *const u32 , lpcchmaxvaluenamelen : *const u32 , lpcbmaxvaluelen : *const u32 , lpcbsecuritydescriptor : *const u32 , lpftlastwritetime : *const super::super::Foundation:: FILETIME ) -> i32 );
    ClusterRegQueryInfoKey(hkey.into(), lpcsubkeys, lpcchmaxsubkeylen, lpcvalues, lpcchmaxvaluenamelen, lpcbmaxvaluelen, lpcbsecuritydescriptor, lpftlastwritetime)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegQueryValue<P0, P1>(hkey: P0, lpszvaluename: P1, lpdwvaluetype: ::core::option::Option<*mut u32>, lpdata: ::core::option::Option<*mut u8>, lpcbdata: ::core::option::Option<*mut u32>) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegQueryValue ( hkey : super::super::System::Registry:: HKEY , lpszvaluename : :: windows::core::PCWSTR , lpdwvaluetype : *mut u32 , lpdata : *mut u8 , lpcbdata : *mut u32 ) -> i32 );
    ClusterRegQueryValue(hkey.into(), lpszvaluename.into().abi(), ::core::mem::transmute(lpdwvaluetype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegReadBatchAddCommand<P0, P1>(hregreadbatch: *const _HREGREADBATCH, wzsubkeyname: P0, wzvaluename: P1) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegReadBatchAddCommand ( hregreadbatch : *const _HREGREADBATCH , wzsubkeyname : :: windows::core::PCWSTR , wzvaluename : :: windows::core::PCWSTR ) -> i32 );
    ClusterRegReadBatchAddCommand(hregreadbatch, wzsubkeyname.into().abi(), wzvaluename.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegReadBatchReplyNextCommand(hregreadbatchreply: *const _HREGREADBATCHREPLY, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegReadBatchReplyNextCommand ( hregreadbatchreply : *const _HREGREADBATCHREPLY , pbatchcommand : *mut CLUSTER_READ_BATCH_COMMAND ) -> i32 );
    ClusterRegReadBatchReplyNextCommand(hregreadbatchreply, pbatchcommand)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegSetKeySecurity<P0, P1>(hkey: P0, securityinformation: u32, psecuritydescriptor: P1) -> i32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegSetKeySecurity ( hkey : super::super::System::Registry:: HKEY , securityinformation : u32 , psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR ) -> i32 );
    ClusterRegSetKeySecurity(hkey.into(), securityinformation, psecuritydescriptor.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegSetValue<P0, P1>(hkey: P0, lpszvaluename: P1, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegSetValue ( hkey : super::super::System::Registry:: HKEY , lpszvaluename : :: windows::core::PCWSTR , dwtype : u32 , lpdata : *const u8 , cbdata : u32 ) -> u32 );
    ClusterRegSetValue(hkey.into(), lpszvaluename.into().abi(), dwtype, lpdata, cbdata)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRegSyncDatabase(hcluster: *const _HCLUSTER, flags: u32) -> i32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRegSyncDatabase ( hcluster : *const _HCLUSTER , flags : u32 ) -> i32 );
    ClusterRegSyncDatabase(hcluster, flags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRemoveAffinityRule<P0>(hcluster: *const _HCLUSTER, rulename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRemoveAffinityRule ( hcluster : *const _HCLUSTER , rulename : :: windows::core::PCWSTR ) -> u32 );
    ClusterRemoveAffinityRule(hcluster, rulename.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRemoveGroupFromAffinityRule<P0>(hcluster: *const _HCLUSTER, rulename: P0, hgroup: *const _HGROUP) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRemoveGroupFromAffinityRule ( hcluster : *const _HCLUSTER , rulename : :: windows::core::PCWSTR , hgroup : *const _HGROUP ) -> u32 );
    ClusterRemoveGroupFromAffinityRule(hcluster, rulename.into().abi(), hgroup)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterRemoveGroupFromGroupSet(hgroup: *const _HGROUP) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterRemoveGroupFromGroupSet ( hgroup : *const _HGROUP ) -> u32 );
    ClusterRemoveGroupFromGroupSet(hgroup)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceCloseEnum(hresenum: *const _HRESENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceCloseEnum ( hresenum : *const _HRESENUM ) -> u32 );
    ClusterResourceCloseEnum(hresenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceCloseEnumEx(hresourceenumex: *const _HRESENUMEX) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceCloseEnumEx ( hresourceenumex : *const _HRESENUMEX ) -> u32 );
    ClusterResourceCloseEnumEx(hresourceenumex)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceControl(hresource: *const _HRESOURCE, hhostnode: ::core::option::Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceControl ( hresource : *const _HRESOURCE , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , cbinbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , cboutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
    ClusterResourceControl(hresource, ::core::mem::transmute(hhostnode.unwrap_or(::std::ptr::null())), dwcontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), cbinbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), cboutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceControlAsUser(hresource: *const _HRESOURCE, hhostnode: ::core::option::Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceControlAsUser ( hresource : *const _HRESOURCE , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , cbinbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , cboutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
    ClusterResourceControlAsUser(hresource, ::core::mem::transmute(hhostnode.unwrap_or(::std::ptr::null())), dwcontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), cbinbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), cboutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceEnum(hresenum: *const _HRESENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceEnum ( hresenum : *const _HRESENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszname : :: windows::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
    ClusterResourceEnum(hresenum, dwindex, lpdwtype, ::core::mem::transmute(lpszname), lpcchname)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceEnumEx(hresourceenumex: *const _HRESENUMEX, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceEnumEx ( hresourceenumex : *const _HRESENUMEX , dwindex : u32 , pitem : *mut CLUSTER_RESOURCE_ENUM_ITEM , cbitem : *mut u32 ) -> u32 );
    ClusterResourceEnumEx(hresourceenumex, dwindex, pitem, cbitem)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceGetEnumCount(hresenum: *const _HRESENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceGetEnumCount ( hresenum : *const _HRESENUM ) -> u32 );
    ClusterResourceGetEnumCount(hresenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceGetEnumCountEx(hresourceenumex: *const _HRESENUMEX) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceGetEnumCountEx ( hresourceenumex : *const _HRESENUMEX ) -> u32 );
    ClusterResourceGetEnumCountEx(hresourceenumex)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceOpenEnum(hresource: *const _HRESOURCE, dwtype: u32) -> *mut _HRESENUM {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceOpenEnum ( hresource : *const _HRESOURCE , dwtype : u32 ) -> *mut _HRESENUM );
    ClusterResourceOpenEnum(hresource, dwtype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceOpenEnumEx<P0, P1>(hcluster: *const _HCLUSTER, lpszproperties: P0, cbproperties: u32, lpszroproperties: P1, cbroproperties: u32, dwflags: u32) -> *mut _HRESENUMEX
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceOpenEnumEx ( hcluster : *const _HCLUSTER , lpszproperties : :: windows::core::PCWSTR , cbproperties : u32 , lpszroproperties : :: windows::core::PCWSTR , cbroproperties : u32 , dwflags : u32 ) -> *mut _HRESENUMEX );
    ClusterResourceOpenEnumEx(hcluster, lpszproperties.into().abi(), cbproperties, lpszroproperties.into().abi(), cbroproperties, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceTypeCloseEnum(hrestypeenum: *const _HRESTYPEENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceTypeCloseEnum ( hrestypeenum : *const _HRESTYPEENUM ) -> u32 );
    ClusterResourceTypeCloseEnum(hrestypeenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceTypeControl<P0>(hcluster: *const _HCLUSTER, lpszresourcetypename: P0, hhostnode: ::core::option::Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceTypeControl ( hcluster : *const _HCLUSTER , lpszresourcetypename : :: windows::core::PCWSTR , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
    ClusterResourceTypeControl(hcluster, lpszresourcetypename.into().abi(), ::core::mem::transmute(hhostnode.unwrap_or(::std::ptr::null())), dwcontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), ninbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), noutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceTypeControlAsUser<P0>(hcluster: *const _HCLUSTER, lpszresourcetypename: P0, hhostnode: ::core::option::Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceTypeControlAsUser ( hcluster : *const _HCLUSTER , lpszresourcetypename : :: windows::core::PCWSTR , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
    ClusterResourceTypeControlAsUser(hcluster, lpszresourcetypename.into().abi(), ::core::mem::transmute(hhostnode.unwrap_or(::std::ptr::null())), dwcontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), ninbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), noutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceTypeEnum(hrestypeenum: *const _HRESTYPEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceTypeEnum ( hrestypeenum : *const _HRESTYPEENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszname : :: windows::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
    ClusterResourceTypeEnum(hrestypeenum, dwindex, lpdwtype, ::core::mem::transmute(lpszname), lpcchname)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceTypeGetEnumCount(hrestypeenum: *const _HRESTYPEENUM) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceTypeGetEnumCount ( hrestypeenum : *const _HRESTYPEENUM ) -> u32 );
    ClusterResourceTypeGetEnumCount(hrestypeenum)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterResourceTypeOpenEnum<P0>(hcluster: *const _HCLUSTER, lpszresourcetypename: P0, dwtype: u32) -> *mut _HRESTYPEENUM
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterResourceTypeOpenEnum ( hcluster : *const _HCLUSTER , lpszresourcetypename : :: windows::core::PCWSTR , dwtype : u32 ) -> *mut _HRESTYPEENUM );
    ClusterResourceTypeOpenEnum(hcluster, lpszresourcetypename.into().abi(), dwtype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterSetAccountAccess<P0>(hcluster: *const _HCLUSTER, szaccountsid: P0, dwaccess: u32, dwcontroltype: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterSetAccountAccess ( hcluster : *const _HCLUSTER , szaccountsid : :: windows::core::PCWSTR , dwaccess : u32 , dwcontroltype : u32 ) -> u32 );
    ClusterSetAccountAccess(hcluster, szaccountsid.into().abi(), dwaccess, dwcontroltype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ClusterSharedVolumeSetSnapshotState<P0>(guidsnapshotset: ::windows::core::GUID, lpszvolumename: P0, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterSharedVolumeSetSnapshotState ( guidsnapshotset : :: windows::core::GUID , lpszvolumename : :: windows::core::PCWSTR , state : CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE ) -> u32 );
    ClusterSharedVolumeSetSnapshotState(::core::mem::transmute(guidsnapshotset), lpszvolumename.into().abi(), state)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClusterUpgradeFunctionalLevel<P0>(hcluster: *const _HCLUSTER, perform: P0, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn ClusterUpgradeFunctionalLevel ( hcluster : *const _HCLUSTER , perform : super::super::Foundation:: BOOL , pfnprogresscallback : PCLUSTER_UPGRADE_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void ) -> u32 );
    ClusterUpgradeFunctionalLevel(hcluster, perform.into(), pfnprogresscallback, ::core::mem::transmute(pvcallbackarg.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateCluster(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut _HCLUSTER {
    ::windows::core::link ! ( "clusapi.dll""system" fn CreateCluster ( pconfig : *const CREATE_CLUSTER_CONFIG , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void ) -> *mut _HCLUSTER );
    CreateCluster(pconfig, pfnprogresscallback, ::core::mem::transmute(pvcallbackarg.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateClusterAvailabilitySet<P0>(hcluster: *const _HCLUSTER, lpavailabilitysetname: P0, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> *mut _HGROUPSET
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn CreateClusterAvailabilitySet ( hcluster : *const _HCLUSTER , lpavailabilitysetname : :: windows::core::PCWSTR , pavailabilitysetconfig : *const CLUSTER_AVAILABILITY_SET_CONFIG ) -> *mut _HGROUPSET );
    CreateClusterAvailabilitySet(hcluster, lpavailabilitysetname.into().abi(), pavailabilitysetconfig)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn CreateClusterGroup<P0>(hcluster: *const _HCLUSTER, lpszgroupname: P0) -> *mut _HGROUP
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn CreateClusterGroup ( hcluster : *const _HCLUSTER , lpszgroupname : :: windows::core::PCWSTR ) -> *mut _HGROUP );
    CreateClusterGroup(hcluster, lpszgroupname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn CreateClusterGroupEx<P0>(hcluster: *const _HCLUSTER, lpszgroupname: P0, pgroupinfo: ::core::option::Option<*const CLUSTER_CREATE_GROUP_INFO>) -> *mut _HGROUP
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn CreateClusterGroupEx ( hcluster : *const _HCLUSTER , lpszgroupname : :: windows::core::PCWSTR , pgroupinfo : *const CLUSTER_CREATE_GROUP_INFO ) -> *mut _HGROUP );
    CreateClusterGroupEx(hcluster, lpszgroupname.into().abi(), ::core::mem::transmute(pgroupinfo.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn CreateClusterGroupSet<P0>(hcluster: *const _HCLUSTER, groupsetname: P0) -> *mut _HGROUPSET
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn CreateClusterGroupSet ( hcluster : *const _HCLUSTER , groupsetname : :: windows::core::PCWSTR ) -> *mut _HGROUPSET );
    CreateClusterGroupSet(hcluster, groupsetname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateClusterNameAccount(hcluster: *const _HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: ::core::option::Option<*const ::core::ffi::c_void>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn CreateClusterNameAccount ( hcluster : *const _HCLUSTER , pconfig : *const CREATE_CLUSTER_NAME_ACCOUNT , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void ) -> u32 );
    CreateClusterNameAccount(hcluster, pconfig, pfnprogresscallback, ::core::mem::transmute(pvcallbackarg.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn CreateClusterNotifyPort(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, dwfilter: u32, dwnotifykey: usize) -> *mut _HCHANGE {
    ::windows::core::link ! ( "clusapi.dll""system" fn CreateClusterNotifyPort ( hchange : *const _HCHANGE , hcluster : *const _HCLUSTER , dwfilter : u32 , dwnotifykey : usize ) -> *mut _HCHANGE );
    CreateClusterNotifyPort(hchange, hcluster, dwfilter, dwnotifykey)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn CreateClusterNotifyPortV2(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> *mut _HCHANGE {
    ::windows::core::link ! ( "clusapi.dll""system" fn CreateClusterNotifyPortV2 ( hchange : *const _HCHANGE , hcluster : *const _HCLUSTER , filters : *const NOTIFY_FILTER_AND_TYPE , dwfiltercount : u32 , dwnotifykey : usize ) -> *mut _HCHANGE );
    CreateClusterNotifyPortV2(hchange, hcluster, filters, dwfiltercount, dwnotifykey)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn CreateClusterResource<P0, P1>(hgroup: *const _HGROUP, lpszresourcename: P0, lpszresourcetype: P1, dwflags: u32) -> *mut _HRESOURCE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn CreateClusterResource ( hgroup : *const _HGROUP , lpszresourcename : :: windows::core::PCWSTR , lpszresourcetype : :: windows::core::PCWSTR , dwflags : u32 ) -> *mut _HRESOURCE );
    CreateClusterResource(hgroup, lpszresourcename.into().abi(), lpszresourcetype.into().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn CreateClusterResourceType<P0, P1, P2>(hcluster: *const _HCLUSTER, lpszresourcetypename: P0, lpszdisplayname: P1, lpszresourcetypedll: P2, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn CreateClusterResourceType ( hcluster : *const _HCLUSTER , lpszresourcetypename : :: windows::core::PCWSTR , lpszdisplayname : :: windows::core::PCWSTR , lpszresourcetypedll : :: windows::core::PCWSTR , dwlooksalivepollinterval : u32 , dwisalivepollinterval : u32 ) -> u32 );
    CreateClusterResourceType(hcluster, lpszresourcetypename.into().abi(), lpszdisplayname.into().abi(), lpszresourcetypedll.into().abi(), dwlooksalivepollinterval, dwisalivepollinterval)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn DeleteClusterGroup(hgroup: *const _HGROUP) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn DeleteClusterGroup ( hgroup : *const _HGROUP ) -> u32 );
    DeleteClusterGroup(hgroup)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn DeleteClusterGroupSet(hgroupset: *const _HGROUPSET) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn DeleteClusterGroupSet ( hgroupset : *const _HGROUPSET ) -> u32 );
    DeleteClusterGroupSet(hgroupset)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn DeleteClusterResource(hresource: *const _HRESOURCE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn DeleteClusterResource ( hresource : *const _HRESOURCE ) -> u32 );
    DeleteClusterResource(hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn DeleteClusterResourceType<P0>(hcluster: *const _HCLUSTER, lpszresourcetypename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn DeleteClusterResourceType ( hcluster : *const _HCLUSTER , lpszresourcetypename : :: windows::core::PCWSTR ) -> u32 );
    DeleteClusterResourceType(hcluster, lpszresourcetypename.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyCluster<P0>(hcluster: *const _HCLUSTER, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: ::core::option::Option<*const ::core::ffi::c_void>, fdeletevirtualcomputerobjects: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn DestroyCluster ( hcluster : *const _HCLUSTER , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void , fdeletevirtualcomputerobjects : super::super::Foundation:: BOOL ) -> u32 );
    DestroyCluster(hcluster, pfnprogresscallback, ::core::mem::transmute(pvcallbackarg.unwrap_or(::std::ptr::null())), fdeletevirtualcomputerobjects.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn DestroyClusterGroup(hgroup: *const _HGROUP) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn DestroyClusterGroup ( hgroup : *const _HGROUP ) -> u32 );
    DestroyClusterGroup(hgroup)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn DetermineCNOResTypeFromCluster(hcluster: *const _HCLUSTER, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn DetermineCNOResTypeFromCluster ( hcluster : *const _HCLUSTER , pcnorestype : *mut CLUSTER_MGMT_POINT_RESTYPE ) -> u32 );
    DetermineCNOResTypeFromCluster(hcluster, pcnorestype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn DetermineCNOResTypeFromNodelist(cnodes: u32, ppsznodenames: *const ::windows::core::PCWSTR, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn DetermineCNOResTypeFromNodelist ( cnodes : u32 , ppsznodenames : *const :: windows::core::PCWSTR , pcnorestype : *mut CLUSTER_MGMT_POINT_RESTYPE ) -> u32 );
    DetermineCNOResTypeFromNodelist(cnodes, ppsznodenames, pcnorestype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn DetermineClusterCloudTypeFromCluster(hcluster: *const _HCLUSTER, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn DetermineClusterCloudTypeFromCluster ( hcluster : *const _HCLUSTER , pcloudtype : *mut CLUSTER_CLOUD_TYPE ) -> u32 );
    DetermineClusterCloudTypeFromCluster(hcluster, pcloudtype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn DetermineClusterCloudTypeFromNodelist(cnodes: u32, ppsznodenames: *const ::windows::core::PCWSTR, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn DetermineClusterCloudTypeFromNodelist ( cnodes : u32 , ppsznodenames : *const :: windows::core::PCWSTR , pcloudtype : *mut CLUSTER_CLOUD_TYPE ) -> u32 );
    DetermineClusterCloudTypeFromNodelist(cnodes, ppsznodenames, pcloudtype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn EvictClusterNode(hnode: *const _HNODE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn EvictClusterNode ( hnode : *const _HNODE ) -> u32 );
    EvictClusterNode(hnode)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn EvictClusterNodeEx(hnode: *const _HNODE, dwtimeout: u32, phrcleanupstatus: *mut ::windows::core::HRESULT) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn EvictClusterNodeEx ( hnode : *const _HNODE , dwtimeout : u32 , phrcleanupstatus : *mut :: windows::core::HRESULT ) -> u32 );
    EvictClusterNodeEx(hnode, dwtimeout, phrcleanupstatus)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn FailClusterResource(hresource: *const _HRESOURCE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn FailClusterResource ( hresource : *const _HRESOURCE ) -> u32 );
    FailClusterResource(hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn FreeClusterCrypt(pcryptinfo: *const ::core::ffi::c_void) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn FreeClusterCrypt ( pcryptinfo : *const ::core::ffi::c_void ) -> u32 );
    FreeClusterCrypt(pcryptinfo)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn FreeClusterHealthFault(clusterhealthfault: *mut CLUSTER_HEALTH_FAULT) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn FreeClusterHealthFault ( clusterhealthfault : *mut CLUSTER_HEALTH_FAULT ) -> u32 );
    FreeClusterHealthFault(clusterhealthfault)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn FreeClusterHealthFaultArray(clusterhealthfaultarray: *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn FreeClusterHealthFaultArray ( clusterhealthfaultarray : *mut CLUSTER_HEALTH_FAULT_ARRAY ) -> u32 );
    FreeClusterHealthFaultArray(clusterhealthfaultarray)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterFromGroup(hgroup: *const _HGROUP) -> *mut _HCLUSTER {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterFromGroup ( hgroup : *const _HGROUP ) -> *mut _HCLUSTER );
    GetClusterFromGroup(hgroup)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterFromNetInterface(hnetinterface: *const _HNETINTERFACE) -> *mut _HCLUSTER {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterFromNetInterface ( hnetinterface : *const _HNETINTERFACE ) -> *mut _HCLUSTER );
    GetClusterFromNetInterface(hnetinterface)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterFromNetwork(hnetwork: *const _HNETWORK) -> *mut _HCLUSTER {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterFromNetwork ( hnetwork : *const _HNETWORK ) -> *mut _HCLUSTER );
    GetClusterFromNetwork(hnetwork)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterFromNode(hnode: *const _HNODE) -> *mut _HCLUSTER {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterFromNode ( hnode : *const _HNODE ) -> *mut _HCLUSTER );
    GetClusterFromNode(hnode)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterFromResource(hresource: *const _HRESOURCE) -> *mut _HCLUSTER {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterFromResource ( hresource : *const _HRESOURCE ) -> *mut _HCLUSTER );
    GetClusterFromResource(hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterGroupKey(hgroup: *const _HGROUP, samdesired: u32) -> ::windows::core::Result<super::super::System::Registry::HKEY> {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterGroupKey ( hgroup : *const _HGROUP , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
    let result__ = GetClusterGroupKey(hgroup, samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterGroupState(hgroup: *const _HGROUP, lpsznodename: ::windows::core::PWSTR, lpcchnodename: ::core::option::Option<*mut u32>) -> CLUSTER_GROUP_STATE {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterGroupState ( hgroup : *const _HGROUP , lpsznodename : :: windows::core::PWSTR , lpcchnodename : *mut u32 ) -> CLUSTER_GROUP_STATE );
    GetClusterGroupState(hgroup, ::core::mem::transmute(lpsznodename), ::core::mem::transmute(lpcchnodename.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterInformation(hcluster: *const _HCLUSTER, lpszclustername: ::windows::core::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: ::core::option::Option<*mut CLUSTERVERSIONINFO>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterInformation ( hcluster : *const _HCLUSTER , lpszclustername : :: windows::core::PWSTR , lpcchclustername : *mut u32 , lpclusterinfo : *mut CLUSTERVERSIONINFO ) -> u32 );
    GetClusterInformation(hcluster, ::core::mem::transmute(lpszclustername), lpcchclustername, ::core::mem::transmute(lpclusterinfo.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterKey(hcluster: *const _HCLUSTER, samdesired: u32) -> ::windows::core::Result<super::super::System::Registry::HKEY> {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterKey ( hcluster : *const _HCLUSTER , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
    let result__ = GetClusterKey(hcluster, samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterNetInterface<P0, P1>(hcluster: *const _HCLUSTER, lpsznodename: P0, lpsznetworkname: P1, lpszinterfacename: ::windows::core::PWSTR, lpcchinterfacename: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterNetInterface ( hcluster : *const _HCLUSTER , lpsznodename : :: windows::core::PCWSTR , lpsznetworkname : :: windows::core::PCWSTR , lpszinterfacename : :: windows::core::PWSTR , lpcchinterfacename : *mut u32 ) -> u32 );
    GetClusterNetInterface(hcluster, lpsznodename.into().abi(), lpsznetworkname.into().abi(), ::core::mem::transmute(lpszinterfacename), lpcchinterfacename)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterNetInterfaceKey(hnetinterface: *const _HNETINTERFACE, samdesired: u32) -> ::windows::core::Result<super::super::System::Registry::HKEY> {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterNetInterfaceKey ( hnetinterface : *const _HNETINTERFACE , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
    let result__ = GetClusterNetInterfaceKey(hnetinterface, samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterNetInterfaceState(hnetinterface: *const _HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterNetInterfaceState ( hnetinterface : *const _HNETINTERFACE ) -> CLUSTER_NETINTERFACE_STATE );
    GetClusterNetInterfaceState(hnetinterface)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterNetworkId(hnetwork: *const _HNETWORK, lpsznetworkid: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterNetworkId ( hnetwork : *const _HNETWORK , lpsznetworkid : :: windows::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
    GetClusterNetworkId(hnetwork, ::core::mem::transmute(lpsznetworkid), lpcchname)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterNetworkKey(hnetwork: *const _HNETWORK, samdesired: u32) -> ::windows::core::Result<super::super::System::Registry::HKEY> {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterNetworkKey ( hnetwork : *const _HNETWORK , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
    let result__ = GetClusterNetworkKey(hnetwork, samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterNetworkState(hnetwork: *const _HNETWORK) -> CLUSTER_NETWORK_STATE {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterNetworkState ( hnetwork : *const _HNETWORK ) -> CLUSTER_NETWORK_STATE );
    GetClusterNetworkState(hnetwork)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterNodeId(hnode: ::core::option::Option<*const _HNODE>, lpsznodeid: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterNodeId ( hnode : *const _HNODE , lpsznodeid : :: windows::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
    GetClusterNodeId(::core::mem::transmute(hnode.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpsznodeid), lpcchname)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterNodeKey(hnode: *const _HNODE, samdesired: u32) -> ::windows::core::Result<super::super::System::Registry::HKEY> {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterNodeKey ( hnode : *const _HNODE , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
    let result__ = GetClusterNodeKey(hnode, samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterNodeState(hnode: *const _HNODE) -> CLUSTER_NODE_STATE {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterNodeState ( hnode : *const _HNODE ) -> CLUSTER_NODE_STATE );
    GetClusterNodeState(hnode)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterNotify(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterNotify ( hchange : *const _HCHANGE , lpdwnotifykey : *mut usize , lpdwfiltertype : *mut u32 , lpszname : :: windows::core::PWSTR , lpcchname : *mut u32 , dwmilliseconds : u32 ) -> u32 );
    GetClusterNotify(hchange, lpdwnotifykey, lpdwfiltertype, ::core::mem::transmute(lpszname), lpcchname, dwmilliseconds)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterNotifyV2(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, pfilterandtype: ::core::option::Option<*mut NOTIFY_FILTER_AND_TYPE>, buffer: ::core::option::Option<*mut u8>, lpbbuffersize: ::core::option::Option<*mut u32>, lpszobjectid: ::windows::core::PWSTR, lpcchobjectid: ::core::option::Option<*mut u32>, lpszparentid: ::windows::core::PWSTR, lpcchparentid: ::core::option::Option<*mut u32>, lpszname: ::windows::core::PWSTR, lpcchname: ::core::option::Option<*mut u32>, lpsztype: ::windows::core::PWSTR, lpcchtype: ::core::option::Option<*mut u32>, dwmilliseconds: u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterNotifyV2 ( hchange : *const _HCHANGE , lpdwnotifykey : *mut usize , pfilterandtype : *mut NOTIFY_FILTER_AND_TYPE , buffer : *mut u8 , lpbbuffersize : *mut u32 , lpszobjectid : :: windows::core::PWSTR , lpcchobjectid : *mut u32 , lpszparentid : :: windows::core::PWSTR , lpcchparentid : *mut u32 , lpszname : :: windows::core::PWSTR , lpcchname : *mut u32 , lpsztype : :: windows::core::PWSTR , lpcchtype : *mut u32 , dwmilliseconds : u32 ) -> u32 );
    GetClusterNotifyV2(
        hchange,
        lpdwnotifykey,
        ::core::mem::transmute(pfilterandtype.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpbbuffersize.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpszobjectid),
        ::core::mem::transmute(lpcchobjectid.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpszparentid),
        ::core::mem::transmute(lpcchparentid.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpszname),
        ::core::mem::transmute(lpcchname.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpsztype),
        ::core::mem::transmute(lpcchtype.unwrap_or(::std::ptr::null_mut())),
        dwmilliseconds,
    )
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterQuorumResource(hcluster: *const _HCLUSTER, lpszresourcename: ::windows::core::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: ::windows::core::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterQuorumResource ( hcluster : *const _HCLUSTER , lpszresourcename : :: windows::core::PWSTR , lpcchresourcename : *mut u32 , lpszdevicename : :: windows::core::PWSTR , lpcchdevicename : *mut u32 , lpdwmaxquorumlogsize : *mut u32 ) -> u32 );
    GetClusterQuorumResource(hcluster, ::core::mem::transmute(lpszresourcename), lpcchresourcename, ::core::mem::transmute(lpszdevicename), lpcchdevicename, lpdwmaxquorumlogsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterResourceDependencyExpression(hresource: *const _HRESOURCE, lpszdependencyexpression: ::windows::core::PWSTR, lpcchdependencyexpression: *mut u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterResourceDependencyExpression ( hresource : *const _HRESOURCE , lpszdependencyexpression : :: windows::core::PWSTR , lpcchdependencyexpression : *mut u32 ) -> u32 );
    GetClusterResourceDependencyExpression(hresource, ::core::mem::transmute(lpszdependencyexpression), lpcchdependencyexpression)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterResourceKey(hresource: *const _HRESOURCE, samdesired: u32) -> ::windows::core::Result<super::super::System::Registry::HKEY> {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterResourceKey ( hresource : *const _HRESOURCE , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
    let result__ = GetClusterResourceKey(hresource, samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClusterResourceNetworkName(hresource: *const _HRESOURCE, lpbuffer: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterResourceNetworkName ( hresource : *const _HRESOURCE , lpbuffer : :: windows::core::PWSTR , nsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetClusterResourceNetworkName(hresource, ::core::mem::transmute(lpbuffer), nsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetClusterResourceState(hresource: *const _HRESOURCE, lpsznodename: ::windows::core::PWSTR, lpcchnodename: ::core::option::Option<*mut u32>, lpszgroupname: ::windows::core::PWSTR, lpcchgroupname: ::core::option::Option<*mut u32>) -> CLUSTER_RESOURCE_STATE {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterResourceState ( hresource : *const _HRESOURCE , lpsznodename : :: windows::core::PWSTR , lpcchnodename : *mut u32 , lpszgroupname : :: windows::core::PWSTR , lpcchgroupname : *mut u32 ) -> CLUSTER_RESOURCE_STATE );
    GetClusterResourceState(hresource, ::core::mem::transmute(lpsznodename), ::core::mem::transmute(lpcchnodename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpszgroupname), ::core::mem::transmute(lpcchgroupname.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterResourceTypeKey<P0>(hcluster: *const _HCLUSTER, lpsztypename: P0, samdesired: u32) -> ::windows::core::Result<super::super::System::Registry::HKEY>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn GetClusterResourceTypeKey ( hcluster : *const _HCLUSTER , lpsztypename : :: windows::core::PCWSTR , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
    let result__ = GetClusterResourceTypeKey(hcluster, lpsztypename.into().abi(), samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetNodeCloudTypeDW<P0>(ppsznodename: P0, nodecloudtype: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn GetNodeCloudTypeDW ( ppsznodename : :: windows::core::PCWSTR , nodecloudtype : *mut u32 ) -> u32 );
    GetNodeCloudTypeDW(ppsznodename.into().abi(), nodecloudtype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn GetNodeClusterState<P0>(lpsznodename: P0, pdwclusterstate: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn GetNodeClusterState ( lpsznodename : :: windows::core::PCWSTR , pdwclusterstate : *mut u32 ) -> u32 );
    GetNodeClusterState(lpsznodename.into().abi(), pdwclusterstate)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNotifyEventHandle(hchange: *const _HCHANGE, lphtargetevent: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn GetNotifyEventHandle ( hchange : *const _HCHANGE , lphtargetevent : *mut super::super::Foundation:: HANDLE ) -> u32 );
    GetNotifyEventHandle(hchange, lphtargetevent)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn InitializeClusterHealthFault(clusterhealthfault: *mut CLUSTER_HEALTH_FAULT) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn InitializeClusterHealthFault ( clusterhealthfault : *mut CLUSTER_HEALTH_FAULT ) -> u32 );
    InitializeClusterHealthFault(clusterhealthfault)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn InitializeClusterHealthFaultArray(clusterhealthfaultarray: *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn InitializeClusterHealthFaultArray ( clusterhealthfaultarray : *mut CLUSTER_HEALTH_FAULT_ARRAY ) -> u32 );
    InitializeClusterHealthFaultArray(clusterhealthfaultarray)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsFileOnClusterSharedVolume<P0>(lpszpathname: P0, pbfileisonsharedvolume: *mut super::super::Foundation::BOOL) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn IsFileOnClusterSharedVolume ( lpszpathname : :: windows::core::PCWSTR , pbfileisonsharedvolume : *mut super::super::Foundation:: BOOL ) -> u32 );
    IsFileOnClusterSharedVolume(lpszpathname.into().abi(), pbfileisonsharedvolume)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn MoveClusterGroup(hgroup: *const _HGROUP, hdestinationnode: ::core::option::Option<*const _HNODE>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn MoveClusterGroup ( hgroup : *const _HGROUP , hdestinationnode : *const _HNODE ) -> u32 );
    MoveClusterGroup(hgroup, ::core::mem::transmute(hdestinationnode.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn MoveClusterGroupEx(hgroup: *const _HGROUP, hdestinationnode: ::core::option::Option<*const _HNODE>, dwmoveflags: u32, lpinbuffer: ::core::option::Option<&[u8]>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn MoveClusterGroupEx ( hgroup : *const _HGROUP , hdestinationnode : *const _HNODE , dwmoveflags : u32 , lpinbuffer : *const u8 , cbinbuffersize : u32 ) -> u32 );
    MoveClusterGroupEx(hgroup, ::core::mem::transmute(hdestinationnode.unwrap_or(::std::ptr::null())), dwmoveflags, ::core::mem::transmute(lpinbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OfflineClusterGroup(hgroup: *const _HGROUP) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn OfflineClusterGroup ( hgroup : *const _HGROUP ) -> u32 );
    OfflineClusterGroup(hgroup)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OfflineClusterGroupEx(hgroup: *const _HGROUP, dwofflineflags: u32, lpinbuffer: ::core::option::Option<&[u8]>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn OfflineClusterGroupEx ( hgroup : *const _HGROUP , dwofflineflags : u32 , lpinbuffer : *const u8 , cbinbuffersize : u32 ) -> u32 );
    OfflineClusterGroupEx(hgroup, dwofflineflags, ::core::mem::transmute(lpinbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OfflineClusterResource(hresource: *const _HRESOURCE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn OfflineClusterResource ( hresource : *const _HRESOURCE ) -> u32 );
    OfflineClusterResource(hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OfflineClusterResourceEx(hresource: *const _HRESOURCE, dwofflineflags: u32, lpinbuffer: ::core::option::Option<&[u8]>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn OfflineClusterResourceEx ( hresource : *const _HRESOURCE , dwofflineflags : u32 , lpinbuffer : *const u8 , cbinbuffersize : u32 ) -> u32 );
    OfflineClusterResourceEx(hresource, dwofflineflags, ::core::mem::transmute(lpinbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OnlineClusterGroup(hgroup: *const _HGROUP, hdestinationnode: ::core::option::Option<*const _HNODE>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn OnlineClusterGroup ( hgroup : *const _HGROUP , hdestinationnode : *const _HNODE ) -> u32 );
    OnlineClusterGroup(hgroup, ::core::mem::transmute(hdestinationnode.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OnlineClusterGroupEx(hgroup: *const _HGROUP, hdestinationnode: ::core::option::Option<*const _HNODE>, dwonlineflags: u32, lpinbuffer: ::core::option::Option<&[u8]>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn OnlineClusterGroupEx ( hgroup : *const _HGROUP , hdestinationnode : *const _HNODE , dwonlineflags : u32 , lpinbuffer : *const u8 , cbinbuffersize : u32 ) -> u32 );
    OnlineClusterGroupEx(hgroup, ::core::mem::transmute(hdestinationnode.unwrap_or(::std::ptr::null())), dwonlineflags, ::core::mem::transmute(lpinbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OnlineClusterResource(hresource: *const _HRESOURCE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn OnlineClusterResource ( hresource : *const _HRESOURCE ) -> u32 );
    OnlineClusterResource(hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OnlineClusterResourceEx(hresource: *const _HRESOURCE, dwonlineflags: u32, lpinbuffer: ::core::option::Option<&[u8]>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn OnlineClusterResourceEx ( hresource : *const _HRESOURCE , dwonlineflags : u32 , lpinbuffer : *const u8 , cbinbuffersize : u32 ) -> u32 );
    OnlineClusterResourceEx(hresource, dwonlineflags, ::core::mem::transmute(lpinbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenCluster<P0>(lpszclustername: P0) -> *mut _HCLUSTER
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenCluster ( lpszclustername : :: windows::core::PCWSTR ) -> *mut _HCLUSTER );
    OpenCluster(lpszclustername.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterCryptProvider<P0>(lpszresource: P0, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn OpenClusterCryptProvider ( lpszresource : :: windows::core::PCWSTR , lpszprovider : *const i8 , dwtype : u32 , dwflags : u32 ) -> *mut _HCLUSCRYPTPROVIDER );
    OpenClusterCryptProvider(lpszresource.into().abi(), lpszprovider, dwtype, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterCryptProviderEx<P0, P1>(lpszresource: P0, lpszkeyname: P1, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn OpenClusterCryptProviderEx ( lpszresource : :: windows::core::PCWSTR , lpszkeyname : :: windows::core::PCWSTR , lpszprovider : *const i8 , dwtype : u32 , dwflags : u32 ) -> *mut _HCLUSCRYPTPROVIDER );
    OpenClusterCryptProviderEx(lpszresource.into().abi(), lpszkeyname.into().abi(), lpszprovider, dwtype, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterEx<P0>(lpszclustername: P0, desiredaccess: u32, grantedaccess: ::core::option::Option<*mut u32>) -> *mut _HCLUSTER
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterEx ( lpszclustername : :: windows::core::PCWSTR , desiredaccess : u32 , grantedaccess : *mut u32 ) -> *mut _HCLUSTER );
    OpenClusterEx(lpszclustername.into().abi(), desiredaccess, ::core::mem::transmute(grantedaccess.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterGroup<P0>(hcluster: *const _HCLUSTER, lpszgroupname: P0) -> *mut _HGROUP
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterGroup ( hcluster : *const _HCLUSTER , lpszgroupname : :: windows::core::PCWSTR ) -> *mut _HGROUP );
    OpenClusterGroup(hcluster, lpszgroupname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterGroupEx<P0>(hcluster: *const _HCLUSTER, lpszgroupname: P0, dwdesiredaccess: u32, lpdwgrantedaccess: ::core::option::Option<*mut u32>) -> *mut _HGROUP
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterGroupEx ( hcluster : *const _HCLUSTER , lpszgroupname : :: windows::core::PCWSTR , dwdesiredaccess : u32 , lpdwgrantedaccess : *mut u32 ) -> *mut _HGROUP );
    OpenClusterGroupEx(hcluster, lpszgroupname.into().abi(), dwdesiredaccess, ::core::mem::transmute(lpdwgrantedaccess.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterGroupSet<P0>(hcluster: *const _HCLUSTER, lpszgroupsetname: P0) -> *mut _HGROUPSET
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterGroupSet ( hcluster : *const _HCLUSTER , lpszgroupsetname : :: windows::core::PCWSTR ) -> *mut _HGROUPSET );
    OpenClusterGroupSet(hcluster, lpszgroupsetname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterNetInterface<P0>(hcluster: *const _HCLUSTER, lpszinterfacename: P0) -> *mut _HNETINTERFACE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterNetInterface ( hcluster : *const _HCLUSTER , lpszinterfacename : :: windows::core::PCWSTR ) -> *mut _HNETINTERFACE );
    OpenClusterNetInterface(hcluster, lpszinterfacename.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterNetInterfaceEx<P0>(hcluster: *const _HCLUSTER, lpszinterfacename: P0, dwdesiredaccess: u32, lpdwgrantedaccess: ::core::option::Option<*mut u32>) -> *mut _HNETINTERFACE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterNetInterfaceEx ( hcluster : *const _HCLUSTER , lpszinterfacename : :: windows::core::PCWSTR , dwdesiredaccess : u32 , lpdwgrantedaccess : *mut u32 ) -> *mut _HNETINTERFACE );
    OpenClusterNetInterfaceEx(hcluster, lpszinterfacename.into().abi(), dwdesiredaccess, ::core::mem::transmute(lpdwgrantedaccess.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterNetwork<P0>(hcluster: *const _HCLUSTER, lpsznetworkname: P0) -> *mut _HNETWORK
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterNetwork ( hcluster : *const _HCLUSTER , lpsznetworkname : :: windows::core::PCWSTR ) -> *mut _HNETWORK );
    OpenClusterNetwork(hcluster, lpsznetworkname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterNetworkEx<P0>(hcluster: *const _HCLUSTER, lpsznetworkname: P0, dwdesiredaccess: u32, lpdwgrantedaccess: ::core::option::Option<*mut u32>) -> *mut _HNETWORK
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterNetworkEx ( hcluster : *const _HCLUSTER , lpsznetworkname : :: windows::core::PCWSTR , dwdesiredaccess : u32 , lpdwgrantedaccess : *mut u32 ) -> *mut _HNETWORK );
    OpenClusterNetworkEx(hcluster, lpsznetworkname.into().abi(), dwdesiredaccess, ::core::mem::transmute(lpdwgrantedaccess.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterNode<P0>(hcluster: *const _HCLUSTER, lpsznodename: P0) -> *mut _HNODE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterNode ( hcluster : *const _HCLUSTER , lpsznodename : :: windows::core::PCWSTR ) -> *mut _HNODE );
    OpenClusterNode(hcluster, lpsznodename.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterNodeById(hcluster: *const _HCLUSTER, nodeid: u32) -> *mut _HNODE {
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterNodeById ( hcluster : *const _HCLUSTER , nodeid : u32 ) -> *mut _HNODE );
    OpenClusterNodeById(hcluster, nodeid)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterNodeEx<P0>(hcluster: *const _HCLUSTER, lpsznodename: P0, dwdesiredaccess: u32, lpdwgrantedaccess: ::core::option::Option<*mut u32>) -> *mut _HNODE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterNodeEx ( hcluster : *const _HCLUSTER , lpsznodename : :: windows::core::PCWSTR , dwdesiredaccess : u32 , lpdwgrantedaccess : *mut u32 ) -> *mut _HNODE );
    OpenClusterNodeEx(hcluster, lpsznodename.into().abi(), dwdesiredaccess, ::core::mem::transmute(lpdwgrantedaccess.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterResource<P0>(hcluster: *const _HCLUSTER, lpszresourcename: P0) -> *mut _HRESOURCE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterResource ( hcluster : *const _HCLUSTER , lpszresourcename : :: windows::core::PCWSTR ) -> *mut _HRESOURCE );
    OpenClusterResource(hcluster, lpszresourcename.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn OpenClusterResourceEx<P0>(hcluster: *const _HCLUSTER, lpszresourcename: P0, dwdesiredaccess: u32, lpdwgrantedaccess: ::core::option::Option<*mut u32>) -> *mut _HRESOURCE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn OpenClusterResourceEx ( hcluster : *const _HCLUSTER , lpszresourcename : :: windows::core::PCWSTR , dwdesiredaccess : u32 , lpdwgrantedaccess : *mut u32 ) -> *mut _HRESOURCE );
    OpenClusterResourceEx(hcluster, lpszresourcename.into().abi(), dwdesiredaccess, ::core::mem::transmute(lpdwgrantedaccess.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn PauseClusterNode(hnode: *const _HNODE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn PauseClusterNode ( hnode : *const _HNODE ) -> u32 );
    PauseClusterNode(hnode)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PauseClusterNodeEx<P0>(hnode: *const _HNODE, bdrainnode: P0, dwpauseflags: u32, hnodedraintarget: ::core::option::Option<*const _HNODE>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn PauseClusterNodeEx ( hnode : *const _HNODE , bdrainnode : super::super::Foundation:: BOOL , dwpauseflags : u32 , hnodedraintarget : *const _HNODE ) -> u32 );
    PauseClusterNodeEx(hnode, bdrainnode.into(), dwpauseflags, ::core::mem::transmute(hnodedraintarget.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryAppInstanceVersion(appinstanceid: *const ::windows::core::GUID, instanceversionhigh: *mut u64, instanceversionlow: *mut u64, versionstatus: *mut super::super::Foundation::NTSTATUS) -> u32 {
    ::windows::core::link ! ( "ntlanman.dll""system" fn QueryAppInstanceVersion ( appinstanceid : *const :: windows::core::GUID , instanceversionhigh : *mut u64 , instanceversionlow : *mut u64 , versionstatus : *mut super::super::Foundation:: NTSTATUS ) -> u32 );
    QueryAppInstanceVersion(appinstanceid, instanceversionhigh, instanceversionlow, versionstatus)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterAppInstance<P0, P1>(processhandle: P0, appinstanceid: *const ::windows::core::GUID, childreninheritappinstance: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ntlanman.dll""system" fn RegisterAppInstance ( processhandle : super::super::Foundation:: HANDLE , appinstanceid : *const :: windows::core::GUID , childreninheritappinstance : super::super::Foundation:: BOOL ) -> u32 );
    RegisterAppInstance(processhandle.into(), appinstanceid, childreninheritappinstance.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn RegisterAppInstanceVersion(appinstanceid: *const ::windows::core::GUID, instanceversionhigh: u64, instanceversionlow: u64) -> u32 {
    ::windows::core::link ! ( "ntlanman.dll""system" fn RegisterAppInstanceVersion ( appinstanceid : *const :: windows::core::GUID , instanceversionhigh : u64 , instanceversionlow : u64 ) -> u32 );
    RegisterAppInstanceVersion(appinstanceid, instanceversionhigh, instanceversionlow)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterClusterNotify<P0>(hchange: *const _HCHANGE, dwfiltertype: u32, hobject: P0, dwnotifykey: usize) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn RegisterClusterNotify ( hchange : *const _HCHANGE , dwfiltertype : u32 , hobject : super::super::Foundation:: HANDLE , dwnotifykey : usize ) -> u32 );
    RegisterClusterNotify(hchange, dwfiltertype, hobject.into(), dwnotifykey)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterClusterNotifyV2<P0>(hchange: *const _HCHANGE, filter: NOTIFY_FILTER_AND_TYPE, hobject: P0, dwnotifykey: usize) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn RegisterClusterNotifyV2 ( hchange : *const _HCHANGE , filter : NOTIFY_FILTER_AND_TYPE , hobject : super::super::Foundation:: HANDLE , dwnotifykey : usize ) -> u32 );
    RegisterClusterNotifyV2(hchange, ::core::mem::transmute(filter), hobject.into(), dwnotifykey)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn RegisterClusterResourceTypeNotifyV2<P0>(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, flags: i64, restypename: P0, dwnotifykey: usize) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn RegisterClusterResourceTypeNotifyV2 ( hchange : *const _HCHANGE , hcluster : *const _HCLUSTER , flags : i64 , restypename : :: windows::core::PCWSTR , dwnotifykey : usize ) -> u32 );
    RegisterClusterResourceTypeNotifyV2(hchange, hcluster, flags, restypename.into().abi(), dwnotifykey)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn RemoveClusterGroupDependency(hgroup: *const _HGROUP, hdependson: *const _HGROUP) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn RemoveClusterGroupDependency ( hgroup : *const _HGROUP , hdependson : *const _HGROUP ) -> u32 );
    RemoveClusterGroupDependency(hgroup, hdependson)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn RemoveClusterGroupSetDependency(hgroupset: *const _HGROUPSET, hdependson: *const _HGROUPSET) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn RemoveClusterGroupSetDependency ( hgroupset : *const _HGROUPSET , hdependson : *const _HGROUPSET ) -> u32 );
    RemoveClusterGroupSetDependency(hgroupset, hdependson)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn RemoveClusterGroupToGroupSetDependency(hgroup: *const _HGROUP, hdependson: *const _HGROUPSET) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn RemoveClusterGroupToGroupSetDependency ( hgroup : *const _HGROUP , hdependson : *const _HGROUPSET ) -> u32 );
    RemoveClusterGroupToGroupSetDependency(hgroup, hdependson)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveClusterNameAccount<P0>(hcluster: *const _HCLUSTER, bdeletecomputerobjects: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn RemoveClusterNameAccount ( hcluster : *const _HCLUSTER , bdeletecomputerobjects : super::super::Foundation:: BOOL ) -> u32 );
    RemoveClusterNameAccount(hcluster, bdeletecomputerobjects.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn RemoveClusterResourceDependency(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn RemoveClusterResourceDependency ( hresource : *const _HRESOURCE , hdependson : *const _HRESOURCE ) -> u32 );
    RemoveClusterResourceDependency(hresource, hdependson)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn RemoveClusterResourceNode(hresource: *const _HRESOURCE, hnode: *const _HNODE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn RemoveClusterResourceNode ( hresource : *const _HRESOURCE , hnode : *const _HNODE ) -> u32 );
    RemoveClusterResourceNode(hresource, hnode)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn RemoveClusterStorageNode<P0>(hcluster: *const _HCLUSTER, lpszclusterstorageenclosurename: P0, dwtimeout: u32, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn RemoveClusterStorageNode ( hcluster : *const _HCLUSTER , lpszclusterstorageenclosurename : :: windows::core::PCWSTR , dwtimeout : u32 , dwflags : u32 ) -> u32 );
    RemoveClusterStorageNode(hcluster, lpszclusterstorageenclosurename.into().abi(), dwtimeout, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn RemoveCrossClusterGroupSetDependency<P0, P1>(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: P0, lpremotegroupsetname: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn RemoveCrossClusterGroupSetDependency ( hdependentgroupset : *const _HGROUPSET , lpremoteclustername : :: windows::core::PCWSTR , lpremotegroupsetname : :: windows::core::PCWSTR ) -> u32 );
    RemoveCrossClusterGroupSetDependency(hdependentgroupset, lpremoteclustername.into().abi(), lpremotegroupsetname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn RemoveResourceFromClusterSharedVolumes(hresource: *const _HRESOURCE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn RemoveResourceFromClusterSharedVolumes ( hresource : *const _HRESOURCE ) -> u32 );
    RemoveResourceFromClusterSharedVolumes(hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilAddUnknownProperties<P0>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilAddUnknownProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutpropertylist : *mut ::core::ffi::c_void , pcboutpropertylistsize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
    ResUtilAddUnknownProperties(hkeyclusterkey.into(), ppropertytable, poutpropertylist, pcboutpropertylistsize, pcbbytesreturned, pcbrequired)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilCreateDirectoryTree<P0>(pszpath: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilCreateDirectoryTree ( pszpath : :: windows::core::PCWSTR ) -> u32 );
    ResUtilCreateDirectoryTree(pszpath.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilDupGroup(group: *mut _HGROUP, copy: *mut *mut _HGROUP) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilDupGroup ( group : *mut _HGROUP , copy : *mut *mut _HGROUP ) -> u32 );
    ResUtilDupGroup(group, copy)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilDupParameterBlock(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilDupParameterBlock ( poutparams : *mut u8 , pinparams : *const u8 , ppropertytable : *const RESUTIL_PROPERTY_ITEM ) -> u32 );
    ResUtilDupParameterBlock(poutparams, pinparams, ppropertytable)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilDupResource(group: *mut _HRESOURCE, copy: *mut *mut _HRESOURCE) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilDupResource ( group : *mut _HRESOURCE , copy : *mut *mut _HRESOURCE ) -> u32 );
    ResUtilDupResource(group, copy)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilDupString<P0>(pszinstring: P0) -> ::windows::core::PWSTR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilDupString ( pszinstring : :: windows::core::PCWSTR ) -> :: windows::core::PWSTR );
    ResUtilDupString(pszinstring.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilEnumGroups(hcluster: *mut _HCLUSTER, hself: *mut _HGROUP, prescallback: LPGROUP_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilEnumGroups ( hcluster : *mut _HCLUSTER , hself : *mut _HGROUP , prescallback : LPGROUP_CALLBACK_EX , pparameter : *mut ::core::ffi::c_void ) -> u32 );
    ResUtilEnumGroups(hcluster, hself, prescallback, pparameter)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilEnumGroupsEx(hcluster: *mut _HCLUSTER, hself: *mut _HGROUP, grouptype: CLUSGROUP_TYPE, prescallback: LPGROUP_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilEnumGroupsEx ( hcluster : *mut _HCLUSTER , hself : *mut _HGROUP , grouptype : CLUSGROUP_TYPE , prescallback : LPGROUP_CALLBACK_EX , pparameter : *mut ::core::ffi::c_void ) -> u32 );
    ResUtilEnumGroupsEx(hcluster, hself, grouptype, prescallback, pparameter)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilEnumPrivateProperties<P0>(hkeyclusterkey: P0, pszoutproperties: ::windows::core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilEnumPrivateProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszoutproperties : :: windows::core::PWSTR , cboutpropertiessize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
    ResUtilEnumPrivateProperties(hkeyclusterkey.into(), ::core::mem::transmute(pszoutproperties), cboutpropertiessize, pcbbytesreturned, pcbrequired)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilEnumProperties(ppropertytable: *const RESUTIL_PROPERTY_ITEM, pszoutproperties: ::windows::core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilEnumProperties ( ppropertytable : *const RESUTIL_PROPERTY_ITEM , pszoutproperties : :: windows::core::PWSTR , cboutpropertiessize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
    ResUtilEnumProperties(ppropertytable, ::core::mem::transmute(pszoutproperties), cboutpropertiessize, pcbbytesreturned, pcbrequired)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilEnumResources<P0>(hself: *mut _HRESOURCE, lpszrestypename: P0, prescallback: LPRESOURCE_CALLBACK, pparameter: *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilEnumResources ( hself : *mut _HRESOURCE , lpszrestypename : :: windows::core::PCWSTR , prescallback : LPRESOURCE_CALLBACK , pparameter : *mut ::core::ffi::c_void ) -> u32 );
    ResUtilEnumResources(hself, lpszrestypename.into().abi(), prescallback, pparameter)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilEnumResourcesEx<P0>(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: P0, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilEnumResourcesEx ( hcluster : *mut _HCLUSTER , hself : *mut _HRESOURCE , lpszrestypename : :: windows::core::PCWSTR , prescallback : LPRESOURCE_CALLBACK_EX , pparameter : *mut ::core::ffi::c_void ) -> u32 );
    ResUtilEnumResourcesEx(hcluster, hself, lpszrestypename.into().abi(), prescallback, pparameter)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilEnumResourcesEx2<P0>(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: P0, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void, dwdesiredaccess: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilEnumResourcesEx2 ( hcluster : *mut _HCLUSTER , hself : *mut _HRESOURCE , lpszrestypename : :: windows::core::PCWSTR , prescallback : LPRESOURCE_CALLBACK_EX , pparameter : *mut ::core::ffi::c_void , dwdesiredaccess : u32 ) -> u32 );
    ResUtilEnumResourcesEx2(hcluster, hself, lpszrestypename.into().abi(), prescallback, pparameter, dwdesiredaccess)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilExpandEnvironmentStrings<P0>(pszsrc: P0) -> ::windows::core::PWSTR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilExpandEnvironmentStrings ( pszsrc : :: windows::core::PCWSTR ) -> :: windows::core::PWSTR );
    ResUtilExpandEnvironmentStrings(pszsrc.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilFindBinaryProperty<P0>(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pbpropertyvalue: ::core::option::Option<*mut *mut u8>, pcbpropertyvaluesize: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFindBinaryProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows::core::PCWSTR , pbpropertyvalue : *mut *mut u8 , pcbpropertyvaluesize : *mut u32 ) -> u32 );
    ResUtilFindBinaryProperty(ppropertylist, cbpropertylistsize, pszpropertyname.into().abi(), ::core::mem::transmute(pbpropertyvalue.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbpropertyvaluesize.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilFindDependentDiskResourceDriveLetter(hcluster: *const _HCLUSTER, hresource: *const _HRESOURCE, pszdriveletter: ::windows::core::PWSTR, pcchdriveletter: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFindDependentDiskResourceDriveLetter ( hcluster : *const _HCLUSTER , hresource : *const _HRESOURCE , pszdriveletter : :: windows::core::PWSTR , pcchdriveletter : *mut u32 ) -> u32 );
    ResUtilFindDependentDiskResourceDriveLetter(hcluster, hresource, ::core::mem::transmute(pszdriveletter), pcchdriveletter)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilFindDwordProperty<P0>(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pdwpropertyvalue: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFindDwordProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows::core::PCWSTR , pdwpropertyvalue : *mut u32 ) -> u32 );
    ResUtilFindDwordProperty(ppropertylist, cbpropertylistsize, pszpropertyname.into().abi(), pdwpropertyvalue)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilFindExpandSzProperty<P0>(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pszpropertyvalue: ::core::option::Option<*mut ::windows::core::PWSTR>) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFindExpandSzProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows::core::PCWSTR , pszpropertyvalue : *mut :: windows::core::PWSTR ) -> u32 );
    ResUtilFindExpandSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.into().abi(), ::core::mem::transmute(pszpropertyvalue.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilFindExpandedSzProperty<P0>(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pszpropertyvalue: ::core::option::Option<*mut ::windows::core::PWSTR>) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFindExpandedSzProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows::core::PCWSTR , pszpropertyvalue : *mut :: windows::core::PWSTR ) -> u32 );
    ResUtilFindExpandedSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.into().abi(), ::core::mem::transmute(pszpropertyvalue.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilFindFileTimeProperty<P0>(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pftpropertyvalue: *mut super::super::Foundation::FILETIME) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFindFileTimeProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows::core::PCWSTR , pftpropertyvalue : *mut super::super::Foundation:: FILETIME ) -> u32 );
    ResUtilFindFileTimeProperty(ppropertylist, cbpropertylistsize, pszpropertyname.into().abi(), pftpropertyvalue)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilFindLongProperty<P0>(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, plpropertyvalue: *mut i32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFindLongProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows::core::PCWSTR , plpropertyvalue : *mut i32 ) -> u32 );
    ResUtilFindLongProperty(ppropertylist, cbpropertylistsize, pszpropertyname.into().abi(), plpropertyvalue)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilFindMultiSzProperty<P0>(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pszpropertyvalue: *mut ::windows::core::PWSTR, pcbpropertyvaluesize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFindMultiSzProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows::core::PCWSTR , pszpropertyvalue : *mut :: windows::core::PWSTR , pcbpropertyvaluesize : *mut u32 ) -> u32 );
    ResUtilFindMultiSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.into().abi(), pszpropertyvalue, pcbpropertyvaluesize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilFindSzProperty<P0>(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pszpropertyvalue: ::core::option::Option<*mut ::windows::core::PWSTR>) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFindSzProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows::core::PCWSTR , pszpropertyvalue : *mut :: windows::core::PWSTR ) -> u32 );
    ResUtilFindSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.into().abi(), ::core::mem::transmute(pszpropertyvalue.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilFindULargeIntegerProperty<P0>(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, plpropertyvalue: *mut u64) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFindULargeIntegerProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows::core::PCWSTR , plpropertyvalue : *mut u64 ) -> u32 );
    ResUtilFindULargeIntegerProperty(ppropertylist, cbpropertylistsize, pszpropertyname.into().abi(), plpropertyvalue)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilFreeEnvironment(lpenvironment: *mut ::core::ffi::c_void) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFreeEnvironment ( lpenvironment : *mut ::core::ffi::c_void ) -> u32 );
    ResUtilFreeEnvironment(lpenvironment)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilFreeParameterBlock(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilFreeParameterBlock ( poutparams : *mut u8 , pinparams : *const u8 , ppropertytable : *const RESUTIL_PROPERTY_ITEM ) -> ( ) );
    ResUtilFreeParameterBlock(poutparams, pinparams, ppropertytable)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilGetAllProperties<P0>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetAllProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutpropertylist : *mut ::core::ffi::c_void , cboutpropertylistsize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
    ResUtilGetAllProperties(hkeyclusterkey.into(), ppropertytable, poutpropertylist, cboutpropertylistsize, pcbbytesreturned, pcbrequired)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetBinaryProperty(ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_BINARY, pboldvalue: ::core::option::Option<&[u8]>, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetBinaryProperty ( ppboutvalue : *mut *mut u8 , pcboutvaluesize : *mut u32 , pvaluestruct : *const CLUSPROP_BINARY , pboldvalue : *const u8 , cboldvaluesize : u32 , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
    ResUtilGetBinaryProperty(ppboutvalue, pcboutvaluesize, pvaluestruct, ::core::mem::transmute(pboldvalue.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pboldvalue.as_deref().map_or(0, |slice| slice.len() as _), pppropertylist, pcbpropertylistsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetBinaryValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetBinaryValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows::core::PCWSTR , ppboutvalue : *mut *mut u8 , pcboutvaluesize : *mut u32 ) -> u32 );
    ResUtilGetBinaryValue(hkeyclusterkey.into(), pszvaluename.into().abi(), ppboutvalue, pcboutvaluesize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetClusterGroupType(hgroup: *mut _HGROUP, grouptype: *mut CLUSGROUP_TYPE) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetClusterGroupType ( hgroup : *mut _HGROUP , grouptype : *mut CLUSGROUP_TYPE ) -> u32 );
    ResUtilGetClusterGroupType(hgroup, grouptype)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetClusterId(hcluster: *mut _HCLUSTER, guid: *mut ::windows::core::GUID) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetClusterId ( hcluster : *mut _HCLUSTER , guid : *mut :: windows::core::GUID ) -> u32 );
    ResUtilGetClusterId(hcluster, guid)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetClusterRoleState(hcluster: *const _HCLUSTER, eclusterrole: CLUSTER_ROLE) -> CLUSTER_ROLE_STATE {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetClusterRoleState ( hcluster : *const _HCLUSTER , eclusterrole : CLUSTER_ROLE ) -> CLUSTER_ROLE_STATE );
    ResUtilGetClusterRoleState(hcluster, eclusterrole)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetCoreClusterResources(hcluster: *const _HCLUSTER, phclusternameresource: *mut *mut _HRESOURCE, phclusteripaddressresource: *mut *mut _HRESOURCE, phclusterquorumresource: *mut *mut _HRESOURCE) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetCoreClusterResources ( hcluster : *const _HCLUSTER , phclusternameresource : *mut *mut _HRESOURCE , phclusteripaddressresource : *mut *mut _HRESOURCE , phclusterquorumresource : *mut *mut _HRESOURCE ) -> u32 );
    ResUtilGetCoreClusterResources(hcluster, phclusternameresource, phclusteripaddressresource, phclusterquorumresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetCoreClusterResourcesEx(hclusterin: *const _HCLUSTER, phclusternameresourceout: ::core::option::Option<*mut *mut _HRESOURCE>, phclusterquorumresourceout: ::core::option::Option<*mut *mut _HRESOURCE>, dwdesiredaccess: u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetCoreClusterResourcesEx ( hclusterin : *const _HCLUSTER , phclusternameresourceout : *mut *mut _HRESOURCE , phclusterquorumresourceout : *mut *mut _HRESOURCE , dwdesiredaccess : u32 ) -> u32 );
    ResUtilGetCoreClusterResourcesEx(hclusterin, ::core::mem::transmute(phclusternameresourceout.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phclusterquorumresourceout.unwrap_or(::std::ptr::null_mut())), dwdesiredaccess)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetCoreGroup(hcluster: *mut _HCLUSTER) -> *mut _HGROUP {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetCoreGroup ( hcluster : *mut _HCLUSTER ) -> *mut _HGROUP );
    ResUtilGetCoreGroup(hcluster)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetDwordProperty(pdwoutvalue: *mut u32, pvaluestruct: *const CLUSPROP_DWORD, dwoldvalue: u32, dwminimum: u32, dwmaximum: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetDwordProperty ( pdwoutvalue : *mut u32 , pvaluestruct : *const CLUSPROP_DWORD , dwoldvalue : u32 , dwminimum : u32 , dwmaximum : u32 , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
    ResUtilGetDwordProperty(pdwoutvalue, pvaluestruct, dwoldvalue, dwminimum, dwmaximum, pppropertylist, pcbpropertylistsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetDwordValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, pdwoutvalue: *mut u32, dwdefaultvalue: u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetDwordValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows::core::PCWSTR , pdwoutvalue : *mut u32 , dwdefaultvalue : u32 ) -> u32 );
    ResUtilGetDwordValue(hkeyclusterkey.into(), pszvaluename.into().abi(), pdwoutvalue, dwdefaultvalue)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetEnvironmentWithNetName(hresource: *const _HRESOURCE) -> *mut ::core::ffi::c_void {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetEnvironmentWithNetName ( hresource : *const _HRESOURCE ) -> *mut ::core::ffi::c_void );
    ResUtilGetEnvironmentWithNetName(hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilGetFileTimeProperty(pftoutvalue: *mut super::super::Foundation::FILETIME, pvaluestruct: *const CLUSPROP_FILETIME, ftoldvalue: super::super::Foundation::FILETIME, ftminimum: super::super::Foundation::FILETIME, ftmaximum: super::super::Foundation::FILETIME, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetFileTimeProperty ( pftoutvalue : *mut super::super::Foundation:: FILETIME , pvaluestruct : *const CLUSPROP_FILETIME , ftoldvalue : super::super::Foundation:: FILETIME , ftminimum : super::super::Foundation:: FILETIME , ftmaximum : super::super::Foundation:: FILETIME , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
    ResUtilGetFileTimeProperty(pftoutvalue, pvaluestruct, ::core::mem::transmute(ftoldvalue), ::core::mem::transmute(ftminimum), ::core::mem::transmute(ftmaximum), pppropertylist, pcbpropertylistsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetLongProperty(ploutvalue: *mut i32, pvaluestruct: *const CLUSPROP_LONG, loldvalue: i32, lminimum: i32, lmaximum: i32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetLongProperty ( ploutvalue : *mut i32 , pvaluestruct : *const CLUSPROP_LONG , loldvalue : i32 , lminimum : i32 , lmaximum : i32 , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
    ResUtilGetLongProperty(ploutvalue, pvaluestruct, loldvalue, lminimum, lmaximum, pppropertylist, pcbpropertylistsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetMultiSzProperty<P0>(ppszoutvalue: *mut ::windows::core::PWSTR, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: P0, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetMultiSzProperty ( ppszoutvalue : *mut :: windows::core::PWSTR , pcboutvaluesize : *mut u32 , pvaluestruct : *const CLUSPROP_SZ , pszoldvalue : :: windows::core::PCWSTR , cboldvaluesize : u32 , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
    ResUtilGetMultiSzProperty(ppszoutvalue, pcboutvaluesize, pvaluestruct, pszoldvalue.into().abi(), cboldvaluesize, pppropertylist, pcbpropertylistsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetPrivateProperties<P0>(hkeyclusterkey: P0, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetPrivateProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , poutpropertylist : *mut ::core::ffi::c_void , cboutpropertylistsize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
    ResUtilGetPrivateProperties(hkeyclusterkey.into(), poutpropertylist, cboutpropertylistsize, pcbbytesreturned, pcbrequired)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilGetProperties<P0>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutpropertylist : *mut ::core::ffi::c_void , cboutpropertylistsize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
    ResUtilGetProperties(hkeyclusterkey.into(), ppropertytable, poutpropertylist, cboutpropertylistsize, pcbbytesreturned, pcbrequired)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilGetPropertiesToParameterBlock<P0, P1>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutparams: *mut u8, bcheckforrequiredproperties: P1, psznameofpropinerror: *mut ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetPropertiesToParameterBlock ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutparams : *mut u8 , bcheckforrequiredproperties : super::super::Foundation:: BOOL , psznameofpropinerror : *mut :: windows::core::PWSTR ) -> u32 );
    ResUtilGetPropertiesToParameterBlock(hkeyclusterkey.into(), ppropertytable, poutparams, bcheckforrequiredproperties.into(), psznameofpropinerror)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilGetProperty<P0>(hkeyclusterkey: P0, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, poutpropertyitem: *mut *mut ::core::ffi::c_void, pcboutpropertyitemsize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetProperty ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytableitem : *const RESUTIL_PROPERTY_ITEM , poutpropertyitem : *mut *mut ::core::ffi::c_void , pcboutpropertyitemsize : *mut u32 ) -> u32 );
    ResUtilGetProperty(hkeyclusterkey.into(), ppropertytableitem, poutpropertyitem, pcboutpropertyitemsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilGetPropertyFormats(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist: *mut ::core::ffi::c_void, cbpropertyformatlistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetPropertyFormats ( ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutpropertyformatlist : *mut ::core::ffi::c_void , cbpropertyformatlistsize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
    ResUtilGetPropertyFormats(ppropertytable, poutpropertyformatlist, cbpropertyformatlistsize, pcbbytesreturned, pcbrequired)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilGetPropertySize<P0>(hkeyclusterkey: P0, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize: *mut u32, pnpropertycount: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetPropertySize ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytableitem : *const RESUTIL_PROPERTY_ITEM , pcboutpropertylistsize : *mut u32 , pnpropertycount : *mut u32 ) -> u32 );
    ResUtilGetPropertySize(hkeyclusterkey.into(), ppropertytableitem, pcboutpropertylistsize, pnpropertycount)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetQwordValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, pqwoutvalue: *mut u64, qwdefaultvalue: u64) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetQwordValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows::core::PCWSTR , pqwoutvalue : *mut u64 , qwdefaultvalue : u64 ) -> u32 );
    ResUtilGetQwordValue(hkeyclusterkey.into(), pszvaluename.into().abi(), pqwoutvalue, qwdefaultvalue)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilGetResourceDependency<P0, P1>(hself: P0, lpszresourcetype: P1) -> *mut _HRESOURCE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetResourceDependency ( hself : super::super::Foundation:: HANDLE , lpszresourcetype : :: windows::core::PCWSTR ) -> *mut _HRESOURCE );
    ResUtilGetResourceDependency(hself.into(), lpszresourcetype.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByClass<P0, P1>(hcluster: *mut _HCLUSTER, hself: P0, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: P1) -> *mut _HRESOURCE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetResourceDependencyByClass ( hcluster : *mut _HCLUSTER , hself : super::super::Foundation:: HANDLE , prci : *mut CLUS_RESOURCE_CLASS_INFO , brecurse : super::super::Foundation:: BOOL ) -> *mut _HRESOURCE );
    ResUtilGetResourceDependencyByClass(hcluster, hself.into(), prci, brecurse.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByClassEx<P0, P1>(hcluster: *mut _HCLUSTER, hself: P0, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: P1, dwdesiredaccess: u32) -> *mut _HRESOURCE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetResourceDependencyByClassEx ( hcluster : *mut _HCLUSTER , hself : super::super::Foundation:: HANDLE , prci : *mut CLUS_RESOURCE_CLASS_INFO , brecurse : super::super::Foundation:: BOOL , dwdesiredaccess : u32 ) -> *mut _HRESOURCE );
    ResUtilGetResourceDependencyByClassEx(hcluster, hself.into(), prci, brecurse.into(), dwdesiredaccess)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByName<P0, P1, P2>(hcluster: *mut _HCLUSTER, hself: P0, lpszresourcetype: P1, brecurse: P2) -> *mut _HRESOURCE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetResourceDependencyByName ( hcluster : *mut _HCLUSTER , hself : super::super::Foundation:: HANDLE , lpszresourcetype : :: windows::core::PCWSTR , brecurse : super::super::Foundation:: BOOL ) -> *mut _HRESOURCE );
    ResUtilGetResourceDependencyByName(hcluster, hself.into(), lpszresourcetype.into().abi(), brecurse.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByNameEx<P0, P1, P2>(hcluster: *mut _HCLUSTER, hself: P0, lpszresourcetype: P1, brecurse: P2, dwdesiredaccess: u32) -> *mut _HRESOURCE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetResourceDependencyByNameEx ( hcluster : *mut _HCLUSTER , hself : super::super::Foundation:: HANDLE , lpszresourcetype : :: windows::core::PCWSTR , brecurse : super::super::Foundation:: BOOL , dwdesiredaccess : u32 ) -> *mut _HRESOURCE );
    ResUtilGetResourceDependencyByNameEx(hcluster, hself.into(), lpszresourcetype.into().abi(), brecurse.into(), dwdesiredaccess)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilGetResourceDependencyEx<P0, P1>(hself: P0, lpszresourcetype: P1, dwdesiredaccess: u32) -> *mut _HRESOURCE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetResourceDependencyEx ( hself : super::super::Foundation:: HANDLE , lpszresourcetype : :: windows::core::PCWSTR , dwdesiredaccess : u32 ) -> *mut _HRESOURCE );
    ResUtilGetResourceDependencyEx(hself.into(), lpszresourcetype.into().abi(), dwdesiredaccess)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetResourceDependentIPAddressProps(hresource: *const _HRESOURCE, pszaddress: ::windows::core::PWSTR, pcchaddress: *mut u32, pszsubnetmask: ::windows::core::PWSTR, pcchsubnetmask: *mut u32, psznetwork: ::windows::core::PWSTR, pcchnetwork: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetResourceDependentIPAddressProps ( hresource : *const _HRESOURCE , pszaddress : :: windows::core::PWSTR , pcchaddress : *mut u32 , pszsubnetmask : :: windows::core::PWSTR , pcchsubnetmask : *mut u32 , psznetwork : :: windows::core::PWSTR , pcchnetwork : *mut u32 ) -> u32 );
    ResUtilGetResourceDependentIPAddressProps(hresource, ::core::mem::transmute(pszaddress), pcchaddress, ::core::mem::transmute(pszsubnetmask), pcchsubnetmask, ::core::mem::transmute(psznetwork), pcchnetwork)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetResourceName(hresource: *const _HRESOURCE, pszresourcename: ::windows::core::PWSTR, pcchresourcenameinout: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetResourceName ( hresource : *const _HRESOURCE , pszresourcename : :: windows::core::PWSTR , pcchresourcenameinout : *mut u32 ) -> u32 );
    ResUtilGetResourceName(hresource, ::core::mem::transmute(pszresourcename), pcchresourcenameinout)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetResourceNameDependency<P0, P1>(lpszresourcename: P0, lpszresourcetype: P1) -> *mut _HRESOURCE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetResourceNameDependency ( lpszresourcename : :: windows::core::PCWSTR , lpszresourcetype : :: windows::core::PCWSTR ) -> *mut _HRESOURCE );
    ResUtilGetResourceNameDependency(lpszresourcename.into().abi(), lpszresourcetype.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetResourceNameDependencyEx<P0, P1>(lpszresourcename: P0, lpszresourcetype: P1, dwdesiredaccess: u32) -> *mut _HRESOURCE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetResourceNameDependencyEx ( lpszresourcename : :: windows::core::PCWSTR , lpszresourcetype : :: windows::core::PCWSTR , dwdesiredaccess : u32 ) -> *mut _HRESOURCE );
    ResUtilGetResourceNameDependencyEx(lpszresourcename.into().abi(), lpszresourcetype.into().abi(), dwdesiredaccess)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilGetSzProperty<P0>(ppszoutvalue: *mut ::windows::core::PWSTR, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: P0, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetSzProperty ( ppszoutvalue : *mut :: windows::core::PWSTR , pvaluestruct : *const CLUSPROP_SZ , pszoldvalue : :: windows::core::PCWSTR , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
    ResUtilGetSzProperty(ppszoutvalue, pvaluestruct, pszoldvalue.into().abi(), pppropertylist, pcbpropertylistsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetSzValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1) -> ::windows::core::PWSTR
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGetSzValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows::core::PCWSTR ) -> :: windows::core::PWSTR );
    ResUtilGetSzValue(hkeyclusterkey.into(), pszvaluename.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilGroupsEqual(hself: *mut _HGROUP, hgroup: *mut _HGROUP, pequal: *mut super::super::Foundation::BOOL) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilGroupsEqual ( hself : *mut _HGROUP , hgroup : *mut _HGROUP , pequal : *mut super::super::Foundation:: BOOL ) -> u32 );
    ResUtilGroupsEqual(hself, hgroup, pequal)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilIsPathValid<P0>(pszpath: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilIsPathValid ( pszpath : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    ResUtilIsPathValid(pszpath.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilIsResourceClassEqual(prci: *mut CLUS_RESOURCE_CLASS_INFO, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilIsResourceClassEqual ( prci : *mut CLUS_RESOURCE_CLASS_INFO , hresource : *mut _HRESOURCE ) -> super::super::Foundation:: BOOL );
    ResUtilIsResourceClassEqual(prci, hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilLeftPaxosIsLessThanRight(left: *const PaxosTagCStruct, right: *const PaxosTagCStruct) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilLeftPaxosIsLessThanRight ( left : *const PaxosTagCStruct , right : *const PaxosTagCStruct ) -> super::super::Foundation:: BOOL );
    ResUtilLeftPaxosIsLessThanRight(left, right)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilNodeEnum(hcluster: *mut _HCLUSTER, pnodecallback: LPNODE_CALLBACK, pparameter: *mut ::core::ffi::c_void) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilNodeEnum ( hcluster : *mut _HCLUSTER , pnodecallback : LPNODE_CALLBACK , pparameter : *mut ::core::ffi::c_void ) -> u32 );
    ResUtilNodeEnum(hcluster, pnodecallback, pparameter)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilPaxosComparer(left: *const PaxosTagCStruct, right: *const PaxosTagCStruct) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilPaxosComparer ( left : *const PaxosTagCStruct , right : *const PaxosTagCStruct ) -> super::super::Foundation:: BOOL );
    ResUtilPaxosComparer(left, right)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilPropertyListFromParameterBlock(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: ::core::option::Option<*mut ::core::ffi::c_void>, pcboutpropertylistsize: *mut u32, pinparams: *const u8, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilPropertyListFromParameterBlock ( ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutpropertylist : *mut ::core::ffi::c_void , pcboutpropertylistsize : *mut u32 , pinparams : *const u8 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
    ResUtilPropertyListFromParameterBlock(ppropertytable, ::core::mem::transmute(poutpropertylist.unwrap_or(::std::ptr::null_mut())), pcboutpropertylistsize, pinparams, pcbbytesreturned, pcbrequired)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilRemoveResourceServiceEnvironment<P0>(pszservicename: P0, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilRemoveResourceServiceEnvironment ( pszservicename : :: windows::core::PCWSTR , pfnlogevent : PLOG_EVENT_ROUTINE , hresourcehandle : isize ) -> u32 );
    ResUtilRemoveResourceServiceEnvironment(pszservicename.into().abi(), pfnlogevent, hresourcehandle)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilResourceDepEnum(hself: *mut _HRESOURCE, enumtype: u32, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilResourceDepEnum ( hself : *mut _HRESOURCE , enumtype : u32 , prescallback : LPRESOURCE_CALLBACK_EX , pparameter : *mut ::core::ffi::c_void ) -> u32 );
    ResUtilResourceDepEnum(hself, enumtype, prescallback, pparameter)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilResourceTypesEqual<P0>(lpszresourcetypename: P0, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilResourceTypesEqual ( lpszresourcetypename : :: windows::core::PCWSTR , hresource : *mut _HRESOURCE ) -> super::super::Foundation:: BOOL );
    ResUtilResourceTypesEqual(lpszresourcetypename.into().abi(), hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilResourcesEqual(hself: *mut _HRESOURCE, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilResourcesEqual ( hself : *mut _HRESOURCE , hresource : *mut _HRESOURCE ) -> super::super::Foundation:: BOOL );
    ResUtilResourcesEqual(hself, hresource)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetBinaryValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, pbnewvalue: &[u8], ppboutvalue: ::core::option::Option<*mut *mut u8>, pcboutvaluesize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetBinaryValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows::core::PCWSTR , pbnewvalue : *const u8 , cbnewvaluesize : u32 , ppboutvalue : *mut *mut u8 , pcboutvaluesize : *mut u32 ) -> u32 );
    ResUtilSetBinaryValue(hkeyclusterkey.into(), pszvaluename.into().abi(), ::core::mem::transmute(pbnewvalue.as_ptr()), pbnewvalue.len() as _, ::core::mem::transmute(ppboutvalue.unwrap_or(::std::ptr::null_mut())), pcboutvaluesize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetDwordValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, dwnewvalue: u32, pdwoutvalue: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetDwordValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows::core::PCWSTR , dwnewvalue : u32 , pdwoutvalue : *mut u32 ) -> u32 );
    ResUtilSetDwordValue(hkeyclusterkey.into(), pszvaluename.into().abi(), dwnewvalue, pdwoutvalue)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetExpandSzValue<P0, P1, P2>(hkeyclusterkey: P0, pszvaluename: P1, psznewvalue: P2, ppszoutstring: ::core::option::Option<*mut ::windows::core::PWSTR>) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetExpandSzValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows::core::PCWSTR , psznewvalue : :: windows::core::PCWSTR , ppszoutstring : *mut :: windows::core::PWSTR ) -> u32 );
    ResUtilSetExpandSzValue(hkeyclusterkey.into(), pszvaluename.into().abi(), psznewvalue.into().abi(), ::core::mem::transmute(ppszoutstring.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetMultiSzValue<P0, P1, P2>(hkeyclusterkey: P0, pszvaluename: P1, psznewvalue: P2, cbnewvaluesize: u32, ppszoutvalue: ::core::option::Option<*mut ::windows::core::PWSTR>, pcboutvaluesize: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetMultiSzValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows::core::PCWSTR , psznewvalue : :: windows::core::PCWSTR , cbnewvaluesize : u32 , ppszoutvalue : *mut :: windows::core::PWSTR , pcboutvaluesize : *mut u32 ) -> u32 );
    ResUtilSetMultiSzValue(hkeyclusterkey.into(), pszvaluename.into().abi(), psznewvalue.into().abi(), cbnewvaluesize, ::core::mem::transmute(ppszoutvalue.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcboutvaluesize.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetPrivatePropertyList<P0>(hkeyclusterkey: P0, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetPrivatePropertyList ( hkeyclusterkey : super::super::System::Registry:: HKEY , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 ) -> u32 );
    ResUtilSetPrivatePropertyList(hkeyclusterkey.into(), pinpropertylist, cbinpropertylistsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilSetPropertyParameterBlock<P0>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetPropertyParameterBlock ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , reserved : *mut ::core::ffi::c_void , pinparams : *const u8 , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 , poutparams : *mut u8 ) -> u32 );
    ResUtilSetPropertyParameterBlock(hkeyclusterkey.into(), ppropertytable, reserved, pinparams, pinpropertylist, cbinpropertylistsize, poutparams)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilSetPropertyParameterBlockEx<P0, P1>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: P1, poutparams: *mut u8) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetPropertyParameterBlockEx ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , reserved : *mut ::core::ffi::c_void , pinparams : *const u8 , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 , bforcewrite : super::super::Foundation:: BOOL , poutparams : *mut u8 ) -> u32 );
    ResUtilSetPropertyParameterBlockEx(hkeyclusterkey.into(), ppropertytable, reserved, pinparams, pinpropertylist, cbinpropertylistsize, bforcewrite.into(), poutparams)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilSetPropertyTable<P0, P1>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: ::core::option::Option<*mut ::core::ffi::c_void>, ballowunknownproperties: P1, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: ::core::option::Option<*mut u8>) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetPropertyTable ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , reserved : *mut ::core::ffi::c_void , ballowunknownproperties : super::super::Foundation:: BOOL , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 , poutparams : *mut u8 ) -> u32 );
    ResUtilSetPropertyTable(hkeyclusterkey.into(), ppropertytable, ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null_mut())), ballowunknownproperties.into(), pinpropertylist, cbinpropertylistsize, ::core::mem::transmute(poutparams.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilSetPropertyTableEx<P0, P1, P2>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: P1, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: P2, poutparams: *mut u8) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetPropertyTableEx ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , reserved : *mut ::core::ffi::c_void , ballowunknownproperties : super::super::Foundation:: BOOL , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 , bforcewrite : super::super::Foundation:: BOOL , poutparams : *mut u8 ) -> u32 );
    ResUtilSetPropertyTableEx(hkeyclusterkey.into(), ppropertytable, reserved, ballowunknownproperties.into(), pinpropertylist, cbinpropertylistsize, bforcewrite.into(), poutparams)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetQwordValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, qwnewvalue: u64, pqwoutvalue: ::core::option::Option<*mut u64>) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetQwordValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows::core::PCWSTR , qwnewvalue : u64 , pqwoutvalue : *mut u64 ) -> u32 );
    ResUtilSetQwordValue(hkeyclusterkey.into(), pszvaluename.into().abi(), qwnewvalue, ::core::mem::transmute(pqwoutvalue.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilSetResourceServiceEnvironment<P0>(pszservicename: P0, hresource: *mut _HRESOURCE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetResourceServiceEnvironment ( pszservicename : :: windows::core::PCWSTR , hresource : *mut _HRESOURCE , pfnlogevent : PLOG_EVENT_ROUTINE , hresourcehandle : isize ) -> u32 );
    ResUtilSetResourceServiceEnvironment(pszservicename.into().abi(), hresource, pfnlogevent, hresourcehandle)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ResUtilSetResourceServiceStartParameters<P0, P1>(pszservicename: P0, schscmhandle: P1, phservice: *mut isize, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetResourceServiceStartParameters ( pszservicename : :: windows::core::PCWSTR , schscmhandle : super::super::Security:: SC_HANDLE , phservice : *mut isize , pfnlogevent : PLOG_EVENT_ROUTINE , hresourcehandle : isize ) -> u32 );
    ResUtilSetResourceServiceStartParameters(pszservicename.into().abi(), schscmhandle.into(), phservice, pfnlogevent, hresourcehandle)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ResUtilSetResourceServiceStartParametersEx<P0, P1>(pszservicename: P0, schscmhandle: P1, phservice: *mut isize, dwdesiredaccess: u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetResourceServiceStartParametersEx ( pszservicename : :: windows::core::PCWSTR , schscmhandle : super::super::Security:: SC_HANDLE , phservice : *mut isize , dwdesiredaccess : u32 , pfnlogevent : PLOG_EVENT_ROUTINE , hresourcehandle : isize ) -> u32 );
    ResUtilSetResourceServiceStartParametersEx(pszservicename.into().abi(), schscmhandle.into(), phservice, dwdesiredaccess, pfnlogevent, hresourcehandle)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetSzValue<P0, P1, P2>(hkeyclusterkey: P0, pszvaluename: P1, psznewvalue: P2, ppszoutstring: ::core::option::Option<*mut ::windows::core::PWSTR>) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetSzValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows::core::PCWSTR , psznewvalue : :: windows::core::PCWSTR , ppszoutstring : *mut :: windows::core::PWSTR ) -> u32 );
    ResUtilSetSzValue(hkeyclusterkey.into(), pszvaluename.into().abi(), psznewvalue.into().abi(), ::core::mem::transmute(ppszoutstring.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilSetUnknownProperties<P0>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetUnknownProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 ) -> u32 );
    ResUtilSetUnknownProperties(hkeyclusterkey.into(), ppropertytable, pinpropertylist, cbinpropertylistsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetValueEx<P0, P1>(hkeyclusterkey: P0, valuename: P1, valuetype: u32, valuedata: &[u8], flags: u32) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilSetValueEx ( hkeyclusterkey : super::super::System::Registry:: HKEY , valuename : :: windows::core::PCWSTR , valuetype : u32 , valuedata : *const u8 , valuesize : u32 , flags : u32 ) -> u32 );
    ResUtilSetValueEx(hkeyclusterkey.into(), valuename.into().abi(), valuetype, ::core::mem::transmute(valuedata.as_ptr()), valuedata.len() as _, flags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilStartResourceService<P0>(pszservicename: P0, phservicehandle: *mut isize) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilStartResourceService ( pszservicename : :: windows::core::PCWSTR , phservicehandle : *mut isize ) -> u32 );
    ResUtilStartResourceService(pszservicename.into().abi(), phservicehandle)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilStopResourceService<P0>(pszservicename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilStopResourceService ( pszservicename : :: windows::core::PCWSTR ) -> u32 );
    ResUtilStopResourceService(pszservicename.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ResUtilStopService<P0>(hservicehandle: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilStopService ( hservicehandle : super::super::Security:: SC_HANDLE ) -> u32 );
    ResUtilStopService(hservicehandle.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilTerminateServiceProcessFromResDll<P0>(dwservicepid: u32, boffline: P0, pdwresourcestate: *mut u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilTerminateServiceProcessFromResDll ( dwservicepid : u32 , boffline : super::super::Foundation:: BOOL , pdwresourcestate : *mut u32 , pfnlogevent : PLOG_EVENT_ROUTINE , hresourcehandle : isize ) -> u32 );
    ResUtilTerminateServiceProcessFromResDll(dwservicepid, boffline.into(), pdwresourcestate, pfnlogevent, hresourcehandle)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilVerifyPrivatePropertyList(pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilVerifyPrivatePropertyList ( pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 ) -> u32 );
    ResUtilVerifyPrivatePropertyList(pinpropertylist, cbinpropertylistsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResUtilVerifyPropertyTable<P0>(ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: ::core::option::Option<*mut ::core::ffi::c_void>, ballowunknownproperties: P0, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: ::core::option::Option<*mut u8>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilVerifyPropertyTable ( ppropertytable : *const RESUTIL_PROPERTY_ITEM , reserved : *mut ::core::ffi::c_void , ballowunknownproperties : super::super::Foundation:: BOOL , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 , poutparams : *mut u8 ) -> u32 );
    ResUtilVerifyPropertyTable(ppropertytable, ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null_mut())), ballowunknownproperties.into(), pinpropertylist, cbinpropertylistsize, ::core::mem::transmute(poutparams.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilVerifyResourceService<P0>(pszservicename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilVerifyResourceService ( pszservicename : :: windows::core::PCWSTR ) -> u32 );
    ResUtilVerifyResourceService(pszservicename.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ResUtilVerifyService<P0>(hservicehandle: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilVerifyService ( hservicehandle : super::super::Security:: SC_HANDLE ) -> u32 );
    ResUtilVerifyService(hservicehandle.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResUtilVerifyShutdownSafe(flags: u32, reason: u32, presult: *mut u32) -> u32 {
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilVerifyShutdownSafe ( flags : u32 , reason : u32 , presult : *mut u32 ) -> u32 );
    ResUtilVerifyShutdownSafe(flags, reason, presult)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ResUtilsDeleteKeyTree<P0, P1, P2>(key: P0, keyname: P1, treatnokeyaserror: P2) -> u32
where
    P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "resutils.dll""system" fn ResUtilsDeleteKeyTree ( key : super::super::System::Registry:: HKEY , keyname : :: windows::core::PCWSTR , treatnokeyaserror : super::super::Foundation:: BOOL ) -> u32 );
    ResUtilsDeleteKeyTree(key.into(), keyname.into().abi(), treatnokeyaserror.into())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResetAllAppInstanceVersions() -> u32 {
    ::windows::core::link ! ( "ntlanman.dll""system" fn ResetAllAppInstanceVersions ( ) -> u32 );
    ResetAllAppInstanceVersions()
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn RestartClusterResource(hresource: *const _HRESOURCE, dwflags: u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn RestartClusterResource ( hresource : *const _HRESOURCE , dwflags : u32 ) -> u32 );
    RestartClusterResource(hresource, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RestoreClusterDatabase<P0, P1, P2>(lpszpathname: P0, bforce: P1, lpszquorumdriveletter: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn RestoreClusterDatabase ( lpszpathname : :: windows::core::PCWSTR , bforce : super::super::Foundation:: BOOL , lpszquorumdriveletter : :: windows::core::PCWSTR ) -> u32 );
    RestoreClusterDatabase(lpszpathname.into().abi(), bforce.into(), lpszquorumdriveletter.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResumeClusterNode(hnode: *const _HNODE) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ResumeClusterNode ( hnode : *const _HNODE ) -> u32 );
    ResumeClusterNode(hnode)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn ResumeClusterNodeEx(hnode: *const _HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn ResumeClusterNodeEx ( hnode : *const _HNODE , eresumefailbacktype : CLUSTER_NODE_RESUME_FAILBACK_TYPE , dwresumeflagsreserved : u32 ) -> u32 );
    ResumeClusterNodeEx(hnode, eresumefailbacktype, dwresumeflagsreserved)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetAppInstanceCsvFlags<P0>(processhandle: P0, mask: u32, flags: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ntlanman.dll""system" fn SetAppInstanceCsvFlags ( processhandle : super::super::Foundation:: HANDLE , mask : u32 , flags : u32 ) -> u32 );
    SetAppInstanceCsvFlags(processhandle.into(), mask, flags)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn SetClusterGroupName<P0>(hgroup: *const _HGROUP, lpszgroupname: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn SetClusterGroupName ( hgroup : *const _HGROUP , lpszgroupname : :: windows::core::PCWSTR ) -> u32 );
    SetClusterGroupName(hgroup, lpszgroupname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn SetClusterGroupNodeList(hgroup: *const _HGROUP, nodelist: ::core::option::Option<&[*const _HNODE]>) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn SetClusterGroupNodeList ( hgroup : *const _HGROUP , nodecount : u32 , nodelist : *const *const _HNODE ) -> u32 );
    SetClusterGroupNodeList(hgroup, nodelist.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(nodelist.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn SetClusterGroupSetDependencyExpression<P0>(hgroupset: *const _HGROUPSET, lpszdependencyexprssion: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn SetClusterGroupSetDependencyExpression ( hgroupset : *const _HGROUPSET , lpszdependencyexprssion : :: windows::core::PCWSTR ) -> u32 );
    SetClusterGroupSetDependencyExpression(hgroupset, lpszdependencyexprssion.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn SetClusterName<P0>(hcluster: *const _HCLUSTER, lpsznewclustername: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn SetClusterName ( hcluster : *const _HCLUSTER , lpsznewclustername : :: windows::core::PCWSTR ) -> u32 );
    SetClusterName(hcluster, lpsznewclustername.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn SetClusterNetworkName<P0>(hnetwork: *const _HNETWORK, lpszname: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn SetClusterNetworkName ( hnetwork : *const _HNETWORK , lpszname : :: windows::core::PCWSTR ) -> u32 );
    SetClusterNetworkName(hnetwork, lpszname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn SetClusterNetworkPriorityOrder(hcluster: *const _HCLUSTER, networklist: &[*const _HNETWORK]) -> u32 {
    ::windows::core::link ! ( "clusapi.dll""system" fn SetClusterNetworkPriorityOrder ( hcluster : *const _HCLUSTER , networkcount : u32 , networklist : *const *const _HNETWORK ) -> u32 );
    SetClusterNetworkPriorityOrder(hcluster, networklist.len() as _, ::core::mem::transmute(networklist.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn SetClusterQuorumResource<P0>(hresource: *const _HRESOURCE, lpszdevicename: P0, dwmaxquologsize: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn SetClusterQuorumResource ( hresource : *const _HRESOURCE , lpszdevicename : :: windows::core::PCWSTR , dwmaxquologsize : u32 ) -> u32 );
    SetClusterQuorumResource(hresource, lpszdevicename.into().abi(), dwmaxquologsize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn SetClusterResourceDependencyExpression<P0>(hresource: *const _HRESOURCE, lpszdependencyexpression: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn SetClusterResourceDependencyExpression ( hresource : *const _HRESOURCE , lpszdependencyexpression : :: windows::core::PCWSTR ) -> u32 );
    SetClusterResourceDependencyExpression(hresource, lpszdependencyexpression.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn SetClusterResourceName<P0>(hresource: *const _HRESOURCE, lpszresourcename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn SetClusterResourceName ( hresource : *const _HRESOURCE , lpszresourcename : :: windows::core::PCWSTR ) -> u32 );
    SetClusterResourceName(hresource, lpszresourcename.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetClusterServiceAccountPassword<P0, P1>(lpszclustername: P0, lpsznewpassword: P1, dwflags: u32, lpreturnstatusbuffer: ::core::option::Option<*mut CLUSTER_SET_PASSWORD_STATUS>, lpcbreturnstatusbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn SetClusterServiceAccountPassword ( lpszclustername : :: windows::core::PCWSTR , lpsznewpassword : :: windows::core::PCWSTR , dwflags : u32 , lpreturnstatusbuffer : *mut CLUSTER_SET_PASSWORD_STATUS , lpcbreturnstatusbuffersize : *mut u32 ) -> u32 );
    SetClusterServiceAccountPassword(lpszclustername.into().abi(), lpsznewpassword.into().abi(), dwflags, ::core::mem::transmute(lpreturnstatusbuffer.unwrap_or(::std::ptr::null_mut())), lpcbreturnstatusbuffersize)
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[inline]
pub unsafe fn SetGroupDependencyExpression<P0>(hgroup: *const _HGROUP, lpszdependencyexpression: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "clusapi.dll""system" fn SetGroupDependencyExpression ( hgroup : *const _HGROUP , lpszdependencyexpression : :: windows::core::PCWSTR ) -> u32 );
    SetGroupDependencyExpression(hgroup, lpszdependencyexpression.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IGetClusterDataInfo(::windows::core::IUnknown);
impl IGetClusterDataInfo {
    pub unsafe fn GetClusterName(&self, lpszname: &::windows::core::BSTR, pcchname: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetClusterName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(lpszname), pcchname).ok()
    }
    pub unsafe fn GetClusterHandle(&self) -> *mut _HCLUSTER {
        (::windows::core::Vtable::vtable(self).GetClusterHandle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetObjectCount(&self) -> i32 {
        (::windows::core::Vtable::vtable(self).GetObjectCount)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IGetClusterDataInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGetClusterDataInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IGetClusterDataInfo {
    type Vtable = IGetClusterDataInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetClusterDataInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede51_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterDataInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetClusterName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszname: *mut ::core::ffi::c_void, pcchname: *mut i32) -> ::windows::core::HRESULT,
    pub GetClusterHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut _HCLUSTER,
    pub GetObjectCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> i32,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IGetClusterGroupInfo(::windows::core::IUnknown);
impl IGetClusterGroupInfo {
    pub unsafe fn GetGroupHandle(&self, lobjindex: i32) -> *mut _HGROUP {
        (::windows::core::Vtable::vtable(self).GetGroupHandle)(::windows::core::Vtable::as_raw(self), lobjindex)
    }
}
::windows::core::interface_hierarchy!(IGetClusterGroupInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGetClusterGroupInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IGetClusterGroupInfo {
    type Vtable = IGetClusterGroupInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetClusterGroupInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede54_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterGroupInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetGroupHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HGROUP,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IGetClusterNetInterfaceInfo(::windows::core::IUnknown);
impl IGetClusterNetInterfaceInfo {
    pub unsafe fn GetNetInterfaceHandle(&self, lobjindex: i32) -> *mut _HNETINTERFACE {
        (::windows::core::Vtable::vtable(self).GetNetInterfaceHandle)(::windows::core::Vtable::as_raw(self), lobjindex)
    }
}
::windows::core::interface_hierarchy!(IGetClusterNetInterfaceInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGetClusterNetInterfaceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IGetClusterNetInterfaceInfo {
    type Vtable = IGetClusterNetInterfaceInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetClusterNetInterfaceInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede57_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterNetInterfaceInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNetInterfaceHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNETINTERFACE,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IGetClusterNetworkInfo(::windows::core::IUnknown);
impl IGetClusterNetworkInfo {
    pub unsafe fn GetNetworkHandle(&self, lobjindex: i32) -> *mut _HNETWORK {
        (::windows::core::Vtable::vtable(self).GetNetworkHandle)(::windows::core::Vtable::as_raw(self), lobjindex)
    }
}
::windows::core::interface_hierarchy!(IGetClusterNetworkInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGetClusterNetworkInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IGetClusterNetworkInfo {
    type Vtable = IGetClusterNetworkInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetClusterNetworkInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede56_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterNetworkInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNetworkHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNETWORK,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IGetClusterNodeInfo(::windows::core::IUnknown);
impl IGetClusterNodeInfo {
    pub unsafe fn GetNodeHandle(&self, lobjindex: i32) -> *mut _HNODE {
        (::windows::core::Vtable::vtable(self).GetNodeHandle)(::windows::core::Vtable::as_raw(self), lobjindex)
    }
}
::windows::core::interface_hierarchy!(IGetClusterNodeInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGetClusterNodeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IGetClusterNodeInfo {
    type Vtable = IGetClusterNodeInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetClusterNodeInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede53_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterNodeInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNodeHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HNODE,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IGetClusterObjectInfo(::windows::core::IUnknown);
impl IGetClusterObjectInfo {
    pub unsafe fn GetObjectName(&self, lobjindex: i32, lpszname: &::windows::core::BSTR, pcchname: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetObjectName)(::windows::core::Vtable::as_raw(self), lobjindex, ::core::mem::transmute_copy(lpszname), pcchname).ok()
    }
    pub unsafe fn GetObjectType(&self, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE {
        (::windows::core::Vtable::vtable(self).GetObjectType)(::windows::core::Vtable::as_raw(self), lobjindex)
    }
}
::windows::core::interface_hierarchy!(IGetClusterObjectInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGetClusterObjectInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IGetClusterObjectInfo {
    type Vtable = IGetClusterObjectInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetClusterObjectInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede52_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterObjectInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjindex: i32, lpszname: *mut ::core::ffi::c_void, pcchname: *mut i32) -> ::windows::core::HRESULT,
    pub GetObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IGetClusterResourceInfo(::windows::core::IUnknown);
impl IGetClusterResourceInfo {
    pub unsafe fn GetResourceHandle(&self, lobjindex: i32) -> *mut _HRESOURCE {
        (::windows::core::Vtable::vtable(self).GetResourceHandle)(::windows::core::Vtable::as_raw(self), lobjindex)
    }
    pub unsafe fn GetResourceTypeName(&self, lobjindex: i32, lpszrestypename: &::windows::core::BSTR, pcchrestypename: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetResourceTypeName)(::windows::core::Vtable::as_raw(self), lobjindex, ::core::mem::transmute_copy(lpszrestypename), pcchrestypename).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResourceNetworkName(&self, lobjindex: i32, lpsznetname: &::windows::core::BSTR, pcchnetname: *mut u32) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).GetResourceNetworkName)(::windows::core::Vtable::as_raw(self), lobjindex, ::core::mem::transmute_copy(lpsznetname), pcchnetname)
    }
}
::windows::core::interface_hierarchy!(IGetClusterResourceInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGetClusterResourceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IGetClusterResourceInfo {
    type Vtable = IGetClusterResourceInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetClusterResourceInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede55_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterResourceInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetResourceHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjindex: i32) -> *mut _HRESOURCE,
    pub GetResourceTypeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjindex: i32, lpszrestypename: *mut ::core::ffi::c_void, pcchrestypename: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetResourceNetworkName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjindex: i32, lpsznetname: *mut ::core::ffi::c_void, pcchnetname: *mut u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetResourceNetworkName: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IGetClusterUIInfo(::windows::core::IUnknown);
impl IGetClusterUIInfo {
    pub unsafe fn GetClusterName(&self, lpszname: &::windows::core::BSTR, pcchname: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetClusterName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(lpszname), pcchname).ok()
    }
    pub unsafe fn GetLocale(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetLocale)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetFont(&self) -> super::super::Graphics::Gdi::HFONT {
        (::windows::core::Vtable::vtable(self).GetFont)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self) -> super::super::UI::WindowsAndMessaging::HICON {
        (::windows::core::Vtable::vtable(self).GetIcon)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IGetClusterUIInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGetClusterUIInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IGetClusterUIInfo {
    type Vtable = IGetClusterUIInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetClusterUIInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede50_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterUIInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetClusterName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszname: *mut ::core::ffi::c_void, pcchname: *mut i32) -> ::windows::core::HRESULT,
    pub GetLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Graphics::Gdi::HFONT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetFont: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::UI::WindowsAndMessaging::HICON,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusApplication(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusApplication {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DomainNames(&self) -> ::windows::core::Result<ISDomainNames> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DomainNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ClusterNames(&self, bstrdomainname: &::windows::core::BSTR) -> ::windows::core::Result<ISClusterNames> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_ClusterNames)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdomainname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenCluster(&self, bstrclustername: &::windows::core::BSTR) -> ::windows::core::Result<ISCluster> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenCluster)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrclustername), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusApplication, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusApplication {
    type Vtable = ISClusApplication_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusApplication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606e6_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusApplication_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DomainNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdomains: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DomainNames: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ClusterNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdomainname: *mut ::core::ffi::c_void, ppclusters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ClusterNames: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenCluster: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclustername: *mut ::core::ffi::c_void, pcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenCluster: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusCryptoKeys(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusCryptoKeys {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddItem(&self, bstrcryptokey: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcryptokey)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RemoveItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusCryptoKeys, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusCryptoKeys {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusCryptoKeys {
    type Vtable = ISClusCryptoKeys_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusCryptoKeys {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6072c_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusCryptoKeys_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, pbstrcyrptokey: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcryptokey: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RemoveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RemoveItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusDisk(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusDisk {
    pub unsafe fn Signature(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Signature)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ScsiAddress(&self) -> ::windows::core::Result<ISClusScsiAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ScsiAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DiskNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DiskNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Partitions(&self) -> ::windows::core::Result<ISClusPartitions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Partitions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusDisk, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusDisk {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusDisk {
    type Vtable = ISClusDisk_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusDisk {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60724_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusDisk_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Signature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsignature: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ScsiAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscsiaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ScsiAddress: usize,
    pub DiskNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldisknumber: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Partitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppartitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Partitions: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusDisks(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusDisks {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusDisk> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusDisks, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusDisks {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusDisks {
    type Vtable = ISClusDisks_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusDisks {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60726_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusDisks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppdisk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusNetInterface(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetInterface {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Handle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<CLUSTER_NETINTERFACE_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> ::windows::core::Result<ISCluster> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cluster)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusNetInterface, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusNetInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusNetInterface {
    type Vtable = ISClusNetInterface_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusNetInterface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606ee_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNetInterface_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NETINTERFACE_STATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusNetInterfaces(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetInterfaces {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetInterface> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusNetInterfaces, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusNetInterfaces {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusNetInterfaces {
    type Vtable = ISClusNetInterfaces_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusNetInterfaces {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606f0_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNetInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusnetinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusNetwork(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetwork {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Handle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, bstrnetworkname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnetworkname)).ok()
    }
    pub unsafe fn NetworkID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NetworkID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<CLUSTER_NETWORK_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NetInterfaces(&self) -> ::windows::core::Result<ISClusNetworkNetInterfaces> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NetInterfaces)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> ::windows::core::Result<ISCluster> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cluster)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusNetwork, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusNetwork {
    type Vtable = ISClusNetwork_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusNetwork {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606f2_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNetwork_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnetworkname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NetworkID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnetworkid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NETWORK_STATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NetInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclusnetinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NetInterfaces: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusNetworkNetInterfaces(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetworkNetInterfaces {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetInterface> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusNetworkNetInterfaces, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusNetworkNetInterfaces {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusNetworkNetInterfaces {
    type Vtable = ISClusNetworkNetInterfaces_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusNetworkNetInterfaces {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606f6_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNetworkNetInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusnetinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusNetworks(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetworks {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetwork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusNetworks, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusNetworks {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusNetworks {
    type Vtable = ISClusNetworks_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusNetworks {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606f4_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNetworks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusnetwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusNode(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNode {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Handle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NodeID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NodeID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<CLUSTER_NODE_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Evict(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Evict)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResourceGroups(&self) -> ::windows::core::Result<ISClusResGroups> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ResourceGroups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> ::windows::core::Result<ISCluster> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cluster)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NetInterfaces(&self) -> ::windows::core::Result<ISClusNodeNetInterfaces> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NetInterfaces)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusNode, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusNode {
    type Vtable = ISClusNode_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606f8_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNode_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT,
    pub NodeID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnodeid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_NODE_STATE) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Evict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResourceGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresourcegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResourceGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NetInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclusnetinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NetInterfaces: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusNodeNetInterfaces(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNodeNetInterfaces {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNetInterface> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusNodeNetInterfaces, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusNodeNetInterfaces {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusNodeNetInterfaces {
    type Vtable = ISClusNodeNetInterfaces_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusNodeNetInterfaces {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606fc_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNodeNetInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusnetinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusNodes(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNodes {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusNodes, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusNodes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusNodes {
    type Vtable = ISClusNodes_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusNodes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606fa_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusPartition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartition {
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Flags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DeviceName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DeviceName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn VolumeLabel(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).VolumeLabel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SerialNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SerialNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MaximumComponentLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MaximumComponentLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FileSystemFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FileSystemFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FileSystem(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FileSystem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusPartition, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusPartition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusPartition {
    type Vtable = ISClusPartition_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusPartition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60720_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPartition_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VolumeLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrvolumelabel: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plserialnumber: *mut i32) -> ::windows::core::HRESULT,
    pub MaximumComponentLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaximumcomponentlength: *mut i32) -> ::windows::core::HRESULT,
    pub FileSystemFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plfilesystemflags: *mut i32) -> ::windows::core::HRESULT,
    pub FileSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilesystem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusPartitionEx(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitionEx {
    pub unsafe fn TotalSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FreeSpace(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FreeSpace)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DeviceNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DeviceNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PartitionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PartitionNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn VolumeGuid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).VolumeGuid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusPartitionEx, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ISClusPartition);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusPartitionEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusPartitionEx {
    type Vtable = ISClusPartitionEx_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusPartitionEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8802d4fe_b32e_4ad1_9dbd_64f18e1166ce);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPartitionEx_Vtbl {
    pub base__: ISClusPartition_Vtbl,
    pub TotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltotalsize: *mut i32) -> ::windows::core::HRESULT,
    pub FreeSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plfreespace: *mut i32) -> ::windows::core::HRESULT,
    pub DeviceNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldevicenumber: *mut i32) -> ::windows::core::HRESULT,
    pub PartitionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpartitionnumber: *mut i32) -> ::windows::core::HRESULT,
    pub VolumeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrvolumeguid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusPartitions(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitions {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusPartition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusPartitions, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusPartitions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusPartitions {
    type Vtable = ISClusPartitions_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusPartitions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60722_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPartitions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, pppartition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusProperties(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusProperties {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateItem(&self, bstrname: &::windows::core::BSTR, varvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(varvalue), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn UseDefaultValue(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UseDefaultValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SaveChanges(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SaveChanges)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReadOnly(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReadOnly)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Private(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Private)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Common(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Common)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Modified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Modified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusProperties, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusProperties {
    type Vtable = ISClusProperties_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60700_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusProperties_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, varvalue: super::super::System::Com::VARIANT, pproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub UseDefaultValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    UseDefaultValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SaveChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarstatuscode: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SaveChanges: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreadonly: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReadOnly: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Private: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprivate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Private: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Common: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcommon: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Common: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Modified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Modified: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusProperty {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Length)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ValueCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ValueCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Values(&self) -> ::windows::core::Result<ISClusPropertyValues> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Values)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, varvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varvalue)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<CLUSTER_PROPERTY_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetType)(::windows::core::Vtable::as_raw(self), r#type).ok()
    }
    pub unsafe fn Format(&self) -> ::windows::core::Result<CLUSTER_PROPERTY_FORMAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Format)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFormat)(::windows::core::Vtable::as_raw(self), format).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReadOnly(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReadOnly)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Private(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Private)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Common(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Common)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Modified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Modified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UseDefaultValue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UseDefaultValue)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusProperty, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusProperty {
    type Vtable = ISClusProperty_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606fe_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusProperty_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plength: *mut i32) -> ::windows::core::HRESULT,
    pub ValueCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Values: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclusterpropertyvalues: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Values: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreadonly: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReadOnly: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Private: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprivate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Private: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Common: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcommon: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Common: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Modified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Modified: usize,
    pub UseDefaultValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusPropertyValue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValue {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, varvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varvalue)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<CLUSTER_PROPERTY_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetType)(::windows::core::Vtable::as_raw(self), r#type).ok()
    }
    pub unsafe fn Format(&self) -> ::windows::core::Result<CLUSTER_PROPERTY_FORMAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Format)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFormat)(::windows::core::Vtable::as_raw(self), format).ok()
    }
    pub unsafe fn Length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Length)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DataCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DataCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Data(&self) -> ::windows::core::Result<ISClusPropertyValueData> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Data)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusPropertyValue, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusPropertyValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusPropertyValue {
    type Vtable = ISClusPropertyValue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusPropertyValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6071a_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPropertyValue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> ::windows::core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plength: *mut i32) -> ::windows::core::HRESULT,
    pub DataCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclusterpropertyvaluedata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Data: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusPropertyValueData(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValueData {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateItem(&self, varvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varvalue), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RemoveItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusPropertyValueData, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusPropertyValueData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusPropertyValueData {
    type Vtable = ISClusPropertyValueData_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusPropertyValueData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6071e_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPropertyValueData_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: super::super::System::Com::VARIANT, pvardata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RemoveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RemoveItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusPropertyValues(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValues {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusPropertyValue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateItem(&self, bstrname: &::windows::core::BSTR, varvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusPropertyValue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(varvalue), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RemoveItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusPropertyValues, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusPropertyValues {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusPropertyValues {
    type Vtable = ISClusPropertyValues_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusPropertyValues {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6071c_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPropertyValues_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, pppropertyvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, varvalue: super::super::System::Com::VARIANT, pppropertyvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RemoveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RemoveItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusRefObject(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusRefObject {
    pub unsafe fn Handle(&self) -> ::windows::core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Handle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusRefObject, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusRefObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusRefObject {
    type Vtable = ISClusRefObject_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusRefObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60702_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusRefObject_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusRegistryKeys(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusRegistryKeys {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddItem(&self, bstrregistrykey: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrregistrykey)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RemoveItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusRegistryKeys, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusRegistryKeys {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusRegistryKeys {
    type Vtable = ISClusRegistryKeys_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusRegistryKeys {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6072a_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusRegistryKeys_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, pbstrregistrykey: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrregistrykey: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RemoveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RemoveItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResDependencies(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResDependencies {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem(&self, bstrresourcename: &::windows::core::BSTR, bstrresourcetype: &::windows::core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrresourcename), ::core::mem::transmute_copy(bstrresourcetype), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddItem<P0>(&self, presource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISClusResource>>,
    {
        (::windows::core::Vtable::vtable(self).AddItem)(::windows::core::Vtable::as_raw(self), presource.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RemoveItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResDependencies, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResDependencies {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResDependencies {
    type Vtable = ISClusResDependencies_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResDependencies {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60704_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResDependencies_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresourcename: *mut ::core::ffi::c_void, bstrresourcetype: *mut ::core::ffi::c_void, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RemoveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RemoveItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResDependents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResDependents {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem(&self, bstrresourcename: &::windows::core::BSTR, bstrresourcetype: &::windows::core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrresourcename), ::core::mem::transmute_copy(bstrresourcetype), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddItem<P0>(&self, presource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISClusResource>>,
    {
        (::windows::core::Vtable::vtable(self).AddItem)(::windows::core::Vtable::as_raw(self), presource.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RemoveItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResDependents, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResDependents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResDependents {
    type Vtable = ISClusResDependents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResDependents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6072e_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResDependents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresourcename: *mut ::core::ffi::c_void, bstrresourcetype: *mut ::core::ffi::c_void, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RemoveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RemoveItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResGroup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroup {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Handle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, bstrgroupname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrgroupname)).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<CLUSTER_GROUP_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OwnerNode(&self) -> ::windows::core::Result<ISClusNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OwnerNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resources(&self) -> ::windows::core::Result<ISClusResGroupResources> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Resources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PreferredOwnerNodes(&self) -> ::windows::core::Result<ISClusResGroupPreferredOwnerNodes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PreferredOwnerNodes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Online(&self, vartimeout: super::super::System::Com::VARIANT, varnode: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Online)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vartimeout), ::core::mem::transmute(varnode), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Move(&self, vartimeout: super::super::System::Com::VARIANT, varnode: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Move)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vartimeout), ::core::mem::transmute(varnode), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Offline(&self, vartimeout: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Offline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vartimeout), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> ::windows::core::Result<ISCluster> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cluster)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResGroup, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResGroup {
    type Vtable = ISClusResGroup_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60706_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResGroup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_GROUP_STATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OwnerNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppownernode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OwnerNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclustergroupresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resources: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PreferredOwnerNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppownernodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PreferredOwnerNodes: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Online: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vartimeout: super::super::System::Com::VARIANT, varnode: super::super::System::Com::VARIANT, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Online: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vartimeout: super::super::System::Com::VARIANT, varnode: super::super::System::Com::VARIANT, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Move: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Offline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vartimeout: super::super::System::Com::VARIANT, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Offline: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResGroupPreferredOwnerNodes(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupPreferredOwnerNodes {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertItem<P0>(&self, pnode: P0, nposition: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISClusNode>>,
    {
        (::windows::core::Vtable::vtable(self).InsertItem)(::windows::core::Vtable::as_raw(self), pnode.into().abi(), nposition).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RemoveItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Modified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Modified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SaveChanges(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SaveChanges)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddItem<P0>(&self, pnode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISClusNode>>,
    {
        (::windows::core::Vtable::vtable(self).AddItem)(::windows::core::Vtable::as_raw(self), pnode.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResGroupPreferredOwnerNodes, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResGroupPreferredOwnerNodes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResGroupPreferredOwnerNodes {
    type Vtable = ISClusResGroupPreferredOwnerNodes_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResGroupPreferredOwnerNodes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606e8_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResGroupPreferredOwnerNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, nposition: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RemoveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RemoveItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Modified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Modified: usize,
    pub SaveChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResGroupResources(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupResources {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem(&self, bstrresourcename: &::windows::core::BSTR, bstrresourcetype: &::windows::core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrresourcename), ::core::mem::transmute_copy(bstrresourcetype), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResGroupResources, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResGroupResources {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResGroupResources {
    type Vtable = ISClusResGroupResources_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResGroupResources {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606ea_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResGroupResources_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresourcename: *mut ::core::ffi::c_void, bstrresourcetype: *mut ::core::ffi::c_void, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResGroups(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroups {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem(&self, bstrresourcegroupname: &::windows::core::BSTR) -> ::windows::core::Result<ISClusResGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrresourcegroupname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResGroups, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResGroups {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResGroups {
    type Vtable = ISClusResGroups_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResGroups {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60708_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResGroups_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusresgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresourcegroupname: *mut ::core::ffi::c_void, ppresourcegroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResPossibleOwnerNodes(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResPossibleOwnerNodes {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddItem<P0>(&self, pnode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISClusNode>>,
    {
        (::windows::core::Vtable::vtable(self).AddItem)(::windows::core::Vtable::as_raw(self), pnode.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RemoveItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Modified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Modified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResPossibleOwnerNodes, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResPossibleOwnerNodes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResPossibleOwnerNodes {
    type Vtable = ISClusResPossibleOwnerNodes_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResPossibleOwnerNodes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6070e_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResPossibleOwnerNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RemoveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RemoveItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Modified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Modified: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResType(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResType {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> ::windows::core::Result<ISCluster> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cluster)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resources(&self) -> ::windows::core::Result<ISClusResTypeResources> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Resources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleOwnerNodes(&self) -> ::windows::core::Result<ISClusResTypePossibleOwnerNodes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PossibleOwnerNodes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AvailableDisks(&self) -> ::windows::core::Result<ISClusDisks> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AvailableDisks)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResType, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResType {
    type Vtable = ISClusResType_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResType {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60710_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResType_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclusterrestyperesources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resources: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PossibleOwnerNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppownernodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PossibleOwnerNodes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AvailableDisks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppavailabledisks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AvailableDisks: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResTypePossibleOwnerNodes(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypePossibleOwnerNodes {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResTypePossibleOwnerNodes, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResTypePossibleOwnerNodes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResTypePossibleOwnerNodes {
    type Vtable = ISClusResTypePossibleOwnerNodes_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResTypePossibleOwnerNodes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60718_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResTypePossibleOwnerNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResTypeResources(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypeResources {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem(&self, bstrresourcename: &::windows::core::BSTR, bstrgroupname: &::windows::core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrresourcename), ::core::mem::transmute_copy(bstrgroupname), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResTypeResources, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResTypeResources {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResTypeResources {
    type Vtable = ISClusResTypeResources_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResTypeResources {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60714_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResTypeResources_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresourcename: *mut ::core::ffi::c_void, bstrgroupname: *mut ::core::ffi::c_void, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResTypes(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypes {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem(&self, bstrresourcetypename: &::windows::core::BSTR, bstrdisplayname: &::windows::core::BSTR, bstrresourcetypedll: &::windows::core::BSTR, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32) -> ::windows::core::Result<ISClusResType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrresourcetypename), ::core::mem::transmute_copy(bstrdisplayname), ::core::mem::transmute_copy(bstrresourcetypedll), dwlooksalivepollinterval, dwisalivepollinterval, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResTypes, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResTypes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResTypes {
    type Vtable = ISClusResTypes_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResTypes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60712_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResTypes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusrestype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresourcetypename: *mut ::core::ffi::c_void, bstrdisplayname: *mut ::core::ffi::c_void, bstrresourcetypedll: *mut ::core::ffi::c_void, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32, ppresourcetype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResource(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResource {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Handle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, bstrresourcename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrresourcename)).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<CLUSTER_RESOURCE_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CoreFlag(&self) -> ::windows::core::Result<CLUS_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CoreFlag)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BecomeQuorumResource(&self, bstrdevicepath: &::windows::core::BSTR, lmaxlogsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).BecomeQuorumResource)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdevicepath), lmaxlogsize).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Fail(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Fail)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Online(&self, ntimeout: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Online)(::windows::core::Vtable::as_raw(self), ntimeout, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Offline(&self, ntimeout: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Offline)(::windows::core::Vtable::as_raw(self), ntimeout, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ChangeResourceGroup<P0>(&self, presourcegroup: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISClusResGroup>>,
    {
        (::windows::core::Vtable::vtable(self).ChangeResourceGroup)(::windows::core::Vtable::as_raw(self), presourcegroup.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddResourceNode<P0>(&self, pnode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISClusNode>>,
    {
        (::windows::core::Vtable::vtable(self).AddResourceNode)(::windows::core::Vtable::as_raw(self), pnode.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveResourceNode<P0>(&self, pnode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISClusNode>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveResourceNode)(::windows::core::Vtable::as_raw(self), pnode.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanResourceBeDependent<P0>(&self, presource: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISClusResource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CanResourceBeDependent)(::windows::core::Vtable::as_raw(self), presource.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleOwnerNodes(&self) -> ::windows::core::Result<ISClusResPossibleOwnerNodes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PossibleOwnerNodes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Dependencies(&self) -> ::windows::core::Result<ISClusResDependencies> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Dependencies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Dependents(&self) -> ::windows::core::Result<ISClusResDependents> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Dependents)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Group(&self) -> ::windows::core::Result<ISClusResGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Group)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OwnerNode(&self) -> ::windows::core::Result<ISClusNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OwnerNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> ::windows::core::Result<ISCluster> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cluster)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClassInfo(&self) -> ::windows::core::Result<CLUSTER_RESOURCE_CLASS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ClassInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Disk(&self) -> ::windows::core::Result<ISClusDisk> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Disk)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegistryKeys(&self) -> ::windows::core::Result<ISClusRegistryKeys> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegistryKeys)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CryptoKeys(&self) -> ::windows::core::Result<ISClusCryptoKeys> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CryptoKeys)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TypeName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TypeName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Type(&self) -> ::windows::core::Result<ISClusResType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MaintenanceMode(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MaintenanceMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMaintenanceMode<P0>(&self, bmaintenancemode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetMaintenanceMode)(::windows::core::Vtable::as_raw(self), bmaintenancemode.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResource, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResource {
    type Vtable = ISClusResource_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6070a_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResource_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresourcename: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstate: *mut CLUSTER_RESOURCE_STATE) -> ::windows::core::HRESULT,
    pub CoreFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcoreflag: *mut CLUS_FLAGS) -> ::windows::core::HRESULT,
    pub BecomeQuorumResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdevicepath: *mut ::core::ffi::c_void, lmaxlogsize: i32) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Fail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Online: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Online: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Offline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Offline: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ChangeResourceGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcegroup: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ChangeResourceGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddResourceNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddResourceNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveResourceNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveResourceNode: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CanResourceBeDependent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pvardependent: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CanResourceBeDependent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PossibleOwnerNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppownernodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PossibleOwnerNodes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Dependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresdependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Dependencies: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Dependents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresdependents: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Dependents: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Group: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OwnerNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppownernode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OwnerNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcluster: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
    pub ClassInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcclassinfo: *mut CLUSTER_RESOURCE_CLASS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Disk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdisk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Disk: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RegistryKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppregistrykeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegistryKeys: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CryptoKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcryptokeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CryptoKeys: usize,
    pub TypeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtypename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresourcetype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Type: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MaintenanceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmaintenancemode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MaintenanceMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMaintenanceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmaintenancemode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMaintenanceMode: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusResources(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResources {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISClusResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem(&self, bstrresourcename: &::windows::core::BSTR, bstrresourcetype: &::windows::core::BSTR, bstrgroupname: &::windows::core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> ::windows::core::Result<ISClusResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrresourcename), ::core::mem::transmute_copy(bstrresourcetype), ::core::mem::transmute_copy(bstrgroupname), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteItem(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusResources, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusResources {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusResources {
    type Vtable = ISClusResources_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusResources {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6070c_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResources_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, ppclusresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresourcename: *mut ::core::ffi::c_void, bstrresourcetype: *mut ::core::ffi::c_void, bstrgroupname: *mut ::core::ffi::c_void, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteItem: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusScsiAddress(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusScsiAddress {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PortNumber(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PortNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PathId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PathId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TargetId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TargetId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Lun(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Lun)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusScsiAddress, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusScsiAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusScsiAddress {
    type Vtable = ISClusScsiAddress_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusScsiAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60728_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusScsiAddress_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PortNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarportnumber: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PortNumber: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PathId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarpathid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PathId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TargetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvartargetid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TargetId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Lun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarlun: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Lun: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusVersion(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusVersion {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MajorVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MinorVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BuildNumber(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BuildNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn VendorId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).VendorId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CSDVersion(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CSDVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClusterHighestVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ClusterHighestVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClusterLowestVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ClusterLowestVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Flags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MixedVersion(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MixedVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusVersion, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusVersion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusVersion {
    type Vtable = ISClusVersion_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusVersion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60716_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusVersion_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrclustername: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnmajorversion: *mut i32) -> ::windows::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnminorversion: *mut i32) -> ::windows::core::HRESULT,
    pub BuildNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnbuildnumber: *mut i16) -> ::windows::core::HRESULT,
    pub VendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CSDVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsdversion: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClusterHighestVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnclusterhighestversion: *mut i32) -> ::windows::core::HRESULT,
    pub ClusterLowestVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnclusterlowestversion: *mut i32) -> ::windows::core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnflags: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MixedVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmixedversion: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MixedVersion: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISCluster(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISCluster {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> ::windows::core::Result<ISClusProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateROProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Handle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Open(&self, bstrclustername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Open)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrclustername)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, bstrclustername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrclustername)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Version(&self) -> ::windows::core::Result<ISClusVersion> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Version)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetQuorumResource<P0>(&self, pclusterresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISClusResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetQuorumResource)(::windows::core::Vtable::as_raw(self), pclusterresource.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QuorumResource(&self) -> ::windows::core::Result<ISClusResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QuorumResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn QuorumLogSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QuorumLogSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetQuorumLogSize(&self, nlogsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetQuorumLogSize)(::windows::core::Vtable::as_raw(self), nlogsize).ok()
    }
    pub unsafe fn QuorumPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QuorumPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetQuorumPath(&self, ppath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetQuorumPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(ppath)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Nodes(&self) -> ::windows::core::Result<ISClusNodes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Nodes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResourceGroups(&self) -> ::windows::core::Result<ISClusResGroups> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ResourceGroups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resources(&self) -> ::windows::core::Result<ISClusResources> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Resources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResourceTypes(&self) -> ::windows::core::Result<ISClusResTypes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ResourceTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Networks(&self) -> ::windows::core::Result<ISClusNetworks> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Networks)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NetInterfaces(&self) -> ::windows::core::Result<ISClusNetInterfaces> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NetInterfaces)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISCluster, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISCluster {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISCluster {
    type Vtable = ISCluster_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISCluster {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606e4_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISCluster_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclustername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclustername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclusversion: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Version: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetQuorumResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclusterresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetQuorumResource: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QuorumResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclusterresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QuorumResource: usize,
    pub QuorumLogSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlogsize: *mut i32) -> ::windows::core::HRESULT,
    pub SetQuorumLogSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nlogsize: i32) -> ::windows::core::HRESULT,
    pub QuorumPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetQuorumPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Nodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Nodes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResourceGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclusterresourcegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResourceGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclusterresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resources: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResourceTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresourcetypes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResourceTypes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Networks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetworks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Networks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NetInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NetInterfaces: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISClusterNames(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISClusterNames {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DomainName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DomainName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISClusterNames, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISClusterNames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISClusterNames {
    type Vtable = ISClusterNames_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISClusterNames {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606ec_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusterNames_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, pbstrclustername: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub DomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdomainname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISDomainNames(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISDomainNames {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISDomainNames, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISDomainNames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISDomainNames {
    type Vtable = ISDomainNames_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISDomainNames {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606e2_2631_11d1_89f1_00a0c90d061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISDomainNames_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, pbstrdomainname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IWCContextMenuCallback(::windows::core::IUnknown);
impl IWCContextMenuCallback {
    pub unsafe fn AddExtensionMenuItem(&self, lpszname: &::windows::core::BSTR, lpszstatusbartext: &::windows::core::BSTR, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddExtensionMenuItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(lpszname), ::core::mem::transmute_copy(lpszstatusbartext), ncommandid, nsubmenucommandid, uflags).ok()
    }
}
::windows::core::interface_hierarchy!(IWCContextMenuCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWCContextMenuCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWCContextMenuCallback {
    type Vtable = IWCContextMenuCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IWCContextMenuCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede64_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWCContextMenuCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddExtensionMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszname: *mut ::core::ffi::c_void, lpszstatusbartext: *mut ::core::ffi::c_void, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IWCPropertySheetCallback(::windows::core::IUnknown);
impl IWCPropertySheetCallback {
    pub unsafe fn AddPropertySheetPage(&self, hpage: *const i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddPropertySheetPage)(::windows::core::Vtable::as_raw(self), hpage).ok()
    }
}
::windows::core::interface_hierarchy!(IWCPropertySheetCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWCPropertySheetCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWCPropertySheetCallback {
    type Vtable = IWCPropertySheetCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IWCPropertySheetCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede60_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWCPropertySheetCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddPropertySheetPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IWCWizard97Callback(::windows::core::IUnknown);
impl IWCWizard97Callback {
    pub unsafe fn AddWizard97Page(&self, hpage: *const i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddWizard97Page)(::windows::core::Vtable::as_raw(self), hpage).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableNext<P0>(&self, hpage: *const i32, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).EnableNext)(::windows::core::Vtable::as_raw(self), hpage, benable.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IWCWizard97Callback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWCWizard97Callback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWCWizard97Callback {
    type Vtable = IWCWizard97Callback_Vtbl;
}
unsafe impl ::windows::core::Interface for IWCWizard97Callback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede67_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWCWizard97Callback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddWizard97Page: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableNext: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IWCWizardCallback(::windows::core::IUnknown);
impl IWCWizardCallback {
    pub unsafe fn AddWizardPage(&self, hpage: *const i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddWizardPage)(::windows::core::Vtable::as_raw(self), hpage).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableNext<P0>(&self, hpage: *const i32, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).EnableNext)(::windows::core::Vtable::as_raw(self), hpage, benable.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IWCWizardCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWCWizardCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWCWizardCallback {
    type Vtable = IWCWizardCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IWCWizardCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede62_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWCWizardCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddWizardPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpage: *const i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableNext: usize,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IWEExtendContextMenu(::windows::core::IUnknown);
impl IWEExtendContextMenu {
    pub unsafe fn AddContextMenuItems<P0, P1>(&self, pidata: P0, picallback: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWCContextMenuCallback>>,
    {
        (::windows::core::Vtable::vtable(self).AddContextMenuItems)(::windows::core::Vtable::as_raw(self), pidata.into().abi(), picallback.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWEExtendContextMenu, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWEExtendContextMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWEExtendContextMenu {
    type Vtable = IWEExtendContextMenu_Vtbl;
}
unsafe impl ::windows::core::Interface for IWEExtendContextMenu {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede65_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWEExtendContextMenu_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddContextMenuItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IWEExtendPropertySheet(::windows::core::IUnknown);
impl IWEExtendPropertySheet {
    pub unsafe fn CreatePropertySheetPages<P0, P1>(&self, pidata: P0, picallback: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWCPropertySheetCallback>>,
    {
        (::windows::core::Vtable::vtable(self).CreatePropertySheetPages)(::windows::core::Vtable::as_raw(self), pidata.into().abi(), picallback.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWEExtendPropertySheet, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWEExtendPropertySheet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWEExtendPropertySheet {
    type Vtable = IWEExtendPropertySheet_Vtbl;
}
unsafe impl ::windows::core::Interface for IWEExtendPropertySheet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede61_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWEExtendPropertySheet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreatePropertySheetPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IWEExtendWizard(::windows::core::IUnknown);
impl IWEExtendWizard {
    pub unsafe fn CreateWizardPages<P0, P1>(&self, pidata: P0, picallback: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWCWizardCallback>>,
    {
        (::windows::core::Vtable::vtable(self).CreateWizardPages)(::windows::core::Vtable::as_raw(self), pidata.into().abi(), picallback.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWEExtendWizard, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWEExtendWizard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWEExtendWizard {
    type Vtable = IWEExtendWizard_Vtbl;
}
unsafe impl ::windows::core::Interface for IWEExtendWizard {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede63_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWEExtendWizard_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateWizardPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IWEExtendWizard97(::windows::core::IUnknown);
impl IWEExtendWizard97 {
    pub unsafe fn CreateWizard97Pages<P0, P1>(&self, pidata: P0, picallback: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWCWizard97Callback>>,
    {
        (::windows::core::Vtable::vtable(self).CreateWizard97Pages)(::windows::core::Vtable::as_raw(self), pidata.into().abi(), picallback.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWEExtendWizard97, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWEExtendWizard97 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWEExtendWizard97 {
    type Vtable = IWEExtendWizard97_Vtbl;
}
unsafe impl ::windows::core::Interface for IWEExtendWizard97 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede68_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWEExtendWizard97_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateWizard97Pages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidata: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
pub struct IWEInvokeCommand(::windows::core::IUnknown);
impl IWEInvokeCommand {
    pub unsafe fn InvokeCommand<P0>(&self, ncommandid: u32, pidata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).InvokeCommand)(::windows::core::Vtable::as_raw(self), ncommandid, pidata.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWEInvokeCommand, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWEInvokeCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWEInvokeCommand {
    type Vtable = IWEInvokeCommand_Vtbl;
}
unsafe impl ::windows::core::Interface for IWEInvokeCommand {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dede66_fc6b_11cf_b5f5_00a0c90ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWEInvokeCommand_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InvokeCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncommandid: u32, pidata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitLockerDecrypted: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitLockerDecrypting: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitLockerEnabled: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitLockerPaused: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitLockerStopped: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitlockerEncrypted: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitlockerEncrypting: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CA_UPGRADE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLUSTER_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GLOBAL_SHIFT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_INTERNAL_SHIFT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_MODIFY_SHIFT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_USER_SHIFT: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLOUD_WITNESS_CONTAINER_NAME: ::windows::core::PCWSTR = ::windows::w!("msft-cloud-witness");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLRES_VERSION_V1_00: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLRES_VERSION_V2_00: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLRES_VERSION_V3_00: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLRES_VERSION_V4_00: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_CHANGE_ACCESS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_CHANGE_RESOURCE_GROUP_FORCE_MOVE_TO_CSV: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_FAILBACK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_HIGH_PRIORITY_START: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_IGNORE_AFFINITY_RULE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_QUEUE_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_ONLINE_BEST_POSSIBLE_NODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_ONLINE_IGNORE_AFFINITY_RULE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_ONLINE_SYNCHRONOUS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_NODE_AVOID_PLACEMENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_NODE_PAUSE_REMAIN_ON_PAUSED_NODE_ON_MOVE_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_NODE_PAUSE_RETRY_DRAIN_ON_FAILURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_NO_ACCESS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_READ_ACCESS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_FORCE_WITH_TERMINATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_DELETED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_RESTARTED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_MOVING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_PREEMPTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_SHUTTING_DOWN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_USER_REQUESTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_ONLINE_BEST_POSSIBLE_NODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_ONLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_AFFINITY_RULE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_ONLINE_NECESSARY_FOR_QUORUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VALID_CHANGE_RESOURCE_GROUP_FLAGS: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_RS3: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_SERVER2008: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_SERVER2008R2: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_WINDOWS8: u32 = 1793u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_WINDOWSBLUE: u32 = 1794u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_WINTHRESHOLD: u32 = 1795u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_ACCESS_MODE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_ACCESS_SHIFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CONTROL_CODE_MASK: u32 = 4194303u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_FUNCTION_SHIFT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GET_OPERATION_CONTEXT_PARAMS_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_OBJECT_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_OBJECT_SHIFT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_ADD_VOLUME_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_FILTER_BY_POOL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_INCLUDE_NON_SHARED_DISKS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGROUPSET_STATUS_APPLICATION_READY: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGROUPSET_STATUS_GROUPS_ONLINE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGROUPSET_STATUS_GROUPS_PENDING: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGROUPSET_STATUS_OS_HEARTBEAT: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_APPLICATION_READY: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_EMBEDDED_FAILURE: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_LOCKED_MODE: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_NETWORK_FAILURE: u64 = 128u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_OFFLINE_DUE_TO_ANTIAFFINITY_CONFLICT: u64 = 64u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 2048u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_OS_HEARTBEAT: u64 = 512u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_PHYSICAL_RESOURCES_LACKING: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_PREEMPTED: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_UNMONITORED: u64 = 256u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_WAITING_FOR_DEPENDENCIES: u64 = 4096u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_WAITING_IN_QUEUE_FOR_MOVE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_WAITING_TO_START: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_DATABASE_ISOLATE_READ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_DATABASE_SYNC_WRITE_TO_ALL_NODES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_KEYNAME_OBJECTGUIDS: ::windows::core::PCWSTR = ::windows::w!("ObjectGUIDs");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_AFFINITYRULE_ENABLED: ::windows::core::PCWSTR = ::windows::w!("Enabled");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_AFFINITYRULE_GROUPS: ::windows::core::PCWSTR = ::windows::w!("Groups");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_AFFINITYRULE_NAME: ::windows::core::PCWSTR = ::windows::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_AFFINITYRULE_TYPE: ::windows::core::PCWSTR = ::windows::w!("RuleType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLOUDWITNESS_ACCOUNT_NAME: ::windows::core::PCWSTR = ::windows::w!("AccountName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLOUDWITNESS_CONTAINER_NAME: ::windows::core::PCWSTR = ::windows::w!("ContainerName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLOUDWITNESS_ENDPOINT_INFO: ::windows::core::PCWSTR = ::windows::w!("EndpointInfo");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLOUDWITNESS_PRIMARY_KEY: ::windows::core::PCWSTR = ::windows::w!("PrimaryKey");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLOUDWITNESS_PRIMARY_TOKEN: ::windows::core::PCWSTR = ::windows::w!("PrimaryToken");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLUS_DEFAULT_NETWORK_ROLE: ::windows::core::PCWSTR = ::windows::w!("DefaultNetworkRole");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLUS_DESC: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLUS_SD: ::windows::core::PCWSTR = ::windows::w!("Security Descriptor");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CROSS_SITE_DELAY: ::windows::core::PCWSTR = ::windows::w!("CrossSiteDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CROSS_SITE_THRESHOLD: ::windows::core::PCWSTR = ::windows::w!("CrossSiteThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CROSS_SUBNET_DELAY: ::windows::core::PCWSTR = ::windows::w!("CrossSubnetDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CROSS_SUBNET_THRESHOLD: ::windows::core::PCWSTR = ::windows::w!("CrossSubnetThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CSV_BLOCK_CACHE: ::windows::core::PCWSTR = ::windows::w!("BlockCacheSize");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CSV_MDS_SD: ::windows::core::PCWSTR = ::windows::w!("SharedVolumeSecurityDescriptor");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_DATABASE_READ_WRITE_MODE: ::windows::core::PCWSTR = ::windows::w!("DatabaseReadWriteMode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_DDA_DEVICE_ALLOCATIONS: ::windows::core::PCWSTR = ::windows::w!("DdaDeviceAllocations");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_DHCP_BACKUP_PATH: ::windows::core::PCWSTR = ::windows::w!("BackupPath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_DHCP_DATABASE_PATH: ::windows::core::PCWSTR = ::windows::w!("DatabasePath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_DRAIN_ON_SHUTDOWN: ::windows::core::PCWSTR = ::windows::w!("DrainOnShutdown");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_ENABLED_EVENT_LOGS: ::windows::core::PCWSTR = ::windows::w!("EnabledEventLogs");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FAILOVER_MOVE_MIGRATION_TYPE: ::windows::core::PCWSTR = ::windows::w!("FailoverMoveMigrationType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_CA_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("CATimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_HIDE_SUBDIR_SHARES: ::windows::core::PCWSTR = ::windows::w!("HideSubDirShares");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_IS_DFS_ROOT: ::windows::core::PCWSTR = ::windows::w!("IsDfsRoot");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_MAX_USERS: ::windows::core::PCWSTR = ::windows::w!("MaxUsers");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_PATH: ::windows::core::PCWSTR = ::windows::w!("Path");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_REMARK: ::windows::core::PCWSTR = ::windows::w!("Remark");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_SD: ::windows::core::PCWSTR = ::windows::w!("Security Descriptor");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_SERVER_NAME: ::windows::core::PCWSTR = ::windows::w!("ServerName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_SHARE_FLAGS: ::windows::core::PCWSTR = ::windows::w!("ShareFlags");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_SHARE_NAME: ::windows::core::PCWSTR = ::windows::w!("ShareName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_SHARE_SUBDIRS: ::windows::core::PCWSTR = ::windows::w!("ShareSubDirs");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FIXQUORUM: ::windows::core::PCWSTR = ::windows::w!("FixQuorum");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FSWITNESS_ARB_DELAY: ::windows::core::PCWSTR = ::windows::w!("ArbitrationDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FSWITNESS_IMPERSONATE_CNO: ::windows::core::PCWSTR = ::windows::w!("ImpersonateCNO");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FSWITNESS_SHARE_PATH: ::windows::core::PCWSTR = ::windows::w!("SharePath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FUNCTIONAL_LEVEL: ::windows::core::PCWSTR = ::windows::w!("ClusterFunctionalLevel");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENAPP_COMMAND_LINE: ::windows::core::PCWSTR = ::windows::w!("CommandLine");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENAPP_CURRENT_DIRECTORY: ::windows::core::PCWSTR = ::windows::w!("CurrentDirectory");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENAPP_USE_NETWORK_NAME: ::windows::core::PCWSTR = ::windows::w!("UseNetworkName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENSCRIPT_SCRIPT_FILEPATH: ::windows::core::PCWSTR = ::windows::w!("ScriptFilepath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENSVC_SERVICE_NAME: ::windows::core::PCWSTR = ::windows::w!("ServiceName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENSVC_STARTUP_PARAMS: ::windows::core::PCWSTR = ::windows::w!("StartupParameters");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENSVC_USE_NETWORK_NAME: ::windows::core::PCWSTR = ::windows::w!("UseNetworkName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GPUP_DEVICE_ALLOCATIONS: ::windows::core::PCWSTR = ::windows::w!("GpupDeviceAllocations");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_AVAILABILITY_SET_INDEX_TO_NODE_MAPPING: ::windows::core::PCWSTR = ::windows::w!("NodeDomainInfo");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_FAULT_DOMAINS: ::windows::core::PCWSTR = ::windows::w!("FaultDomains");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_IS_AVAILABILITY_SET: ::windows::core::PCWSTR = ::windows::w!("IsAvailabilitySet");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_IS_GLOBAL: ::windows::core::PCWSTR = ::windows::w!("IsGlobal");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_NAME: ::windows::core::PCWSTR = ::windows::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_RESERVE_NODE: ::windows::core::PCWSTR = ::windows::w!("ReserveSpareNode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_STARTUP_COUNT: ::windows::core::PCWSTR = ::windows::w!("StartupCount");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_STARTUP_DELAY: ::windows::core::PCWSTR = ::windows::w!("StartupDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_STARTUP_SETTING: ::windows::core::PCWSTR = ::windows::w!("StartupSetting");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_STATUS_INFORMATION: ::windows::core::PCWSTR = ::windows::w!("StatusInformation");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_UPDATE_DOMAINS: ::windows::core::PCWSTR = ::windows::w!("UpdateDomains");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUP_DEPENDENCY_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("GroupDependencyTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_ANTI_AFFINITY_CLASS_NAME: ::windows::core::PCWSTR = ::windows::w!("AntiAffinityClassNames");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_CCF_EPOCH: ::windows::core::PCWSTR = ::windows::w!("CCFEpoch");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_CCF_EPOCH_HIGH: ::windows::core::PCWSTR = ::windows::w!("CCFEpochHigh");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_COLD_START_SETTING: ::windows::core::PCWSTR = ::windows::w!("ColdStartSetting");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_DEFAULT_OWNER: ::windows::core::PCWSTR = ::windows::w!("DefaultOwner");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_DESC: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAILBACK_TYPE: ::windows::core::PCWSTR = ::windows::w!("AutoFailbackType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAILBACK_WIN_END: ::windows::core::PCWSTR = ::windows::w!("FailbackWindowEnd");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAILBACK_WIN_START: ::windows::core::PCWSTR = ::windows::w!("FailbackWindowStart");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAILOVER_PERIOD: ::windows::core::PCWSTR = ::windows::w!("FailoverPeriod");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAILOVER_THRESHOLD: ::windows::core::PCWSTR = ::windows::w!("FailoverThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAULT_DOMAIN: ::windows::core::PCWSTR = ::windows::w!("FaultDomain");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_LOCK_MOVE: ::windows::core::PCWSTR = ::windows::w!("LockedFromMoving");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_NAME: ::windows::core::PCWSTR = ::windows::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_PERSISTENT_STATE: ::windows::core::PCWSTR = ::windows::w!("PersistentState");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_PLACEMENT_OPTIONS: ::windows::core::PCWSTR = ::windows::w!("PlacementOptions");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_PREFERRED_SITE: ::windows::core::PCWSTR = ::windows::w!("PreferredSite");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_PRIORITY: ::windows::core::PCWSTR = ::windows::w!("Priority");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_RESILIENCY_PERIOD: ::windows::core::PCWSTR = ::windows::w!("ResiliencyPeriod");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_START_DELAY: ::windows::core::PCWSTR = ::windows::w!("GroupStartDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_STATUS_INFORMATION: ::windows::core::PCWSTR = ::windows::w!("StatusInformation");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_TYPE: ::windows::core::PCWSTR = ::windows::w!("GroupType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_UPDATE_DOMAIN: ::windows::core::PCWSTR = ::windows::w!("UpdateDomain");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IGNORE_PERSISTENT_STATE: ::windows::core::PCWSTR = ::windows::w!("IgnorePersistentStateOnStartup");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_DHCP_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("DhcpAddress");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_DHCP_SERVER: ::windows::core::PCWSTR = ::windows::w!("DhcpServer");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_DHCP_SUBNET_MASK: ::windows::core::PCWSTR = ::windows::w!("DhcpSubnetMask");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_ENABLE_DHCP: ::windows::core::PCWSTR = ::windows::w!("EnableDhcp");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_ENABLE_NETBIOS: ::windows::core::PCWSTR = ::windows::w!("EnableNetBIOS");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_LEASE_OBTAINED_TIME: ::windows::core::PCWSTR = ::windows::w!("LeaseObtainedTime");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_LEASE_TERMINATES_TIME: ::windows::core::PCWSTR = ::windows::w!("LeaseExpiresTime");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_NETWORK: ::windows::core::PCWSTR = ::windows::w!("Network");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_OVERRIDE_ADDRMATCH: ::windows::core::PCWSTR = ::windows::w!("OverrideAddressMatch");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_PROBE_FAILURE_THRESHOLD: ::windows::core::PCWSTR = ::windows::w!("ProbeFailureThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_PROBE_PORT: ::windows::core::PCWSTR = ::windows::w!("ProbePort");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_SHARED_NETNAME: ::windows::core::PCWSTR = ::windows::w!("SharedNetname");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_SUBNET_MASK: ::windows::core::PCWSTR = ::windows::w!("SubnetMask");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_T1: ::windows::core::PCWSTR = ::windows::w!("T1");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_T2: ::windows::core::PCWSTR = ::windows::w!("T2");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPV6_NATIVE_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPV6_NATIVE_NETWORK: ::windows::core::PCWSTR = ::windows::w!("Network");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPV6_NATIVE_PREFIX_LENGTH: ::windows::core::PCWSTR = ::windows::w!("PrefixLength");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPV6_TUNNEL_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPV6_TUNNEL_TUNNELTYPE: ::windows::core::PCWSTR = ::windows::w!("TunnelType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_LAST_RECENT_EVENTS_RESET_TIME: ::windows::core::PCWSTR = ::windows::w!("RecentEventsResetTime");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_LOG_FILE_PATH: ::windows::core::PCWSTR = ::windows::w!("LogFilePath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_MESSAGE_BUFFER_LENGTH: ::windows::core::PCWSTR = ::windows::w!("MessageBufferLength");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_MIXED_MODE: ::windows::core::PCWSTR = ::windows::w!("MixedMode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETFT_IPSEC_ENABLED: ::windows::core::PCWSTR = ::windows::w!("NetftIPSecEnabled");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_ADAPTER_ID: ::windows::core::PCWSTR = ::windows::w!("AdapterId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_ADAPTER_NAME: ::windows::core::PCWSTR = ::windows::w!("Adapter");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_DESC: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_DHCP_ENABLED: ::windows::core::PCWSTR = ::windows::w!("DhcpEnabled");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_IPV4_ADDRESSES: ::windows::core::PCWSTR = ::windows::w!("IPv4Addresses");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_IPV6_ADDRESSES: ::windows::core::PCWSTR = ::windows::w!("IPv6Addresses");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_NAME: ::windows::core::PCWSTR = ::windows::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_NETWORK: ::windows::core::PCWSTR = ::windows::w!("Network");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_NODE: ::windows::core::PCWSTR = ::windows::w!("Node");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_AD_AWARE: ::windows::core::PCWSTR = ::windows::w!("ADAware");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_ALIASES: ::windows::core::PCWSTR = ::windows::w!("Aliases");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_CONTAINERGUID: ::windows::core::PCWSTR = ::windows::w!("CryptoContainerGUID");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_CREATING_DC: ::windows::core::PCWSTR = ::windows::w!("CreatingDC");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_DNN_DISABLE_CLONES: ::windows::core::PCWSTR = ::windows::w!("DisableClones");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_DNS_NAME: ::windows::core::PCWSTR = ::windows::w!("DnsName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_DNS_SUFFIX: ::windows::core::PCWSTR = ::windows::w!("DnsSuffix");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_EXCLUDE_NETWORKS: ::windows::core::PCWSTR = ::windows::w!("ExcludeNetworks");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_HOST_TTL: ::windows::core::PCWSTR = ::windows::w!("HostRecordTTL");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_IN_USE_NETWORKS: ::windows::core::PCWSTR = ::windows::w!("InUseNetworks");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_LAST_DNS_UPDATE: ::windows::core::PCWSTR = ::windows::w!("LastDNSUpdateTime");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_NAME: ::windows::core::PCWSTR = ::windows::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_OBJECT_ID: ::windows::core::PCWSTR = ::windows::w!("ObjectGUID");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_PUBLISH_PTR: ::windows::core::PCWSTR = ::windows::w!("PublishPTRRecords");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_REGISTER_ALL_IP: ::windows::core::PCWSTR = ::windows::w!("RegisterAllProvidersIP");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_REMAP_PIPE_NAMES: ::windows::core::PCWSTR = ::windows::w!("RemapPipeNames");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_REMOVEVCO_ONDELETE: ::windows::core::PCWSTR = ::windows::w!("DeleteVcoOnResCleanup");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_RESOURCE_DATA: ::windows::core::PCWSTR = ::windows::w!("ResourceData");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_STATUS_DNS: ::windows::core::PCWSTR = ::windows::w!("StatusDNS");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_STATUS_KERBEROS: ::windows::core::PCWSTR = ::windows::w!("StatusKerberos");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_STATUS_NETBIOS: ::windows::core::PCWSTR = ::windows::w!("StatusNetBIOS");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_VCO_CONTAINER: ::windows::core::PCWSTR = ::windows::w!("VcoContainer");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_ADDRESS_MASK: ::windows::core::PCWSTR = ::windows::w!("AddressMask");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_AUTOMETRIC: ::windows::core::PCWSTR = ::windows::w!("AutoMetric");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_DESC: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_IPV4_ADDRESSES: ::windows::core::PCWSTR = ::windows::w!("IPv4Addresses");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_IPV4_PREFIXLENGTHS: ::windows::core::PCWSTR = ::windows::w!("IPv4PrefixLengths");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_IPV6_ADDRESSES: ::windows::core::PCWSTR = ::windows::w!("IPv6Addresses");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_IPV6_PREFIXLENGTHS: ::windows::core::PCWSTR = ::windows::w!("IPv6PrefixLengths");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_METRIC: ::windows::core::PCWSTR = ::windows::w!("Metric");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_NAME: ::windows::core::PCWSTR = ::windows::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_RDMA_CAPABLE: ::windows::core::PCWSTR = ::windows::w!("RdmaCapable");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_ROLE: ::windows::core::PCWSTR = ::windows::w!("Role");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_RSS_CAPABLE: ::windows::core::PCWSTR = ::windows::w!("RssCapable");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_SPEED: ::windows::core::PCWSTR = ::windows::w!("LinkSpeed");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_BUILD_NUMBER: ::windows::core::PCWSTR = ::windows::w!("BuildNumber");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_CSDVERSION: ::windows::core::PCWSTR = ::windows::w!("CSDVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_DESC: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_DRAIN_STATUS: ::windows::core::PCWSTR = ::windows::w!("NodeDrainStatus");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_DRAIN_TARGET: ::windows::core::PCWSTR = ::windows::w!("NodeDrainTarget");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_DYNAMIC_WEIGHT: ::windows::core::PCWSTR = ::windows::w!("DynamicWeight");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_FAULT_DOMAIN: ::windows::core::PCWSTR = ::windows::w!("FaultDomain");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_FDID: ::windows::core::PCWSTR = ::windows::w!("FaultDomainId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_HIGHEST_VERSION: ::windows::core::PCWSTR = ::windows::w!("NodeHighestVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_IS_PRIMARY: ::windows::core::PCWSTR = ::windows::w!("IsPrimary");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_LOWEST_VERSION: ::windows::core::PCWSTR = ::windows::w!("NodeLowestVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_MAJOR_VERSION: ::windows::core::PCWSTR = ::windows::w!("MajorVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_MANUFACTURER: ::windows::core::PCWSTR = ::windows::w!("Manufacturer");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_MINOR_VERSION: ::windows::core::PCWSTR = ::windows::w!("MinorVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_MODEL: ::windows::core::PCWSTR = ::windows::w!("Model");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_NAME: ::windows::core::PCWSTR = ::windows::w!("NodeName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_NEEDS_PQ: ::windows::core::PCWSTR = ::windows::w!("NeedsPreventQuorum");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_SERIALNUMBER: ::windows::core::PCWSTR = ::windows::w!("SerialNumber");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_STATUS_INFO: ::windows::core::PCWSTR = ::windows::w!("StatusInformation");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_WEIGHT: ::windows::core::PCWSTR = ::windows::w!("NodeWeight");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_CSVBLOCKCACHE: ::windows::core::PCWSTR = ::windows::w!("EnableBlockCache");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTAGELIMIT: ::windows::core::PCWSTR = ::windows::w!("SnapshotAgeLimit");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTDIFFAREASIZE: ::windows::core::PCWSTR = ::windows::w!("SnapshotDiffSize");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_CSVWRITETHROUGH: ::windows::core::PCWSTR = ::windows::w!("CsvEnforceWriteThrough");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKARBINTERVAL: ::windows::core::PCWSTR = ::windows::w!("DiskArbInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKARBTYPE: ::windows::core::PCWSTR = ::windows::w!("DiskArbType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKGUID: ::windows::core::PCWSTR = ::windows::w!("DiskGuid");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKIDGUID: ::windows::core::PCWSTR = ::windows::w!("DiskIdGuid");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKIDTYPE: ::windows::core::PCWSTR = ::windows::w!("DiskIdType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKIODELAY: ::windows::core::PCWSTR = ::windows::w!("MaxIoLatency");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKPATH: ::windows::core::PCWSTR = ::windows::w!("DiskPath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKRECOVERYACTION: ::windows::core::PCWSTR = ::windows::w!("DiskRecoveryAction");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKRELOAD: ::windows::core::PCWSTR = ::windows::w!("DiskReload");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKRUNCHKDSK: ::windows::core::PCWSTR = ::windows::w!("DiskRunChkDsk");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKSIGNATURE: ::windows::core::PCWSTR = ::windows::w!("DiskSignature");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKUNIQUEIDS: ::windows::core::PCWSTR = ::windows::w!("DiskUniqueIds");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKVOLUMEINFO: ::windows::core::PCWSTR = ::windows::w!("DiskVolumeInfo");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_FASTONLINEARBITRATE: ::windows::core::PCWSTR = ::windows::w!("FastOnlineArbitrate");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_MAINTMODE: ::windows::core::PCWSTR = ::windows::w!("MaintenanceMode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_MIGRATEFIXUP: ::windows::core::PCWSTR = ::windows::w!("MigrateDriveLetters");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_SPACEIDGUID: ::windows::core::PCWSTR = ::windows::w!("VirtualDiskId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_VOLSNAPACTIVATETIMEOUT: ::windows::core::PCWSTR = ::windows::w!("VolsnapActivateTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PLACEMENT_OPTIONS: ::windows::core::PCWSTR = ::windows::w!("PlacementOptions");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PLUMB_ALL_CROSS_SUBNET_ROUTES: ::windows::core::PCWSTR = ::windows::w!("PlumbAllCrossSubnetRoutes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PREVENTQUORUM: ::windows::core::PCWSTR = ::windows::w!("PreventQuorum");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PRTSPOOL_DEFAULT_SPOOL_DIR: ::windows::core::PCWSTR = ::windows::w!("DefaultSpoolDirectory");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PRTSPOOL_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("JobCompletionTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_QUARANTINE_DURATION: ::windows::core::PCWSTR = ::windows::w!("QuarantineDuration");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_QUARANTINE_THRESHOLD: ::windows::core::PCWSTR = ::windows::w!("QuarantineThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_QUORUM_ARBITRATION_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("QuorumArbitrationTimeMax");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESILIENCY_DEFAULT_SECONDS: ::windows::core::PCWSTR = ::windows::w!("ResiliencyDefaultPeriod");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESILIENCY_LEVEL: ::windows::core::PCWSTR = ::windows::w!("ResiliencyLevel");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_ADMIN_EXTENSIONS: ::windows::core::PCWSTR = ::windows::w!("AdminExtensions");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DEADLOCK_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("DeadlockTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DESC: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DLL_NAME: ::windows::core::PCWSTR = ::windows::w!("DllName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DUMP_LOG_QUERY: ::windows::core::PCWSTR = ::windows::w!("DumpLogQuery");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DUMP_POLICY: ::windows::core::PCWSTR = ::windows::w!("DumpPolicy");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DUMP_SERVICES: ::windows::core::PCWSTR = ::windows::w!("DumpServices");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_ENABLED_EVENT_LOGS: ::windows::core::PCWSTR = ::windows::w!("EnabledEventLogs");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_IS_ALIVE: ::windows::core::PCWSTR = ::windows::w!("IsAlivePollInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_LOOKS_ALIVE: ::windows::core::PCWSTR = ::windows::w!("LooksAlivePollInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_MAX_MONITORS: ::windows::core::PCWSTR = ::windows::w!("MaximumMonitors");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_NAME: ::windows::core::PCWSTR = ::windows::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_PENDING_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("PendingTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_WPR_PROFILES: ::windows::core::PCWSTR = ::windows::w!("WprProfiles");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_WPR_START_AFTER: ::windows::core::PCWSTR = ::windows::w!("WprStartAfter");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_DATA1: ::windows::core::PCWSTR = ::windows::w!("ResourceSpecificData1");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_DATA2: ::windows::core::PCWSTR = ::windows::w!("ResourceSpecificData2");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_DEADLOCK_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("DeadlockTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_DESC: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_EMBEDDED_FAILURE_ACTION: ::windows::core::PCWSTR = ::windows::w!("EmbeddedFailureAction");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_IS_ALIVE: ::windows::core::PCWSTR = ::windows::w!("IsAlivePollInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_LAST_OPERATION_STATUS_CODE: ::windows::core::PCWSTR = ::windows::w!("LastOperationStatusCode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_LOOKS_ALIVE: ::windows::core::PCWSTR = ::windows::w!("LooksAlivePollInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_MONITOR_PID: ::windows::core::PCWSTR = ::windows::w!("MonitorProcessId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_NAME: ::windows::core::PCWSTR = ::windows::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_PENDING_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("PendingTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_PERSISTENT_STATE: ::windows::core::PCWSTR = ::windows::w!("PersistentState");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_RESTART_ACTION: ::windows::core::PCWSTR = ::windows::w!("RestartAction");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_RESTART_DELAY: ::windows::core::PCWSTR = ::windows::w!("RestartDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_RESTART_PERIOD: ::windows::core::PCWSTR = ::windows::w!("RestartPeriod");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_RESTART_THRESHOLD: ::windows::core::PCWSTR = ::windows::w!("RestartThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_RETRY_PERIOD_ON_FAILURE: ::windows::core::PCWSTR = ::windows::w!("RetryPeriodOnFailure");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_SEPARATE_MONITOR: ::windows::core::PCWSTR = ::windows::w!("SeparateMonitor");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_STATUS: ::windows::core::PCWSTR = ::windows::w!("ResourceSpecificStatus");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_STATUS_INFORMATION: ::windows::core::PCWSTR = ::windows::w!("StatusInformation");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_TYPE: ::windows::core::PCWSTR = ::windows::w!("Type");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_ROUTE_HISTORY_LENGTH: ::windows::core::PCWSTR = ::windows::w!("RouteHistoryLength");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_SAME_SUBNET_DELAY: ::windows::core::PCWSTR = ::windows::w!("SameSubnetDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_SAME_SUBNET_THRESHOLD: ::windows::core::PCWSTR = ::windows::w!("SameSubnetThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_SHUTDOWN_TIMEOUT_MINUTES: ::windows::core::PCWSTR = ::windows::w!("ShutdownTimeoutInMinutes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_SOFS_SMBASYMMETRYMODE: ::windows::core::PCWSTR = ::windows::w!("SmbAsymmetryMode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_START_MEMORY: ::windows::core::PCWSTR = ::windows::w!("StartMemory");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_DESCRIPTION: ::windows::core::PCWSTR = ::windows::w!("VirtualDiskDescription");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_HEALTH: ::windows::core::PCWSTR = ::windows::w!("VirtualDiskHealth");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_NAME: ::windows::core::PCWSTR = ::windows::w!("VirtualDiskName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLARBITRATE: ::windows::core::PCWSTR = ::windows::w!("Arbitrate");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLCONSUMEDCAPACITY: ::windows::core::PCWSTR = ::windows::w!("ConsumedCapacity");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLDESC: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLDRIVEIDS: ::windows::core::PCWSTR = ::windows::w!("DriveIds");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLHEALTH: ::windows::core::PCWSTR = ::windows::w!("Health");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLIDGUID: ::windows::core::PCWSTR = ::windows::w!("PoolId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLNAME: ::windows::core::PCWSTR = ::windows::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLQUORUMSHARE: ::windows::core::PCWSTR = ::windows::w!("PoolQuorumShare");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLQUORUMUSERACCOUNT: ::windows::core::PCWSTR = ::windows::w!("PoolQuorumUserAccount");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLREEVALTIMEOUT: ::windows::core::PCWSTR = ::windows::w!("ReEvaluatePlacementTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLSTATE: ::windows::core::PCWSTR = ::windows::w!("State");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLTOTALCAPACITY: ::windows::core::PCWSTR = ::windows::w!("TotalCapacity");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_PROVISIONING: ::windows::core::PCWSTR = ::windows::w!("VirtualDiskProvisioning");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYCOLUMNS: ::windows::core::PCWSTR = ::windows::w!("VirtualDiskResiliencyColumns");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYINTERLEAVE: ::windows::core::PCWSTR = ::windows::w!("VirtualDiskResiliencyInterleave");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYTYPE: ::windows::core::PCWSTR = ::windows::w!("VirtualDiskResiliencyType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_STATE: ::windows::core::PCWSTR = ::windows::w!("VirtualDiskState");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_UPGRADE_VERSION: ::windows::core::PCWSTR = ::windows::w!("ClusterUpgradeVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIP_ADAPTER_NAME: ::windows::core::PCWSTR = ::windows::w!("AdapterName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIP_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIP_PREFIX_LENGTH: ::windows::core::PCWSTR = ::windows::w!("PrefixLength");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIP_RDID: ::windows::core::PCWSTR = ::windows::w!("RDID");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIP_VSID: ::windows::core::PCWSTR = ::windows::w!("VSID");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIRTUAL_NUMA_COUNT: ::windows::core::PCWSTR = ::windows::w!("VirtualNumaCount");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VSSTASK_APPNAME: ::windows::core::PCWSTR = ::windows::w!("ApplicationName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VSSTASK_APPPARAMS: ::windows::core::PCWSTR = ::windows::w!("ApplicationParams");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VSSTASK_CURRENTDIRECTORY: ::windows::core::PCWSTR = ::windows::w!("CurrentDirectory");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VSSTASK_TRIGGERARRAY: ::windows::core::PCWSTR = ::windows::w!("TriggerArray");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_WINS_BACKUP_PATH: ::windows::core::PCWSTR = ::windows::w!("BackupPath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_WINS_DATABASE_PATH: ::windows::core::PCWSTR = ::windows::w!("DatabasePath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_WITNESS_DYNAMIC_WEIGHT: ::windows::core::PCWSTR = ::windows::w!("WitnessDynamicWeight");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_DO_NOT_COLLECT_WER_REPORT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_DUMP_NOW: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_INSUFFICIENT_MEMORY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_INSUFFICIENT_OTHER_RESOURCES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_INSUFFICIENT_PROCESSOR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_INVALID_PARAMETERS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_NETWORK_NOT_AVAILABLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_BUSY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_REJECTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_THROTTLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_SOURCE_THROTTLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_NAME_GET_OPERATION_CONTEXT_FLAGS: ::windows::core::PCWSTR = ::windows::w!("Flags");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_APPLICATION_READY: u64 = 256u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_EMBEDDED_FAILURE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_CPU: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_GENERIC_RESOURCES: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_MEMORY: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_LOCKED_MODE: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_NETWORK_FAILURE: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 512u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_OS_HEARTBEAT: u64 = 128u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_UNMONITORED: u64 = 64u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ADD_EVICT_DELAY: ::windows::core::PCWSTR = ::windows::w!("AddEvictDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_AVAILABILITY_SET_CONFIG_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CONFIGURED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CREATE_GROUP_INFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CREATE_GROUP_INFO_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CSA_VSS_STATE: ::windows::core::PCWSTR = ::windows::w!("BackupInProgress");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CSV_COMPATIBLE_FILTERS: ::windows::core::PCWSTR = ::windows::w!("SharedVolumeCompatibleFilters");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CSV_INCOMPATIBLE_FILTERS: ::windows::core::PCWSTR = ::windows::w!("SharedVolumeIncompatibleFilters");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_DELETE_ACCESS_CONTROL_ENTRY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENFORCED_ANTIAFFINITY: ::windows::core::PCWSTR = ::windows::w!("ClusterEnforcedAntiaffinity");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_ITEM_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_ITEM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_WAIT_DELAY: ::windows::core::PCWSTR = ::windows::w!("ClusterGroupWaitDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HANG_RECOVERY_ACTION_KEYNAME: ::windows::core::PCWSTR = ::windows::w!("HangRecoveryAction");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HANG_TIMEOUT_KEYNAME: ::windows::core::PCWSTR = ::windows::w!("ClusSvcHangTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ARGS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_DESCRIPTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_DESCRIPTION_LABEL: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ERRORCODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ERRORCODE_LABEL: ::windows::core::PCWSTR = ::windows::w!("ErrorCode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ERRORTYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ERRORTYPE_LABEL: ::windows::core::PCWSTR = ::windows::w!("ErrorType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_FLAGS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_FLAGS_LABEL: ::windows::core::PCWSTR = ::windows::w!("Flags");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ID_LABEL: ::windows::core::PCWSTR = ::windows::w!("Id");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_PROPERTY_NAME: ::windows::core::PCWSTR = ::windows::w!("ClusterHealth");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_PROVIDER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_PROVIDER_LABEL: ::windows::core::PCWSTR = ::windows::w!("Provider");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_RESERVED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_RESERVED_LABEL: ::windows::core::PCWSTR = ::windows::w!("Reserved");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_INSTALLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NAME_AUTO_BALANCER_LEVEL: ::windows::core::PCWSTR = ::windows::w!("AutoBalancerLevel");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NAME_AUTO_BALANCER_MODE: ::windows::core::PCWSTR = ::windows::w!("AutoBalancerMode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NAME_PREFERRED_SITE: ::windows::core::PCWSTR = ::windows::w!("PreferredSite");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_REQUEST_REPLY_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("RequestReplyTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RUNNING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_BUS_TYPES: ::windows::core::PCWSTR = ::windows::w!("S2DBusTypes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_CACHE_BEHAVIOR_FLAGS: ::windows::core::PCWSTR = ::windows::w!("S2DCacheBehavior");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_CACHE_DESIRED_STATE: ::windows::core::PCWSTR = ::windows::w!("S2DCacheDesiredState");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_CACHE_FLASH_RESERVE_PERCENT: ::windows::core::PCWSTR = ::windows::w!("S2DCacheFlashReservePercent");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_CACHE_METADATA_RESERVE: ::windows::core::PCWSTR = ::windows::w!("S2DCacheMetadataReserveBytes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_CACHE_PAGE_SIZE_KBYTES: ::windows::core::PCWSTR = ::windows::w!("S2DCachePageSizeKBytes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_ENABLED: ::windows::core::PCWSTR = ::windows::w!("S2DEnabled");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_IO_LATENCY_THRESHOLD: ::windows::core::PCWSTR = ::windows::w!("S2DIOLatencyThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_OPTIMIZATIONS: ::windows::core::PCWSTR = ::windows::w!("S2DOptimizations");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_SET_ACCESS_TYPE_ALLOWED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_SET_ACCESS_TYPE_DENIED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_SHARED_VOLUMES_ROOT: ::windows::core::PCWSTR = ::windows::w!("SharedVolumesRoot");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_SHARED_VOLUME_VSS_WRITER_OPERATION_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("SharedVolumeVssWriterOperationTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_VERSION_FLAG_MIXED_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_VERSION_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_WITNESS_DATABASE_WRITE_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("WitnessDatabaseWriteTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_WITNESS_FAILED_RESTART_INTERVAL: ::windows::core::PCWSTR = ::windows::w!("WitnessRestartInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_ACCESS_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_ACCESS_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_ACCESS_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CREATE_CRYPT_CONTAINER_NOT_FOUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GLOBAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GRP_MOVE_ALLOWED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GRP_MOVE_LOCKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_HYBRID_QUORUM: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_MODIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_CLUSTER_GROUPID: ::windows::core::PCWSTR = ::windows::w!("ClusterGroupId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_DATA_RESID: ::windows::core::PCWSTR = ::windows::w!("DataResourceId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_LOG_MULTIPLE: ::windows::core::PCWSTR = ::windows::w!("LogSizeMultiple");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_LOG_RESID: ::windows::core::PCWSTR = ::windows::w!("LogResourceId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_LOG_VOLUME: ::windows::core::PCWSTR = ::windows::w!("LogVolume");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_MINIMUM_LOG_SIZE: ::windows::core::PCWSTR = ::windows::w!("MinimumLogSizeInBytes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_REPLICATION_GROUPID: ::windows::core::PCWSTR = ::windows::w!("ReplicationGroupId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_REPLICATION_GROUP_TYPE: ::windows::core::PCWSTR = ::windows::w!("ReplicationClusterGroupType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_SOURCE_RESID: ::windows::core::PCWSTR = ::windows::w!("SourceResourceId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_SOURCE_VOLUMES: ::windows::core::PCWSTR = ::windows::w!("SourceVolumes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_TARGET_RESID: ::windows::core::PCWSTR = ::windows::w!("TargetResourceId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_TARGET_VOLUMES: ::windows::core::PCWSTR = ::windows::w!("TargetVolumes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_UNIT_LOG_SIZE_CHANGE: ::windows::core::PCWSTR = ::windows::w!("UnitOfLogSizeChangeInBytes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NODE_MAJORITY_QUORUM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NOT_GLOBAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NO_MODIFY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_DUE_TO_EMBEDDED_FAILURE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_QUEUE_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_RETURNING_TO_SOURCE_NODE_BECAUSE_OF_ERROR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_ONLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_ONLINE_RECOVER_MONITOR_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_ONLINE_RESTORE_ONLINE_STATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_ONLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OPEN_DONT_DELETE_TEMP_DISK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OPEN_RECOVER_MONITOR_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_CAU: ::windows::core::PCWSTR = ::windows::w!("ClusterAwareUpdatingResource");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_CLOUD_WITNESS: ::windows::core::PCWSTR = ::windows::w!("Cloud Witness");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_CONTAINER: ::windows::core::PCWSTR = ::windows::w!("Container");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_CROSS_CLUSTER: ::windows::core::PCWSTR = ::windows::w!("Cross Cluster Dependency Orchestrator");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_DFS: ::windows::core::PCWSTR = ::windows::w!("Distributed File System");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_DFSR: ::windows::core::PCWSTR = ::windows::w!("DFS Replicated Folder");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_DHCP: ::windows::core::PCWSTR = ::windows::w!("DHCP Service");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_DNN: ::windows::core::PCWSTR = ::windows::w!("Distributed Network Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_FILESERVER: ::windows::core::PCWSTR = ::windows::w!("File Server");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_FILESHR: ::windows::core::PCWSTR = ::windows::w!("File Share");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_FSWITNESS: ::windows::core::PCWSTR = ::windows::w!("File Share Witness");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_GENAPP: ::windows::core::PCWSTR = ::windows::w!("Generic Application");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_GENSCRIPT: ::windows::core::PCWSTR = ::windows::w!("Generic Script");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_GENSVC: ::windows::core::PCWSTR = ::windows::w!("Generic Service");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_HARDDISK: ::windows::core::PCWSTR = ::windows::w!("Physical Disk");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_HCSVM: ::windows::core::PCWSTR = ::windows::w!("HCS Virtual Machine");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_HEALTH_SERVICE: ::windows::core::PCWSTR = ::windows::w!("Health Service");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_IPADDR: ::windows::core::PCWSTR = ::windows::w!("IP Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_IPV6_NATIVE: ::windows::core::PCWSTR = ::windows::w!("IPv6 Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_IPV6_TUNNEL: ::windows::core::PCWSTR = ::windows::w!("IPv6 Tunnel Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_ISCSITARGET: ::windows::core::PCWSTR = ::windows::w!("iSCSI Target Server");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_ISNS: ::windows::core::PCWSTR = ::windows::w!("Microsoft iSNS");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_MSDTC: ::windows::core::PCWSTR = ::windows::w!("Distributed Transaction Coordinator");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_MSMQ: ::windows::core::PCWSTR = ::windows::w!("Microsoft Message Queue Server");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_MSMQ_TRIGGER: ::windows::core::PCWSTR = ::windows::w!("MSMQTriggers");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NAT: ::windows::core::PCWSTR = ::windows::w!("Nat");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NETNAME: ::windows::core::PCWSTR = ::windows::w!("Network Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NETWORK_FILE_SYSTEM: ::windows::core::PCWSTR = ::windows::w!("Network File System");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NEW_MSMQ: ::windows::core::PCWSTR = ::windows::w!("MSMQ");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NFS: ::windows::core::PCWSTR = ::windows::w!("NFS Share");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NFS_MSNS: ::windows::core::PCWSTR = ::windows::w!("NFS Multi Server Namespace");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NFS_V2: ::windows::core::PCWSTR = ::windows::w!("Network File System");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NV_PROVIDER_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("Provider Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_PHYS_DISK: ::windows::core::PCWSTR = ::windows::w!("Physical Disk");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_PRTSPLR: ::windows::core::PCWSTR = ::windows::w!("Print Spooler");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_SCALEOUT_MASTER: ::windows::core::PCWSTR = ::windows::w!("Scaleout Master");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_SCALEOUT_WORKER: ::windows::core::PCWSTR = ::windows::w!("Scaleout Worker");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_SDDC_MANAGEMENT: ::windows::core::PCWSTR = ::windows::w!("SDDC Management");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_SODAFILESERVER: ::windows::core::PCWSTR = ::windows::w!("Scale Out File Server");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_STORAGE_POLICIES: ::windows::core::PCWSTR = ::windows::w!("Storage Policies");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_STORAGE_POOL: ::windows::core::PCWSTR = ::windows::w!("Storage Pool");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_STORAGE_REPLICA: ::windows::core::PCWSTR = ::windows::w!("Storage Replica");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_STORQOS: ::windows::core::PCWSTR = ::windows::w!("Storage QoS Policy Manager");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_TASKSCHEDULER: ::windows::core::PCWSTR = ::windows::w!("Task Scheduler");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VIRTUAL_IPV4: ::windows::core::PCWSTR = ::windows::w!("Disjoint IPv4 Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VIRTUAL_IPV6: ::windows::core::PCWSTR = ::windows::w!("Disjoint IPv6 Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VM: ::windows::core::PCWSTR = ::windows::w!("Virtual Machine");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VMREPLICA_BROKER: ::windows::core::PCWSTR = ::windows::w!("Virtual Machine Replication Broker");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VMREPLICA_COORDINATOR: ::windows::core::PCWSTR = ::windows::w!("Virtual Machine Replication Coordinator");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VM_CONFIG: ::windows::core::PCWSTR = ::windows::w!("Virtual Machine Configuration");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VM_WMI: ::windows::core::PCWSTR = ::windows::w!("Virtual Machine Cluster WMI");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VSSTASK: ::windows::core::PCWSTR = ::windows::w!("Volume Shadow Copy Service Task");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_WINS: ::windows::core::PCWSTR = ::windows::w!("WINS Service");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RES_NAME_SCALEOUT_MASTER: ::windows::core::PCWSTR = ::windows::w!("Scaleout Master");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RES_NAME_SCALEOUT_WORKER: ::windows::core::PCWSTR = ::windows::w!("Scaleout Worker");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CREATEDC_PRESENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CREATE_CLUSTER_MAJOR_VERSION_MASK: u32 = 4294967040u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CREATE_CLUSTER_VERSION: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusApplication: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606e5_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusCryptoKeys: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6072b_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusDisk: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60723_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusDisks: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60725_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNetInterface: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606ed_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNetInterfaces: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606ef_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNetwork: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606f1_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNetworkNetInterfaces: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606f5_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNetworks: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606f3_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606f7_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNodeNetInterfaces: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606fb_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNodes: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606f9_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPartition: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6071f_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPartitionEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53d51d26_b51b_4a79_b2c3_5048d93a98fc);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPartitions: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60721_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusProperties: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606ff_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusProperty: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606fd_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPropertyValue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60719_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPropertyValueData: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6071d_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPropertyValues: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6071b_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusRefObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60701_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusRegistryKeys: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60729_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResDependencies: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60703_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResDependents: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6072d_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResGroup: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60705_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResGroupPreferredOwnerNodes: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606e7_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResGroupResources: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606e9_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResGroups: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60707_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResPossibleOwnerNodes: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6070d_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResType: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6070f_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResTypePossibleOwnerNodes: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60717_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResTypeResources: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60713_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResTypes: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60711_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResource: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60709_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResources: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e6070b_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusScsiAddress: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60727_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusVersion: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e60715_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const Cluster: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606e3_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNames: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606eb_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const DNS_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const DomainNames: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e606e1_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ENABLE_CLUSTER_SHARED_VOLUMES: ::windows::core::PCWSTR = ::windows::w!("EnableSharedVolumes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FE_UPGRADE_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GROUPSET_READY_SETTING_APPLICATION_READY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GROUPSET_READY_SETTING_DELAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GROUPSET_READY_SETTING_ONLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GROUPSET_READY_SETTING_OS_HEARTBEAT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GROUP_FAILURE_INFO_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GUID_PRESENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const HCI_UPGRADE_BIT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const LOCKED_MODE_FLAGS_DONT_REMOVE_FROM_MOVE_QUEUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAINTENANCE_MODE_V2_SIG: u32 = 2881155087u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_CLUSTERNAME_LENGTH: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_CO_PASSWORD_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_CO_PASSWORD_LENGTHEX: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_CO_PASSWORD_STORAGEEX: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_CREATINGDC_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_OBJECTID: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MINIMUM_NEVER_PREEMPT_PRIORITY: ::windows::core::PCWSTR = ::windows::w!("MinimumNeverPreemptPriority");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MINIMUM_PREEMPTOR_PRIORITY: ::windows::core::PCWSTR = ::windows::w!("MinimumPreemptorPriority");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MN_UPGRADE_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NINETEEN_H1_UPGRADE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NINETEEN_H2_UPGRADE_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NNLEN: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT10_MAJOR_VERSION: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT11_MAJOR_VERSION: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT12_MAJOR_VERSION: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT13_MAJOR_VERSION: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT4SP4_MAJOR_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT4_MAJOR_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT51_MAJOR_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT5_MAJOR_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT6_MAJOR_VERSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT7_MAJOR_VERSION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT8_MAJOR_VERSION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT9_MAJOR_VERSION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESOURCE_FAILURE_INFO_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESTYPE_MONITOR_SHUTTING_DOWN_CLUSSVC_CRASH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESTYPE_MONITOR_SHUTTING_DOWN_NODE_STOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESUTIL_PROPITEM_IN_MEMORY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESUTIL_PROPITEM_READ_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESUTIL_PROPITEM_REQUIRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESUTIL_PROPITEM_SIGNED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RS3_UPGRADE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RS4_UPGRADE_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RS5_UPGRADE_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonBitLockerInitializing: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonFileSystemTiering: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonMax: u64 = 9223372036854775808u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonReFs: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonUnsafeFileSystemFilter: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonUnsafeVolumeFilter: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonUserRequest: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SET_APPINSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SR_REPLICATED_PARTITION_DISALLOW_MULTINODE_IO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const STARTUP_EX_ROUTINE: ::windows::core::PCSTR = ::windows::s!("StartupEx");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const STARTUP_ROUTINE: ::windows::core::PCSTR = ::windows::s!("Startup");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const USE_CLIENT_ACCESS_NETWORKS_FOR_CSV: ::windows::core::PCWSTR = ::windows::w!("UseClientAccessNetworksForSharedVolumes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeRedirectedIOReasonMax: u64 = 9223372036854775808u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeRedirectedIOReasonNoDiskConnectivity: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeRedirectedIOReasonStorageSpaceNotAttached: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeRedirectedIOReasonVolumeReplicationEnabled: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const WS2016_RTM_UPGRADE_VERSION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const WS2016_TP4_UPGRADE_VERSION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const WS2016_TP5_UPGRADE_VERSION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLCTL_CODES(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_UNKNOWN: CLCTL_CODES = CLCTL_CODES(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_CHARACTERISTICS: CLCTL_CODES = CLCTL_CODES(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_FLAGS: CLCTL_CODES = CLCTL_CODES(9i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_CLASS_INFO: CLCTL_CODES = CLCTL_CODES(13i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_REQUIRED_DEPENDENCIES: CLCTL_CODES = CLCTL_CODES(17i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_ARB_TIMEOUT: CLCTL_CODES = CLCTL_CODES(21i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_FAILURE_INFO: CLCTL_CODES = CLCTL_CODES(25i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_NAME: CLCTL_CODES = CLCTL_CODES(41i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_RESOURCE_TYPE: CLCTL_CODES = CLCTL_CODES(45i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_NODE: CLCTL_CODES = CLCTL_CODES(49i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_NETWORK: CLCTL_CODES = CLCTL_CODES(53i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_ID: CLCTL_CODES = CLCTL_CODES(57i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_FQDN: CLCTL_CODES = CLCTL_CODES(61i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_CLUSTER_SERVICE_ACCOUNT_NAME: CLCTL_CODES = CLCTL_CODES(65i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CHECK_VOTER_EVICT: CLCTL_CODES = CLCTL_CODES(69i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CHECK_VOTER_DOWN: CLCTL_CODES = CLCTL_CODES(73i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SHUTDOWN: CLCTL_CODES = CLCTL_CODES(77i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ENUM_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(81i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_RO_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(85i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(89i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(4194398i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(97i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_COMMON_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(101i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_COMMON_RESOURCE_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(105i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ENUM_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(121i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_RO_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(125i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(129i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(4194438i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(137i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_PRIVATE_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(141i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(145i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_REGISTRY_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194466i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_DELETE_REGISTRY_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194470i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_REGISTRY_CHECKPOINTS: CLCTL_CODES = CLCTL_CODES(169i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_CRYPTO_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194478i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_DELETE_CRYPTO_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194482i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_CRYPTO_CHECKPOINTS: CLCTL_CODES = CLCTL_CODES(181i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RESOURCE_UPGRADE_DLL: CLCTL_CODES = CLCTL_CODES(4194490i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_64BIT: CLCTL_CODES = CLCTL_CODES(4194494i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_32BIT: CLCTL_CODES = CLCTL_CODES(4194498i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_LOADBAL_PROCESS_LIST: CLCTL_CODES = CLCTL_CODES(201i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_ACCOUNT_ACCESS: CLCTL_CODES = CLCTL_CODES(4194546i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_NETWORK_NAME: CLCTL_CODES = CLCTL_CODES(361i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN: CLCTL_CODES = CLCTL_CODES(365i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_REGISTER_DNS_RECORDS: CLCTL_CODES = CLCTL_CODES(370i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_DNS_NAME: CLCTL_CODES = CLCTL_CODES(373i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_SET_PWD_INFO: CLCTL_CODES = CLCTL_CODES(378i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_DELETE_CO: CLCTL_CODES = CLCTL_CODES(382i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_VALIDATE_VCO: CLCTL_CODES = CLCTL_CODES(385i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_RESET_VCO: CLCTL_CODES = CLCTL_CODES(389i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_REPAIR_VCO: CLCTL_CODES = CLCTL_CODES(397i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DISK_INFO: CLCTL_CODES = CLCTL_CODES(401i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS: CLCTL_CODES = CLCTL_CODES(405i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_IS_PATH_VALID: CLCTL_CODES = CLCTL_CODES(409i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_SYNC_CLUSDISK_DB: CLCTL_CODES = CLCTL_CODES(4194718i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DISK_NUMBER_INFO: CLCTL_CODES = CLCTL_CODES(417i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_QUERY_DELETE: CLCTL_CODES = CLCTL_CODES(441i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_IPADDRESS_RENEW_LEASE: CLCTL_CODES = CLCTL_CODES(4194750i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_IPADDRESS_RELEASE_LEASE: CLCTL_CODES = CLCTL_CODES(4194754i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_QUERY_MAINTENANCE_MODE: CLCTL_CODES = CLCTL_CODES(481i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_MAINTENANCE_MODE: CLCTL_CODES = CLCTL_CODES(4194790i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_SET_DRIVELETTER: CLCTL_CODES = CLCTL_CODES(4194794i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DRIVELETTERS: CLCTL_CODES = CLCTL_CODES(493i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DISK_INFO_EX: CLCTL_CODES = CLCTL_CODES(497i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX: CLCTL_CODES = CLCTL_CODES(501i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DISK_INFO_EX2: CLCTL_CODES = CLCTL_CODES(505i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_CLUSPORT_DISK_COUNT: CLCTL_CODES = CLCTL_CODES(509i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_REMAP_DRIVELETTER: CLCTL_CODES = CLCTL_CODES(513i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DISKID: CLCTL_CODES = CLCTL_CODES(517i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_IS_CLUSTERABLE: CLCTL_CODES = CLCTL_CODES(521i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_REMOVE_VM_OWNERSHIP: CLCTL_CODES = CLCTL_CODES(4194830i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_MOUNTPOINTS: CLCTL_CODES = CLCTL_CODES(529i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DIRTY: CLCTL_CODES = CLCTL_CODES(537i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_INFO: CLCTL_CODES = CLCTL_CODES(549i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_IS_CSV_FILE: CLCTL_CODES = CLCTL_CODES(553i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_RESOURCEID: CLCTL_CODES = CLCTL_CODES(557i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_PATH: CLCTL_CODES = CLCTL_CODES(561i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_NETNAME: CLCTL_CODES = CLCTL_CODES(565i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_DIRECTORY: CLCTL_CODES = CLCTL_CODES(569i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_BATCH_BLOCK_KEY: CLCTL_CODES = CLCTL_CODES(574i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_BATCH_UNBLOCK_KEY: CLCTL_CODES = CLCTL_CODES(577i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FILESERVER_SHARE_ADD: CLCTL_CODES = CLCTL_CODES(4194886i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FILESERVER_SHARE_DEL: CLCTL_CODES = CLCTL_CODES(4194890i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FILESERVER_SHARE_MODIFY: CLCTL_CODES = CLCTL_CODES(4194894i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FILESERVER_SHARE_REPORT: CLCTL_CODES = CLCTL_CODES(593i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_GET_OU_FOR_VCO: CLCTL_CODES = CLCTL_CODES(4194926i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ENABLE_SHARED_VOLUME_DIRECTIO: CLCTL_CODES = CLCTL_CODES(4194954i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_DISABLE_SHARED_VOLUME_DIRECTIO: CLCTL_CODES = CLCTL_CODES(4194958i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_SHARED_VOLUME_ID: CLCTL_CODES = CLCTL_CODES(657i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_CSV_MAINTENANCE_MODE: CLCTL_CODES = CLCTL_CODES(4194966i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_SHARED_VOLUME_BACKUP_MODE: CLCTL_CODES = CLCTL_CODES(4194970i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: CLCTL_CODES = CLCTL_CODES(669i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_STATES: CLCTL_CODES = CLCTL_CODES(4194978i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_IS_SHARED_VOLUME: CLCTL_CODES = CLCTL_CODES(677i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_CLUSDB_TIMESTAMP: CLCTL_CODES = CLCTL_CODES(681i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RW_MODIFY_NOOP: CLCTL_CODES = CLCTL_CODES(4194990i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_IS_QUORUM_BLOCKED: CLCTL_CODES = CLCTL_CODES(689i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_POOL_GET_DRIVE_INFO: CLCTL_CODES = CLCTL_CODES(693i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_GUM_LOCK_OWNER: CLCTL_CODES = CLCTL_CODES(697i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_STUCK_NODES: CLCTL_CODES = CLCTL_CODES(701i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_INJECT_GEM_FAULT: CLCTL_CODES = CLCTL_CODES(705i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_INTRODUCE_GEM_REPAIR_DELAY: CLCTL_CODES = CLCTL_CODES(709i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SEND_DUMMY_GEM_MESSAGES: CLCTL_CODES = CLCTL_CODES(713i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_BLOCK_GEM_SEND_RECV: CLCTL_CODES = CLCTL_CODES(717i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_GEMID_VECTOR: CLCTL_CODES = CLCTL_CODES(721i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_CRYPTO_CHECKPOINT_EX: CLCTL_CODES = CLCTL_CODES(4195030i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUP_GET_LAST_MOVE_TIME: CLCTL_CODES = CLCTL_CODES(729i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_STORAGE_CONFIGURATION: CLCTL_CODES = CLCTL_CODES(4195042i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_STORAGE_CONFIGURATION: CLCTL_CODES = CLCTL_CODES(741i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_STORAGE_CONFIG_ATTRIBUTES: CLCTL_CODES = CLCTL_CODES(745i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REMOVE_NODE: CLCTL_CODES = CLCTL_CODES(4195054i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_IS_FEATURE_INSTALLED: CLCTL_CODES = CLCTL_CODES(753i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_IS_S2D_FEATURE_SUPPORTED: CLCTL_CODES = CLCTL_CODES(757i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_PHYSICAL_DISK_INFO: CLCTL_CODES = CLCTL_CODES(761i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHS: CLCTL_CODES = CLCTL_CODES(765i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHINFO: CLCTL_CODES = CLCTL_CODES(769i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLEAR_NODE_CONNECTION_INFO: CLCTL_CODES = CLCTL_CODES(4195078i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_DNS_DOMAIN: CLCTL_CODES = CLCTL_CODES(4195082i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CTCTL_GET_ROUTESTATUS_BASIC: CLCTL_CODES = CLCTL_CODES(781i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CTCTL_GET_ROUTESTATUS_EXTENDED: CLCTL_CODES = CLCTL_CODES(785i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CTCTL_GET_FAULT_DOMAIN_STATE: CLCTL_CODES = CLCTL_CODES(789i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_SET_PWD_INFOEX: CLCTL_CODES = CLCTL_CODES(794i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: CLCTL_CODES = CLCTL_CODES(8161i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: CLCTL_CODES = CLCTL_CODES(8417i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: CLCTL_CODES = CLCTL_CODES(4202726i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RESOURCE_PREPARE_UPGRADE: CLCTL_CODES = CLCTL_CODES(4202730i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RESOURCE_UPGRADE_COMPLETED: CLCTL_CODES = CLCTL_CODES(4202734i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: CLCTL_CODES = CLCTL_CODES(8433i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: CLCTL_CODES = CLCTL_CODES(4202742i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_ADD_REPLICATION_GROUP: CLCTL_CODES = CLCTL_CODES(8514i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_LOG_INFO: CLCTL_CODES = CLCTL_CODES(8517i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_ELIGIBLE_LOGDISKS: CLCTL_CODES = CLCTL_CODES(8521i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: CLCTL_CODES = CLCTL_CODES(8525i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: CLCTL_CODES = CLCTL_CODES(8529i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_REPLICATED_DISKS: CLCTL_CODES = CLCTL_CODES(8533i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_REPLICA_VOLUMES: CLCTL_CODES = CLCTL_CODES(8537i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_LOG_VOLUME: CLCTL_CODES = CLCTL_CODES(8541i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_RESOURCE_GROUP: CLCTL_CODES = CLCTL_CODES(8545i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_REPLICATED_PARTITION_INFO: CLCTL_CODES = CLCTL_CODES(8549i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_STATE_CHANGE_TIME: CLCTL_CODES = CLCTL_CODES(11613i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_CLUSTER_S2D_ENABLED: CLCTL_CODES = CLCTL_CODES(4205922i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: CLCTL_CODES = CLCTL_CODES(4205934i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUPSET_GET_GROUPS: CLCTL_CODES = CLCTL_CODES(11633i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPS: CLCTL_CODES = CLCTL_CODES(11637i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPSETS: CLCTL_CODES = CLCTL_CODES(11641i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUP_GET_PROVIDER_GROUPS: CLCTL_CODES = CLCTL_CODES(11645i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUP_GET_PROVIDER_GROUPSETS: CLCTL_CODES = CLCTL_CODES(11649i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUP_SET_CCF_FROM_MASTER: CLCTL_CODES = CLCTL_CODES(4205958i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_INFRASTRUCTURE_SOFS_BUFFER: CLCTL_CODES = CLCTL_CODES(11657i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_INFRASTRUCTURE_SOFS_BUFFER: CLCTL_CODES = CLCTL_CODES(4205966i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NOTIFY_INFRASTRUCTURE_SOFS_CHANGED: CLCTL_CODES = CLCTL_CODES(4205970i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SCALEOUT_COMMAND: CLCTL_CODES = CLCTL_CODES(4205974i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SCALEOUT_CONTROL: CLCTL_CODES = CLCTL_CODES(4205978i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SCALEOUT_GET_CLUSTERS: CLCTL_CODES = CLCTL_CODES(4205981i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RELOAD_AUTOLOGGER_CONFIG: CLCTL_CODES = CLCTL_CODES(11730i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME: CLCTL_CODES = CLCTL_CODES(11734i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID: CLCTL_CODES = CLCTL_CODES(11738i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ENUM_AFFINITY_RULE_NAMES: CLCTL_CODES = CLCTL_CODES(11741i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_NODES_IN_FD: CLCTL_CODES = CLCTL_CODES(11745i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FORCE_DB_FLUSH: CLCTL_CODES = CLCTL_CODES(4206054i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_DELETE: CLCTL_CODES = CLCTL_CODES(5242886i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_INSTALL_NODE: CLCTL_CODES = CLCTL_CODES(5242890i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_EVICT_NODE: CLCTL_CODES = CLCTL_CODES(5242894i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_DEPENDENCY: CLCTL_CODES = CLCTL_CODES(5242898i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REMOVE_DEPENDENCY: CLCTL_CODES = CLCTL_CODES(5242902i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_OWNER: CLCTL_CODES = CLCTL_CODES(5242906i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REMOVE_OWNER: CLCTL_CODES = CLCTL_CODES(5242910i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_NAME: CLCTL_CODES = CLCTL_CODES(5242918i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLUSTER_NAME_CHANGED: CLCTL_CODES = CLCTL_CODES(5242922i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLUSTER_VERSION_CHANGED: CLCTL_CODES = CLCTL_CODES(5242926i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FIXUP_ON_UPGRADE: CLCTL_CODES = CLCTL_CODES(5242930i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STARTING_PHASE1: CLCTL_CODES = CLCTL_CODES(5242934i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STARTING_PHASE2: CLCTL_CODES = CLCTL_CODES(5242938i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_HOLD_IO: CLCTL_CODES = CLCTL_CODES(5242942i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RESUME_IO: CLCTL_CODES = CLCTL_CODES(5242946i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FORCE_QUORUM: CLCTL_CODES = CLCTL_CODES(5242950i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_INITIALIZE: CLCTL_CODES = CLCTL_CODES(5242954i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STATE_CHANGE_REASON: CLCTL_CODES = CLCTL_CODES(5242958i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_PROVIDER_STATE_CHANGE: CLCTL_CODES = CLCTL_CODES(5242962i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_LEAVING_GROUP: CLCTL_CODES = CLCTL_CODES(5242966i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_JOINING_GROUP: CLCTL_CODES = CLCTL_CODES(5242970i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FSWITNESS_GET_EPOCH_INFO: CLCTL_CODES = CLCTL_CODES(1048669i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FSWITNESS_SET_EPOCH_INFO: CLCTL_CODES = CLCTL_CODES(5242978i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FSWITNESS_RELEASE_LOCK: CLCTL_CODES = CLCTL_CODES(5242982i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_CREDS_NOTIFYCAM: CLCTL_CODES = CLCTL_CODES(5242986i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NOTIFY_QUORUM_STATUS: CLCTL_CODES = CLCTL_CODES(5243006i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NOTIFY_MONITOR_SHUTTING_DOWN: CLCTL_CODES = CLCTL_CODES(1048705i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_UNDELETE: CLCTL_CODES = CLCTL_CODES(5243014i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_OPERATION_CONTEXT: CLCTL_CODES = CLCTL_CODES(1057001i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NOTIFY_OWNER_CHANGE: CLCTL_CODES = CLCTL_CODES(5251362i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_CHANGE_GROUP: CLCTL_CODES = CLCTL_CODES(1057061i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CHECK_DRAIN_VETO: CLCTL_CODES = CLCTL_CODES(1057069i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NOTIFY_DRAIN_COMPLETE: CLCTL_CODES = CLCTL_CODES(1057073i32);
impl ::core::marker::Copy for CLCTL_CODES {}
impl ::core::clone::Clone for CLCTL_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLCTL_CODES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUADMEX_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_NONE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_CLUSTER: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_NODE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_GROUP: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_RESOURCE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_RESOURCETYPE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_NETWORK: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_NETINTERFACE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(7i32);
impl ::core::marker::Copy for CLUADMEX_OBJECT_TYPE {}
impl ::core::clone::Clone for CLUADMEX_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUADMEX_OBJECT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSCTL_AFFINITYRULE_CODES(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_AFFINITYRULE_GET_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(150995033i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_AFFINITYRULE_GET_RO_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(150995029i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_AFFINITYRULE_SET_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(155189342i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_AFFINITYRULE_GET_ID: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(150995001i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_AFFINITYRULE_GET_GROUPNAMES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(151006577i32);
impl ::core::marker::Copy for CLUSCTL_AFFINITYRULE_CODES {}
impl ::core::clone::Clone for CLUSCTL_AFFINITYRULE_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSCTL_AFFINITYRULE_CODES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSCTL_CLUSTER_CODES(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_UNKNOWN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440512i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_FQDN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440573i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_STORAGE_CONFIGURATION: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635554i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIGURATION: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441253i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIG_ATTRIBUTES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441257i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_ENUM_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440593i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_RO_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440597i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440601i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121634910i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_VALIDATE_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440609i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_ENUM_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440633i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440637i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440641i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121634950i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440649i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTY_FMTS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440613i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440653i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_CHECK_VOTER_EVICT: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440581i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_CHECK_VOTER_DOWN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440585i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SHUTDOWN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440589i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_BATCH_BLOCK_KEY: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441086i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_BATCH_UNBLOCK_KEY: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441089i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_SHARED_VOLUME_ID: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441169i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_CLUSDB_TIMESTAMP: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441193i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_GUM_LOCK_OWNER: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441209i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_REMOVE_NODE: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635566i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_ACCOUNT_ACCESS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635058i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_CLEAR_NODE_CONNECTION_INFO: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635590i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_DNS_DOMAIN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635594i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_ENABLED: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121646434i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121646446i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452246i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME_GUID: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452250i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_RELOAD_AUTOLOGGER_CONFIG: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452242i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_ENUM_AFFINITY_RULE_NAMES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452253i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_NODES_IN_FD: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452257i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_FORCE_FLUSH_DB: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121646566i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_CLMUSR_TOKEN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440877i32);
impl ::core::marker::Copy for CLUSCTL_CLUSTER_CODES {}
impl ::core::clone::Clone for CLUSCTL_CLUSTER_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSCTL_CLUSTER_CODES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSCTL_GROUPSET_CODES(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134217817i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_RO_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134217813i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_SET_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(138412126i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_GROUPS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229361i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229365i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPSETS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229369i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229373i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPSETS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229377i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_ID: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134217785i32);
impl ::core::marker::Copy for CLUSCTL_GROUPSET_CODES {}
impl ::core::clone::Clone for CLUSCTL_GROUPSET_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSCTL_GROUPSET_CODES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSCTL_GROUP_CODES(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_UNKNOWN: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331648i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_CHARACTERISTICS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331653i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_FLAGS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331657i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_NAME: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331689i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_ID: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331705i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_ENUM_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331729i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_RO_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331733i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331737i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_SET_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(54526046i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_VALIDATE_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331745i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_ENUM_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331769i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331773i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331777i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_SET_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(54526086i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331785i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_QUERY_DELETE: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50332089i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTY_FMTS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331749i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331789i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_FAILURE_INFO: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331673i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_LAST_MOVE_TIME: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50332377i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_SET_CCF_FROM_MASTER: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(54537606i32);
impl ::core::marker::Copy for CLUSCTL_GROUP_CODES {}
impl ::core::clone::Clone for CLUSCTL_GROUP_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSCTL_GROUP_CODES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSCTL_NETINTERFACE_CODES(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_UNKNOWN: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663296i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_CHARACTERISTICS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663301i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_FLAGS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663305i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_NAME: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663337i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_ID: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663353i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_NODE: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663345i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_NETWORK: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663349i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_ENUM_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663377i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_RO_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663381i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663385i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_SET_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(104857694i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663393i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663417i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663421i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663425i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_SET_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(104857734i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663433i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663397i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663437i32);
impl ::core::marker::Copy for CLUSCTL_NETINTERFACE_CODES {}
impl ::core::clone::Clone for CLUSCTL_NETINTERFACE_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSCTL_NETINTERFACE_CODES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSCTL_NETWORK_CODES(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_UNKNOWN: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886080i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_CHARACTERISTICS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886085i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_FLAGS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886089i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_NAME: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886121i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_ID: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886137i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_ENUM_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886161i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_RO_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886165i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886169i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_SET_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(88080478i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886177i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886201i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886205i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886209i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_SET_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(88080518i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886217i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886181i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886221i32);
impl ::core::marker::Copy for CLUSCTL_NETWORK_CODES {}
impl ::core::clone::Clone for CLUSCTL_NETWORK_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSCTL_NETWORK_CODES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSCTL_NODE_CODES(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_UNKNOWN: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108864i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_CHARACTERISTICS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108869i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_FLAGS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108873i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_NAME: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108905i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_ID: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108921i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_ENUM_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108945i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_RO_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108949i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108953i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_SET_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(71303262i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108961i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108985i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108989i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108993i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_SET_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(71303302i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109001i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108965i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109005i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_CLUSTER_SERVICE_ACCOUNT_NAME: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108929i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_STUCK_NODES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109565i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_INJECT_GEM_FAULT: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109569i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_INTRODUCE_GEM_REPAIR_DELAY: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109573i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_SEND_DUMMY_GEM_MESSAGES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109577i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_BLOCK_GEM_SEND_RECV: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109581i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_GEMID_VECTOR: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109585i32);
impl ::core::marker::Copy for CLUSCTL_NODE_CODES {}
impl ::core::clone::Clone for CLUSCTL_NODE_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSCTL_NODE_CODES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSCTL_RESOURCE_CODES(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_UNKNOWN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777216i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_CHARACTERISTICS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777221i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_FLAGS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777225i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_CLASS_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777229i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_REQUIRED_DEPENDENCIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777233i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777257i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_ID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777273i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_RESOURCE_TYPE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777261i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ENUM_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777297i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_RO_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777301i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777305i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971614i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777313i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777317i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777337i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777341i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777345i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971654i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777353i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777357i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971682i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_DELETE_REGISTRY_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971686i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_REGISTRY_CHECKPOINTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777385i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971694i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_DELETE_CRYPTO_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971698i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT_EX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972246i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_CRYPTO_CHECKPOINTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777397i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_LOADBAL_PROCESS_LIST: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777417i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_NETWORK_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777577i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_GET_VIRTUAL_SERVER_TOKEN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777581i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777594i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFOEX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16778010i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_DELETE_CO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777598i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_VALIDATE_VCO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777601i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_RESET_VCO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777605i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_REPAIR_VCO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777613i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_REGISTER_DNS_RECORDS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777586i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_DNS_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777589i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777617i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_NUMBER_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777633i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_IS_PATH_VALID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777625i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_QUERY_DELETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777657i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_UPGRADE_DLL: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971706i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_IPADDRESS_RENEW_LEASE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971966i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_IPADDRESS_RELEASE_LEASE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971970i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_64BIT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971710i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_32BIT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971714i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_QUERY_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777697i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972006i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_SET_DRIVELETTER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972010i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777713i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX2: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777721i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_MOUNTPOINTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777745i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DIRTY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777753i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777765i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_CSV_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972182i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ENABLE_SHARED_VOLUME_DIRECTIO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972170i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_DISABLE_SHARED_VOLUME_DIRECTIO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972174i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_SHARED_VOLUME_BACKUP_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972186i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777885i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_FAILURE_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777241i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISKID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777733i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_STATES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972194i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_IS_SHARED_VOLUME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777893i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_IS_QUORUM_BLOCKED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777905i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_POOL_GET_DRIVE_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777909i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_RLUA_GET_VIRTUAL_SERVER_TOKEN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777581i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777594i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFOEX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16778010i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_DELETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020102i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_UNDELETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020230i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_INSTALL_NODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020106i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_EVICT_NODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020110i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_DEPENDENCY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020114i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_REMOVE_DEPENDENCY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020118i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_OWNER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020122i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_REMOVE_OWNER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020126i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020134i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_CLUSTER_NAME_CHANGED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020138i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_CLUSTER_VERSION_CHANGED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020142i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_FORCE_QUORUM: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020166i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_INITIALIZE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020170i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020174i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_PROVIDER_STATE_CHANGE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020178i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_LEAVING_GROUP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020182i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_JOINING_GROUP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020186i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_FSWITNESS_GET_EPOCH_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17825885i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_FSWITNESS_SET_EPOCH_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020194i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_FSWITNESS_RELEASE_LOCK: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020198i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_CREDS_NOTIFYCAM: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020202i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_OPERATION_CONTEXT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834217i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_RW_MODIFY_NOOP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972206i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NOTIFY_QUORUM_STATUS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020222i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NOTIFY_OWNER_CHANGE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22028578i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_VALIDATE_CHANGE_GROUP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834277i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788950i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME_GUID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788954i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979942i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979958i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_PREPARE_UPGRADE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979946i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_UPGRADE_COMPLETED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979950i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_STATE_CHANGE_TIME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788829i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_INFRASTRUCTURE_SOFS_BUFFER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788873i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_INFRASTRUCTURE_SOFS_BUFFER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983182i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SCALEOUT_COMMAND: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983190i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SCALEOUT_CONTROL: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983194i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SCALEOUT_GET_CLUSTERS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983197i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_CHECK_DRAIN_VETO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834285i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NOTIFY_DRAIN_COMPLETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834289i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_NODES_IN_FD: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788961i32);
impl ::core::marker::Copy for CLUSCTL_RESOURCE_CODES {}
impl ::core::clone::Clone for CLUSCTL_RESOURCE_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSCTL_RESOURCE_CODES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSCTL_RESOURCE_TYPE_CODES(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_UNKNOWN: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554432i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_CHARACTERISTICS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554437i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_FLAGS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554441i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_CLASS_INFO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554445i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_REQUIRED_DEPENDENCIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554449i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_ARB_TIMEOUT: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554453i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_ENUM_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554513i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554517i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554521i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554529i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_SET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37748830i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554533i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_RESOURCE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554537i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554553i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554557i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554561i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_SET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37748870i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554569i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554573i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554577i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_REGISTRY_CHECKPOINTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554601i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_CRYPTO_CHECKPOINTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554613i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554837i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_SYNC_CLUSDISK_DB: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37749150i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_VALIDATE_NETNAME: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554997i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_GET_OU_FOR_VCO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37749358i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554993i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_DIRECTORY: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33555001i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GEN_SCRIPT_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554993i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_QUERY_DELETE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554873i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DRIVELETTERS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554925i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554933i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMAP_DRIVELETTER: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554945i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DISKID: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554949i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_RESOURCEID: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554989i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CLUSTERABLE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554953i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMOVE_VM_OWNERSHIP: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37749262i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CSV_FILE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(16777769i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_WITNESS_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554993i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_INSTALL_NODE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797322i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_EVICT_NODE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797326i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_CLUSTER_VERSION_CHANGED: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797358i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_FIXUP_ON_UPGRADE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797362i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE1: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797366i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE2: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797370i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_HOLD_IO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797374i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_RESUME_IO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797378i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562593i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_LOGDISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562953i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562957i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562961i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_DISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562965i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICA_VOLUMES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562969i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_VOLUME: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562973i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_RESOURCE_GROUP: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562977i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_PARTITION_INFO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562981i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_INFO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562949i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_ADD_REPLICATION_GROUP: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562946i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562849i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562865i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_PREPARE_UPGRADE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37757162i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_UPGRADE_COMPLETED: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37757166i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_MONITOR_SHUTTING_DOWN: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(34603137i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_CHECK_DRAIN_VETO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(34611501i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_DRAIN_COMPLETE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(34611505i32);
impl ::core::marker::Copy for CLUSCTL_RESOURCE_TYPE_CODES {}
impl ::core::clone::Clone for CLUSCTL_RESOURCE_TYPE_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSCTL_RESOURCE_TYPE_CODES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSGROUP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeCoreCluster: CLUSGROUP_TYPE = CLUSGROUP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeAvailableStorage: CLUSGROUP_TYPE = CLUSGROUP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeTemporary: CLUSGROUP_TYPE = CLUSGROUP_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeSharedVolume: CLUSGROUP_TYPE = CLUSGROUP_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeStoragePool: CLUSGROUP_TYPE = CLUSGROUP_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeFileServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(100i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypePrintServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(101i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeDhcpServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(102i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeDtc: CLUSGROUP_TYPE = CLUSGROUP_TYPE(103i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeMsmq: CLUSGROUP_TYPE = CLUSGROUP_TYPE(104i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeWins: CLUSGROUP_TYPE = CLUSGROUP_TYPE(105i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeStandAloneDfs: CLUSGROUP_TYPE = CLUSGROUP_TYPE(106i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeGenericApplication: CLUSGROUP_TYPE = CLUSGROUP_TYPE(107i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeGenericService: CLUSGROUP_TYPE = CLUSGROUP_TYPE(108i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeGenericScript: CLUSGROUP_TYPE = CLUSGROUP_TYPE(109i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeIScsiNameService: CLUSGROUP_TYPE = CLUSGROUP_TYPE(110i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeVirtualMachine: CLUSGROUP_TYPE = CLUSGROUP_TYPE(111i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeTsSessionBroker: CLUSGROUP_TYPE = CLUSGROUP_TYPE(112i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeIScsiTarget: CLUSGROUP_TYPE = CLUSGROUP_TYPE(113i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeScaleoutFileServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(114i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeVMReplicaBroker: CLUSGROUP_TYPE = CLUSGROUP_TYPE(115i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeTaskScheduler: CLUSGROUP_TYPE = CLUSGROUP_TYPE(116i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeClusterUpdateAgent: CLUSGROUP_TYPE = CLUSGROUP_TYPE(117i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeScaleoutCluster: CLUSGROUP_TYPE = CLUSGROUP_TYPE(118i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeStorageReplica: CLUSGROUP_TYPE = CLUSGROUP_TYPE(119i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeVMReplicaCoordinator: CLUSGROUP_TYPE = CLUSGROUP_TYPE(120i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeCrossClusterOrchestrator: CLUSGROUP_TYPE = CLUSGROUP_TYPE(121i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeInfrastructureFileServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(122i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeCoreSddc: CLUSGROUP_TYPE = CLUSGROUP_TYPE(123i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeUnknown: CLUSGROUP_TYPE = CLUSGROUP_TYPE(9999i32);
impl ::core::marker::Copy for CLUSGROUP_TYPE {}
impl ::core::clone::Clone for CLUSGROUP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSGROUP_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSPROP_IPADDR_ENABLENETBIOS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_IPADDR_ENABLENETBIOS_DISABLED: CLUSPROP_IPADDR_ENABLENETBIOS = CLUSPROP_IPADDR_ENABLENETBIOS(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_IPADDR_ENABLENETBIOS_ENABLED: CLUSPROP_IPADDR_ENABLENETBIOS = CLUSPROP_IPADDR_ENABLENETBIOS(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_IPADDR_ENABLENETBIOS_TRACK_NIC: CLUSPROP_IPADDR_ENABLENETBIOS = CLUSPROP_IPADDR_ENABLENETBIOS(2i32);
impl ::core::marker::Copy for CLUSPROP_IPADDR_ENABLENETBIOS {}
impl ::core::clone::Clone for CLUSPROP_IPADDR_ENABLENETBIOS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_IPADDR_ENABLENETBIOS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSPROP_PIFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_STICKY: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_REMOVABLE: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_USABLE: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_DEFAULT_QUORUM: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_USABLE_FOR_CSV: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_ENCRYPTION_ENABLED: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_RAW: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(64i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_UNKNOWN: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(-2147483648i32);
impl ::core::marker::Copy for CLUSPROP_PIFLAGS {}
impl ::core::clone::Clone for CLUSPROP_PIFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_PIFLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTERSET_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTERSET_OBJECT_TYPE_NONE: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTERSET_OBJECT_TYPE_MEMBER: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTERSET_OBJECT_TYPE_WORKLOAD: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTERSET_OBJECT_TYPE_DATABASE: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(3i32);
impl ::core::marker::Copy for CLUSTERSET_OBJECT_TYPE {}
impl ::core::clone::Clone for CLUSTERSET_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTERSET_OBJECT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_NAME: CLUSTER_CHANGE = CLUSTER_CHANGE(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES: CLUSTER_CHANGE = CLUSTER_CHANGE(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_VALUE: CLUSTER_CHANGE = CLUSTER_CHANGE(64i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE: CLUSTER_CHANGE = CLUSTER_CHANGE(128i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(256i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(512i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(1024i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(2048i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(4096i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(8192i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(16384i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(32768i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(65536i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(131072i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(262144i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT: CLUSTER_CHANGE = CLUSTER_CHANGE(524288i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(1048576i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(2097152i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(4194304i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(8388608i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(16777216i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(33554432i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(67108864i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(134217728i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_QUORUM_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(268435456i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(536870912i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(1073741824i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_HANDLE_CLOSE: CLUSTER_CHANGE = CLUSTER_CHANGE(-2147483648i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_ALL: CLUSTER_CHANGE = CLUSTER_CHANGE(-1i32);
impl ::core::marker::Copy for CLUSTER_CHANGE {}
impl ::core::clone::Clone for CLUSTER_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_CLUSTER_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_STATE_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_GROUP_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_HANDLE_CLOSE_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_NETWORK_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_NODE_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_RESOURCE_TYPE_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(64i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_COMMON_PROPERTY_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(128i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(256i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_LOST_NOTIFICATIONS_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(512i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_RENAME_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(1024i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_MEMBERSHIP_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(2048i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_UPGRADED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(4096i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_ALL_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(8191i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_CLUSTER_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_CLUSTER_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_CLUSTER_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_GROUPSET_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_DELETED_v2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_COMMON_PROPERTY_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_STATE_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_GROUP_ADDED: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_GROUP_REMOVED: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENCIES_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(64i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENTS_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(128i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_HANDLE_CLOSE_v2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(256i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_ALL_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(511i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_GROUPSET_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_GROUPSET_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_GROUPSET_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_GROUP_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_DELETED_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_COMMON_PROPERTY_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_STATE_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_OWNER_NODE_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_PREFERRED_OWNERS_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_RESOURCE_ADDED_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(64i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_RESOURCE_GAINED_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(128i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_RESOURCE_LOST_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(256i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_HANDLE_CLOSE_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(512i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_ALL_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(1023i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_GROUP_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_GROUP_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_GROUP_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_NETINTERFACE_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_STATE_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_ALL_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(31i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_NETINTERFACE_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_NETINTERFACE_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_NETINTERFACE_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_NETWORK_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_DELETED_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_STATE_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_ALL_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(31i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_NETWORK_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_NETWORK_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_NETWORK_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_UPGRADE_NODE_PREPARE: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_UPGRADE_NODE_COMMIT: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_UPGRADE_NODE_POSTCOMMIT: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_UPGRADE_ALL: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(7i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_NODE_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_NETINTERFACE_ADDED_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_DELETED_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_STATE_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_GROUP_GAINED_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_GROUP_LOST_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(64i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(128i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_ALL_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(255i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_NODE_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_NODE_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_NODE_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_QUORUM_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_QUORUM_STATE_V2: CLUSTER_CHANGE_QUORUM_V2 = CLUSTER_CHANGE_QUORUM_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_QUORUM_ALL_V2: CLUSTER_CHANGE_QUORUM_V2 = CLUSTER_CHANGE_QUORUM_V2(1i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_QUORUM_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_QUORUM_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_QUORUM_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_REGISTRY_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_NAME_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_VALUE_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_HANDLE_CLOSE_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_ALL_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(31i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_REGISTRY_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_REGISTRY_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_REGISTRY_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_RESOURCE_TYPE_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_POSSIBLE_OWNERS_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DLL_UPGRADED_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_TYPE_SPECIFIC_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ALL_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(63i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_RESOURCE_TYPE_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_RESOURCE_TYPE_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_RESOURCE_TYPE_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_RESOURCE_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_STATE_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_OWNER_GROUP_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENCIES_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENTS_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_POSSIBLE_OWNERS_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(64i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_DELETED_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(128i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_DLL_UPGRADED_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(256i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(512i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TERMINAL_STATE_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(1024i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_ALL_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(2047i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_RESOURCE_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_RESOURCE_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_RESOURCE_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_SHARED_VOLUME_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_SHARED_VOLUME_STATE_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_SHARED_VOLUME_ADDED_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_SHARED_VOLUME_REMOVED_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_SHARED_VOLUME_ALL_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(7i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_SHARED_VOLUME_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_SHARED_VOLUME_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_SHARED_VOLUME_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CHANGE_SPACEPORT_V2(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_SPACEPORT_CUSTOM_PNP_V2: CLUSTER_CHANGE_SPACEPORT_V2 = CLUSTER_CHANGE_SPACEPORT_V2(1i32);
impl ::core::marker::Copy for CLUSTER_CHANGE_SPACEPORT_V2 {}
impl ::core::clone::Clone for CLUSTER_CHANGE_SPACEPORT_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CHANGE_SPACEPORT_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CLOUD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CLOUD_TYPE_NONE: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CLOUD_TYPE_AZURE: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CLOUD_TYPE_MIXED: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(128i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CLOUD_TYPE_UNKNOWN: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(-1i32);
impl ::core::marker::Copy for CLUSTER_CLOUD_TYPE {}
impl ::core::clone::Clone for CLUSTER_CLOUD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CLOUD_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CONTROL_OBJECT(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_INVALID: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_RESOURCE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_RESOURCE_TYPE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_GROUP: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_NODE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_NETWORK: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_NETINTERFACE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(6i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_CLUSTER: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(7i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_GROUPSET: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_AFFINITYRULE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(9i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_USER: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(128i32);
impl ::core::marker::Copy for CLUSTER_CONTROL_OBJECT {}
impl ::core::clone::Clone for CLUSTER_CONTROL_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CONTROL_OBJECT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_CSV_VOLUME_FAULT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeStateNoFaults: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeStateNoDirectIO: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeStateNoAccess: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeStateInMaintenance: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeStateDismounted: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(8i32);
impl ::core::marker::Copy for CLUSTER_CSV_VOLUME_FAULT_STATE {}
impl ::core::clone::Clone for CLUSTER_CSV_VOLUME_FAULT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CSV_VOLUME_FAULT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_NODE: CLUSTER_ENUM = CLUSTER_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_RESTYPE: CLUSTER_ENUM = CLUSTER_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_RESOURCE: CLUSTER_ENUM = CLUSTER_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_GROUP: CLUSTER_ENUM = CLUSTER_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_NETWORK: CLUSTER_ENUM = CLUSTER_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_NETINTERFACE: CLUSTER_ENUM = CLUSTER_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_SHARED_VOLUME_GROUP: CLUSTER_ENUM = CLUSTER_ENUM(536870912i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_SHARED_VOLUME_RESOURCE: CLUSTER_ENUM = CLUSTER_ENUM(1073741824i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_INTERNAL_NETWORK: CLUSTER_ENUM = CLUSTER_ENUM(-2147483648i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_ALL: CLUSTER_ENUM = CLUSTER_ENUM(63i32);
impl ::core::marker::Copy for CLUSTER_ENUM {}
impl ::core::clone::Clone for CLUSTER_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_GROUP_AUTOFAILBACK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupPreventFailback: CLUSTER_GROUP_AUTOFAILBACK_TYPE = CLUSTER_GROUP_AUTOFAILBACK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupAllowFailback: CLUSTER_GROUP_AUTOFAILBACK_TYPE = CLUSTER_GROUP_AUTOFAILBACK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupFailbackTypeCount: CLUSTER_GROUP_AUTOFAILBACK_TYPE = CLUSTER_GROUP_AUTOFAILBACK_TYPE(2i32);
impl ::core::marker::Copy for CLUSTER_GROUP_AUTOFAILBACK_TYPE {}
impl ::core::clone::Clone for CLUSTER_GROUP_AUTOFAILBACK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_GROUP_AUTOFAILBACK_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_GROUP_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_ENUM_CONTAINS: CLUSTER_GROUP_ENUM = CLUSTER_GROUP_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_ENUM_NODES: CLUSTER_GROUP_ENUM = CLUSTER_GROUP_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_ENUM_ALL: CLUSTER_GROUP_ENUM = CLUSTER_GROUP_ENUM(3i32);
impl ::core::marker::Copy for CLUSTER_GROUP_ENUM {}
impl ::core::clone::Clone for CLUSTER_GROUP_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_GROUP_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_GROUP_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PriorityDisabled: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PriorityLow: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(1000i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PriorityMedium: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(2000i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PriorityHigh: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(3000i32);
impl ::core::marker::Copy for CLUSTER_GROUP_PRIORITY {}
impl ::core::clone::Clone for CLUSTER_GROUP_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_GROUP_PRIORITY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_GROUP_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupStateUnknown: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(-1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupOnline: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupOffline: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupFailed: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupPartialOnline: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupPending: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(4i32);
impl ::core::marker::Copy for CLUSTER_GROUP_STATE {}
impl ::core::clone::Clone for CLUSTER_GROUP_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_GROUP_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_MGMT_POINT_RESTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_RESTYPE_AUTO: CLUSTER_MGMT_POINT_RESTYPE = CLUSTER_MGMT_POINT_RESTYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_RESTYPE_SNN: CLUSTER_MGMT_POINT_RESTYPE = CLUSTER_MGMT_POINT_RESTYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_RESTYPE_DNN: CLUSTER_MGMT_POINT_RESTYPE = CLUSTER_MGMT_POINT_RESTYPE(2i32);
impl ::core::marker::Copy for CLUSTER_MGMT_POINT_RESTYPE {}
impl ::core::clone::Clone for CLUSTER_MGMT_POINT_RESTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_MGMT_POINT_RESTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_MGMT_POINT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_TYPE_NONE: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_TYPE_CNO: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_TYPE_DNS_ONLY: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_TYPE_CNO_ONLY: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(3i32);
impl ::core::marker::Copy for CLUSTER_MGMT_POINT_TYPE {}
impl ::core::clone::Clone for CLUSTER_MGMT_POINT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_MGMT_POINT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_NETINTERFACE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetInterfaceStateUnknown: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(-1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetInterfaceUnavailable: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetInterfaceFailed: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetInterfaceUnreachable: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetInterfaceUp: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(3i32);
impl ::core::marker::Copy for CLUSTER_NETINTERFACE_STATE {}
impl ::core::clone::Clone for CLUSTER_NETINTERFACE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_NETINTERFACE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_NETWORK_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NETWORK_ENUM_NETINTERFACES: CLUSTER_NETWORK_ENUM = CLUSTER_NETWORK_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NETWORK_ENUM_ALL: CLUSTER_NETWORK_ENUM = CLUSTER_NETWORK_ENUM(1i32);
impl ::core::marker::Copy for CLUSTER_NETWORK_ENUM {}
impl ::core::clone::Clone for CLUSTER_NETWORK_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_NETWORK_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_NETWORK_ROLE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkRoleNone: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkRoleInternalUse: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkRoleClientAccess: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkRoleInternalAndClient: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(3i32);
impl ::core::marker::Copy for CLUSTER_NETWORK_ROLE {}
impl ::core::clone::Clone for CLUSTER_NETWORK_ROLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_NETWORK_ROLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_NETWORK_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkStateUnknown: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(-1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkUnavailable: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkDown: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkPartitioned: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkUp: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(3i32);
impl ::core::marker::Copy for CLUSTER_NETWORK_STATE {}
impl ::core::clone::Clone for CLUSTER_NETWORK_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_NETWORK_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_NODE_DRAIN_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeDrainStatusNotInitiated: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeDrainStatusInProgress: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeDrainStatusCompleted: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeDrainStatusFailed: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeDrainStatusCount: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(4i32);
impl ::core::marker::Copy for CLUSTER_NODE_DRAIN_STATUS {}
impl ::core::clone::Clone for CLUSTER_NODE_DRAIN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_NODE_DRAIN_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_NODE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NODE_ENUM_NETINTERFACES: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NODE_ENUM_GROUPS: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NODE_ENUM_PREFERRED_GROUPS: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NODE_ENUM_ALL: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(3i32);
impl ::core::marker::Copy for CLUSTER_NODE_ENUM {}
impl ::core::clone::Clone for CLUSTER_NODE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_NODE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_NODE_RESUME_FAILBACK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const DoNotFailbackGroups: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FailbackGroupsImmediately: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FailbackGroupsPerPolicy: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeResumeFailbackTypeCount: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(3i32);
impl ::core::marker::Copy for CLUSTER_NODE_RESUME_FAILBACK_TYPE {}
impl ::core::clone::Clone for CLUSTER_NODE_RESUME_FAILBACK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_NODE_RESUME_FAILBACK_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_NODE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeStateUnknown: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(-1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeUp: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeDown: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodePaused: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeJoining: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(3i32);
impl ::core::marker::Copy for CLUSTER_NODE_STATE {}
impl ::core::clone::Clone for CLUSTER_NODE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_NODE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_NODE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusNormal: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusIsolated: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusQuarantined: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusDrainInProgress: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusDrainCompleted: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusDrainFailed: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusAvoidPlacement: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusMax: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(51i32);
impl ::core::marker::Copy for CLUSTER_NODE_STATUS {}
impl ::core::clone::Clone for CLUSTER_NODE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_NODE_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_NOTIFICATIONS_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NOTIFICATIONS_V1: CLUSTER_NOTIFICATIONS_VERSION = CLUSTER_NOTIFICATIONS_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NOTIFICATIONS_V2: CLUSTER_NOTIFICATIONS_VERSION = CLUSTER_NOTIFICATIONS_VERSION(2i32);
impl ::core::marker::Copy for CLUSTER_NOTIFICATIONS_VERSION {}
impl ::core::clone::Clone for CLUSTER_NOTIFICATIONS_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_NOTIFICATIONS_VERSION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_NONE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_CLUSTER: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_GROUP: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_RESOURCE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_RESOURCE_TYPE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_NETWORK_INTERFACE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_NETWORK: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_NODE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_REGISTRY: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_QUORUM: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_SHARED_VOLUME: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_GROUPSET: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_AFFINITYRULE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(16i32);
impl ::core::marker::Copy for CLUSTER_OBJECT_TYPE {}
impl ::core::clone::Clone for CLUSTER_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_OBJECT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_PROPERTY_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_UNKNOWN: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_BINARY: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_DWORD: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_EXPAND_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_MULTI_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_ULARGE_INTEGER: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(6i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_LONG: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(7i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_EXPANDED_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_SECURITY_DESCRIPTOR: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(9i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_LARGE_INTEGER: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(10i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_WORD: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(11i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_FILETIME: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(12i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_VALUE_LIST: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(13i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_PROPERTY_LIST: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(14i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_USER: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(32768i32);
impl ::core::marker::Copy for CLUSTER_PROPERTY_FORMAT {}
impl ::core::clone::Clone for CLUSTER_PROPERTY_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_PROPERTY_FORMAT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_PROPERTY_SYNTAX(pub u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_ENDMARK: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(0u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_NAME: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(262147u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_RESCLASS: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(131074u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65539u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPAND_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65540u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_DWORD: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65538u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_BINARY: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65537u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_MULTI_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65541u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_LONG: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65543u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPANDED_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65544u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_SECURITY_DESCRIPTOR: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65545u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_LARGE_INTEGER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65546u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_ULARGE_INTEGER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65542u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_WORD: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65547u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_PROPERTY_LIST: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65550u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_FILETIME: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65548u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_DISK_SIGNATURE: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(327682u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_SCSI_ADDRESS: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(393218u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_DISK_NUMBER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(458754u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_PARTITION_INFO: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(524289u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_FTSET_INFO: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(589825u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_DISK_SERIALNUMBER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(655363u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_DISK_GUID: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(720899u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_DISK_SIZE: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(786438u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(851969u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX2: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(917505u32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_STORAGE_DEVICE_ID_DESCRIPTOR: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(983041u32);
impl ::core::marker::Copy for CLUSTER_PROPERTY_SYNTAX {}
impl ::core::clone::Clone for CLUSTER_PROPERTY_SYNTAX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_PROPERTY_SYNTAX {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_PROPERTY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_UNKNOWN: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(-1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_ENDMARK: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_LIST_VALUE: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_RESCLASS: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_RESERVED1: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_NAME: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_SIGNATURE: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_SCSI_ADDRESS: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_DISK_NUMBER: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_PARTITION_INFO: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_FTSET_INFO: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_DISK_SERIALNUMBER: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_DISK_GUID: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_DISK_SIZE: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_PARTITION_INFO_EX: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_PARTITION_INFO_EX2: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_STORAGE_DEVICE_ID_DESCRIPTOR: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_USER: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(32768i32);
impl ::core::marker::Copy for CLUSTER_PROPERTY_TYPE {}
impl ::core::clone::Clone for CLUSTER_PROPERTY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_PROPERTY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_QUORUM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const OperationalQuorum: CLUSTER_QUORUM_TYPE = CLUSTER_QUORUM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ModifyQuorum: CLUSTER_QUORUM_TYPE = CLUSTER_QUORUM_TYPE(1i32);
impl ::core::marker::Copy for CLUSTER_QUORUM_TYPE {}
impl ::core::clone::Clone for CLUSTER_QUORUM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_QUORUM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_QUORUM_VALUE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_QUORUM_MAINTAINED: CLUSTER_QUORUM_VALUE = CLUSTER_QUORUM_VALUE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_QUORUM_LOST: CLUSTER_QUORUM_VALUE = CLUSTER_QUORUM_VALUE(1i32);
impl ::core::marker::Copy for CLUSTER_QUORUM_VALUE {}
impl ::core::clone::Clone for CLUSTER_QUORUM_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_QUORUM_VALUE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_REG_COMMAND(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_COMMAND_NONE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_SET_VALUE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CREATE_KEY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_DELETE_KEY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_DELETE_VALUE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_SET_KEY_SECURITY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_VALUE_DELETED: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(6i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_READ_KEY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(7i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_READ_VALUE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_READ_ERROR: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(9i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONTROL_COMMAND: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(10i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(11i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_NOT_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(12i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_IS_EQUAL: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(13i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_IS_NOT_EQUAL: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(14i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_IS_GREATER_THAN: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(15i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_IS_LESS_THAN: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_KEY_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(17i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_KEY_NOT_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(18i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_LAST_COMMAND: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(19i32);
impl ::core::marker::Copy for CLUSTER_REG_COMMAND {}
impl ::core::clone::Clone for CLUSTER_REG_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_REG_COMMAND {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_RESOURCE_APPLICATION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceApplicationStateUnknown: CLUSTER_RESOURCE_APPLICATION_STATE = CLUSTER_RESOURCE_APPLICATION_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceApplicationOSHeartBeat: CLUSTER_RESOURCE_APPLICATION_STATE = CLUSTER_RESOURCE_APPLICATION_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceApplicationReady: CLUSTER_RESOURCE_APPLICATION_STATE = CLUSTER_RESOURCE_APPLICATION_STATE(3i32);
impl ::core::marker::Copy for CLUSTER_RESOURCE_APPLICATION_STATE {}
impl ::core::clone::Clone for CLUSTER_RESOURCE_APPLICATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_RESOURCE_APPLICATION_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_RESOURCE_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESCLASS_UNKNOWN: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESCLASS_STORAGE: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESCLASS_NETWORK: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESCLASS_USER: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(32768i32);
impl ::core::marker::Copy for CLUSTER_RESOURCE_CLASS {}
impl ::core::clone::Clone for CLUSTER_RESOURCE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_RESOURCE_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_RESOURCE_CREATE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_DEFAULT_MONITOR: CLUSTER_RESOURCE_CREATE_FLAGS = CLUSTER_RESOURCE_CREATE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_SEPARATE_MONITOR: CLUSTER_RESOURCE_CREATE_FLAGS = CLUSTER_RESOURCE_CREATE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_VALID_FLAGS: CLUSTER_RESOURCE_CREATE_FLAGS = CLUSTER_RESOURCE_CREATE_FLAGS(1i32);
impl ::core::marker::Copy for CLUSTER_RESOURCE_CREATE_FLAGS {}
impl ::core::clone::Clone for CLUSTER_RESOURCE_CREATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_RESOURCE_CREATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceEmbeddedFailureActionNone: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceEmbeddedFailureActionLogOnly: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceEmbeddedFailureActionRecover: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(2i32);
impl ::core::marker::Copy for CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION {}
impl ::core::clone::Clone for CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_RESOURCE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_DEPENDS: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_PROVIDES: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_NODES: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_ALL: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(7i32);
impl ::core::marker::Copy for CLUSTER_RESOURCE_ENUM {}
impl ::core::clone::Clone for CLUSTER_RESOURCE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_RESOURCE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_RESOURCE_RESTART_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceDontRestart: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceRestartNoNotify: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceRestartNotify: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceRestartActionCount: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(3i32);
impl ::core::marker::Copy for CLUSTER_RESOURCE_RESTART_ACTION {}
impl ::core::clone::Clone for CLUSTER_RESOURCE_RESTART_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_RESOURCE_RESTART_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_RESOURCE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceStateUnknown: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(-1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceInherited: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceInitializing: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceOnline: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceOffline: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceFailed: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourcePending: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(128i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceOnlinePending: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(129i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceOfflinePending: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(130i32);
impl ::core::marker::Copy for CLUSTER_RESOURCE_STATE {}
impl ::core::clone::Clone for CLUSTER_RESOURCE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_RESOURCE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_RESOURCE_STATE_CHANGE_REASON(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonUnknown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonMove: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonFailover: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonFailedMove: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonShutdown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonRundown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(5i32);
impl ::core::marker::Copy for CLUSTER_RESOURCE_STATE_CHANGE_REASON {}
impl ::core::clone::Clone for CLUSTER_RESOURCE_STATE_CHANGE_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_RESOURCE_STATE_CHANGE_REASON {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_RESOURCE_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_TYPE_ENUM_NODES: CLUSTER_RESOURCE_TYPE_ENUM = CLUSTER_RESOURCE_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_TYPE_ENUM_RESOURCES: CLUSTER_RESOURCE_TYPE_ENUM = CLUSTER_RESOURCE_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_TYPE_ENUM_ALL: CLUSTER_RESOURCE_TYPE_ENUM = CLUSTER_RESOURCE_TYPE_ENUM(3i32);
impl ::core::marker::Copy for CLUSTER_RESOURCE_TYPE_ENUM {}
impl ::core::clone::Clone for CLUSTER_RESOURCE_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_RESOURCE_TYPE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_ROLE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleDHCP: CLUSTER_ROLE = CLUSTER_ROLE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleDTC: CLUSTER_ROLE = CLUSTER_ROLE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleFileServer: CLUSTER_ROLE = CLUSTER_ROLE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleGenericApplication: CLUSTER_ROLE = CLUSTER_ROLE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleGenericScript: CLUSTER_ROLE = CLUSTER_ROLE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleGenericService: CLUSTER_ROLE = CLUSTER_ROLE(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleISCSINameServer: CLUSTER_ROLE = CLUSTER_ROLE(6i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleMSMQ: CLUSTER_ROLE = CLUSTER_ROLE(7i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleNFS: CLUSTER_ROLE = CLUSTER_ROLE(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRolePrintServer: CLUSTER_ROLE = CLUSTER_ROLE(9i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleStandAloneNamespaceServer: CLUSTER_ROLE = CLUSTER_ROLE(10i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleVolumeShadowCopyServiceTask: CLUSTER_ROLE = CLUSTER_ROLE(11i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleWINS: CLUSTER_ROLE = CLUSTER_ROLE(12i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleTaskScheduler: CLUSTER_ROLE = CLUSTER_ROLE(13i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleNetworkFileSystem: CLUSTER_ROLE = CLUSTER_ROLE(14i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleDFSReplicatedFolder: CLUSTER_ROLE = CLUSTER_ROLE(15i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleDistributedFileSystem: CLUSTER_ROLE = CLUSTER_ROLE(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleDistributedNetworkName: CLUSTER_ROLE = CLUSTER_ROLE(17i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleFileShare: CLUSTER_ROLE = CLUSTER_ROLE(18i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleFileShareWitness: CLUSTER_ROLE = CLUSTER_ROLE(19i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleHardDisk: CLUSTER_ROLE = CLUSTER_ROLE(20i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleIPAddress: CLUSTER_ROLE = CLUSTER_ROLE(21i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleIPV6Address: CLUSTER_ROLE = CLUSTER_ROLE(22i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleIPV6TunnelAddress: CLUSTER_ROLE = CLUSTER_ROLE(23i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleISCSITargetServer: CLUSTER_ROLE = CLUSTER_ROLE(24i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleNetworkName: CLUSTER_ROLE = CLUSTER_ROLE(25i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRolePhysicalDisk: CLUSTER_ROLE = CLUSTER_ROLE(26i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleSODAFileServer: CLUSTER_ROLE = CLUSTER_ROLE(27i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleStoragePool: CLUSTER_ROLE = CLUSTER_ROLE(28i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleVirtualMachine: CLUSTER_ROLE = CLUSTER_ROLE(29i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleVirtualMachineConfiguration: CLUSTER_ROLE = CLUSTER_ROLE(30i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleVirtualMachineReplicaBroker: CLUSTER_ROLE = CLUSTER_ROLE(31i32);
impl ::core::marker::Copy for CLUSTER_ROLE {}
impl ::core::clone::Clone for CLUSTER_ROLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_ROLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_ROLE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleUnknown: CLUSTER_ROLE_STATE = CLUSTER_ROLE_STATE(-1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleClustered: CLUSTER_ROLE_STATE = CLUSTER_ROLE_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleUnclustered: CLUSTER_ROLE_STATE = CLUSTER_ROLE_STATE(1i32);
impl ::core::marker::Copy for CLUSTER_ROLE_STATE {}
impl ::core::clone::Clone for CLUSTER_ROLE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_ROLE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_SETUP_PHASE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseInitialize: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseValidateNodeState: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(100i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseValidateNetft: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(102i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseValidateClusDisk: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(103i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseConfigureClusSvc: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(104i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseStartingClusSvc: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(105i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseQueryClusterNameAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(106i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseValidateClusterNameAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(107i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCreateClusterAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(108i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseConfigureClusterAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(109i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseFormingCluster: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(200i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseAddClusterProperties: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(201i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCreateResourceTypes: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(202i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCreateGroups: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(203i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCreateIPAddressResources: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(204i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCreateNetworkName: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(205i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseClusterGroupOnline: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(206i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseGettingCurrentMembership: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(300i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseAddNodeToCluster: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(301i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseNodeUp: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(302i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseMoveGroup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(400i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseDeleteGroup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(401i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCleanupCOs: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(402i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseOfflineGroup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(403i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseEvictNode: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(404i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCleanupNode: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(405i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCoreGroupCleanup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(406i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseFailureCleanup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(999i32);
impl ::core::marker::Copy for CLUSTER_SETUP_PHASE {}
impl ::core::clone::Clone for CLUSTER_SETUP_PHASE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SETUP_PHASE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_SETUP_PHASE_SEVERITY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseInformational: CLUSTER_SETUP_PHASE_SEVERITY = CLUSTER_SETUP_PHASE_SEVERITY(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseWarning: CLUSTER_SETUP_PHASE_SEVERITY = CLUSTER_SETUP_PHASE_SEVERITY(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseFatal: CLUSTER_SETUP_PHASE_SEVERITY = CLUSTER_SETUP_PHASE_SEVERITY(3i32);
impl ::core::marker::Copy for CLUSTER_SETUP_PHASE_SEVERITY {}
impl ::core::clone::Clone for CLUSTER_SETUP_PHASE_SEVERITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SETUP_PHASE_SEVERITY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_SETUP_PHASE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseStart: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseContinue: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseEnd: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseReport: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(4i32);
impl ::core::marker::Copy for CLUSTER_SETUP_PHASE_TYPE {}
impl ::core::clone::Clone for CLUSTER_SETUP_PHASE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SETUP_PHASE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_SHARED_VOLUME_BACKUP_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeBackupNone: CLUSTER_SHARED_VOLUME_BACKUP_STATE = CLUSTER_SHARED_VOLUME_BACKUP_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeBackupInProgress: CLUSTER_SHARED_VOLUME_BACKUP_STATE = CLUSTER_SHARED_VOLUME_BACKUP_STATE(1i32);
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_BACKUP_STATE {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_BACKUP_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_BACKUP_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeRenameInputTypeNone: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeRenameInputTypeVolumeOffset: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeRenameInputTypeVolumeId: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeRenameInputTypeVolumeName: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeRenameInputTypeVolumeGuid: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(4i32);
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeSnapshotStateUnknown: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumePrepareForHWSnapshot: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeHWSnapshotCompleted: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumePrepareForFreeze: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(3i32);
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_SHARED_VOLUME_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SharedVolumeStateUnavailable: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SharedVolumeStatePaused: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SharedVolumeStateActive: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SharedVolumeStateActiveRedirected: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SharedVolumeStateActiveVolumeRedirected: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(4i32);
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_STATE {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_STORAGENODE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodeStateUnknown: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodeUp: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodeDown: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodePaused: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodeStarting: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodeStopping: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(5i32);
impl ::core::marker::Copy for CLUSTER_STORAGENODE_STATE {}
impl ::core::clone::Clone for CLUSTER_STORAGENODE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_STORAGENODE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUSTER_UPGRADE_PHASE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterUpgradePhaseInitialize: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterUpgradePhaseValidatingUpgrade: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterUpgradePhaseUpgradingComponents: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterUpgradePhaseInstallingNewComponents: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterUpgradePhaseUpgradeComplete: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(5i32);
impl ::core::marker::Copy for CLUSTER_UPGRADE_PHASE {}
impl ::core::clone::Clone for CLUSTER_UPGRADE_PHASE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_UPGRADE_PHASE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUS_AFFINITY_RULE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_NONE: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_SAME_FAULT_DOMAIN: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_SAME_NODE: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_DIFFERENT_FAULT_DOMAIN: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_DIFFERENT_NODE: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_MIN: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_MAX: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(4i32);
impl ::core::marker::Copy for CLUS_AFFINITY_RULE_TYPE {}
impl ::core::clone::Clone for CLUS_AFFINITY_RULE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_AFFINITY_RULE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUS_CHARACTERISTICS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_UNKNOWN: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_QUORUM: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_DELETE_REQUIRES_ALL_NODES: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_LOCAL_QUORUM: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_LOCAL_QUORUM_DEBUG: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_REQUIRES_STATE_CHANGE_REASON: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_BROADCAST_DELETE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_SINGLE_CLUSTER_INSTANCE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(64i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_SINGLE_GROUP_INSTANCE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(128i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_COEXIST_IN_SHARED_VOLUME_GROUP: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(256i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_PLACEMENT_DATA: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(512i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_MONITOR_DETACH: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(1024i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_MONITOR_REATTACH: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(2048i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_OPERATION_CONTEXT: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(4096i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_CLONES: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(8192i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_NOT_PREEMPTABLE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(16384i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_NOTIFY_NEW_OWNER: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(32768i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_SUPPORTS_UNMONITORED_STATE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(65536i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_INFRASTRUCTURE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(131072i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_VETO_DRAIN: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(262144i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_DRAIN_LOCAL_OFFLINE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(524288i32);
impl ::core::marker::Copy for CLUS_CHARACTERISTICS {}
impl ::core::clone::Clone for CLUS_CHARACTERISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_CHARACTERISTICS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUS_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_FLAG_CORE: CLUS_FLAGS = CLUS_FLAGS(1i32);
impl ::core::marker::Copy for CLUS_FLAGS {}
impl ::core::clone::Clone for CLUS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUS_GROUP_START_SETTING(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GROUP_START_ALWAYS: CLUS_GROUP_START_SETTING = CLUS_GROUP_START_SETTING(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GROUP_DO_NOT_START: CLUS_GROUP_START_SETTING = CLUS_GROUP_START_SETTING(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GROUP_START_ALLOWED: CLUS_GROUP_START_SETTING = CLUS_GROUP_START_SETTING(2i32);
impl ::core::marker::Copy for CLUS_GROUP_START_SETTING {}
impl ::core::clone::Clone for CLUS_GROUP_START_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_GROUP_START_SETTING {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUS_RESSUBCLASS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESSUBCLASS_SHARED: CLUS_RESSUBCLASS = CLUS_RESSUBCLASS(-2147483648i32);
impl ::core::marker::Copy for CLUS_RESSUBCLASS {}
impl ::core::clone::Clone for CLUS_RESSUBCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_RESSUBCLASS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUS_RESSUBCLASS_NETWORK(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESSUBCLASS_NETWORK_INTERNET_PROTOCOL: CLUS_RESSUBCLASS_NETWORK = CLUS_RESSUBCLASS_NETWORK(-2147483648i32);
impl ::core::marker::Copy for CLUS_RESSUBCLASS_NETWORK {}
impl ::core::clone::Clone for CLUS_RESSUBCLASS_NETWORK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_RESSUBCLASS_NETWORK {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLUS_RESSUBCLASS_STORAGE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESSUBCLASS_STORAGE_SHARED_BUS: CLUS_RESSUBCLASS_STORAGE = CLUS_RESSUBCLASS_STORAGE(-2147483648i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESSUBCLASS_STORAGE_DISK: CLUS_RESSUBCLASS_STORAGE = CLUS_RESSUBCLASS_STORAGE(1073741824i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESSUBCLASS_STORAGE_REPLICATION: CLUS_RESSUBCLASS_STORAGE = CLUS_RESSUBCLASS_STORAGE(268435456i32);
impl ::core::marker::Copy for CLUS_RESSUBCLASS_STORAGE {}
impl ::core::clone::Clone for CLUS_RESSUBCLASS_STORAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_RESSUBCLASS_STORAGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FAILURE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FAILURE_TYPE_GENERAL: FAILURE_TYPE = FAILURE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FAILURE_TYPE_EMBEDDED: FAILURE_TYPE = FAILURE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FAILURE_TYPE_NETWORK_LOSS: FAILURE_TYPE = FAILURE_TYPE(2i32);
impl ::core::marker::Copy for FAILURE_TYPE {}
impl ::core::clone::Clone for FAILURE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FAILURE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILESHARE_CHANGE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FILESHARE_CHANGE_NONE: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FILESHARE_CHANGE_ADD: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FILESHARE_CHANGE_DEL: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FILESHARE_CHANGE_MODIFY: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(3i32);
impl ::core::marker::Copy for FILESHARE_CHANGE_ENUM {}
impl ::core::clone::Clone for FILESHARE_CHANGE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FILESHARE_CHANGE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GRP_PLACEMENT_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GRP_PLACEMENT_OPTIONS_MIN_VALUE: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GRP_PLACEMENT_OPTIONS_DEFAULT: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GRP_PLACEMENT_OPTIONS_DISABLE_AUTOBALANCING: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GRP_PLACEMENT_OPTIONS_ALL: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(1i32);
impl ::core::marker::Copy for GRP_PLACEMENT_OPTIONS {}
impl ::core::clone::Clone for GRP_PLACEMENT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GRP_PLACEMENT_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LOG_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const LOG_INFORMATION: LOG_LEVEL = LOG_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const LOG_WARNING: LOG_LEVEL = LOG_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const LOG_ERROR: LOG_LEVEL = LOG_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const LOG_SEVERE: LOG_LEVEL = LOG_LEVEL(3i32);
impl ::core::marker::Copy for LOG_LEVEL {}
impl ::core::clone::Clone for LOG_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LOG_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MAINTENANCE_MODE_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MaintenanceModeTypeDisableIsAliveCheck: MAINTENANCE_MODE_TYPE_ENUM = MAINTENANCE_MODE_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MaintenanceModeTypeOfflineResource: MAINTENANCE_MODE_TYPE_ENUM = MAINTENANCE_MODE_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MaintenanceModeTypeUnclusterResource: MAINTENANCE_MODE_TYPE_ENUM = MAINTENANCE_MODE_TYPE_ENUM(3i32);
impl ::core::marker::Copy for MAINTENANCE_MODE_TYPE_ENUM {}
impl ::core::clone::Clone for MAINTENANCE_MODE_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MAINTENANCE_MODE_TYPE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NODE_CLUSTER_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStateNotInstalled: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStateNotConfigured: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStateNotRunning: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStateRunning: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(19i32);
impl ::core::marker::Copy for NODE_CLUSTER_STATE {}
impl ::core::clone::Clone for NODE_CLUSTER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NODE_CLUSTER_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PLACEMENT_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_MIN_VALUE: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DEFAULT_PLACEMENT_OPTIONS: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DISABLE_CSV_VM_DEPENDENCY: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_CONSIDER_OFFLINE_VMS: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DONT_USE_MEMORY: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DONT_USE_CPU: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DONT_USE_LOCAL_TEMP_DISK: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(16i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DONT_RESUME_VMS_WITH_EXISTING_TEMP_DISK: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(32i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_SAVE_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(64i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DONT_RESUME_AVAILABILTY_SET_VMS_WITH_EXISTING_TEMP_DISK: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(128i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_SAVE_AVAILABILTY_SET_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(256i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_AVAILABILITY_SET_DOMAIN_AFFINITY: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(512i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_ALL: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(1023i32);
impl ::core::marker::Copy for PLACEMENT_OPTIONS {}
impl ::core::clone::Clone for PLACEMENT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PLACEMENT_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RESDLL_CONTEXT_OPERATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeFailback: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeDrain: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeDrainFailure: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeEmbeddedFailure: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypePreemption: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeNetworkDisconnect: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeNetworkDisconnectMoveRetry: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(6i32);
impl ::core::marker::Copy for RESDLL_CONTEXT_OPERATION_TYPE {}
impl ::core::clone::Clone for RESDLL_CONTEXT_OPERATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RESDLL_CONTEXT_OPERATION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RESOURCE_EXIT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResourceExitStateContinue: RESOURCE_EXIT_STATE = RESOURCE_EXIT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResourceExitStateTerminate: RESOURCE_EXIT_STATE = RESOURCE_EXIT_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResourceExitStateMax: RESOURCE_EXIT_STATE = RESOURCE_EXIT_STATE(2i32);
impl ::core::marker::Copy for RESOURCE_EXIT_STATE {}
impl ::core::clone::Clone for RESOURCE_EXIT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RESOURCE_EXIT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RESOURCE_MONITOR_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonInitializing: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonIdle: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonStartingResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonInitializingResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonOnlineResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonOfflineResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonShutdownResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(6i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonDeletingResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(7i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonIsAlivePoll: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonLooksAlivePoll: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(9i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonArbitrateResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(10i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonReleaseResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(11i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonResourceControl: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(12i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonResourceTypeControl: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(13i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonTerminateResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(14i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonDeadlocked: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(15i32);
impl ::core::marker::Copy for RESOURCE_MONITOR_STATE {}
impl ::core::clone::Clone for RESOURCE_MONITOR_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RESOURCE_MONITOR_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SR_DISK_REPLICATION_ELIGIBLE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleNone: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleYes: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleOffline: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleNotGpt: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligiblePartitionLayoutMismatch: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleInsufficientFreeSpace: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleNotInSameSite: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(6i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleInSameSite: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(7i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleFileSystemNotSupported: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(8i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleAlreadyInReplication: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(9i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleSameAsSpecifiedDisk: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(10i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleOther: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(9999i32);
impl ::core::marker::Copy for SR_DISK_REPLICATION_ELIGIBLE {}
impl ::core::clone::Clone for SR_DISK_REPLICATION_ELIGIBLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SR_DISK_REPLICATION_ELIGIBLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SR_REPLICATED_DISK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeNone: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeSource: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeLogSource: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeDestination: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeLogDestination: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeNotInParthership: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeLogNotInParthership: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeOther: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(7i32);
impl ::core::marker::Copy for SR_REPLICATED_DISK_TYPE {}
impl ::core::clone::Clone for SR_REPLICATED_DISK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SR_REPLICATED_DISK_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VM_RESDLL_CONTEXT(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VmResdllContextTurnOff: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(0i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VmResdllContextSave: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(1i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VmResdllContextShutdown: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(2i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VmResdllContextShutdownForce: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(3i32);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VmResdllContextLiveMigration: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(4i32);
impl ::core::marker::Copy for VM_RESDLL_CONTEXT {}
impl ::core::clone::Clone for VM_RESDLL_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VM_RESDLL_CONTEXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLRES_CALLBACK_FUNCTION_TABLE {
    pub LogEvent: PLOG_EVENT_ROUTINE,
    pub SetResourceStatusEx: PSET_RESOURCE_STATUS_ROUTINE_EX,
    pub SetResourceLockedMode: PSET_RESOURCE_LOCKED_MODE_ROUTINE,
    pub SignalFailure: PSIGNAL_FAILURE_ROUTINE,
    pub SetResourceInMemoryNodeLocalProperties: PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE,
    pub EndControlCall: PEND_CONTROL_CALL,
    pub EndTypeControlCall: PEND_TYPE_CONTROL_CALL,
    pub ExtendControlCall: PEXTEND_RES_CONTROL_CALL,
    pub ExtendTypeControlCall: PEXTEND_RES_TYPE_CONTROL_CALL,
    pub RaiseResTypeNotification: PRAISE_RES_TYPE_NOTIFICATION,
    pub ChangeResourceProcessForDumps: PCHANGE_RESOURCE_PROCESS_FOR_DUMPS,
    pub ChangeResTypeProcessForDumps: PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS,
    pub SetInternalState: PSET_INTERNAL_STATE,
    pub SetResourceLockedModeEx: PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE,
    pub RequestDump: PREQUEST_DUMP_ROUTINE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLRES_CALLBACK_FUNCTION_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLRES_CALLBACK_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLRES_CALLBACK_FUNCTION_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_FUNCTION_TABLE {
    pub TableSize: u32,
    pub Version: u32,
    pub Anonymous: CLRES_FUNCTION_TABLE_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_FUNCTION_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
unsafe impl ::windows::core::Abi for CLRES_FUNCTION_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub union CLRES_FUNCTION_TABLE_0 {
    pub V1Functions: CLRES_V1_FUNCTIONS,
    pub V2Functions: CLRES_V2_FUNCTIONS,
    pub V3Functions: CLRES_V3_FUNCTIONS,
    pub V4Functions: CLRES_V4_FUNCTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_FUNCTION_TABLE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_FUNCTION_TABLE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
unsafe impl ::windows::core::Abi for CLRES_FUNCTION_TABLE_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V1_FUNCTIONS {
    pub Open: POPEN_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_ROUTINE,
    pub Offline: POFFLINE_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub ResourceControl: PRESOURCE_CONTROL_ROUTINE,
    pub ResourceTypeControl: PRESOURCE_TYPE_CONTROL_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V1_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V1_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
unsafe impl ::windows::core::Abi for CLRES_V1_FUNCTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V2_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub ResourceControl: PRESOURCE_CONTROL_ROUTINE,
    pub ResourceTypeControl: PRESOURCE_TYPE_CONTROL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V2_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V2_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
unsafe impl ::windows::core::Abi for CLRES_V2_FUNCTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V3_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub BeginResourceControl: PBEGIN_RESCALL_ROUTINE,
    pub BeginResourceTypeControl: PBEGIN_RESTYPECALL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V3_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V3_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
unsafe impl ::windows::core::Abi for CLRES_V3_FUNCTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V4_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub BeginResourceControl: PBEGIN_RESCALL_ROUTINE,
    pub BeginResourceTypeControl: PBEGIN_RESTYPECALL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
    pub BeginResourceControlAsUser: PBEGIN_RESCALL_AS_USER_ROUTINE,
    pub BeginResourceTypeControlAsUser: PBEGIN_RESTYPECALL_AS_USER_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V4_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V4_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
unsafe impl ::windows::core::Abi for CLRES_V4_FUNCTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    pub GetTickCount64: u64,
    pub GetSystemTime: super::super::Foundation::SYSTEMTIME,
    pub NodeId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub eReason: CLUSTER_RESOURCE_STATE_CHANGE_REASON,
}
impl ::core::marker::Copy for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {}
impl ::core::clone::Clone for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    pub dwFlags: u32,
    pub guidPoolFilter: ::windows::core::GUID,
}
impl ::core::marker::Copy for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {}
impl ::core::clone::Clone for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_BINARY {
    pub Base: CLUSPROP_VALUE,
    pub rgb: [u8; 1],
}
impl ::core::marker::Copy for CLUSPROP_BINARY {}
impl ::core::clone::Clone for CLUSPROP_BINARY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_BINARY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union CLUSPROP_BUFFER_HELPER {
    pub pb: *mut u8,
    pub pw: *mut u16,
    pub pdw: *mut u32,
    pub pl: *mut i32,
    pub psz: ::windows::core::PWSTR,
    pub pList: *mut CLUSPROP_LIST,
    pub pSyntax: *mut CLUSPROP_SYNTAX,
    pub pName: *mut CLUSPROP_SZ,
    pub pValue: *mut CLUSPROP_VALUE,
    pub pBinaryValue: *mut CLUSPROP_BINARY,
    pub pWordValue: *mut CLUSPROP_WORD,
    pub pDwordValue: *mut CLUSPROP_DWORD,
    pub pLongValue: *mut CLUSPROP_LONG,
    pub pULargeIntegerValue: *mut CLUSPROP_ULARGE_INTEGER,
    pub pLargeIntegerValue: *mut CLUSPROP_LARGE_INTEGER,
    pub pStringValue: *mut CLUSPROP_SZ,
    pub pMultiSzValue: *mut CLUSPROP_SZ,
    pub pSecurityDescriptor: *mut CLUSPROP_SECURITY_DESCRIPTOR,
    pub pResourceClassValue: *mut CLUSPROP_RESOURCE_CLASS,
    pub pResourceClassInfoValue: *mut CLUSPROP_RESOURCE_CLASS_INFO,
    pub pDiskSignatureValue: *mut CLUSPROP_DWORD,
    pub pScsiAddressValue: *mut CLUSPROP_SCSI_ADDRESS,
    pub pDiskNumberValue: *mut CLUSPROP_DWORD,
    pub pPartitionInfoValue: *mut CLUSPROP_PARTITION_INFO,
    pub pRequiredDependencyValue: *mut CLUSPROP_REQUIRED_DEPENDENCY,
    pub pPartitionInfoValueEx: *mut CLUSPROP_PARTITION_INFO_EX,
    pub pPartitionInfoValueEx2: *mut CLUSPROP_PARTITION_INFO_EX2,
    pub pFileTimeValue: *mut CLUSPROP_FILETIME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for CLUSPROP_BUFFER_HELPER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for CLUSPROP_BUFFER_HELPER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for CLUSPROP_BUFFER_HELPER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_DWORD {
    pub Base: CLUSPROP_VALUE,
    pub dw: u32,
}
impl ::core::marker::Copy for CLUSPROP_DWORD {}
impl ::core::clone::Clone for CLUSPROP_DWORD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_DWORD {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSPROP_FILETIME {
    pub Base: CLUSPROP_VALUE,
    pub ft: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSPROP_FILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSPROP_FILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUSPROP_FILETIME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_FTSET_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_FTSET_INFO,
}
impl ::core::marker::Copy for CLUSPROP_FTSET_INFO {}
impl ::core::clone::Clone for CLUSPROP_FTSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_FTSET_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_LARGE_INTEGER {
    pub Base: CLUSPROP_VALUE,
    pub li: i64,
}
impl ::core::marker::Copy for CLUSPROP_LARGE_INTEGER {}
impl ::core::clone::Clone for CLUSPROP_LARGE_INTEGER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_LARGE_INTEGER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_LIST {
    pub nPropertyCount: u32,
    pub PropertyName: CLUSPROP_SZ,
}
impl ::core::marker::Copy for CLUSPROP_LIST {}
impl ::core::clone::Clone for CLUSPROP_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_LONG {
    pub Base: CLUSPROP_VALUE,
    pub l: i32,
}
impl ::core::marker::Copy for CLUSPROP_LONG {}
impl ::core::clone::Clone for CLUSPROP_LONG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_LONG {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_PARTITION_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_PARTITION_INFO,
}
impl ::core::marker::Copy for CLUSPROP_PARTITION_INFO {}
impl ::core::clone::Clone for CLUSPROP_PARTITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_PARTITION_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_PARTITION_INFO_EX {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_PARTITION_INFO_EX,
}
impl ::core::marker::Copy for CLUSPROP_PARTITION_INFO_EX {}
impl ::core::clone::Clone for CLUSPROP_PARTITION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_PARTITION_INFO_EX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_PARTITION_INFO_EX2 {
    pub Base: CLUSPROP_PARTITION_INFO_EX,
    pub Base2: CLUS_PARTITION_INFO_EX2,
}
impl ::core::marker::Copy for CLUSPROP_PARTITION_INFO_EX2 {}
impl ::core::clone::Clone for CLUSPROP_PARTITION_INFO_EX2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_PARTITION_INFO_EX2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUSPROP_REQUIRED_DEPENDENCY {
    pub Value: CLUSPROP_VALUE,
    pub ResClass: CLUSPROP_RESOURCE_CLASS,
    pub ResTypeName: CLUSPROP_SZ,
}
impl ::core::marker::Copy for CLUSPROP_REQUIRED_DEPENDENCY {}
impl ::core::clone::Clone for CLUSPROP_REQUIRED_DEPENDENCY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_REQUIRED_DEPENDENCY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_RESOURCE_CLASS {
    pub Base: CLUSPROP_VALUE,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl ::core::marker::Copy for CLUSPROP_RESOURCE_CLASS {}
impl ::core::clone::Clone for CLUSPROP_RESOURCE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_RESOURCE_CLASS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_RESOURCE_CLASS_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_RESOURCE_CLASS_INFO,
}
impl ::core::marker::Copy for CLUSPROP_RESOURCE_CLASS_INFO {}
impl ::core::clone::Clone for CLUSPROP_RESOURCE_CLASS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_RESOURCE_CLASS_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_SCSI_ADDRESS {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_SCSI_ADDRESS,
}
impl ::core::marker::Copy for CLUSPROP_SCSI_ADDRESS {}
impl ::core::clone::Clone for CLUSPROP_SCSI_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_SCSI_ADDRESS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct CLUSPROP_SECURITY_DESCRIPTOR {
    pub Base: CLUSPROP_VALUE,
    pub Anonymous: CLUSPROP_SECURITY_DESCRIPTOR_0,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for CLUSPROP_SECURITY_DESCRIPTOR {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for CLUSPROP_SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for CLUSPROP_SECURITY_DESCRIPTOR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub union CLUSPROP_SECURITY_DESCRIPTOR_0 {
    pub sd: super::super::Security::SECURITY_DESCRIPTOR_RELATIVE,
    pub rgbSecurityDescriptor: [u8; 1],
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for CLUSPROP_SECURITY_DESCRIPTOR_0 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for CLUSPROP_SECURITY_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for CLUSPROP_SECURITY_DESCRIPTOR_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUSPROP_SYNTAX {
    pub dw: u32,
    pub Anonymous: CLUSPROP_SYNTAX_0,
}
impl ::core::marker::Copy for CLUSPROP_SYNTAX {}
impl ::core::clone::Clone for CLUSPROP_SYNTAX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_SYNTAX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_SYNTAX_0 {
    pub wFormat: u16,
    pub wType: u16,
}
impl ::core::marker::Copy for CLUSPROP_SYNTAX_0 {}
impl ::core::clone::Clone for CLUSPROP_SYNTAX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_SYNTAX_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_SZ {
    pub Base: CLUSPROP_VALUE,
    pub sz: [u16; 1],
}
impl ::core::marker::Copy for CLUSPROP_SZ {}
impl ::core::clone::Clone for CLUSPROP_SZ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_SZ {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_ULARGE_INTEGER {
    pub Base: CLUSPROP_VALUE,
    pub li: u64,
}
impl ::core::marker::Copy for CLUSPROP_ULARGE_INTEGER {}
impl ::core::clone::Clone for CLUSPROP_ULARGE_INTEGER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_ULARGE_INTEGER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_VALUE {
    pub Syntax: CLUSPROP_SYNTAX,
    pub cbLength: u32,
}
impl ::core::marker::Copy for CLUSPROP_VALUE {}
impl ::core::clone::Clone for CLUSPROP_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_VALUE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_WORD {
    pub Base: CLUSPROP_VALUE,
    pub w: u16,
}
impl ::core::marker::Copy for CLUSPROP_WORD {}
impl ::core::clone::Clone for CLUSPROP_WORD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSPROP_WORD {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTERVERSIONINFO {
    pub dwVersionInfoSize: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub szVendorId: [u16; 64],
    pub szCSDVersion: [u16; 64],
    pub dwClusterHighestVersion: u32,
    pub dwClusterLowestVersion: u32,
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for CLUSTERVERSIONINFO {}
impl ::core::clone::Clone for CLUSTERVERSIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTERVERSIONINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTERVERSIONINFO_NT4 {
    pub dwVersionInfoSize: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub szVendorId: [u16; 64],
    pub szCSDVersion: [u16; 64],
}
impl ::core::marker::Copy for CLUSTERVERSIONINFO_NT4 {}
impl ::core::clone::Clone for CLUSTERVERSIONINFO_NT4 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTERVERSIONINFO_NT4 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_AVAILABILITY_SET_CONFIG {
    pub dwVersion: u32,
    pub dwUpdateDomains: u32,
    pub dwFaultDomains: u32,
    pub bReserveSpareNode: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_AVAILABILITY_SET_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_AVAILABILITY_SET_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUSTER_AVAILABILITY_SET_CONFIG {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzName: ::windows::core::PCWSTR,
    pub lpData: *const u8,
    pub cbData: u32,
}
impl ::core::marker::Copy for CLUSTER_BATCH_COMMAND {}
impl ::core::clone::Clone for CLUSTER_BATCH_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_BATCH_COMMAND {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_CREATE_GROUP_INFO {
    pub dwVersion: u32,
    pub groupType: CLUSGROUP_TYPE,
}
impl ::core::marker::Copy for CLUSTER_CREATE_GROUP_INFO {}
impl ::core::clone::Clone for CLUSTER_CREATE_GROUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_CREATE_GROUP_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_ENUM_ITEM {
    pub dwVersion: u32,
    pub dwType: u32,
    pub cbId: u32,
    pub lpszId: ::windows::core::PWSTR,
    pub cbName: u32,
    pub lpszName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CLUSTER_ENUM_ITEM {}
impl ::core::clone::Clone for CLUSTER_ENUM_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_ENUM_ITEM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_GROUP_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: ::windows::core::PWSTR,
    pub cbName: u32,
    pub lpszName: ::windows::core::PWSTR,
    pub state: CLUSTER_GROUP_STATE,
    pub cbOwnerNode: u32,
    pub lpszOwnerNode: ::windows::core::PWSTR,
    pub dwFlags: u32,
    pub cbProperties: u32,
    pub pProperties: *mut ::core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CLUSTER_GROUP_ENUM_ITEM {}
impl ::core::clone::Clone for CLUSTER_GROUP_ENUM_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_GROUP_ENUM_ITEM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_HEALTH_FAULT {
    pub Id: ::windows::core::PWSTR,
    pub ErrorType: u32,
    pub ErrorCode: u32,
    pub Description: ::windows::core::PWSTR,
    pub Provider: ::windows::core::PWSTR,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for CLUSTER_HEALTH_FAULT {}
impl ::core::clone::Clone for CLUSTER_HEALTH_FAULT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_HEALTH_FAULT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_HEALTH_FAULT_ARRAY {
    pub numFaults: u32,
    pub faults: *mut CLUSTER_HEALTH_FAULT,
}
impl ::core::marker::Copy for CLUSTER_HEALTH_FAULT_ARRAY {}
impl ::core::clone::Clone for CLUSTER_HEALTH_FAULT_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_HEALTH_FAULT_ARRAY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_IP_ENTRY {
    pub lpszIpAddress: ::windows::core::PCWSTR,
    pub dwPrefixLength: u32,
}
impl ::core::marker::Copy for CLUSTER_IP_ENTRY {}
impl ::core::clone::Clone for CLUSTER_IP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_IP_ENTRY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_MEMBERSHIP_INFO {
    pub HasQuorum: super::super::Foundation::BOOL,
    pub UpnodesSize: u32,
    pub Upnodes: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_MEMBERSHIP_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_MEMBERSHIP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUSTER_MEMBERSHIP_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_READ_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzSubkeyName: ::windows::core::PCWSTR,
    pub wzValueName: ::windows::core::PCWSTR,
    pub lpData: *const u8,
    pub cbData: u32,
}
impl ::core::marker::Copy for CLUSTER_READ_BATCH_COMMAND {}
impl ::core::clone::Clone for CLUSTER_READ_BATCH_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_READ_BATCH_COMMAND {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_RESOURCE_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: ::windows::core::PWSTR,
    pub cbName: u32,
    pub lpszName: ::windows::core::PWSTR,
    pub cbOwnerGroupName: u32,
    pub lpszOwnerGroupName: ::windows::core::PWSTR,
    pub cbOwnerGroupId: u32,
    pub lpszOwnerGroupId: ::windows::core::PWSTR,
    pub cbProperties: u32,
    pub pProperties: *mut ::core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CLUSTER_RESOURCE_ENUM_ITEM {}
impl ::core::clone::Clone for CLUSTER_RESOURCE_ENUM_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_RESOURCE_ENUM_ITEM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_SET_PASSWORD_STATUS {
    pub NodeId: u32,
    pub SetAttempted: super::super::Foundation::BOOLEAN,
    pub ReturnStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_SET_PASSWORD_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_SET_PASSWORD_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUSTER_SET_PASSWORD_STATUS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    pub Base: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub Base2: CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    pub Base: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub Base2: CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    pub NewVolumeName: [u16; 260],
    pub NewVolumeGuid: [u16; 50],
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    pub NewVolumeName: [u16; 260],
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    pub InputType: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE,
    pub Anonymous: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    pub VolumeOffset: u64,
    pub VolumeId: [u16; 260],
    pub VolumeName: [u16; 260],
    pub VolumeGuid: [u16; 50],
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_STATE_INFO {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_STATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_STATE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub RedirectedIOReason: u64,
    pub VolumeRedirectedIOReason: u64,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_VALIDATE_CSV_FILENAME {
    pub szFileName: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_CSV_FILENAME {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_CSV_FILENAME {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_VALIDATE_CSV_FILENAME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_VALIDATE_DIRECTORY {
    pub szPath: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_DIRECTORY {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_DIRECTORY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_VALIDATE_DIRECTORY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_VALIDATE_NETNAME {
    pub szNetworkName: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_NETNAME {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_NETNAME {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_VALIDATE_NETNAME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_VALIDATE_PATH {
    pub szPath: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_PATH {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_PATH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_VALIDATE_PATH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_CHKDSK_INFO {
    pub PartitionNumber: u32,
    pub ChkdskState: u32,
    pub FileIdCount: u32,
    pub FileIdList: [u64; 1],
}
impl ::core::marker::Copy for CLUS_CHKDSK_INFO {}
impl ::core::clone::Clone for CLUS_CHKDSK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_CHKDSK_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    pub FileServerName: [u16; 16],
}
impl ::core::marker::Copy for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {}
impl ::core::clone::Clone for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    pub FileServerName: [u16; 260],
}
impl ::core::marker::Copy for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {}
impl ::core::clone::Clone for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_CSV_MAINTENANCE_MODE_INFO {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub VolumeName: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_CSV_MAINTENANCE_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_CSV_MAINTENANCE_MODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUS_CSV_MAINTENANCE_MODE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_CSV_VOLUME_INFO {
    pub VolumeOffset: u64,
    pub PartitionNumber: u32,
    pub FaultState: CLUSTER_CSV_VOLUME_FAULT_STATE,
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub szVolumeName: [u16; 50],
}
impl ::core::marker::Copy for CLUS_CSV_VOLUME_INFO {}
impl ::core::clone::Clone for CLUS_CSV_VOLUME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_CSV_VOLUME_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_CSV_VOLUME_NAME {
    pub VolumeOffset: i64,
    pub szVolumeName: [u16; 260],
    pub szRootPath: [u16; 263],
}
impl ::core::marker::Copy for CLUS_CSV_VOLUME_NAME {}
impl ::core::clone::Clone for CLUS_CSV_VOLUME_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_CSV_VOLUME_NAME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_DISK_NUMBER_INFO {
    pub DiskNumber: u32,
    pub BytesPerSector: u32,
}
impl ::core::marker::Copy for CLUS_DISK_NUMBER_INFO {}
impl ::core::clone::Clone for CLUS_DISK_NUMBER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_DISK_NUMBER_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_DNN_LEADER_STATUS {
    pub IsOnline: super::super::Foundation::BOOL,
    pub IsFileServerPresent: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_DNN_LEADER_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_DNN_LEADER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUS_DNN_LEADER_STATUS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_DNN_SODAFS_CLONE_STATUS {
    pub NodeId: u32,
    pub Status: CLUSTER_RESOURCE_STATE,
}
impl ::core::marker::Copy for CLUS_DNN_SODAFS_CLONE_STATUS {}
impl ::core::clone::Clone for CLUS_DNN_SODAFS_CLONE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_DNN_SODAFS_CLONE_STATUS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_FORCE_QUORUM_INFO {
    pub dwSize: u32,
    pub dwNodeBitMask: u32,
    pub dwMaxNumberofNodes: u32,
    pub multiszNodeList: [u16; 1],
}
impl ::core::marker::Copy for CLUS_FORCE_QUORUM_INFO {}
impl ::core::clone::Clone for CLUS_FORCE_QUORUM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_FORCE_QUORUM_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_FTSET_INFO {
    pub dwRootSignature: u32,
    pub dwFtType: u32,
}
impl ::core::marker::Copy for CLUS_FTSET_INFO {}
impl ::core::clone::Clone for CLUS_FTSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_FTSET_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_MAINTENANCE_MODE_INFO {
    pub InMaintenance: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_MAINTENANCE_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_MAINTENANCE_MODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUS_MAINTENANCE_MODE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_MAINTENANCE_MODE_INFOEX {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub MaintainenceModeType: MAINTENANCE_MODE_TYPE_ENUM,
    pub InternalState: CLUSTER_RESOURCE_STATE,
    pub Signature: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_MAINTENANCE_MODE_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_MAINTENANCE_MODE_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUS_MAINTENANCE_MODE_INFOEX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_NETNAME_IP_INFO_ENTRY {
    pub NodeId: u32,
    pub AddressSize: u32,
    pub Address: [u8; 1],
}
impl ::core::marker::Copy for CLUS_NETNAME_IP_INFO_ENTRY {}
impl ::core::clone::Clone for CLUS_NETNAME_IP_INFO_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_NETNAME_IP_INFO_ENTRY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    pub szName: [u16; 64],
    pub NumEntries: u32,
    pub IpInfo: [CLUS_NETNAME_IP_INFO_ENTRY; 1],
}
impl ::core::marker::Copy for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {}
impl ::core::clone::Clone for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_NETNAME_PWD_INFO {
    pub Flags: u32,
    pub Password: [u16; 16],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl ::core::marker::Copy for CLUS_NETNAME_PWD_INFO {}
impl ::core::clone::Clone for CLUS_NETNAME_PWD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_NETNAME_PWD_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_NETNAME_PWD_INFOEX {
    pub Flags: u32,
    pub Password: [u16; 128],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl ::core::marker::Copy for CLUS_NETNAME_PWD_INFOEX {}
impl ::core::clone::Clone for CLUS_NETNAME_PWD_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_NETNAME_PWD_INFOEX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_NETNAME_VS_TOKEN_INFO {
    pub ProcessID: u32,
    pub DesiredAccess: u32,
    pub InheritHandle: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_NETNAME_VS_TOKEN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_NETNAME_VS_TOKEN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUS_NETNAME_VS_TOKEN_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_PARTITION_INFO {
    pub dwFlags: u32,
    pub szDeviceName: [u16; 260],
    pub szVolumeLabel: [u16; 260],
    pub dwSerialNumber: u32,
    pub rgdwMaximumComponentLength: u32,
    pub dwFileSystemFlags: u32,
    pub szFileSystem: [u16; 32],
}
impl ::core::marker::Copy for CLUS_PARTITION_INFO {}
impl ::core::clone::Clone for CLUS_PARTITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_PARTITION_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_PARTITION_INFO_EX {
    pub dwFlags: u32,
    pub szDeviceName: [u16; 260],
    pub szVolumeLabel: [u16; 260],
    pub dwSerialNumber: u32,
    pub rgdwMaximumComponentLength: u32,
    pub dwFileSystemFlags: u32,
    pub szFileSystem: [u16; 32],
    pub TotalSizeInBytes: u64,
    pub FreeSizeInBytes: u64,
    pub DeviceNumber: u32,
    pub PartitionNumber: u32,
    pub VolumeGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for CLUS_PARTITION_INFO_EX {}
impl ::core::clone::Clone for CLUS_PARTITION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_PARTITION_INFO_EX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_PARTITION_INFO_EX2 {
    pub GptPartitionId: ::windows::core::GUID,
    pub szPartitionName: [u16; 260],
    pub EncryptionFlags: u32,
}
impl ::core::marker::Copy for CLUS_PARTITION_INFO_EX2 {}
impl ::core::clone::Clone for CLUS_PARTITION_INFO_EX2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_PARTITION_INFO_EX2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_PROVIDER_STATE_CHANGE_INFO {
    pub dwSize: u32,
    pub resourceState: CLUSTER_RESOURCE_STATE,
    pub szProviderId: [u16; 1],
}
impl ::core::marker::Copy for CLUS_PROVIDER_STATE_CHANGE_INFO {}
impl ::core::clone::Clone for CLUS_PROVIDER_STATE_CHANGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_PROVIDER_STATE_CHANGE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_RESOURCE_CLASS_INFO {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_RESOURCE_CLASS_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUS_RESOURCE_CLASS_INFO_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0,
    pub li: u64,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO_0 {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_RESOURCE_CLASS_INFO_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_RESOURCE_CLASS_INFO_0_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0_0,
    pub SubClass: u32,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO_0_0 {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_RESOURCE_CLASS_INFO_0_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    pub dw: u32,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO_0_0_0 {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_SCSI_ADDRESS {
    pub Anonymous: CLUS_SCSI_ADDRESS_0,
}
impl ::core::marker::Copy for CLUS_SCSI_ADDRESS {}
impl ::core::clone::Clone for CLUS_SCSI_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_SCSI_ADDRESS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUS_SCSI_ADDRESS_0 {
    pub Anonymous: CLUS_SCSI_ADDRESS_0_0,
    pub dw: u32,
}
impl ::core::marker::Copy for CLUS_SCSI_ADDRESS_0 {}
impl ::core::clone::Clone for CLUS_SCSI_ADDRESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_SCSI_ADDRESS_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_SCSI_ADDRESS_0_0 {
    pub PortNumber: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
impl ::core::marker::Copy for CLUS_SCSI_ADDRESS_0_0 {}
impl ::core::clone::Clone for CLUS_SCSI_ADDRESS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_SCSI_ADDRESS_0_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_SET_MAINTENANCE_MODE_INPUT {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub ExtraParameterSize: u32,
    pub ExtraParameter: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_SET_MAINTENANCE_MODE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_SET_MAINTENANCE_MODE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUS_SET_MAINTENANCE_MODE_INPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_SHARED_VOLUME_BACKUP_MODE {
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub DelayTimerInSecs: u32,
    pub VolumeName: [u16; 260],
}
impl ::core::marker::Copy for CLUS_SHARED_VOLUME_BACKUP_MODE {}
impl ::core::clone::Clone for CLUS_SHARED_VOLUME_BACKUP_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_SHARED_VOLUME_BACKUP_MODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_STARTING_PARAMS {
    pub dwSize: u32,
    pub bForm: super::super::Foundation::BOOL,
    pub bFirst: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_STARTING_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_STARTING_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUS_STARTING_PARAMS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    pub AvailDrivelettersMask: u32,
}
impl ::core::marker::Copy for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {}
impl ::core::clone::Clone for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_STORAGE_REMAP_DRIVELETTER {
    pub CurrentDriveLetterMask: u32,
    pub TargetDriveLetterMask: u32,
}
impl ::core::marker::Copy for CLUS_STORAGE_REMAP_DRIVELETTER {}
impl ::core::clone::Clone for CLUS_STORAGE_REMAP_DRIVELETTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_STORAGE_REMAP_DRIVELETTER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_STORAGE_SET_DRIVELETTER {
    pub PartitionNumber: u32,
    pub DriveLetterMask: u32,
}
impl ::core::marker::Copy for CLUS_STORAGE_SET_DRIVELETTER {}
impl ::core::clone::Clone for CLUS_STORAGE_SET_DRIVELETTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLUS_STORAGE_SET_DRIVELETTER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_WORKER {
    pub hThread: super::super::Foundation::HANDLE,
    pub Terminate: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_WORKER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_WORKER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLUS_WORKER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_CLUSTER_CONFIG {
    pub dwVersion: u32,
    pub lpszClusterName: ::windows::core::PCWSTR,
    pub cNodes: u32,
    pub ppszNodeNames: *mut ::windows::core::PWSTR,
    pub cIpEntries: u32,
    pub pIpEntries: *mut CLUSTER_IP_ENTRY,
    pub fEmptyCluster: super::super::Foundation::BOOLEAN,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_CLUSTER_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_CLUSTER_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CREATE_CLUSTER_CONFIG {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_CLUSTER_NAME_ACCOUNT {
    pub dwVersion: u32,
    pub lpszClusterName: ::windows::core::PCWSTR,
    pub dwFlags: u32,
    pub pszUserName: ::windows::core::PCWSTR,
    pub pszPassword: ::windows::core::PCWSTR,
    pub pszDomain: ::windows::core::PCWSTR,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
    pub bUpgradeVCOs: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_CLUSTER_NAME_ACCOUNT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_CLUSTER_NAME_ACCOUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CREATE_CLUSTER_NAME_ACCOUNT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct FILESHARE_CHANGE {
    pub Change: FILESHARE_CHANGE_ENUM,
    pub ShareName: [u16; 84],
}
impl ::core::marker::Copy for FILESHARE_CHANGE {}
impl ::core::clone::Clone for FILESHARE_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FILESHARE_CHANGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct FILESHARE_CHANGE_LIST {
    pub NumEntries: u32,
    pub ChangeEntry: [FILESHARE_CHANGE; 1],
}
impl ::core::marker::Copy for FILESHARE_CHANGE_LIST {}
impl ::core::clone::Clone for FILESHARE_CHANGE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FILESHARE_CHANGE_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct GET_OPERATION_CONTEXT_PARAMS {
    pub Size: u32,
    pub Version: u32,
    pub Type: RESDLL_CONTEXT_OPERATION_TYPE,
    pub Priority: u32,
}
impl ::core::marker::Copy for GET_OPERATION_CONTEXT_PARAMS {}
impl ::core::clone::Clone for GET_OPERATION_CONTEXT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GET_OPERATION_CONTEXT_PARAMS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct GROUP_FAILURE_INFO {
    pub dwFailoverAttemptsRemaining: u32,
    pub dwFailoverPeriodRemaining: u32,
}
impl ::core::marker::Copy for GROUP_FAILURE_INFO {}
impl ::core::clone::Clone for GROUP_FAILURE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GROUP_FAILURE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct GROUP_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: GROUP_FAILURE_INFO,
}
impl ::core::marker::Copy for GROUP_FAILURE_INFO_BUFFER {}
impl ::core::clone::Clone for GROUP_FAILURE_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GROUP_FAILURE_INFO_BUFFER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONITOR_STATE {
    pub LastUpdate: i64,
    pub State: RESOURCE_MONITOR_STATE,
    pub ActiveResource: super::super::Foundation::HANDLE,
    pub ResmonStop: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONITOR_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONITOR_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONITOR_STATE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct NOTIFY_FILTER_AND_TYPE {
    pub dwObjectType: u32,
    pub FilterFlags: i64,
}
impl ::core::marker::Copy for NOTIFY_FILTER_AND_TYPE {}
impl ::core::clone::Clone for NOTIFY_FILTER_AND_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NOTIFY_FILTER_AND_TYPE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct NodeUtilizationInfoElement {
    pub Id: u64,
    pub AvailableMemory: u64,
    pub AvailableMemoryAfterReclamation: u64,
}
impl ::core::marker::Copy for NodeUtilizationInfoElement {}
impl ::core::clone::Clone for NodeUtilizationInfoElement {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NodeUtilizationInfoElement {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct POST_UPGRADE_VERSION_INFO {
    pub newMajorVersion: u32,
    pub newUpgradeVersion: u32,
    pub oldMajorVersion: u32,
    pub oldUpgradeVersion: u32,
    pub reserved: u32,
}
impl ::core::marker::Copy for POST_UPGRADE_VERSION_INFO {}
impl ::core::clone::Clone for POST_UPGRADE_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POST_UPGRADE_VERSION_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct PaxosTagCStruct {
    pub __padding__PaxosTagVtable: u64,
    pub __padding__NextEpochVtable: u64,
    pub __padding__NextEpoch_DateTimeVtable: u64,
    pub NextEpoch_DateTime_ticks: u64,
    pub NextEpoch_Value: i32,
    pub __padding__BoundryNextEpoch: u32,
    pub __padding__EpochVtable: u64,
    pub __padding__Epoch_DateTimeVtable: u64,
    pub Epoch_DateTime_ticks: u64,
    pub Epoch_Value: i32,
    pub __padding__BoundryEpoch: u32,
    pub Sequence: i32,
    pub __padding__BoundrySequence: u32,
}
impl ::core::marker::Copy for PaxosTagCStruct {}
impl ::core::clone::Clone for PaxosTagCStruct {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PaxosTagCStruct {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct RESOURCE_FAILURE_INFO {
    pub dwRestartAttemptsRemaining: u32,
    pub dwRestartPeriodRemaining: u32,
}
impl ::core::marker::Copy for RESOURCE_FAILURE_INFO {}
impl ::core::clone::Clone for RESOURCE_FAILURE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RESOURCE_FAILURE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct RESOURCE_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: RESOURCE_FAILURE_INFO,
}
impl ::core::marker::Copy for RESOURCE_FAILURE_INFO_BUFFER {}
impl ::core::clone::Clone for RESOURCE_FAILURE_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RESOURCE_FAILURE_INFO_BUFFER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESOURCE_STATUS {
    pub ResourceState: CLUSTER_RESOURCE_STATE,
    pub CheckPoint: u32,
    pub WaitHint: u32,
    pub EventHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESOURCE_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESOURCE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESOURCE_STATUS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESOURCE_STATUS_EX {
    pub ResourceState: CLUSTER_RESOURCE_STATE,
    pub CheckPoint: u32,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub ApplicationSpecificErrorCode: u32,
    pub Flags: u32,
    pub WaitHint: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESOURCE_STATUS_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESOURCE_STATUS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESOURCE_STATUS_EX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    pub isTerminalFailure: super::super::Foundation::BOOL,
    pub restartPeriodRemaining: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESUTIL_FILETIME_DATA {
    pub Default: super::super::Foundation::FILETIME,
    pub Minimum: super::super::Foundation::FILETIME,
    pub Maximum: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESUTIL_FILETIME_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESUTIL_FILETIME_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESUTIL_FILETIME_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct RESUTIL_LARGEINT_DATA {
    pub Default: i64,
    pub Minimum: i64,
    pub Maximum: i64,
}
impl ::core::marker::Copy for RESUTIL_LARGEINT_DATA {}
impl ::core::clone::Clone for RESUTIL_LARGEINT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RESUTIL_LARGEINT_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESUTIL_PROPERTY_ITEM {
    pub Name: ::windows::core::PWSTR,
    pub KeyName: ::windows::core::PWSTR,
    pub Format: u32,
    pub Anonymous: RESUTIL_PROPERTY_ITEM_0,
    pub Minimum: u32,
    pub Maximum: u32,
    pub Flags: u32,
    pub Offset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESUTIL_PROPERTY_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESUTIL_PROPERTY_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESUTIL_PROPERTY_ITEM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union RESUTIL_PROPERTY_ITEM_0 {
    pub DefaultPtr: usize,
    pub Default: u32,
    pub lpDefault: *mut ::core::ffi::c_void,
    pub LargeIntData: *mut RESUTIL_LARGEINT_DATA,
    pub ULargeIntData: *mut RESUTIL_ULARGEINT_DATA,
    pub FileTimeData: *mut RESUTIL_FILETIME_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESUTIL_PROPERTY_ITEM_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESUTIL_PROPERTY_ITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESUTIL_PROPERTY_ITEM_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct RESUTIL_ULARGEINT_DATA {
    pub Default: u64,
    pub Minimum: u64,
    pub Maximum: u64,
}
impl ::core::marker::Copy for RESUTIL_ULARGEINT_DATA {}
impl ::core::clone::Clone for RESUTIL_ULARGEINT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RESUTIL_ULARGEINT_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct ResourceUtilizationInfoElement {
    pub PhysicalNumaId: u64,
    pub CurrentMemory: u64,
}
impl ::core::marker::Copy for ResourceUtilizationInfoElement {}
impl ::core::clone::Clone for ResourceUtilizationInfoElement {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ResourceUtilizationInfoElement {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    pub ReplicationGroupName: [u16; 260],
    pub Description: [u16; 260],
    pub LogPath: [u16; 260],
    pub MaxLogSizeInBytes: u64,
    pub LogType: u16,
    pub ReplicationMode: u32,
    pub MinimumPartnersInSync: u32,
    pub EnableWriteConsistency: super::super::Foundation::BOOLEAN,
    pub EnableEncryption: super::super::Foundation::BOOLEAN,
    pub CertificateThumbprint: [u16; 260],
    pub VolumeNameCount: u32,
    pub VolumeNames: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    pub Result: u32,
    pub ErrorString: [u16; 260],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_DISK_INFO {
    pub Reason: SR_DISK_REPLICATION_ELIGIBLE,
    pub DiskGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_DISK_INFO {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_DISK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SR_RESOURCE_TYPE_DISK_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    pub Count: u16,
    pub DiskInfo: [SR_RESOURCE_TYPE_DISK_INFO; 1],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    pub DataDiskGuid: ::windows::core::GUID,
    pub IncludeOfflineDisks: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    pub DataDiskGuid: ::windows::core::GUID,
    pub IncludeAvailableStoargeDisks: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    pub SourceDataDiskGuid: ::windows::core::GUID,
    pub TargetReplicationGroupGuid: ::windows::core::GUID,
    pub SkipConnectivityCheck: super::super::Foundation::BOOLEAN,
    pub IncludeOfflineDisks: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISK {
    pub Type: SR_REPLICATED_DISK_TYPE,
    pub ClusterDiskResourceGuid: ::windows::core::GUID,
    pub ReplicationGroupId: ::windows::core::GUID,
    pub ReplicationGroupName: [u16; 260],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_DISK {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_DISK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SR_RESOURCE_TYPE_REPLICATED_DISK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    pub Count: u16,
    pub ReplicatedDisks: [SR_RESOURCE_TYPE_REPLICATED_DISK; 1],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    pub Count: u32,
    pub PartitionArray: [SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO; 1],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    pub PartitionOffset: u64,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct WitnessTagHelper {
    pub Version: i32,
    pub paxosToValidate: PaxosTagCStruct,
}
impl ::core::marker::Copy for WitnessTagHelper {}
impl ::core::clone::Clone for WitnessTagHelper {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WitnessTagHelper {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct WitnessTagUpdateHelper {
    pub Version: i32,
    pub paxosToSet: PaxosTagCStruct,
    pub paxosToValidate: PaxosTagCStruct,
}
impl ::core::marker::Copy for WitnessTagUpdateHelper {}
impl ::core::clone::Clone for WitnessTagUpdateHelper {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WitnessTagUpdateHelper {
    type Abi = Self;
}
#[repr(C)]
pub struct _HCHANGE(pub u8);
#[repr(C)]
pub struct _HCLUSCRYPTPROVIDER(pub u8);
#[repr(C)]
pub struct _HCLUSENUM(pub u8);
#[repr(C)]
pub struct _HCLUSENUMEX(pub u8);
#[repr(C)]
pub struct _HCLUSTER(pub u8);
#[repr(C)]
pub struct _HGROUP(pub u8);
#[repr(C)]
pub struct _HGROUPENUM(pub u8);
#[repr(C)]
pub struct _HGROUPENUMEX(pub u8);
#[repr(C)]
pub struct _HGROUPSET(pub u8);
#[repr(C)]
pub struct _HGROUPSETENUM(pub u8);
#[repr(C)]
pub struct _HNETINTERFACE(pub u8);
#[repr(C)]
pub struct _HNETINTERFACEENUM(pub u8);
#[repr(C)]
pub struct _HNETWORK(pub u8);
#[repr(C)]
pub struct _HNETWORKENUM(pub u8);
#[repr(C)]
pub struct _HNODE(pub u8);
#[repr(C)]
pub struct _HNODEENUM(pub u8);
#[repr(C)]
pub struct _HNODEENUMEX(pub u8);
#[repr(C)]
pub struct _HREGBATCH(pub u8);
#[repr(C)]
pub struct _HREGBATCHNOTIFICATION(pub u8);
#[repr(C)]
pub struct _HREGBATCHPORT(pub u8);
#[repr(C)]
pub struct _HREGREADBATCH(pub u8);
#[repr(C)]
pub struct _HREGREADBATCHREPLY(pub u8);
#[repr(C)]
pub struct _HRESENUM(pub u8);
#[repr(C)]
pub struct _HRESENUMEX(pub u8);
#[repr(C)]
pub struct _HRESOURCE(pub u8);
#[repr(C)]
pub struct _HRESTYPEENUM(pub u8);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type LPGROUP_CALLBACK_EX = ::core::option::Option<unsafe extern "system" fn(param0: *mut _HCLUSTER, param1: *mut _HGROUP, param2: *mut _HGROUP, param3: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type LPNODE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut _HCLUSTER, param1: *mut _HNODE, param2: CLUSTER_NODE_STATE, param3: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type LPRESOURCE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut _HRESOURCE, param1: *mut _HRESOURCE, param2: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type LPRESOURCE_CALLBACK_EX = ::core::option::Option<unsafe extern "system" fn(param0: *mut _HCLUSTER, param1: *mut _HRESOURCE, param2: *mut _HRESOURCE, param3: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PARBITRATE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, lostquorumresource: PQUORUM_RESOURCE_LOST) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESCALL_AS_USER_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, tokenhandle: super::super::Foundation::HANDLE, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESCALL_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESTYPECALL_AS_USER_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcetypename: ::windows::core::PCWSTR, tokenhandle: super::super::Foundation::HANDLE, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESTYPECALL_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcetypename: ::windows::core::PCWSTR, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCANCEL_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, cancelflags_reserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCHANGE_RESOURCE_PROCESS_FOR_DUMPS = ::core::option::Option<unsafe extern "system" fn(resource: isize, processname: ::windows::core::PCWSTR, processid: u32, isadd: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS = ::core::option::Option<unsafe extern "system" fn(resourcetypename: ::windows::core::PCWSTR, processname: ::windows::core::PCWSTR, processid: u32, isadd: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLOSE_CLUSTER_CRYPT_PROVIDER = ::core::option::Option<unsafe extern "system" fn(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLOSE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPIClusWorkerCheckTerminate = ::core::option::Option<unsafe extern "system" fn(lpworker: *mut CLUS_WORKER) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hdependentgroup: *const _HGROUP, hprovidergroup: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, hprovidergroupset: *const _HGROUPSET) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hdependentgroup: *const _HGROUP, hprovidergroupset: *const _HGROUPSET) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_ADD_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: ::windows::core::PCWSTR, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_ADD_CLUSTER_NODE_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: ::windows::core::PCWSTR, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hdependson: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hnode: *mut _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CROSS_CLUSTER_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: ::windows::core::PCWSTR, lpremotegroupsetname: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_RESOURCE_TO_CLUSTER_SHARED_VOLUMES = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_BACKUP_CLUSTER_DATABASE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszpathname: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CAN_RESOURCE_BE_DEPENDENT = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hresourcedependent: *mut _HRESOURCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hgroup: *mut _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hgroup: *mut _HGROUP, flags: u64) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NETWORK = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NET_INTERFACE = ::core::option::Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NOTIFY_PORT = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_AFFINITY_RULE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: ::windows::core::PCWSTR, hgroup: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_AFFINITY_RULE_CONTROL = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, affinityrulename: ::windows::core::PCWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(henum: *const _HCLUSENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_CONTROL = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_CREATE_AFFINITY_RULE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: ::windows::core::PCWSTR, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_ENUM = ::core::option::Option<unsafe extern "system" fn(henum: *const _HCLUSENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(henum: *const _HCLUSENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT_EX = ::core::option::Option<unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(hgroupenum: *mut _HGROUPENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_CONTROL = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_ENUM = ::core::option::Option<unsafe extern "system" fn(hgroupenum: *const _HGROUPENUM, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(hgroupenum: *const _HGROUPENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT_EX = ::core::option::Option<unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, dwtype: u32) -> *mut _HGROUPENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszproperties: ::windows::core::PCWSTR, cbproperties: u32, lpszroproperties: ::windows::core::PCWSTR, cbroproperties: u32, dwflags: u32) -> *mut _HGROUPENUMEX>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NETWORK_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NETWORK_CONTROL = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NETWORK_ENUM = ::core::option::Option<unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NETWORK_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NETWORK_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, dwtype: u32) -> *mut _HNETWORKENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL = ::core::option::Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_CONTROL = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_ENUM = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT_EX = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, dwtype: u32) -> *mut _HNODEENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, dwtype: u32, poptions: *const ::core::ffi::c_void) -> *mut _HNODEENUMEX>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, dwtype: u32) -> *mut _HCLUSENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_OPEN_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, dwtype: u32, poptions: *const ::core::ffi::c_void) -> *mut _HCLUSENUMEX>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_CLOSE_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_CREATE_BATCH = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phregbatch: *mut *mut _HREGBATCH) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_CREATE_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: ::windows::core::PCWSTR, dwoptions: u32, samdesired: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: ::windows::core::PCWSTR) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_VALUE = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_ENUM_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_ENUM_VALUE = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszvaluename: ::windows::core::PWSTR, lpcchvaluename: *mut u32, lpdwtype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_GET_KEY_SECURITY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, requestedinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_OPEN_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: ::windows::core::PCWSTR, samdesired: u32, phkresult: *mut super::super::System::Registry::HKEY) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_QUERY_INFO_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_QUERY_VALUE = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: ::windows::core::PCWSTR, lpdwvaluetype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_SET_VALUE = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: ::windows::core::PCWSTR, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_REG_SYNC_DATABASE = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, flags: u32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_REMOVE_AFFINITY_RULE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_AFFINITY_RULE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: ::windows::core::PCWSTR, hgroup: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hgroupname: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(hresenum: *mut _HRESENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM = ::core::option::Option<unsafe extern "system" fn(hresenum: *const _HRESENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(hresenum: *const _HRESENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT_EX = ::core::option::Option<unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, dwtype: u32) -> *mut _HRESENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszproperties: ::windows::core::PCWSTR, cbproperties: u32, lpszroproperties: ::windows::core::PCWSTR, cbroproperties: u32, dwflags: u32) -> *mut _HRESENUMEX>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: ::windows::core::PCWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_ENUM = ::core::option::Option<unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: ::windows::core::PCWSTR, dwtype: u32) -> *mut _HRESTYPEENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_UPGRADE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, perform: super::super::Foundation::BOOL, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUS_WORKER_CREATE = ::core::option::Option<unsafe extern "system" fn(lpworker: *mut CLUS_WORKER, lpstartaddress: PWORKER_START_ROUTINE, lpparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUS_WORKER_TERMINATE = ::core::option::Option<unsafe extern "system" fn(lpworker: *const CLUS_WORKER) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER = ::core::option::Option<unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_AVAILABILITY_SET = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpavailabilitysetname: ::windows::core::PCWSTR, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> *mut _HGROUPSET>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_CNOLESS = ::core::option::Option<unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: ::windows::core::PCWSTR) -> *mut _HGROUP>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_GROUPEX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: ::windows::core::PCWSTR, pgroupinfo: *const CLUSTER_CREATE_GROUP_INFO) -> *mut _HGROUP>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupsetname: ::windows::core::PCWSTR) -> *mut _HGROUPSET>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_NAME_ACCOUNT = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, dwfilter: u32, dwnotifykey: usize) -> *mut _HCHANGE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT_V2 = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> *mut _HCHANGE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, lpszresourcename: ::windows::core::PCWSTR, lpszresourcetype: ::windows::core::PCWSTR, dwflags: u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: ::windows::core::PCWSTR, lpszdisplayname: ::windows::core::PCWSTR, lpszresourcetypedll: ::windows::core::PCWSTR, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_DELETE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, lpszresourcetypename: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_DESTROY_CLUSTER = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void, fdeletevirtualcomputerobjects: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_DESTROY_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_EVICT_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_EVICT_CLUSTER_NODE_EX = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, dwtimeout: u32, phrcleanupstatus: *mut ::windows::core::HRESULT) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_FAIL_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_NETWORK = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_NET_INTERFACE = ::core::option::Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_NODE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_GROUP_KEY = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_GROUP_STATE = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, lpsznodename: ::windows::core::PWSTR, lpcchnodename: *mut u32) -> CLUSTER_GROUP_STATE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_INFORMATION = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszclustername: ::windows::core::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: *mut CLUSTERVERSIONINFO) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_KEY = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NETWORK_ID = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, lpsznetworkid: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NETWORK_KEY = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NETWORK_STATE = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> CLUSTER_NETWORK_STATE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: ::windows::core::PCWSTR, lpsznetworkname: ::windows::core::PCWSTR, lpszinterfacename: ::windows::core::PWSTR, lpcchinterfacename: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_KEY = ::core::option::Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_STATE = ::core::option::Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NODE_ID = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, lpsznodeid: ::windows::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NODE_KEY = ::core::option::Option<unsafe extern "system" fn(hnode: *mut _HNODE, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NODE_STATE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> CLUSTER_NODE_STATE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NOTIFY = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NOTIFY_V2 = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, pfilterandtype: *mut NOTIFY_FILTER_AND_TYPE, buffer: *mut u8, lpcchbuffersize: *mut u32, lpszobjectid: ::windows::core::PWSTR, lpcchobjectid: *mut u32, lpszparentid: ::windows::core::PWSTR, lpcchparentid: *mut u32, lpszname: ::windows::core::PWSTR, lpcchname: *mut u32, lpsztype: ::windows::core::PWSTR, lpcchtype: *mut u32, dwmilliseconds: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_QUORUM_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcename: ::windows::core::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: ::windows::core::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdependencyexpression: ::windows::core::PWSTR, lpcchdependencyexpression: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_KEY = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_NETWORK_NAME = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpbuffer: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_STATE = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpsznodename: ::windows::core::PWSTR, lpcchnodename: *mut u32, lpszgroupname: ::windows::core::PWSTR, lpcchgroupname: *mut u32) -> CLUSTER_RESOURCE_STATE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_TYPE_KEY = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsztypename: ::windows::core::PCWSTR, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_NODE_CLUSTER_STATE = ::core::option::Option<unsafe extern "system" fn(lpsznodename: ::windows::core::PCWSTR, pdwclusterstate: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_NOTIFY_EVENT_HANDLE_V2 = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, lphtargetevent: *mut super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_IS_FILE_ON_CLUSTER_SHARED_VOLUME = ::core::option::Option<unsafe extern "system" fn(lpszpathname: ::windows::core::PCWSTR, pbfileisonsharedvolume: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_MOVE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OFFLINE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OFFLINE_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ONLINE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ONLINE_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER = ::core::option::Option<unsafe extern "system" fn(lpszclustername: ::windows::core::PCWSTR) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_EX = ::core::option::Option<unsafe extern "system" fn(lpszclustername: ::windows::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: ::windows::core::PCWSTR) -> *mut _HGROUP>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: ::windows::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HGROUP>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupsetname: ::windows::core::PCWSTR) -> *mut _HGROUPSET>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NETINTERFACE_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetinterfacename: ::windows::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNETINTERFACE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetworkname: ::windows::core::PCWSTR) -> *mut _HNETWORK>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetworkname: ::windows::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNETWORK>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NET_INTERFACE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszinterfacename: ::windows::core::PCWSTR) -> *mut _HNETINTERFACE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: ::windows::core::PCWSTR) -> *mut _HNODE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NODE_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: ::windows::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNODE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, lpszresourcename: ::windows::core::PCWSTR) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcename: ::windows::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_NODE_BY_ID = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, nodeid: u32) -> *mut _HNODE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_PAUSE_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_PAUSE_CLUSTER_NODE_EX = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, bdrainnode: super::super::Foundation::BOOL, dwpauseflags: u32, hnodedraintarget: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, dwfiltertype: u32, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY_V2 = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, filter: NOTIFY_FILTER_AND_TYPE, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdependson: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hdependson: *const _HGROUPSET) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdependson: *const _HGROUPSET) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_NAME_ACCOUNT = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hdependson: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hnode: *mut _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CROSS_CLUSTER_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: ::windows::core::PCWSTR, lpremotegroupsetname: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_RESOURCE_FROM_CLUSTER_SHARED_VOLUMES = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_RESTART_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, dwflags: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_RESTORE_CLUSTER_DATABASE = ::core::option::Option<unsafe extern "system" fn(lpszpathname: ::windows::core::PCWSTR, bforce: super::super::Foundation::BOOL, lpszquorumdriveletter: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_RESUME_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_RESUME_CLUSTER_NODE_EX = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, lpszdependencyexpression: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_GROUP_NAME = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, lpszgroupname: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, nodecount: u32, nodelist: *const *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_NETWORK_NAME = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, lpszname: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_NETWORK_PRIORITY_ORDER = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, networkcount: u32, networklist: *const *const _HNETWORK) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdevicename: ::windows::core::PCWSTR, dwmaxquologsize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdependencyexpression: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_NAME = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, lpszresourcename: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SET_CLUSTER_SERVICE_ACCOUNT_PASSWORD = ::core::option::Option<unsafe extern "system" fn(lpszclustername: ::windows::core::PCWSTR, lpsznewpassword: ::windows::core::PCWSTR, dwflags: u32, lpreturnstatusbuffer: *mut CLUSTER_SET_PASSWORD_STATUS, lpcbreturnstatusbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUP, lpszdependencyexpression: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SHARED_VOLUME_SET_SNAPSHOT_STATE = ::core::option::Option<unsafe extern "system" fn(guidsnapshotset: ::windows::core::GUID, lpszvolumename: ::windows::core::PCWSTR, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SetClusterName = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznewclustername: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_CLEAR_BACKUP_STATE_FOR_SHARED_VOLUME = ::core::option::Option<unsafe extern "system" fn(lpszvolumepathname: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_DECRYPT = ::core::option::Option<unsafe extern "system" fn(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pcryptinput: *const u8, cbcryptinput: u32, ppcryptoutput: *mut *mut u8, pcbcryptoutput: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_ENCRYPT = ::core::option::Option<unsafe extern "system" fn(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pdata: *const u8, cbdata: u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT = ::core::option::Option<unsafe extern "system" fn(lpszvolumemountpoint: ::windows::core::PCWSTR, lpszvolumename: ::windows::core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_GET_VOLUME_PATH_NAME = ::core::option::Option<unsafe extern "system" fn(lpszfilename: ::windows::core::PCWSTR, lpszvolumepathname: ::windows::core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_IS_PATH_ON_SHARED_VOLUME = ::core::option::Option<unsafe extern "system" fn(lpszpathname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_PREPARE_SHARED_VOLUME_FOR_BACKUP = ::core::option::Option<unsafe extern "system" fn(lpszfilename: ::windows::core::PCWSTR, lpszvolumepathname: ::windows::core::PWSTR, lpcchvolumepathname: *mut u32, lpszvolumename: ::windows::core::PWSTR, lpcchvolumename: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_BATCH_ADD_COMMAND = ::core::option::Option<unsafe extern "system" fn(hregbatch: *const _HREGBATCH, dwcommand: CLUSTER_REG_COMMAND, wzname: ::windows::core::PCWSTR, dwoptions: u32, lpdata: *const ::core::ffi::c_void, cbdata: u32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_BATCH_CLOSE_NOTIFICATION = ::core::option::Option<unsafe extern "system" fn(hbatchnotification: *const _HREGBATCHNOTIFICATION) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_BATCH_READ_COMMAND = ::core::option::Option<unsafe extern "system" fn(hbatchnotification: *const _HREGBATCHNOTIFICATION, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_REG_CLOSE_BATCH = ::core::option::Option<unsafe extern "system" fn(hregbatch: *const _HREGBATCH, bcommit: super::super::Foundation::BOOL, failedcommandnumber: *mut i32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_CLOSE_BATCH_NOTIFY_PORT = ::core::option::Option<unsafe extern "system" fn(hbatchnotifyport: *const _HREGBATCHPORT) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_CLOSE_READ_BATCH = ::core::option::Option<unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_CLOSE_READ_BATCH_EX = ::core::option::Option<unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, flags: u32, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_CLOSE_READ_BATCH_REPLY = ::core::option::Option<unsafe extern "system" fn(hregreadbatchreply: *const _HREGREADBATCHREPLY) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSTER_REG_CREATE_BATCH_NOTIFY_PORT = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phbatchnotifyport: *mut *mut _HREGBATCHPORT) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSTER_REG_CREATE_READ_BATCH = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phregreadbatch: *mut *mut _HREGREADBATCH) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_GET_BATCH_NOTIFICATION = ::core::option::Option<unsafe extern "system" fn(hbatchnotify: *const _HREGBATCHPORT, phbatchnotification: *mut *mut _HREGBATCHNOTIFICATION) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_READ_BATCH_ADD_COMMAND = ::core::option::Option<unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, wzsubkeyname: ::windows::core::PCWSTR, wzvaluename: ::windows::core::PCWSTR) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_READ_BATCH_REPLY_NEXT_COMMAND = ::core::option::Option<unsafe extern "system" fn(hregreadbatchreply: *const _HREGREADBATCHREPLY, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_SETUP_PROGRESS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pvcallbackarg: *mut ::core::ffi::c_void, esetupphase: CLUSTER_SETUP_PHASE, ephasetype: CLUSTER_SETUP_PHASE_TYPE, ephaseseverity: CLUSTER_SETUP_PHASE_SEVERITY, dwpercentcomplete: u32, lpszobjectname: ::windows::core::PCWSTR, dwstatus: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_SET_ACCOUNT_ACCESS = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, szaccountsid: ::windows::core::PCWSTR, dwaccess: u32, dwcontroltype: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_UPGRADE_PROGRESS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pvcallbackarg: *mut ::core::ffi::c_void, eupgradephase: CLUSTER_UPGRADE_PHASE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PEND_CONTROL_CALL = ::core::option::Option<unsafe extern "system" fn(context: i64, status: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PEND_TYPE_CONTROL_CALL = ::core::option::Option<unsafe extern "system" fn(context: i64, status: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PEXTEND_RES_CONTROL_CALL = ::core::option::Option<unsafe extern "system" fn(context: i64, newtimeoutinms: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PEXTEND_RES_TYPE_CONTROL_CALL = ::core::option::Option<unsafe extern "system" fn(context: i64, newtimeoutinms: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PFREE_CLUSTER_CRYPT = ::core::option::Option<unsafe extern "system" fn(pcryptinfo: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PIS_ALIVE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PLOG_EVENT_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, loglevel: LOG_LEVEL, formatstring: ::windows::core::PCWSTR) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLOOKS_ALIVE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type POFFLINE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type POFFLINE_V2_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *const ::core::ffi::c_void, destinationnodename: ::windows::core::PCWSTR, offlineflags: u32, inbuffer: *const u8, inbuffersize: u32, reserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PONLINE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PONLINE_V2_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *const ::core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE, onlineflags: u32, inbuffer: *const u8, inbuffersize: u32, reserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type POPEN_CLUSTER_CRYPT_PROVIDER = ::core::option::Option<unsafe extern "system" fn(lpszresource: ::windows::core::PCWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type POPEN_CLUSTER_CRYPT_PROVIDEREX = ::core::option::Option<unsafe extern "system" fn(lpszresource: ::windows::core::PCWSTR, lpszkeyname: ::windows::core::PCWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type POPEN_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcename: ::windows::core::PCWSTR, resourcekey: super::super::System::Registry::HKEY, resourcehandle: isize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type POPEN_V2_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcename: ::windows::core::PCWSTR, resourcekey: super::super::System::Registry::HKEY, resourcehandle: isize, openflags: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PQUERY_APPINSTANCE_VERSION = ::core::option::Option<unsafe extern "system" fn(appinstanceid: *const ::windows::core::GUID, instanceversionhigh: *mut u64, instanceversionlow: *mut u64, versionstatus: *mut super::super::Foundation::NTSTATUS) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PQUORUM_RESOURCE_LOST = ::core::option::Option<unsafe extern "system" fn(resource: isize) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRAISE_RES_TYPE_NOTIFICATION = ::core::option::Option<unsafe extern "system" fn(resourcetype: ::windows::core::PCWSTR, ppayload: *const u8, payloadsize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PREGISTER_APPINSTANCE = ::core::option::Option<unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, appinstanceid: *const ::windows::core::GUID, childreninheritappinstance: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PREGISTER_APPINSTANCE_VERSION = ::core::option::Option<unsafe extern "system" fn(appinstanceid: *const ::windows::core::GUID, instanceversionhigh: u64, instanceversionlow: u64) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRELEASE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PREQUEST_DUMP_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, dumpduetocallinprogress: super::super::Foundation::BOOL, dumpdelayinms: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESET_ALL_APPINSTANCE_VERSIONS = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESOURCE_CONTROL_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESOURCE_TYPE_CONTROL_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcetypename: ::windows::core::PCWSTR, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_ADD_UNKNOWN_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_CREATE_DIRECTORY_TREE = ::core::option::Option<unsafe extern "system" fn(pszpath: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_DUP_PARAMETER_BLOCK = ::core::option::Option<unsafe extern "system" fn(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_DUP_STRING = ::core::option::Option<unsafe extern "system" fn(pszinstring: ::windows::core::PCWSTR) -> ::windows::core::PWSTR>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_ENUM_PRIVATE_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszoutproperties: ::windows::core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_ENUM_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, pszoutproperties: ::windows::core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_ENUM_RESOURCES = ::core::option::Option<unsafe extern "system" fn(hself: *mut _HRESOURCE, lpszrestypename: ::windows::core::PCWSTR, prescallback: LPRESOURCE_CALLBACK, pparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_ENUM_RESOURCES_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: ::windows::core::PCWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_ENUM_RESOURCES_EX2 = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: ::windows::core::PCWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void, dwdesiredaccess: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_EXPAND_ENVIRONMENT_STRINGS = ::core::option::Option<unsafe extern "system" fn(pszsrc: ::windows::core::PCWSTR) -> ::windows::core::PWSTR>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_BINARY_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows::core::PCWSTR, pbpropertyvalue: *mut *mut u8, pcbpropertyvaluesize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_DEPENDENT_DISK_RESOURCE_DRIVE_LETTER = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, hresource: *const _HRESOURCE, pszdriveletter: ::windows::core::PWSTR, pcchdriveletter: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_DWORD_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows::core::PCWSTR, pdwpropertyvalue: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_EXPANDED_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows::core::PCWSTR, pszpropertyvalue: *mut ::windows::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_EXPAND_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows::core::PCWSTR, pszpropertyvalue: *mut ::windows::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_FILETIME_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows::core::PCWSTR, pftpropertyvalue: *mut super::super::Foundation::FILETIME) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_LONG_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows::core::PCWSTR, plpropertyvalue: *mut i32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_MULTI_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows::core::PCWSTR, pszpropertyvalue: *mut ::windows::core::PWSTR, pcbpropertyvaluesize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows::core::PCWSTR, pszpropertyvalue: *mut ::windows::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_ULARGEINTEGER_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows::core::PCWSTR, plpropertyvalue: *mut u64) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FREE_ENVIRONMENT = ::core::option::Option<unsafe extern "system" fn(lpenvironment: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FREE_PARAMETER_BLOCK = ::core::option::Option<unsafe extern "system" fn(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_ALL_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_BINARY_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_BINARY, pboldvalue: *const u8, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_BINARY_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows::core::PCWSTR, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_CORE_CLUSTER_RESOURCES = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, phclusternameresource: *mut *mut _HRESOURCE, phclusteripaddressresource: *mut *mut _HRESOURCE, phclusterquorumresource: *mut *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_CORE_CLUSTER_RESOURCES_EX = ::core::option::Option<unsafe extern "system" fn(hclusterin: *const _HCLUSTER, phclusternameresourceout: *mut *mut _HRESOURCE, phclusteripaddressresourceout: *mut *mut _HRESOURCE, phclusterquorumresourceout: *mut *mut _HRESOURCE, dwdesiredaccess: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_DWORD_PROPERTY = ::core::option::Option<unsafe extern "system" fn(pdwoutvalue: *mut u32, pvaluestruct: *const CLUSPROP_DWORD, dwoldvalue: u32, dwminimum: u32, dwmaximum: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_DWORD_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows::core::PCWSTR, pdwoutvalue: *mut u32, dwdefaultvalue: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_ENVIRONMENT_WITH_NET_NAME = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_EXPAND_SZ_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows::core::PCWSTR, bexpand: super::super::Foundation::BOOL) -> ::windows::core::PWSTR>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_FILETIME_PROPERTY = ::core::option::Option<unsafe extern "system" fn(pftoutvalue: *mut super::super::Foundation::FILETIME, pvaluestruct: *const CLUSPROP_FILETIME, ftoldvalue: super::super::Foundation::FILETIME, ftminimum: super::super::Foundation::FILETIME, ftmaximum: super::super::Foundation::FILETIME, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_LONG_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ploutvalue: *mut i32, pvaluestruct: *const CLUSPROP_LONG, loldvalue: i32, lminimum: i32, lmaximum: i32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_MULTI_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppszoutvalue: *mut ::windows::core::PWSTR, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: ::windows::core::PCWSTR, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PRIVATE_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTIES_TO_PARAMETER_BLOCK = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutparams: *mut u8, bcheckforrequiredproperties: super::super::Foundation::BOOL, psznameofpropinerror: *mut ::windows::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTY = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, poutpropertyitem: *mut *mut ::core::ffi::c_void, pcboutpropertyitemsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_PROPERTY_FORMATS = ::core::option::Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist: *mut ::core::ffi::c_void, cbpropertyformatlistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTY_SIZE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize: *mut u32, pnpropertycount: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_QWORD_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows::core::PCWSTR, pqwoutvalue: *mut u64, qwdefaultvalue: u64) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hself: super::super::Foundation::HANDLE, lpszresourcetype: ::windows::core::PCWSTR) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: ::windows::core::PCWSTR, brecurse: super::super::Foundation::BOOL) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: ::windows::core::PCWSTR, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_EX = ::core::option::Option<unsafe extern "system" fn(hself: super::super::Foundation::HANDLE, lpszresourcetype: ::windows::core::PCWSTR, dwdesiredaccess: u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_RESOURCE_DEPENDENTIP_ADDRESS_PROPS = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, pszaddress: ::windows::core::PWSTR, pcchaddress: *mut u32, pszsubnetmask: ::windows::core::PWSTR, pcchsubnetmask: *mut u32, psznetwork: ::windows::core::PWSTR, pcchnetwork: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_RESOURCE_NAME = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, pszresourcename: ::windows::core::PWSTR, pcchresourcenameinout: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(lpszresourcename: ::windows::core::PCWSTR, lpszresourcetype: ::windows::core::PCWSTR) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY_EX = ::core::option::Option<unsafe extern "system" fn(lpszresourcename: ::windows::core::PCWSTR, lpszresourcetype: ::windows::core::PCWSTR, dwdesiredaccess: u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppszoutvalue: *mut ::windows::core::PWSTR, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: ::windows::core::PCWSTR, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_SZ_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows::core::PCWSTR) -> ::windows::core::PWSTR>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_IS_PATH_VALID = ::core::option::Option<unsafe extern "system" fn(pszpath: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_IS_RESOURCE_CLASS_EQUAL = ::core::option::Option<unsafe extern "system" fn(prci: *mut CLUS_RESOURCE_CLASS_INFO, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_PROPERTY_LIST_FROM_PARAMETER_BLOCK = ::core::option::Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: *mut u32, pinparams: *const u8, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_REMOVE_RESOURCE_SERVICE_ENVIRONMENT = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows::core::PCWSTR, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_RESOURCES_EQUAL = ::core::option::Option<unsafe extern "system" fn(hself: *mut _HRESOURCE, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_RESOURCE_TYPES_EQUAL = ::core::option::Option<unsafe extern "system" fn(lpszresourcetypename: ::windows::core::PCWSTR, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_BINARY_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows::core::PCWSTR, pbnewvalue: *const u8, cbnewvaluesize: u32, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_DWORD_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows::core::PCWSTR, dwnewvalue: u32, pdwoutvalue: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_EXPAND_SZ_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows::core::PCWSTR, psznewvalue: ::windows::core::PCWSTR, ppszoutstring: *mut ::windows::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_MULTI_SZ_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows::core::PCWSTR, psznewvalue: ::windows::core::PCWSTR, cbnewvaluesize: u32, ppszoutvalue: *mut ::windows::core::PWSTR, pcboutvaluesize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PRIVATE_PROPERTY_LIST = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK_EX = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_TABLE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_TABLE_EX = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_QWORD_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows::core::PCWSTR, qwnewvalue: u64, pqwoutvalue: *mut u64) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_SET_RESOURCE_SERVICE_ENVIRONMENT = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows::core::PCWSTR, hresource: *mut _HRESOURCE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows::core::PCWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS_EX = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows::core::PCWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, dwdesiredaccess: u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_SZ_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows::core::PCWSTR, psznewvalue: ::windows::core::PCWSTR, ppszoutstring: *mut ::windows::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_UNKNOWN_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_START_RESOURCE_SERVICE = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows::core::PCWSTR, phservicehandle: *mut isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_STOP_RESOURCE_SERVICE = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_STOP_SERVICE = ::core::option::Option<unsafe extern "system" fn(hservicehandle: super::super::Security::SC_HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_TERMINATE_SERVICE_PROCESS_FROM_RES_DLL = ::core::option::Option<unsafe extern "system" fn(dwservicepid: u32, boffline: super::super::Foundation::BOOL, pdwresourcestate: *mut u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_VERIFY_PRIVATE_PROPERTY_LIST = ::core::option::Option<unsafe extern "system" fn(pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_VERIFY_PROPERTY_TABLE = ::core::option::Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_VERIFY_RESOURCE_SERVICE = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_VERIFY_SERVICE = ::core::option::Option<unsafe extern "system" fn(hservicehandle: super::super::Security::SC_HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRES_UTIL_VERIFY_SHUTDOWN_SAFE = ::core::option::Option<unsafe extern "system" fn(flags: u32, reason: u32, presult: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSET_INTERNAL_STATE = ::core::option::Option<unsafe extern "system" fn(param0: isize, statetype: CLUSTER_RESOURCE_APPLICATION_STATE, active: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, propertylistbuffer: *const u8, propertylistbuffersize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, lockedmodeenabled: super::super::Foundation::BOOL, lockedmodereason: u32, lockedmodeflags: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_LOCKED_MODE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, lockedmodeenabled: super::super::Foundation::BOOL, lockedmodereason: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_STATUS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, resourcestatus: *mut RESOURCE_STATUS) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_STATUS_ROUTINE_EX = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, resourcestatus: *mut RESOURCE_STATUS_EX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PSIGNAL_FAILURE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, failuretype: FAILURE_TYPE, applicationspecificerrorcode: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PSTARTUP_EX_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcetype: ::windows::core::PCWSTR, minversionsupported: u32, maxversionsupported: u32, monitorcallbackfunctions: *mut CLRES_CALLBACK_FUNCTION_TABLE, resourcedllinterfacefunctions: *mut *mut CLRES_FUNCTION_TABLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PSTARTUP_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcetype: ::windows::core::PCWSTR, minversionsupported: u32, maxversionsupported: u32, setresourcestatus: PSET_RESOURCE_STATUS_ROUTINE, logevent: PLOG_EVENT_ROUTINE, functiontable: *mut *mut CLRES_FUNCTION_TABLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PTERMINATE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWORKER_START_ROUTINE = ::core::option::Option<unsafe extern "system" fn(pworker: *mut CLUS_WORKER, lpthreadparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SET_APP_INSTANCE_CSV_FLAGS = ::core::option::Option<unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, mask: u32, flags: u32) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
