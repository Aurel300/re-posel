use crate::patches::{Patch, PatchChange};

pub struct DisLine {
    pub span: std::ops::Range<usize>,
    pub hex: String,
    pub asm: Option<String>,
    pub decomp: Option<String>,
    pub comments: Option<String>,
}

pub struct DisCode<'a> {
    pub error: bool,
    pub lines: Vec<DisLine>,
    pub code: &'a [u8],
    offset: usize,
}

impl<'a> DisCode<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Self {
            error: false,
            lines: Vec::new(),
            code,
            offset: 0,
        }
    }

    pub fn line(
        &mut self,
        mut start: usize,
        mut end: usize,
        asm: Option<String>,
        decomp: Option<String>,
        comments: Option<String>,
     ) {
        start += self.offset;
        end += self.offset;
        assert!(start <= end && end <= self.code.len());
        self.lines.push(DisLine {
            span: start..end,
            hex: super::hexdump(&self.code[start..end]),
            asm,
            decomp,
            comments,
        });
    }

    pub fn finalise(&mut self) {
        self.lines.sort_by_key(|e| (e.span.start, e.span.end));
        let mut last_end = 0;
        let mut unknown = Vec::new();
        for line in &self.lines {
            if line.span.start > last_end {
                unknown.push(DisLine {
                    span: last_end..line.span.start,
                    hex: super::hexdump(&self.code[last_end..line.span.start]),
                    asm: None,
                    decomp: None,
                    comments: Some(format!("unknown data ({} / 0x{:04x} bytes)", line.span.start - last_end, line.span.start - last_end)),
                });
            }
            last_end = line.span.end;
        }
        if last_end < self.code.len() {
            unknown.push(DisLine {
                span: last_end..self.code.len(),
                hex: super::hexdump(&self.code[last_end..self.code.len()]),
                asm: None,
                decomp: None,
                comments: Some(format!("unknown data ({} / 0x{:04x} bytes)", self.code.len() - last_end, self.code.len() - last_end)),
            });
        }
        self.lines.extend(unknown);
        self.lines.sort_by_key(|e| (e.span.start, e.span.end));
    }

    pub fn with_offset<R>(&mut self, offset: usize, f: impl FnOnce(&mut DisCode<'_>) -> R) -> R {
        self.offset += offset;
        let res = f(self);
        self.offset -= offset;
        res
    }

    pub fn finalise_with_patches(mut self, patches: &[(&Patch, &PatchChange)]) -> DisCode<'static> {
        pub fn marked_hexdump(code: &[u8], marked: &[Option<&str>]) -> String {
            assert_eq!(code.len(), marked.len());
            if code.is_empty() {
                return "".to_string();
            }
            let mut ret = String::new();
            for (i, b) in code.iter().enumerate() {
                if i > 0 {
                    ret.push(' ');
                }
                if let Some(desc) = marked[i] {
                    ret.push_str(&format!("<span class=\"mark\" title=\"{desc}\">{b:02x}</span>"));
                } else {
                    ret.push_str(&format!("{b:02x}"));
                }
            }
            ret
        }

        self.finalise();
        for (patch, change) in patches {
            let range = change.range();
            /* match change {
                PatchChange::DataEntry { range, .. } => range,
            };*/
            // TODO: be more efficient
            // TODO: support multiple patches in same line
            for line in &mut self.lines {
                let marked = line.span.clone().map(|pos| if range.contains(&pos) { Some(patch.description) } else { None }).collect::<Vec<_>>();
                line.hex = marked_hexdump(&self.code[line.span.clone()], &marked[..]);
                /*
                for pos in line.span {
                    if range.contains(&pos) {

                    }
                }
                */
                //let starts_here = range.start >= line.span.start && 
            }
        }
        DisCode {
            error: self.error,
            lines: self.lines,
            code: &[],
            offset: self.offset,
        }
    }
}
