#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct k_mem_domain {
    _unused: [u8; 0],
}

#[doc = " @addtogroup heap_apis\n @{"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k_heap {
    pub heap: sys_heap,
    pub wait_q: _wait_q_t,
    pub lock: k_spinlock,
}
impl Default for k_heap {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

extern "C" {
    #[doc = " @brief Initialize a k_heap\n\n This constructs a synchronized k_heap object over a memory region\n specified by the user.  Note that while any alignment and size can\n be passed as valid parameters, internal alignment restrictions\n inside the inner sys_heap mean that not all bytes may be usable as\n allocated memory.\n\n @param h Heap struct to initialize\n @param mem Pointer to memory.\n @param bytes Size of memory region, in bytes"]
    pub fn k_heap_init(h: *mut k_heap, mem: *mut c_types::c_void, bytes: usize);
}
extern "C" {
    #[doc = " @brief Allocate aligned memory from a k_heap\n\n Behaves in all ways like k_heap_alloc(), except that the returned\n memory (if available) will have a starting address in memory which\n is a multiple of the specified power-of-two alignment value in\n bytes.  The resulting memory can be returned to the heap using\n k_heap_free().\n\n @note @a timeout must be set to K_NO_WAIT if called from ISR.\n @note When CONFIG_MULTITHREADING=n any @a timeout is treated as K_NO_WAIT.\n\n @funcprops \\isr_ok\n\n @param h Heap from which to allocate\n @param align Alignment in bytes, must be a power of two\n @param bytes Number of bytes requested\n @param timeout How long to wait, or K_NO_WAIT\n @return Pointer to memory the caller can now use"]
    pub fn k_heap_aligned_alloc(
        h: *mut k_heap,
        align: usize,
        bytes: usize,
        timeout: k_timeout_t,
    ) -> *mut c_types::c_void;
}
extern "C" {
    #[doc = " @brief Allocate memory from a k_heap\n\n Allocates and returns a memory buffer from the memory region owned\n by the heap.  If no memory is available immediately, the call will\n block for the specified timeout (constructed via the standard\n timeout API, or K_NO_WAIT or K_FOREVER) waiting for memory to be\n freed.  If the allocation cannot be performed by the expiration of\n the timeout, NULL will be returned.\n Allocated memory is aligned on a multiple of pointer sizes.\n\n @note @a timeout must be set to K_NO_WAIT if called from ISR.\n @note When CONFIG_MULTITHREADING=n any @a timeout is treated as K_NO_WAIT.\n\n @funcprops \\isr_ok\n\n @param h Heap from which to allocate\n @param bytes Desired size of block to allocate\n @param timeout How long to wait, or K_NO_WAIT\n @return A pointer to valid heap memory, or NULL"]
    pub fn k_heap_alloc(h: *mut k_heap, bytes: usize, timeout: k_timeout_t)
                        -> *mut c_types::c_void;
}
extern "C" {
    #[doc = " @brief Free memory allocated by k_heap_alloc()\n\n Returns the specified memory block, which must have been returned\n from k_heap_alloc(), to the heap for use by other callers.  Passing\n a NULL block is legal, and has no effect.\n\n @param h Heap to which to return the memory\n @param mem A valid memory block, or NULL"]
    pub fn k_heap_free(h: *mut k_heap, mem: *mut c_types::c_void);
}
extern "C" {
    #[doc = " @brief Allocate memory from the heap with a specified alignment.\n\n This routine provides semantics similar to aligned_alloc(); memory is\n allocated from the heap with a specified alignment. However, one minor\n difference is that k_aligned_alloc() accepts any non-zero @p size,\n whereas aligned_alloc() only accepts a @p size that is an integral\n multiple of @p align.\n\n Above, aligned_alloc() refers to:\n C11 standard (ISO/IEC 9899:2011): 7.22.3.1\n The aligned_alloc function (p: 347-348)\n\n @param align Alignment of memory requested (in bytes).\n @param size Amount of memory requested (in bytes).\n\n @return Address of the allocated memory if successful; otherwise NULL."]
    pub fn k_aligned_alloc(align: usize, size: usize) -> *mut c_types::c_void;
}
extern "C" {
    #[doc = " @brief Allocate memory from the heap.\n\n This routine provides traditional malloc() semantics. Memory is\n allocated from the heap memory pool.\n Allocated memory is aligned on a multiple of pointer sizes.\n\n @param size Amount of memory requested (in bytes).\n\n @return Address of the allocated memory if successful; otherwise NULL."]
    pub fn k_malloc(size: usize) -> *mut c_types::c_void;
}
extern "C" {
    #[doc = " @brief Free memory allocated from heap.\n\n This routine provides traditional free() semantics. The memory being\n returned must have been allocated from the heap memory pool.\n\n If @a ptr is NULL, no operation is performed.\n\n @param ptr Pointer to previously allocated memory."]
    pub fn k_free(ptr: *mut c_types::c_void);
}
extern "C" {
    #[doc = " @brief Allocate memory from heap, array style\n\n This routine provides traditional calloc() semantics. Memory is\n allocated from the heap memory pool and zeroed.\n\n @param nmemb Number of elements in the requested array\n @param size Size of each array element (in bytes).\n\n @return Address of the allocated memory if successful; otherwise NULL."]
    pub fn k_calloc(nmemb: usize, size: usize) -> *mut c_types::c_void;
}
