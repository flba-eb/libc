pub type c_char = u8;
pub type wchar_t = u32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type time_t = i64;
pub type ino_t = u64;
pub type off_t = i64;
pub type blkcnt_t = u64;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;
pub type rlim_t = u64;

s! {
    pub struct aarch64_qreg_t {
        pub qlo: u64,
        pub qhi: u64,
    }

    pub struct aarch64_fpu_registers {
        pub reg: [::aarch64_qreg_t; 32],
        pub fpsr: u32,
        pub fpcr: u32,
    }

    pub struct aarch64_cpu_registers {
        pub gpr: [u64; 32],
        pub elr: u64,
        pub pstate: u64,
    }

    #[repr(align(16))]
    pub struct mcontext_t {
        pub cpu: ::aarch64_cpu_registers,
        pub fpu: ::aarch64_fpu_registers,
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_size: ::size_t,
        pub ss_flags: ::c_int,
    }
}

pub const RLIM_INFINITY: ::rlim_t = 0xfffffffffffffffd;

pub const F_SETLK: ::c_int = 106;
pub const F_SETLKW: ::c_int = 107;
pub const F_ALLOCSP: ::c_int = 110;
pub const F_FREESP: ::c_int = 111;
pub const F_GETLK: ::c_int = 114;

pub const BIOCGDLTLIST: ::c_int = -1072676233;
pub const BIOCSETF: ::c_int = -2146418073;
pub const BIOCGRTIMEOUT: ::c_int = 1074807406;
pub const BIOCSRTIMEOUT: ::c_int = -2146418067;

pub const SIGEV_SIGNAL: ::c_int = 129;
pub const SIGEV_THREAD: ::c_int = 135;
