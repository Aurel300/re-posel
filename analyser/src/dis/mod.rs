mod code;
mod error;
mod lines;

use std::collections::HashMap;

pub use error::*;
pub use lines::*;

use crate::{adb::{AdbEntry, AdbEntryKind, AdbXref, AdbXrefKind}, Resources};

pub fn hexdump(code: &[u8]) -> String {
    if code.is_empty() {
        return "".to_string();
    }
    let mut ret = String::new();
    ret.push_str(&format!("{:02x}", code[0]));
    for b in &code[1..] {
        ret.push_str(&format!(" {:02x}", b));
    }
    ret
}

pub fn show_data(path: &str, res: Resources) -> String {
    // TODO: don't hardcode paths
    let Some((_grp, res_path, file_name)) = res.data.get(&path.to_ascii_lowercase()) else { return "missing data".to_string(); };
    if path.ends_with(".ogg") || path.ends_with(".wav") || path.ends_with(".mp3") {
        format!("<audio class=\"idata\" controls src=\"../../../exported/{res_path}\"></audio>")
    } else if path.ends_with(".flc") || path.ends_with(".mjpg") {
        let cpath = file_name.replace(".flc", ".mp4").replace(".mjpg", ".mp4");
        format!("<video class=\"idata\" controls src=\"../../../exported/converted/{cpath}\"></video>")
    } else if path.ends_with(".bmp") {
        format!("<img class=\"idata\" src=\"../../../exported/{res_path}\">")
    } else if path.ends_with(".pcx") {
        let cpath = file_name.replace(".pcx", ".png");
        format!("<img class=\"idata\" src=\"../../../exported/converted/{cpath}\">")
    } else {
        "data?".to_string()
    }
}

fn htmlsan(s: &str) -> String {
    s
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\n", "<br>")
}

fn shortsan(s: &str) -> String {
    if s.len() > 100 {
        s[0..s.ceil_char_boundary(100)]
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\n", "  ")
            .replace("\r", "  ")
            + "..."
    } else {
        s
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\n", "  ")
            .replace("\r", "  ")
    }
}

pub fn show_string(s: &str, res: Resources) -> String {
    let Some(entry) = res.entries.get(s) else {
        if res.data.contains_key(&s.to_ascii_lowercase()) {
            return format!("<span class=\"hl-str\">\"{s}\"</span>(-> {})", show_data(s, res));
        }
        return format!("<span class=\"hl-str\">\"{}\"</span>", htmlsan(s))
    };
    let rev_name = if let Some(name) = entry.name.as_ref() {
        format!(" ({})", name)
    } else {
        "".to_string()
    };
    match &entry.kind {
        AdbEntryKind::String { decoded, .. } if res.data.contains_key(&decoded.to_ascii_lowercase()) =>
            format!("<span class=\"hl-str\">\"{s}\"</span>(<a href=\"{s}.html\">-> S{rev_name}<span class=\"hl-str\">\"{}\"</span></a> -> {})", shortsan(decoded), show_data(decoded, res)),
        AdbEntryKind::String { decoded, .. } =>
            format!("<span class=\"hl-str\">\"{s}\"</span>(<a href=\"{s}.html\">-> S{rev_name}<span class=\"hl-str\">\"{}\"</span></a>)", shortsan(decoded)),
        AdbEntryKind::Raw(..) =>
            format!("<span class=\"hl-str\">\"{s}\"</span>(<a href=\"{s}.html\">-> R{rev_name}</a>)"),
        AdbEntryKind::Code(_) =>
            format!("<span class=\"hl-str\">\"{s}\"</span>(<a href=\"{s}.html\">-> C{rev_name}</a>)"),
        AdbEntryKind::Dummy =>
            format!("<span class=\"hl-str\">\"{s}\"</span>(<a href=\"{s}.html\">-> D{rev_name}</a>)"),
        AdbEntryKind::Global =>
            format!("<span class=\"hl-str\">\"{s}\"</span>(<a href=\"{s}.html\">-> G{rev_name}</a>)"),
        AdbEntryKind::Scene =>
            format!("<span class=\"hl-str\">\"{s}\"</span>(<a href=\"{s}.html\">-> Sc{rev_name}</a>)"),
    }
}

pub fn analyse_region<'a>(entry: &'a AdbEntry, res: Resources<'a>) -> Result<(String, DisCode<'a>), DisError> {
    let code = entry.raw();
    let mut output = DisCode::new(code, res.first_pass);
    if code.len() < 0x26 {
        return Err(DisError::TooShort);
    }

    // header
    let name_end = code.iter().position(|b| *b == 0).unwrap_or(0x20);
    let scene = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(&code[0..name_end]).unwrap();
    output.xrefs.push(AdbXref {
        other_key: scene.to_string(),
        loc: None,
        kind: AdbXrefKind::Scene,
    });
    output.line(0, name_end, Some(format!("scene: <a href=\"{scene}.html\">{scene}</a>")), None, None);
    if name_end < 0x20 {
        output.line(name_end, 0x20, Some("garbage?".to_string()), None, None);
    }
    let base_x = u16::from_le_bytes(code[0x20..0x22].try_into().unwrap());
    output.line(0x20, 0x22, Some(format!("pos x: {base_x}")), None, None);
    let base_y = u16::from_le_bytes(code[0x22..0x24].try_into().unwrap());
    output.line(0x22, 0x24, Some(format!("pos y: {base_y}")), None, None);
    let shape_count = u16::from_le_bytes(code[0x24..0x26].try_into().unwrap());
    output.line(0x24, 0x26, Some(format!("shape count: {shape_count}")), None, None);

    let mut pos = 0x26;
    let mut shapes = Vec::new();
    for shape_idx in 0..shape_count {
        output.line(pos, pos, None, None, Some(format!("shape #{shape_idx}")));
        if pos + 1 >= code.len() {
            return Err(DisError::TooShort);
        }

        let point_count = u16::from_le_bytes(code[pos..pos + 2].try_into().unwrap());
        output.line(pos, pos + 2, Some(format!("point count: {point_count}")), None, None);
        pos += 2;
        if pos + 6 * point_count as usize > code.len() {
            return Err(DisError::TooShort);
        }
        let mut points = Vec::new();
        for point_idx in 0..point_count {
            let pos_x = u16::from_le_bytes(code[pos..pos + 2].try_into().unwrap());
            pos += 2;
            let pos_y = u16::from_le_bytes(code[pos..pos + 2].try_into().unwrap());
            pos += 2;
            let pos_z = u16::from_le_bytes(code[pos..pos + 2].try_into().unwrap());
            pos += 2;
            if pos_z == 0xCDCD {
                output.line(pos - 6, pos, Some(format!("point {point_idx}: {pos_x}, {pos_y}")), None, None);
            } else {
                output.line(pos - 6, pos, Some(format!("point {point_idx}: {pos_x}, {pos_y}, {pos_z}")), None, None);
            }
            points.push((pos_x, pos_y));
        }
        shapes.push(points);
    }
    let mut svg = String::new();
    let reg_width = entry.region.as_ref().and_then(|r| r.width).map(|w| w as u16)
        .or(res.entries.get(scene.as_ref()).and_then(|e| e.scene.as_ref()).and_then(|s| s.width).map(|w| w as u16))
        .unwrap_or(800);
    let reg_height = 600u16;
    const SCALE: u16 = 1;
    svg.push_str(&format!("<svg class=\"idata\" width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">", reg_width / SCALE, reg_height / SCALE));
    svg.push_str(&format!("<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"black\"/>", reg_width / SCALE, reg_height / SCALE));
    for (x, y, bg) in entry.region.as_ref().iter().flat_map(|i| i.bg_reference.iter()) {
        svg.push_str(&format!("<image x=\"{x}\" y=\"{y}\" href=\"../../../exported/{bg}\"/>"));
    }
    for (x, y, bg) in res.entries.get(scene.as_ref()).and_then(|e| e.scene.as_ref()).into_iter().flat_map(|s| s.bg_reference.iter()) {
        svg.push_str(&format!("<image x=\"{x}\" y=\"{y}\" href=\"../../../exported/{bg}\"/>"));
    }
    for (shape_idx, points) in shapes.into_iter().enumerate() {
        svg.push_str("<path d=\"M");
        for (idx, (x, y)) in points.into_iter().enumerate() {
            if idx != 0 {
                svg.push('L');
            }
            svg.push_str(&format!("{} {}", x / SCALE, y / SCALE))
        }
        svg.push_str(&format!("Z\" fill=\"rgba(255,0,{},0.5)\"/>", shape_idx * 30));
    }
    svg.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"green\"/>", base_x / SCALE, base_y / SCALE, 5 / SCALE));
    svg.push_str("</svg>");

    output.finalise();
    Ok((svg, output))
}

pub fn analyse_dialogue_text<'a>(raw: &'a [u8], res: Resources<'a>) -> Result<DisCode<'a>, DisError> {
    let mut output = DisCode::new(raw, res.first_pass);

    let mut pos = 0;
    let mut chars = HashMap::new();
    for raw_line in raw.split(|b| *b == b'\n') {
        let mut trimmed_line = raw_line;
        if trimmed_line.ends_with(&[b'\r']) {
            trimmed_line = &trimmed_line[..trimmed_line.len() - 1];
        }
        let line = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(trimmed_line).unwrap();
        if line.starts_with("@") {
            if line.starts_with("@@") {
                output.line(pos, (pos + raw_line.len() + 1).min(raw.len()), None, Some(format!("<span class=\"hl-com\">{line}</span>")), None);
            } else if line.starts_with("@sound(") {
                use once_cell::sync::Lazy;
                use regex::Regex;
                static SOUND_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^@sound\(([^,]+), ?([^,]+)([0-9]{4}), ?([0-9]+)\)$").unwrap());
                let captures = SOUND_RE.captures(&line).unwrap();
                let char_name = captures.get(1).unwrap().as_str();
                let sound_id = captures.get(2).unwrap().as_str();
                let sound_num = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let advance = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
                chars.insert(char_name.to_ascii_lowercase(), (
                    sound_id.to_string(),
                    sound_num,
                    advance,
                ));
                output.line(pos, (pos + raw_line.len() + 1).min(raw.len()), None, Some(format!("<span class=\"hl-dyn\">{line}</span>")), None);
            } else if let Some((sound_id, sound_num, advance)) = chars.get_mut(&line[1..].to_ascii_lowercase()) {
                let sound_file = format!("{sound_id}{sound_num:04}.ogg");
                output.line(pos, (pos + raw_line.len() + 1).min(raw.len()), None, Some(format!("<span class=\"hl-dyn\">{line}</span> {}", show_data(&sound_file, res))), None);
                *sound_num += 1; // *advance;
            } else {
                // ...
                output.line(pos, (pos + raw_line.len() + 1).min(raw.len()), None, Some(line.to_string()), None);
            }
        } else {
            output.line(pos, (pos + raw_line.len() + 1).min(raw.len()), None, Some(line.to_string()), None);
        }
        pos += raw_line.len() + 1;
    }

    output.finalise();
    Ok(output)
}

pub fn analyse_raw<'a>(code: &'a [u8], res: Resources<'a>) -> Result<DisCode<'a>, DisError> {
    let mut output = DisCode::new(code, res.first_pass);
    let s = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(code).unwrap();
    output.line(0, code.len(), None, Some(show_string(&s, res)), None);
    output.finalise();
    Ok(output)
}

pub fn analyse_string<'a>(raw: &'a [u8], decoded: &str, res: Resources<'a>) -> Result<DisCode<'a>, DisError> {
    let mut output = DisCode::new(raw, res.first_pass);
    output.line(0, raw.len(), None, Some(show_string(decoded, res)), None);
    output.finalise();
    Ok(output)
}

pub fn analyse_code<'a>(code: &'a [u8], res: Resources<'a>) -> Result<(Option<String>, DisCode<'a>), DisError> {
    let mut output = DisCode::new(code, res.first_pass);
    if code.len() < 0x18 {
        return Err(DisError::TooShort);
    }

    // header
    if &code[8..12] != b"\xAD\xDE\x0C\x00" {
        return Err(DisError::MagicMismatch);
    }
    // TODO: maybe more magic?
    let size = u16::from_le_bytes(code[4..6].try_into().unwrap()) as usize + 7;
    if size != code.len() {
        return Err(DisError::LengthMismatch);
    }
    let code_size = u16::from_le_bytes(code[0x12..0x14].try_into().unwrap()) as usize;
    let string_count = u16::from_le_bytes(code[0x14..0x16].try_into().unwrap()) as usize;

    output.line(4, 6, Some(format!("object size: {size} / 0x{size:04x} bytes")), None, None);
    output.line(8, 12, Some("magic".to_string()), None, None);
    output.line(0x12, 0x14, Some(format!("code size: {code_size} / 0x{code_size:04x} bytes")), None, None);
    output.line(0x14, 0x16, Some(format!("string count: {string_count}")), None, None);

    // string pool
    let string_pool_start = 0x18 + code_size;
    let string_pool_meta = 5 + 4 * string_count;
    output.line(string_pool_start, string_pool_start + string_pool_meta, None, None, Some(format!("string pool metadata")));
    let mut strings = Vec::new();
    {
        let mut pos = string_pool_start + string_pool_meta;
        output.line(pos, pos, None, None, Some(format!("string pool start: {string_count} strings")));
        for string_idx in 0..string_count {
            let string_end = pos + code[pos..].iter().position(|b| *b == 0).expect("unterminated string");
            let s = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(&code[pos..string_end]).unwrap();
            output.line(pos, string_end + 1, Some(show_string(&s, res)), None, Some(format!("string #{string_idx} / 0x{string_idx:02x}")));
            strings.push(s.to_string());
            pos = string_end + 1;
        }
    };

    // code
    {
        let code_start = 0x18;
        let code_end = string_pool_start;
        output.line(code_start, code_start, None, None, Some("<span class=\"jump\"></span>code start".to_string()));
        match output.with_offset(code_start, |output| code::analyse(&code[code_start..code_end], code_start, &strings[..], res, output)) {
            Ok(pretty) => {
                if !pretty.is_empty() {
                    return Ok((Some(pretty), output));
                }
            },
            Err(err) => {
                output.error = true;
                output.line(
                    code_start,
                    code_start,
                    None,
                    Some("<span class=\"hl-err\">ERROR</span>".to_string()),
                    Some(format!("analysis failed: {err:?}")),
                );
            }
        }
    };

    Ok((None, output))
}

pub fn analyse_dummy(res: Resources<'_>) -> Result<DisCode<'_>, DisError> {
    let mut output = DisCode::new(&[], res.first_pass);
    output.finalise();
    Ok(output)
}
