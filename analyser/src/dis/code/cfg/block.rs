use std::collections::HashSet;

use crate::dis::code::DisJump;

use super::AstToken;

/// Location reachable in control flow.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum BlockId {
    /// Block, identified by its first instruction offset (in the code section,
    /// so relative to `code_start`).
    Block(usize),

    /// Code end; not a real block.
    End,
}

impl PartialOrd for BlockId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BlockId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        block_key(self).cmp(&block_key(other))
    }
}

fn block_key(id: &BlockId) -> usize {
    match id {
        BlockId::Block(id) => *id,
        BlockId::End => usize::MAX,
    }
}

impl std::fmt::Debug for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block(id) => write!(f, "B({:04x})", id + 0x18),
            Self::End => write!(f, "E"),
        }
    }
}

impl From<usize> for BlockId {
    fn from(value: usize) -> Self {
        Self::Block(value)
    }
}

#[derive(Clone)]
pub(super) struct BlockEdge {
    pub(super) line: Option<usize>,
    pub(super) to: BlockId,
    pub(super) kind: DisJump,
}

/// All offsets are relative to the code start, i.e., should be displayed with
/// `code_start` added if showing to the user.
pub(super) struct Block {
    /// Offset to the first instruction.
    pub(super) start: usize,

    /// Offset to the last instruction.
    pub(super) term: usize,

    /// Offset to just after the last instruction.
    pub(super) end: usize,

    /// Decompiled lines.
    pub(super) lines: Vec<AstToken>,

    /// Predecessors.
    pub(super) pred: HashSet<BlockId>,

    /// Successors.
    pub(super) succ: Vec<BlockEdge>,

    /// Whether this block ends with an exit instruction.
    pub(super) exit: bool,
}

impl Block {
    pub(super) fn new(start: usize) -> Self {
        Self {
            start,
            end: start,
            term: start,
            lines: Vec::new(),
            pred: HashSet::new(),
            succ: Vec::new(),
            exit: false,
        }
    }
}
