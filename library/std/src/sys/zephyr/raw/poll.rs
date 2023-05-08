pub const K_POLL_TYPE_IGNORE: u32 = 0;
pub const K_POLL_STATE_NOT_READY: u32 = 0;

pub const _poll_types_bits__POLL_TYPE_IGNORE: _poll_types_bits = 0;
pub const _poll_types_bits__POLL_TYPE_SIGNAL: _poll_types_bits = 1;
pub const _poll_types_bits__POLL_TYPE_SEM_AVAILABLE: _poll_types_bits = 2;
pub const _poll_types_bits__POLL_TYPE_DATA_AVAILABLE: _poll_types_bits = 3;
pub const _poll_types_bits__POLL_TYPE_MSGQ_DATA_AVAILABLE: _poll_types_bits = 4;
pub const _poll_types_bits__POLL_TYPE_PIPE_DATA_AVAILABLE: _poll_types_bits = 5;
pub const _poll_types_bits__POLL_NUM_TYPES: _poll_types_bits = 6;

pub type _poll_types_bits = c_types::c_uint;

pub const _poll_states_bits__POLL_STATE_NOT_READY: _poll_states_bits = 0;
pub const _poll_states_bits__POLL_STATE_SIGNALED: _poll_states_bits = 1;
pub const _poll_states_bits__POLL_STATE_SEM_AVAILABLE: _poll_states_bits = 2;
pub const _poll_states_bits__POLL_STATE_DATA_AVAILABLE: _poll_states_bits = 3;
pub const _poll_states_bits__POLL_STATE_CANCELLED: _poll_states_bits = 4;
pub const _poll_states_bits__POLL_STATE_MSGQ_DATA_AVAILABLE: _poll_states_bits = 5;
pub const _poll_states_bits__POLL_STATE_PIPE_DATA_AVAILABLE: _poll_states_bits = 6;
pub const _poll_states_bits__POLL_NUM_STATES: _poll_states_bits = 7;

pub type _poll_states_bits = c_types::c_uint;

pub const k_poll_modes_K_POLL_MODE_NOTIFY_ONLY: k_poll_modes = 0;
pub const k_poll_modes_K_POLL_NUM_MODES: k_poll_modes = 1;

pub type k_poll_modes = c_types::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k_poll_signal {
    #[doc = " PRIVATE - DO NOT TOUCH"]
    pub poll_events: sys_dlist_t,
    #[doc = " 1 if the event has been signaled, 0 otherwise. Stays set to 1 until\n user resets it to 0."]
    pub signaled: c_types::c_uint,
    #[doc = " custom result value passed to k_poll_signal_raise() if needed"]
    pub result: c_types::c_int,
}

impl Default for k_poll_signal {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

#[doc = " @brief Poll Event\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k_poll_event {
    #[doc = " PRIVATE - DO NOT TOUCH"]
    pub _node: sys_dnode_t,
    #[doc = " PRIVATE - DO NOT TOUCH"]
    pub poller: *mut z_poller,
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub __bindgen_anon_1: k_poll_event__bindgen_ty_1,
}

#[doc = " per-type data"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union k_poll_event__bindgen_ty_1 {
    pub obj: *mut c_types::c_void,
    pub signal: *mut k_poll_signal,
    pub sem: *mut k_sem,
    pub fifo: *mut k_fifo,
    pub queue: *mut k_queue,
    pub msgq: *mut k_msgq,
}

impl Default for k_poll_event__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

impl Default for k_poll_event {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

impl k_poll_event {
    #[inline]
    pub fn tag(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_tag(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn type_(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 6u8) as u32) }
    }
    #[inline]
    pub fn set_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub fn state(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(14usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_state(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(14usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn mode(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(21usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_mode(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(21usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn unused(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(22usize, 10u8) as u32) }
    }
    #[inline]
    pub fn set_unused(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(22usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        tag: u32,
        type_: u32,
        state: u32,
        mode: u32,
        unused: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let tag: u32 = unsafe { ::core::mem::transmute(tag) };
            tag as u64
        });
        __bindgen_bitfield_unit.set(8usize, 6u8, {
            let type_: u32 = unsafe { ::core::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit.set(14usize, 7u8, {
            let state: u32 = unsafe { ::core::mem::transmute(state) };
            state as u64
        });
        __bindgen_bitfield_unit.set(21usize, 1u8, {
            let mode: u32 = unsafe { ::core::mem::transmute(mode) };
            mode as u64
        });
        __bindgen_bitfield_unit.set(22usize, 10u8, {
            let unused: u32 = unsafe { ::core::mem::transmute(unused) };
            unused as u64
        });
        __bindgen_bitfield_unit
    }
}

extern "C" {
    #[doc = " @brief Initialize one struct k_poll_event instance\n\n After this routine is called on a poll event, the event it ready to be\n placed in an event array to be passed to k_poll().\n\n @param event The event to initialize.\n @param type A bitfield of the types of event, from the K_POLL_TYPE_xxx\n             values. Only values that apply to the same object being polled\n             can be used together. Choosing K_POLL_TYPE_IGNORE disables the\n             event.\n @param mode Future. Use K_POLL_MODE_NOTIFY_ONLY.\n @param obj Kernel object or poll signal."]
    pub fn k_poll_event_init(
        event: *mut k_poll_event,
        type_: u32,
        mode: c_types::c_int,
        obj: *mut c_types::c_void,
    );
}
