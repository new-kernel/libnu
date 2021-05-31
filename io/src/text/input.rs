use novusk_syscalls::NovuskSysCalls;
use uefi::proto::console::text::Input;

pub fn input() -> &'static mut Input {
    let mut syscalls = NovuskSysCalls;
    unsafe { syscalls.sys_input() }
}
