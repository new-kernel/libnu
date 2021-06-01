/// ApplicationType is a enum type used for determining what process Novusk's userspace will run. An
/// OS would use the OperatingSystem option. If you are going to setup your own hardware drivers for
/// example, you would use the KernelExtension option (because that's what a kernel is) then
/// reinitialize the userspace with the OperatingSystem option.
#[derive(Copy, Clone, PartialEq)]
pub enum ApplicationType {
    None,
    KernelExtension,
    OperatingSystem,
}
