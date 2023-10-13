#[derive(Copy, Clone)]
pub enum ExecutionMode {
    Running,
    Stopped,
    CpuOperation,
    Frame,
}
