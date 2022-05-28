pub mod breakpoint;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DebuggerType {
    GDB,
}

pub trait Debugger {
    
}
