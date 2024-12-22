mod code;
mod error;
mod lines;

pub use error::*;
pub use lines::*;

use crate::{adb::{AdbEntry, AdbEntryKind}, Resources};

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
    let (_grp, res_path, file_name) = res.data.get(&path.to_ascii_lowercase()).unwrap();
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
    }
}

pub fn analyse_region<'a>(entry: &'a AdbEntry, res: Resources<'a>) -> Result<DisCode<'a>, DisError> {
    let code = entry.raw();
    let mut output = DisCode::new(code, res.first_pass);
    if code.len() < 0x26 {
        return Err(DisError::TooShort);
    }

    // header
    let name_end = code.iter().position(|b| *b == 0).unwrap_or(0x20);
    let s = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(&code[0..name_end]).unwrap();
    output.line(0, name_end, Some(format!("name?: {s}")), None, None);
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
    let reg_width = entry.region.as_ref().and_then(|r| r.width).map(|w| w as u16).unwrap_or(800);
    let reg_height = 600u16;
    const SCALE: u16 = 1;
    svg.push_str(&format!("<svg class=\"idata\" width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">", reg_width / SCALE, reg_height / SCALE));
    svg.push_str(&format!("<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"black\"/>", reg_width / SCALE, reg_height / SCALE));
    for (x, y, bg) in entry.region.as_ref().iter().flat_map(|i| i.bg_reference.iter()) {
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
    output.line(pos, pos, Some(svg), None, None);

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
    let string_count = u16::from_le_bytes(code[0x14..0x16].try_into().unwrap()) as usize;

    output.line(4, 6, Some(format!("size: {size} / 0x{size:04x} bytes")), None, None);
    output.line(8, 12, Some("magic".to_string()), None, None);
    output.line(0x14, 0x16, Some(format!("string count: {string_count}")), None, None);

    // string pool
    let string_pool_start = code.len() - 1;
    let mut strings = Vec::new();
    {
        let mut pos = code.len() - 1;
        for i in 0..string_count {
            let string_idx = string_count - i - 1;
            if code[pos] != 0 {
                return Err(DisError::MalformedString);
            }
            let string_end = pos;
            pos -= 1;
            while pos > 0x18 && code[pos] != 0 {
                pos -= 1;
            }
            if pos == 0x18 {
                return Err(DisError::MalformedString);
            }
            if i == string_count - 1 && !(0x21 <= code[pos + 1] && code[pos + 1] < 0x7F) {
                pos += 1;
            }
            let s = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(&code[pos + 1..string_end]).unwrap();
            output.line(pos + 1, string_end + 1, Some(show_string(&s, res)), None, Some(format!("string #{string_idx} / 0x{string_idx:02x}")));
            strings.push(s.to_string());
        }
        output.line(pos + 1, pos + 1, None, None, Some(format!("string pool start: {string_count} strings")));
        strings.reverse();
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
