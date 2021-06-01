use novusk_syscalls::NovuskSysCalls;
use uefi::proto::console::text::Input;

/// The input function safely calls sys_input from Novusk System Calls. It's return type is
/// &mut Input (Input is from UEFI rs).
///
/// ### Example:
///
/// let mut user_input = input();
/// println!("Entered: {:?}", user_input);
pub fn input() -> &'static mut Input {
    let mut syscalls = NovuskSysCalls;
    unsafe { syscalls.sys_input() }
}
