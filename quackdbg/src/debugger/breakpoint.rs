use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SrcPos {
    pub file: PathBuf,
    pub line: usize,
}

pub struct Breakpoint {
    pub number: usize,
    pub enabled: bool,
    pub address: Option<usize>,
    pub src_pos: Option<SrcPos>,
}
