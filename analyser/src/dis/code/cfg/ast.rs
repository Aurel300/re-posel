use crate::dis::code::show_addr;

#[derive(PartialEq, Eq, Clone)]
pub(super) enum AstToken {
    Line(Option<usize>, String),
    Break,
    Continue,
    Exit(Option<usize>),
    Tick(Option<usize>),
    Loop(Vec<AstToken>),
    While(Option<usize>, String, Vec<AstToken>),
    Chain(Vec<(Option<usize>, String, bool, Vec<AstToken>)>),
    Switch(String, Vec<(Option<usize>, String, Vec<AstToken>)>),
}

#[derive(Default)]
pub(super) struct AstStack {
    pub(super) current: Vec<AstToken>,
    pub(super) parent: Option<Box<AstStack>>,
}

pub(super) fn make_chain(mut branches: Vec<(Option<usize>, String, bool, Vec<AstToken>)>) -> Vec<AstToken> {
    assert!(!branches.is_empty());
    if branches.len() == 2
        && !branches[0].2
        && branches[1].2
        && matches!(branches[0].3.last(), Some(AstToken::Exit(..))) {
        let (_, _, _, rest) = branches.pop().unwrap();
        let (cline, cond, _, nested) = branches.pop().unwrap();
        let mut output = Vec::new();
        output.push(AstToken::Chain(vec![(cline, cond, false, nested)]));
        output.extend(rest);
        return output;
    }
    use once_cell::sync::Lazy;
    use regex::Regex;
    static SWITCH_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"if \(\((.*)\) ==s? \((.*)\)\)").unwrap());
    loop {
        match branches.last() {
            Some((_, _, true, last)) if last.len() == 1 => match &last[0] {
                // merge conditionals into chains
                AstToken::Chain(..) => {
                    let AstToken::Chain(nested) = branches.pop().unwrap().3.pop().unwrap() else { unreachable!(); };
                    branches.extend(nested);
                    continue;
                }
                // create switch statements when testing the same value in a chain
                AstToken::Switch(test, ..) => {
                    let mut lhs_matches = vec![];
                    let mut rhs_matches = vec![];
                    branches[..branches.len() - 1].iter().for_each(|(_, cond, _, _)| {
                        let Some(captures) = SWITCH_RE.captures(cond) else { return; };
                        let Some(lhs) = captures.get(1).map(|m| m.as_str()) else { return; };
                        let Some(rhs) = captures.get(2).map(|m| m.as_str()) else { return; };
                        if test == lhs {
                            lhs_matches.push(rhs.to_string());
                        }
                        if test == rhs {
                            rhs_matches.push(lhs.to_string());
                        }
                    });
                    if lhs_matches.len() == branches.len() - 1 {
                        let AstToken::Switch(test, old_cases) = branches.pop().unwrap().3.pop().unwrap() else { unreachable!(); };
                        let mut cases = branches.into_iter().enumerate()
                            .map(|(idx, (cline, _, _, nested))| (cline, lhs_matches[idx].clone(), nested))
                            .collect::<Vec<_>>();
                        cases.extend(old_cases);
                        return vec![AstToken::Switch(test, cases)];
                    } else if rhs_matches.len() == branches.len() - 1 {
                        let AstToken::Switch(test, old_cases) = branches.pop().unwrap().3.pop().unwrap() else { unreachable!(); };
                        let mut cases = branches.into_iter().enumerate()
                            .map(|(idx, (cline, _, _, nested))| (cline, rhs_matches[idx].clone(), nested))
                            .collect::<Vec<_>>();
                        cases.extend(old_cases);
                        return vec![AstToken::Switch(test, cases)];
                    }
                }
                _ => (),
            }
            _ => (),
        }
        break;
    }
    if branches.len() >= 2
        && branches.iter().all(|(_, _, fallthrough, _)| !fallthrough)
        && let Some(first_captures) = SWITCH_RE.captures(&branches[0].1)
        && let Some(first_lhs) = first_captures.get(1).map(|m| m.as_str())
        && let Some(first_rhs) = first_captures.get(2).map(|m| m.as_str()) {
        let mut lhs_matches = vec![first_rhs.to_string()];
        let mut rhs_matches = vec![first_lhs.to_string()];
        branches[1..].iter().for_each(|(_, cond, _, _)| {
            let Some(captures) = SWITCH_RE.captures(cond) else { return; };
            let Some(lhs) = captures.get(1).map(|m| m.as_str()) else { return; };
            let Some(rhs) = captures.get(2).map(|m| m.as_str()) else { return; };
            if first_lhs == lhs {
                lhs_matches.push(rhs.to_string());
            }
            if first_rhs == rhs {
                rhs_matches.push(lhs.to_string());
            }
        });
        if lhs_matches.len() == branches.len() {
            return vec![AstToken::Switch(first_lhs.to_string(), branches.into_iter().enumerate().map(|(idx, (cline, _, _, nested))| (cline, lhs_matches[idx].to_string(), nested)).collect())];
        } else if rhs_matches.len() == branches.len() {
            return vec![AstToken::Switch(first_rhs.to_string(), branches.into_iter().enumerate().map(|(idx, (cline, _, _, nested))| (cline, rhs_matches[idx].to_string(), nested)).collect())];
        }
    }
    vec![AstToken::Chain(branches)]
}

pub(super) fn make_loop(mut content: Vec<AstToken>) -> AstToken {
    // identify while loops
    if content.len() == 2
        && matches!(&content[0], AstToken::Chain(b) if b.len() == 1 && b[0].1.starts_with("if ") && b[0].3.ends_with(&[AstToken::Continue]))
        && content[1] == AstToken::Break {
        let AstToken::Break = content.pop().unwrap() else { unreachable!(); };
        let AstToken::Chain(mut branches) = content.pop().unwrap() else { unreachable!(); };
        let (cline, cond, _, mut nested) = branches.pop().unwrap();
        let AstToken::Continue = nested.pop().unwrap() else { unreachable!(); };
        // convert a while (..) { tick } loop to wait while
        if nested.len() == 1 && matches!(nested[0], AstToken::Tick(..)) {
            let AstToken::Tick(cline) = nested[0] else { unreachable!(); };
            return AstToken::Line(
                cline,
                format!("<span class=\"hl-kw\">wait while</span> {}", &cond["if ".len()..]),
            );
        }
        return AstToken::While(
            cline,
            cond["if ".len()..].to_string(),
            nested,
        );
    }
    AstToken::Loop(content)
}

pub(super) fn build(
    code_start: usize,
    ast: &[AstToken],
    output: &mut String,
    depth: usize,
    block_counter: &mut usize,
) {

    let addr_indent = "  ".repeat(depth);
    let indent = "  ".repeat(depth + 3);
    let cline_indent = |cline: Option<usize>| if let Some(addr) = cline {
        format!("{}  {addr_indent}", show_addr(addr + code_start))
    } else {
        indent.to_string()
    };
    for token in ast {
        match token {
            AstToken::Line(cline, line) => {
                output.push_str(&cline_indent(*cline));
                output.push_str(&line.split("\n").collect::<Vec<_>>().join(&format!("\n{indent}")));
                output.push('\n');
            }
            AstToken::Break => {
                output.push_str(&indent);
                output.push_str("<span class=\"hl-kw\">break</span>\n");
            }
            AstToken::Continue => {
                output.push_str(&indent);
                output.push_str("<span class=\"hl-kw\">continue</span>\n");
            }
            AstToken::Exit(cline) => {
                output.push_str(&cline_indent(*cline));
                output.push_str("<span class=\"hl-kw\">exit</span>\n");
            }
            AstToken::Tick(cline) => {
                output.push_str(&cline_indent(*cline));
                output.push_str("<span class=\"hl-kw\">tick</span>\n");
            }
            AstToken::Loop(ast) => {
                let bid = *block_counter;
                *block_counter += 1;
                output.push_str(&format!("{indent}<span class=\"hl-kw\">loop</span> <a href=\"#be-{bid}\" id=\"bb-{bid}\">{{</a>\n"));
                build(code_start, ast, output, depth + 1, block_counter);
                output.push_str(&format!("{indent}<a href=\"#bb-{bid}\" id=\"be-{bid}\">}}</a>\n"));
            }
            AstToken::While(cline, cond, ast) => {
                output.push_str(&cline_indent(*cline));
                let bid = *block_counter;
                *block_counter += 1;
                output.push_str(&format!("<span class=\"hl-kw\">while</span> {cond} <a href=\"#be-{bid}\" id=\"bb-{bid}\">{{</a>\n"));
                build(code_start, ast, output, depth + 1, block_counter);
                output.push_str(&format!("{indent}<a href=\"#bb-{bid}\" id=\"be-{bid}\">}}</a>\n"));
            }
            AstToken::Chain(branches) => {
                let mut bid = 0;
                for (idx, (cline, cond, _, branch)) in branches.iter().enumerate() {
                    output.push_str(&cline_indent(*cline));
                    bid = *block_counter;
                    *block_counter += 1;
                    if idx > 0 {
                        output.push_str(&format!("<a href=\"#bb-{}\" id=\"be-{}\">}}</a> <span class=\"hl-kw\">else</span> ", bid - 1, bid - 1));
                    }
                    if let Some(cond) = cond.strip_prefix("if ") {
                        output.push_str(&format!("<span class=\"hl-kw\">if</span> {cond} <a href=\"#be-{bid}\" id=\"bb-{bid}\">{{</a>\n"));
                    } else {
                        output.push_str(&format!("{cond} <a href=\"#be-{bid}\" id=\"bb-{bid}\">{{</a>\n"));
                    }
                    build(code_start, branch, output, depth + 1, block_counter);
                }
                output.push_str(&format!("{indent}<a href=\"#bb-{}\" id=\"be-{}\">}}</a>\n", bid, bid));
            }
            AstToken::Switch(test, cases) => {
                let bid = *block_counter;
                *block_counter += 1;
                output.push_str(&format!("{indent}<span class=\"hl-kw\">switch</span> ({test}) <a href=\"#be-{bid}\" id=\"bb-{bid}\">{{</a>\n"));
                for (cline, value, branch) in cases {
                    output.push_str(&cline_indent(*cline));
                    output.push_str(&format!("<span class=\"hl-kw\">case</span> {value}:\n"));
                    build(code_start, branch, output, depth + 1, block_counter);
                }
                output.push_str(&format!("{indent}<a href=\"#bb-{bid}\" id=\"be-{bid}\">}}</a>\n"));
            }
        }
    }
}
