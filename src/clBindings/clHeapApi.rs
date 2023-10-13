pub use crate::clBindings::clPoolIpi::*;
pub use crate::clBindings::clMemStats::*;
/* automatically generated by rust-bindgen 0.66.1 */

#[doc = " OpenClovis implementation of the memory management library."]
pub const ClHeapModeT_CL_HEAP_PREALLOCATED_MODE: ClHeapModeT = 0;
#[doc = " Native C mode. It maps to the memory APIs provided by \\e libc."]
pub const ClHeapModeT_CL_HEAP_NATIVE_MODE: ClHeapModeT = 1;
#[doc = " Custom pools. The application developer can plug in customized\n memory management library calls."]
pub const ClHeapModeT_CL_HEAP_CUSTOM_MODE: ClHeapModeT = 2;
#[doc = " Heap Allocation modes. System mode maps to malloc/free calls."]
pub type ClHeapModeT = ::std::os::raw::c_uint;
#[doc = " ClHeapConfigT to be fetched by EO and contains the configuration of the heap\n library"]
#[repr(C)]
pub struct ClHeapConfigT {
    #[doc = " Allocation mode. This can be either CL_HEAP_NATIVE_MODE,\n CL_HEAP_PREALLOCATED_MODE, or CL_HEAP_CUSTOM_MODE."]
    pub mode: ClHeapModeT,
    #[doc = " A pool can grow even after it exhausts its current allocation.\n This attribute configures a pool in lazy mode for pool expansion.\n There are two modes for configuring the pool:\n    \\arg Lazy mode - The incremented pool does not initialize until an\n         allocation is made from the extended portion of the pool.\n         Lazy mode speeds up the initialization of the application,\n         but shifts the penalty of pool initialization to a later\n         allocation.\n    \\arg Normal mode - The incremented pool initializes as soon as it is\n         acquired by the memory management library during the creation of\n         the pool."]
    pub lazy: ClBoolT,
    #[doc = " Array of pool configurations"]
    pub pPoolConfig: *mut ClPoolConfigT,
    #[doc = " Number of pools in the \\e pPoolConfig array"]
    pub numPools: ClUint32T,
}
#[test]
fn bindgen_test_layout_ClHeapConfigT() {
    const UNINIT: ::std::mem::MaybeUninit<ClHeapConfigT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClHeapConfigT>(),
        24usize,
        concat!("Size of: ", stringify!(ClHeapConfigT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClHeapConfigT>(),
        8usize,
        concat!("Alignment of ", stringify!(ClHeapConfigT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClHeapConfigT),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lazy) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ClHeapConfigT),
            "::",
            stringify!(lazy)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pPoolConfig) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClHeapConfigT),
            "::",
            stringify!(pPoolConfig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).numPools) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ClHeapConfigT),
            "::",
            stringify!(numPools)
        )
    );
}
extern "C" {
    #[doc = "  \\brief Allocates memory of the requested size.\n\n  \\par Header File:\n  clHeapApi.h\n\n  \\param size (in) Number of memory bytes to be allocated.\n\n  \\par Return values:\n  \\arg A valid pointer on success. \\n\n  \\arg \\e NULL On memory allocation failure.\n\n  \\par Description:\n  This function allocates memory of a specified size. The returned memory is\n  aligned at an 8-byte boundary. When the heap library is configured to\n  \\e CL_HEAP_PREALLOCATED_MODE, it returns memory of minimum size. This memory\n  size is greater than or equal to the requested size. If size is specified\n  as 0, the system returns a valid pointer pointing to a chunk of minimum\n  size that is previously configured.\n  If heap library is configured to \\e CL_HEAP_PREALLOCATED_MODE, failure\n  of clHeapAllocate() for one size of memory does not mean that this function\n  will fail for other sizes. For more information, refer to man page of\n  malloc(3).\n\n  \\par Library File:\n  libClUtils\n\n  \\sa clHeapCalloc(), clHeapFree(), clHeapRealloc(),\n"]
    pub fn clHeapAllocate(size: ClUint32T) -> ClPtrT;
}
extern "C" {
    #[doc = "  \\brief Frees a pre-allocated memory.\n\n  \\par Header File:\n  clHeapApi.h\n\n  \\param  pAddress (in) Block of memory to be freed.\n\n  \\par Return values:\n   None.\n\n  \\par Description:\n  This function is used to free memory. \\e pAddress should be a valid pointer\n  allocated through a previous call to either clHeapAllocate(),\n  clHeapRealloc(), or clHeapCalloc(). \\e pAddress should not be used after\n  a call to clHeapFree(). NULL is a valid value for \\e pAddress.\n  For more information, refer to man page of free(3).\n\n  \\par Library File:\n  libClUtils\n\n  \\sa clHeapAllocate(), clHeapCalloc(), clHeapRealloc()"]
    pub fn clHeapFree(pAddress: ClPtrT);
}
extern "C" {
    #[doc = "  \\brief Allocates memory for an array and initializes it to zero.\n\n  \\par Header File:\n  clHeapApi.h\n\n  \\param numChunks (in) Number of chunks to be allocated.\n  \\param chunkSize (in) Size of each chunk\n  \\par Return values:\n  \\arg A valid pointer on success. \\n\n  \\arg \\e NULL On memory allocation failure.\n\n  \\par Description:\n  This function allocates memory of a specific size. The memory chunk,\n  it returns, is aligned at an 8-byte boundary. If \\e CL_HEAP_PREALLOCATED_MODE\n  is selected during heap configuration, failure of clHeapAllocate() for one\n  size of memory does not mean that it will fail for other sizes also. For\n  more information, refer to man page, \\e malloc(3). Also refer to man page\n  of \\e calloc(3).\n\n  \\par Library File:\n  libClUtils\n\n  \\sa clHeapAllocate(), clHeapFree(), clHeapRealloc()"]
    pub fn clHeapCalloc(numChunks: ClUint32T, chunkSize: ClUint32T) -> ClPtrT;
}
extern "C" {
    #[doc = "/\n/**\n\n  \\brief Initializes the heap library.\n\n  \\par Header File:\n   clHeapApi.h\n\n  \\param pHeapConfig (in) Pointer to the configuration to be used by heap library\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_NULL_POINTER  \\e pHeapConfig or \\e pPoolConfig\n   (member of \\e pHeapConfig) is NULL.\n  \\retval CL_ERR_INVALID_PARAMETER \\e numPools (member of \\e pHeapConfig) is 0.\n  \\retval CL_ERR_NO_MEMORY Heap library is out of memory and cannot proceed\n   further.\n\n  \\par Description:\n  This function is used to initialize the heap library. It is called during\n  the initialization of the EO (Execution Object).\n  The heap library must be initialized before it can be used to allocate\n  memory. The caller should allocate and free the \\e pHeapConfig parameter.\n  Any configuration provided through this function to the heap library\n  cannot be changed while the process calling this function is executing.\n  \\par\n  During the lifetime of a process, this function must be called only once,\n  preferably during process initialization. Subsequent calls to this function\n  are ignored and it returns \\e CL_OK without changing anything.\n\n  \\par Library File:\n   libClUtils\n\n  \\sa clHeapLibFinalize()\n"]
    pub fn clHeapLibInitialize(pHeapConfig: *const ClHeapConfigT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Finalizes the heap library.\n\n  \\par Header File:\n   clHeapApi.h\n\n  \\par Parameters:\n   None\n\n  \\retval CL_OK The API executed successfully\n  \\retval CL_ERR_NOT_INITIALIZED heap library is not initialized through a\n   previous call to clHeapLibInitialize() or it is finalized through a\n   call to clHeapLibFinalize().\n\n  \\par Description:\n  This function is used to finalize the heap library. After finalizing\n  the heap library, it must not be used to allocate any memory or free\n  previously allocated memory.\n\n  \\par Library File:\n   libClUtils\n\n  \\sa clHeapLibInitialize()\n"]
    pub fn clHeapLibFinalize() -> ClRcT;
}
extern "C" {
    pub fn clHeapInit() -> ClRcT;
}
extern "C" {
    pub fn clHeapExit() -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Changes the size of the memory block (chunk).\n\n  \\par Header File:\n   clHeapApi.h\n\n  \\param pAddress (in) Original pointer to the memory block (chunk).\n  \\param size (in) New size of the memory block (chunk).\n\n  \\returns The function returns a valid pointer on success and returns NULL\n   on memory allocation failure.\n\n  \\par Description:\n  This function is used to change the size of the memory block pointed by\n  \\e pAddress to \\e size in bytes. The new address returned is aligned at\n  an 8-byte boundary. The contents of the returned memory block is unchanged\n  to the minimum of the old and new sizes. The contents of the memory over\n  the size of the previous block of memory is un initialized.\n  - If \\e pAddress is NULL, the call is equivalent to \\e clHeapAllocate (size).\n  - If \\e size is zero, the call is equivalent to \\e clHeapFree(pAddress) and\n    the function returns NULL.\n  - If \\e pAddress is NULL and size is zero, it is still equal to\n    \\e clHeapAllocate (0).\n  - If \\e pAddress is not NULL, it must have been returned by an earlier call\n    to clHeapAllocate()/clHeapRealloc()/clHeapCalloc().\n  \\par\n  If clHeapRealloc() fails, the original memory block remains untouched\n  (if is not freed or moved). If  clHeapRealloc() succeeds, \\e pAddress\n  should no longer be used. For more information, refer to man page of\n  \\e realloc(3).\n\n  \\par Library File:\n  libClUtils\n\n  \\sa clHeapAllocate(), clHeapFree(), clHeapCalloc()\n"]
    pub fn clHeapRealloc(pAddress: ClPtrT, size: ClUint32T) -> ClPtrT;
}
extern "C" {
    #[doc = "  \\brief Shrinks the configured pools of memory.\n\n  \\par Header File:\n   clHeapApi.h\n\n  \\param pShrinkOptions (in) \\e shrinkFlags, a member of this structure\n  indicates to what extent the existing pools can be shrunk.\n\n  \\retval CL_OK Function completed successfully.\n  \\retval CL_ERR_NOT_INITIALIZED The heap library is not initialized by a\n   previous call to clHeapInitialize()\n\n  \\par Description:\n  This function is used to shrink the memory used by the pools of all\n  \\e chunkSize so that next usages of pools can grow without violating\n  the configured process wide upper limit of memory. These shrink options\n  are used for all pools of heap library.\n\n  \\par Library File:\n   libClUtils\n"]
    pub fn clHeapShrink(pShrinkOptions: *const ClPoolShrinkOptionsT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Returns the mode set during configuration.\n\n  \\par Header File:\n   clHeapApi.h\n\n  \\param pMode (out) Configuration mode returned by the function\n\n  \\retval CL_OK Function completed successfully.\n  \\retval CL_ERR_NOT_INITIALIZED If this function is called before heap\n  initialization through a clHeapIntialize() function.\n  \\retval CL_ERR_NULL_POINTER If \\e pMode is passed as NULL.\n\n  \\par Description:\n  This function returns the configuration mode of the heap in the current\n  process. It can be one of the following values:\n  CL_HEAP_NATIVE_MODE,\n  CL_HEAP_PREALLOCATED_MODE, or\n  CL_HEAP_CUSTOM_MODE.\n\n  \\par Library File:\n   libClUtils\n"]
    pub fn clHeapModeGet(pMode: *mut ClHeapModeT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Returns the statistics collected by heap module.\n\n  \\par Header File:\n   clHeapApi.h\n\n  \\param pHeapStats (out) Pointer to the memory block where heap module will\n   copy the statistics.\n\n  \\retval CL_OK Function completed successfully.\n  \\retval CL_ERR_NOT_INITIALIZED Heap library is not initialized.\n  \\retval CL_ERR_NULL_POINTER The parameter \\e pHeapStats is passed as NULL.\n\n  \\par Description:\n  This function is used by heap to collect the statistics about its usage.\n  Other components can invoke this function to access these statistics.\n\n  \\par Library File:\n   libClUtils\n"]
    pub fn clHeapStatsGet(pHeapStats: *mut ClMemStatsT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Returns the statistics collected by heap library for an individual\n   pool.\n\n  \\par Header File:\n   clHeapApi.h\n\n  \\param numPools (in) Number of pools for which the statistics are required.\n   This is used as the size for \\e pPoolSize and \\e pPoolStats.\n  \\param pPoolSize (out) Pointer to array which contains the sizes of various\n   pools.\n  \\param pHeapPoolStats (out) Pointer to array which contains the statistics\n   of various pools.\n\n  \\retval CL_OK Function completed successfully.\n  \\retval CL_ERR_NOT_INITIALIZED This function is called before initializing\n   the heap library using the clHeapIntialize() function.\n  \\retval CL_ERR_NULL_POINTER Either \\e pPoolSize or \\e pPoolStats or both\n   the parameters is NULL.\n\n  \\par Description:\n   This function is used by components to retrieve statistics about usage of\n   various pools and their current size. The heap module gathers this information.\n  \\par\n   If \\e numPools is less than the number of pools configured in heap, the\n   function returns the size and statistics of the first pool arranged in\n   increasing order of chunk sizes. If \\e numPools is greater than the number\n   of pools configured, only first n entries of \\e pPoolSize and\n   \\e pPoolStats are valid, where \\e n is the number of pools configured.\n\n  \\par Library File:\n   libClUtils\n"]
    pub fn clHeapPoolStatsGet(
        numPools: ClUint32T,
        pPoolSize: *mut ClUint32T,
        pHeapPoolStats: *mut ClPoolStatsT,
    ) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Customizes the initialization of heap library in CL_HEAP_CUSTOM_MODE.\n\n  \\par Header File:\n   clHeapApi.h\n\n  \\param pHeapConfig (in) Pointer to configuration information used by heap\n  library.\n\n  \\retval CL_OK Function completed successfully.\n  \\retval CL_ERR_NULL_POINTER Either \\e pHeapConfig or \\e pPoolConfig,\n   a member of \\e pHeapConfig is passed as NULL.\n  \\retval CL_ERR_NO_MEMORY Heap library is out of memory and cannot proceed\n   further.\n\n  \\par Description:\n  This function is used to customize the initialization of the heap library.\n  This is an open function and the application developer should implement\n  this function.This is called only when an application indicates that it\n  needs to customize heap through CL_HEAP_CUSTOM_MODE. The function is\n  available in <em> ASP/models/<model-name>/config/clHeapCustom.c.</em>\n  This function must call clHeapHooksRegister() with the appropriate\n  function pointers to override the implementation of heap provided by\n  OpenClovis.\n\n  \\par Library File:\n  libClUtils\n\n  \\sa clHeapLibInitialize(), clHeapLibCustomFinalize()\n"]
    pub fn clHeapLibCustomInitialize(pHeapConfig: *const ClHeapConfigT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Customizes the finalization of heap library in CL_HEAP_CUSTOM_MODE.\n\n  \\par Header File:\n   clHeapApi.h\n\n  \\par Parameters:\n   None\n\n  \\retval CL_OK Function completed successfully.\n  \\retval CL_ERR_NOT_INITIALIZED Heap library is not initialized through a\n   previous call to clHeapLibInitialize() or it is finalized using\n   clHeapLibFinalize().\n\n  \\par Description:\n   This function is used to customize the finalization of the heap library.\n   This is an open function and the application developer should implement\n   this function. This is called only when an application indicates that\n   it needs to customize heap through CL_HEAP_CUSTOM_MODE. The function is\n   available in <em>ASP/models/<model-name>/config/clHeapCustom.c</em>.\n   This function must call clHeapHooksRegister() with the appropriate\n   function pointers to override the implementation of heap provided by\n   OpenClovis.\n\n  \\par Library File:\n   libClUtils\n\n  \\sa clHeapLibFinalize(), clHeapLibCustomInitialize()\n"]
    pub fn clHeapLibCustomFinalize() -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Register functions to be used in CL_HEAP_CUSTOM_MODE\n\n  \\par Header File:\n   clHeapApi.h\n\n  \\param allocHook (in) Function that allocates memory. This function is\n   invoked when an application calls clHeapAllocate() function.\n  \\param reallocHook (in) Function that changes the size of the memory.\n   This function is invoked when an application calls clHeapRealloc().\n  \\param callocHook (in) Function that allocates memory for an array.\n   This function is invoked when an application calls clHeapCalloc().\n  \\param freeHook (in) Function that frees the memory.\n   This function is invoked when an application calls clHeapFree().\n\n  \\retval CL_OK Function completed successfully.\n  \\retval CL_ERR_INITIALIZED Heap library is not initialized or it is\n   finalized.\n  \\retval CL_ERR_NULL_POINTER One of the input argument is a NULL pointer.\n\n  \\par Description:\n  In CL_HEAP_CUSTOM_MODE, this function is used to register the hooks\n  for memory management. The application should override the default\n  implementation of clHeapLibCustomInitialize() and\n  clHeapLibCustomFinalize() functions present in\n  <em> ASP/models/<model-name>/config/clHeapCustom.c</em>. This\n  function should be called from clHeapLibCustomInitialize().\n  In CL_HEAP_CUSTOM_MODE, no memory allocation should be made before\n  registering these functions.\n  \\par\n  During the life of a process, two different set of hooks for memory\n  allocation should not be used. Unless due care is taken, memory\n  allocated by one set of hooks cannot be freed using the other set\n  of hooks as it leads to corruption of meta data structures.\n\n  \\par Library File:\n   libClUtils\n\n  \\sa clHeapHooksDeregister()\n"]
    pub fn clHeapHooksRegister(
        allocHook: ::std::option::Option<unsafe extern "C" fn(arg1: ClUint32T) -> ClPtrT>,
        reallocHook: ::std::option::Option<
            unsafe extern "C" fn(arg1: ClPtrT, arg2: ClUint32T) -> ClPtrT,
        >,
        callocHook: ::std::option::Option<
            unsafe extern "C" fn(arg1: ClUint32T, arg2: ClUint32T) -> ClPtrT,
        >,
        freeHook: ::std::option::Option<unsafe extern "C" fn(arg1: ClPtrT)>,
    ) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief De-registers the hooks registered for CL_HEAP_CUSTOM_MODE\n\n  \\par Header File:\n   clHeapApi.h\n\n  \\par Parameters:\n   None\n\n  \\retval CL_OK Function completed successfully.\n  \\retval CL_ERR_INITIALIZED Heap library is not finalized through a previous\n   call to the clHeapLibFinalize() function.\n  \\retval CL_ERR_NOT_INITIALIZED Hooks were not registered through a previous\n   call to clHeapHooksRegister() function.\n\n  \\par Description:\n  This function is used to De-register the hooks registered for memory\n  management in CL_HEAP_CUSTOM_MODE. After a call to this function,\n  the memory related calls cannot be made until another call is made to\n  clHeapHooksRegister(). This function should be called during the\n  finalization of clHeapLibFinalize() through a call to open function\n  clHeapLibCustomFinalize() present in\n  <em> ASP/models/<model-name>/config/clHeapCustom.c</em>.\n  Detailed desciption\n\n  \\par Library File:\n   libClUtils\n\n  \\sa clHeapHooksRegister()\n"]
    pub fn clHeapHooksDeregister() -> ClRcT;
}