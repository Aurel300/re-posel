use std::collections::{HashMap, HashSet, VecDeque};

use crate::dis::DisError;

use super::{opcodes::DisIns, DisJump, DisOp};

mod ast;
mod block;

use ast::{AstStack, AstToken};
use block::*;

pub(crate) struct Decompiler<'a> {
    code_start: usize,
    opcode_offset: u8,
    code: &'a [u8],
    block_starts: HashSet<usize>,
    pos_decomp: HashMap<usize, String>,
    pos_jump: HashMap<usize, Vec<BlockEdge>>,
}

impl<'a> Decompiler<'a> {
    pub(crate) fn new(code_start: usize, opcode_offset: u8, code: &'a [u8]) -> Self {
        Self {
            code_start,
            opcode_offset,
            code,
            block_starts: [0].into(),
            pos_decomp: HashMap::new(),
            pos_jump: HashMap::new(),
        }
    }

    pub(crate) fn add_decomp(&mut self, pos: usize, decomp: String) {
        self.pos_decomp.insert(pos, decomp.clone());
    }

    pub(crate) fn add_jump(&mut self, from: usize, to: usize, jump: DisJump) {
        self.pos_jump.entry(from)
            .or_default()
            .push(BlockEdge {
                line: Some(from),
                to: to.into(),
                kind: jump,
            });
        self.block_starts.insert(to);
    }

    pub(crate) fn analyse(self) -> Result<String, DisError> {
        let mut analysis = CfgAnalysis::new(self.code_start);
        analysis.create_blocks(self);
        analysis.compute_dominators();
        analysis.walk(0.into(), BlockId::End)?;
        Ok(analysis.build())
    }
}


struct CfgAnalysis {
    #[allow(dead_code)]
    code_start: usize,
    blocks: HashMap<BlockId, Block>,
    output: AstStack,
    //predoms: HashMap<BlockId, HashSet<BlockId>>,
    postdoms: HashMap<BlockId, HashSet<BlockId>>,
    //reach: HashMap<BlockId, HashSet<BlockId>>,
    path: Vec<BlockId>,
    path_loops: HashSet<BlockId>,
}

impl CfgAnalysis {
    fn new(code_start: usize) -> Self {
        Self {
            code_start,
            blocks: HashMap::new(),
            output: AstStack {
                current: Vec::new(),
                parent: None,
            },
            //predoms: HashMap::new(),
            postdoms: HashMap::new(),
            //reach: HashMap::new(),
            path: Vec::new(),
            path_loops: HashSet::new(),
        }
    }

    fn create_blocks(&mut self, mut info: Decompiler) {
        let mut pred = Vec::new();
        for start in info.block_starts.iter().copied() {
            let mut block = Block::new(start);
            while block.end < info.code.len() {
                if block.start != block.end && info.block_starts.contains(&block.end) {
                    block.term = block.end;
                    break;
                }
                let (pos, ins) = DisIns::analyse_one(info.code, info.opcode_offset, block.end).unwrap();
                if matches!(ins.op, DisOp::Exit) {
                    block.lines.push(AstToken::Exit(Some(block.end)));
                    block.term = block.end;
                    block.end = pos;
                    block.exit = true;
                    break;
                }
                if matches!(ins.op, DisOp::Tick) {
                    block.lines.push(AstToken::Tick(Some(block.end)));
                } else if !ins.op.is_terminator() && let Some(decomp) = info.pos_decomp.get(&block.end) {
                    block.lines.push(AstToken::Line(Some(block.end), decomp.to_string()));
                }
                if let Some(new_jumps) = info.pos_jump.remove(&block.end) {
                    block.succ.extend(new_jumps.clone());
                }
                block.term = block.end;
                block.end = pos;
            }
            if !block.exit && block.succ.is_empty() {
                block.succ.push(BlockEdge {
                    line: None,
                    to: block.end.into(),
                    kind: DisJump::Straight,
                });
            }
            if block.exit {
                block.succ.push(BlockEdge {
                    line: None,
                    to: BlockId::End,
                    kind: DisJump::Straight,
                });
            }
            for jump in &block.succ {
                pred.push((start, jump.clone()));
            }
            self.blocks.insert(start.into(), block);
        }
        for (pred, edge) in pred {
            if edge.to == BlockId::End {
                continue;
            }
            self.blocks.get_mut(&edge.to)
                .unwrap()
                .pred
                .insert(pred.into());
        }
    }

    fn compute_dominators(&mut self) {
        fn compute(
            keys: &HashSet<BlockId>,
            doms: &mut HashMap<BlockId, HashSet<BlockId>>,
            initial: BlockId,
            last: BlockId,
            pred: impl Fn(&BlockId) -> HashSet<BlockId>,
        ) {
            doms.insert(initial, HashSet::from([initial]));
            for id in keys {
                if *id == initial {
                    continue;
                }
                doms.insert(*id, keys.clone());
            }
            let mut changed = true;
            while changed {
                changed = false;
                for id in keys {
                    if *id == initial || *id == last {
                        continue;
                    }
                    let prev = doms.remove(id).unwrap();
                    let mut new = pred(id)
                        .iter()
                        .map(|id| doms.get(id).unwrap().clone())
                        .reduce(|a, b| a.intersection(&b).copied().collect())
                        .unwrap_or_default();
                    new.insert(*id);
                    changed |= prev != new;
                    doms.insert(*id, new);
                }
            }
        }
        let mut keys = self.blocks.keys().copied().collect::<HashSet<BlockId>>();
        keys.insert(BlockId::End);
        //compute(&keys, &mut self.predoms, 0.into(), BlockId::End, |id| self.blocks.get(id).unwrap().pred.clone());
        compute(&keys, &mut self.postdoms, BlockId::End, 0.into(), |id| self.blocks.get(id).unwrap().succ.iter()
            //.filter(|e| !e.back_edge)
            .map(|e| e.to).collect());
        /*
        for key in &keys {
            let mut reachable = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(*key);
            while let Some(id) = queue.pop_front() {
                if reachable.contains(&id) {
                    continue;
                }
                reachable.insert(id);
                if id == BlockId::End {
                    continue;
                }
                queue.extend(self.blocks.get(&id).unwrap().succ.iter().map(|e| e.to).filter(|nid| !reachable.contains(nid)));
            }
            self.reach.insert(*key, reachable);
        }
        */
    }

    fn find_join_point(
        &self,
        cond_id: BlockId,
        end: BlockId,
    ) -> BlockId {
        let block = self.blocks.get(&cond_id).unwrap();
        let next = block.succ.to_vec();
        self.reachable(cond_id, &[end].into())
            .into_iter()
            .filter(|(id, _)| next.iter().all(|e| self.postdoms.get(&e.to).unwrap().contains(id)))
            .min_by_key(|(_, dist)| *dist)
            .unwrap().0
    }

    fn reachable(
        &self,
        start: BlockId,
        ends: &HashSet<BlockId>,
    ) -> Vec<(BlockId, usize)> {
        let mut reachable = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        while let Some((id, dist)) = queue.pop_front() {
            if let Some(prev) = reachable.get(&id) {
                if *prev <= dist {
                    continue;
                }
            }
            if dist != 0 {
                reachable.insert(id, dist);
            }
            if id == BlockId::End || ends.contains(&id) {
                continue;
            }
            queue.extend(self.blocks.get(&id)
                .unwrap()
                .succ
                .iter()
                .map(|e| (e.to, dist + 1))
                .filter(|(e, d)| if let Some(prev) = reachable.get(e) {
                    prev > d
                } else {
                    true
                }));
        }
        let mut result = reachable.into_iter().collect::<Vec<_>>();
        result.sort();
        result
    }

    fn find_loop_end(
        &self,
        loop_id: BlockId,
        end: BlockId,
    ) -> Option<BlockId> {
        let mut backedges: HashSet<BlockId> = [end].into();
        backedges.extend(self.path_loops.clone());
        self.reachable(loop_id, &backedges)
            .into_iter()
            .filter_map(|(id, dist)| {
                if id == BlockId::End { return None; }
                if backedges.contains(&id) { return None; }
                if dist == 0 { return None; }
                let block = self.blocks.get(&id).unwrap();
                if id != loop_id && block.succ.len() != 1 { return None; }
                if !self.reachable(id, &backedges).into_iter().any(|(r, _)| r == loop_id) { return None; }
                for next in &block.succ {
                    if next.to == BlockId::End { continue; }
                    if backedges.contains(&next.to) { continue; }
                    if self.reachable(next.to, &backedges).into_iter().any(|(r, _)| r == loop_id) { continue; }
                    return Some((next.to, dist));
                }
                None
            })
            .min_by_key(|(_, dist)| *dist)
            .map(|(id, _)| id)
    }

    fn output(&mut self) -> &mut Vec<AstToken> {
        &mut self.output.current
    }

    fn walk(
        &mut self,
        start: BlockId,
        end: BlockId,
    ) -> Result<(), DisError> {
        if start == end || start == BlockId::End {
            return Ok(());
        }

        if self.path.contains(&start) {
            self.output().push(AstToken::Continue);
            return Ok(());
        }

        let block = self.blocks.get(&start).unwrap();
        if !self.path_loops.contains(&start) && let Some(loop_end) = self.find_loop_end(start, end) {
            let prev_output = std::mem::take(&mut self.output);
            self.output = AstStack {
                current: Vec::new(),
                parent: Some(Box::new(prev_output)),
            };
            self.path_loops.insert(start);
            self.walk(start, loop_end)?;

            let mut block_content;
            (block_content, self.output) = match std::mem::take(&mut self.output) {
                AstStack { current, parent: Some(parent) } => (current, *parent),
                _ => unreachable!(),
            };
            block_content.push(AstToken::Break);
            self.output().push(ast::make_loop(block_content));
            assert!(self.path_loops.remove(&start));

            self.walk(loop_end, end)?;
            return Ok(());
        }

        self.path.push(start);
        for line in &block.lines {
            self.output.current.push(line.clone());
        }
        let next = block.succ.to_vec();
        assert!(next.len() <= 2);
        let join = self.find_join_point(start, end);
        if !block.exit {
            if !next.is_empty() {
                if next.len() == 1 && matches!(next[0].kind, DisJump::Straight | DisJump::Unconditional) {
                    self.walk(next[0].to, join)?;
                } else {
                    let mut output_branches = Vec::new();
                    for (branch, edge) in next.into_iter().enumerate() {
                        let prev_output = std::mem::take(&mut self.output);
                        self.output = AstStack {
                            current: Vec::new(),
                            parent: Some(Box::new(prev_output)),
                        };
                        self.walk(edge.to, join)?;
                        let block_content;
                        (block_content, self.output) = match std::mem::take(&mut self.output) {
                            AstStack { current, parent: Some(parent) } => (current, *parent),
                            _ => unreachable!(),
                        };
                        if branch != 0 && block_content.is_empty() {
                            continue;
                        }
                        output_branches.push((
                            edge.line,
                            edge.kind.show_decomp(),
                            edge.kind.is_fallthrough(),
                            block_content,
                        ));
                    }
                    self.output().extend(ast::make_chain(output_branches));
                }
            }
            self.walk(join, end)?;
        }
        assert_eq!(self.path.pop().unwrap(), start);
        Ok(())
    }

    fn build(self) -> String {
        assert!(self.output.parent.is_none());
        let mut output = String::new();
        ast::build(self.code_start, &self.output.current, &mut output, 0, &mut 0);
        output
    }
}
