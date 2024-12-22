#![allow(clippy::manual_range_contains)]
#![feature(box_patterns)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]
#![feature(option_get_or_insert_default)]
#![feature(round_char_boundary)]

use clap::{Parser, Subcommand};
use sailfish::Template;
use templates::nav::NavTree;
use std::{collections::{HashMap, HashSet}, path::PathBuf};

mod adb;
mod dis;
mod grp;
mod known;
mod patches;
mod templates;
mod xor;

use adb::{AdbEntry, AdbEntryKind, AdbXref, AdbXrefKind};

#[derive(Clone, Copy)]
pub struct Resources<'a> {
    entries: &'a HashMap<String, AdbEntry>,
    data: &'a HashMap<String, (String, String, String)>,
    do_analyse: bool,
    first_pass: bool,
}


#[derive(Parser)]
#[command(name = "re-posel-analyser")]
#[command(about = "Analyser and patcher for Posel Smrti / Black Mirror game files.", long_about = None)]
// #[command(version, about, long_about = None)]
struct Cli {
    /// When provided, only the specified patches will be applied. Only has
    /// effect when decompiling and creating patched .adb files.
    #[arg(long)]
    patch: Vec<String>,

    #[command(subcommand)]
    command: Option<CliCommand>,
}

#[derive(Subcommand)]
enum CliCommand {
    #[command(about = "Extract assets from a .grp file.", long_about = None)]
    Extract {
        /// Path to a .grp file.
        input: PathBuf,

        /// When provided, only the specified filenames will be extracted.
        #[arg(long)]
        name: Vec<String>,

        /// Output path: a directory will be created at this path, if one does
        /// not exist, and the selected assets will be extracted into it.
        output: PathBuf,
    },

    #[command(about = "Decompile a .adb file into objects.", long_about = None)]
    Decompile {
        /// Path to the original data.adb file.
        input: PathBuf,

        /// Path to an extracted asset group. The first value is the name of
        /// the group (e.g., "gfx1.grp"), the second is the path to it.
        #[arg(long)]
        #[arg(num_args(2..=2))]
        group: Vec<PathBuf>,

        /// When provided, only the given objects will be decompiled. This
        /// value is a regular expression.
        #[arg(long)]
        filter: Option<String>,

        /// When provided, bytecode will be decompiled into readable script.
        /// This may be slow on larger objects.
        #[arg(long)]
        analyse: bool,

        /// When provided, cross references will be identified.
        #[arg(long)]
        crossref: bool,

        /// Output path: a directory will be created at this path, if one does
        /// not exist, and the selected objects will be decompiled into it.
        output: PathBuf,
    },

    #[command(about = "Create a patched .adb file.", long_about = None)]
    Patch {
        /// Path to original data.adb file.
        input: PathBuf,

        /// Path to target data.adb file. Cannot be the same as input.
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    let Some(command) = cli.command else {
        println!("no command specified, exiting");
        return;
    };

    // For extraction, we don't need an ADB input.
    if let CliCommand::Extract { input, name, output } = command {
        println!("extracting .grp file {input:?} into {output:?} ...");
        let grp = std::fs::read(input).unwrap();
        std::fs::create_dir_all(&output).unwrap();
        let filter = (!name.is_empty()).then(|| name.into_iter().collect::<HashSet<String>>());
        let mut output = output.clone();
        for (name, data) in grp::extract(grp, filter) {
            println!("  {name} ({} bytes) ...", data.len());
            output.push(name);
            std::fs::write(&output, data).unwrap();
            output.pop();
        }
        return;
    }

    // Prepare the selected patches.
    let mut patcher = patches::Patcher::new();
    let mut patch_count = 0;
    let patch_filter = (!cli.patch.is_empty()).then(|| cli.patch.into_iter().collect::<HashSet<String>>());
    for patch in patches::ACTIVE_PATCHES {
        if patch_filter.as_ref().map(|f| !f.contains(patch.name)).unwrap_or(false) {
            continue;
        }
        patcher.add_patch(patch);
        patch_count += 1;
    }
    println!("{patch_count} patches ready");

    // Read .adb file.
    let db_path = match &command {
        CliCommand::Decompile { input, .. }
        | CliCommand::Patch { input, .. } => input.clone(),
        _ => unreachable!(),
    };
    let db = std::fs::read(&db_path).unwrap();

    match command {
        CliCommand::Decompile { group, filter, output, analyse: do_analyse, crossref: do_xref, .. } => {
            // Load objects.
            let mut entries = adb::extract(db).collect::<HashMap<_, _>>();
            println!("{} objects loaded from .adb file", entries.len());

            // Read .grp files.
            let mut data = HashMap::new();
            assert_eq!(group.len() % 2, 0);
            for [name, path] in group.into_iter().array_chunks() {
                let grp_name = name.to_string_lossy().to_string();
                for file in std::fs::read_dir(path).unwrap() {
                    let file = file.unwrap();
                    let name = file.file_name().into_string().unwrap();
                    data.insert(name.to_ascii_lowercase(), (grp_name.clone(), format!("{grp_name}/{name}"), name));
                }
            }
            println!("{} assets in .grp file(s)", data.len());

            // First pass: find cross references in code objects and regions.
            if do_xref {
                let res = Resources {
                    entries: &entries,
                    data: &data,
                    do_analyse: false,
                    first_pass: true,
                };
                let mut xrefs = Vec::new();
                for (key, entry) in &entries {
                    xrefs.extend(match &entry.kind {
                        AdbEntryKind::Code(c) => {
                            patcher.with_data(key, c, |c, _patches| {
                                dis::analyse_code(c, res).unwrap().1.finalise_xrefs()
                            })
                        }
                        _ => continue,
                        /*
                        AdbEntryKind::String { raw, decoded, .. } => {
                            dis::analyse_string(raw, decoded, res).unwrap();
                        }
                        AdbEntryKind::Raw(c) => {
                            if entry.region.is_some() || key.ends_with(".r") || key.ends_with(".rp") {
                                dis::analyse_region(entry, res).unwrap();
                            } else {
                                dis::analyse_raw(c, res).unwrap();
                            }
                        }
                        AdbEntryKind::Dummy
                        | AdbEntryKind::Global => {
                            dis::analyse_dummy(res).unwrap();
                        }
                        */
                    }.into_iter().map(|xref| (key.to_string(), xref)));
                }

                // Process cross references from code objects.
                for (from, xref) in xrefs {
                    entries.entry(xref.other_key)
                        .or_insert_with(|| AdbEntry::new(match xref.kind {
                            AdbXrefKind::GlobalR | AdbXrefKind::GlobalW => AdbEntryKind::Global,
                            AdbXrefKind::Scene => AdbEntryKind::Scene,
                            _ => AdbEntryKind::Dummy,
                        }))
                        .xrefs
                        .push(AdbXref {
                            other_key: from.to_string(),
                            loc: xref.loc,
                            kind: xref.kind,
                        });
                }

                // Find references to scenes in regions.
                let res = Resources {
                    entries: &entries,
                    data: &data,
                    do_analyse: false,
                    first_pass: true,
                };
                let mut xrefs = Vec::new();
                for (key, entry) in &entries {
                    xrefs.extend(match &entry.kind {
                        AdbEntryKind::Raw(..) if entry.is_region(key) => {
                            dis::analyse_region(entry, res).unwrap().1.finalise_xrefs()
                        }
                        _ => continue,
                    }.into_iter().map(|xref| (key.to_string(), xref)));
                }

                // Process cross references from code objects.
                // TODO: code duplication
                for (from, xref) in xrefs {
                    entries.entry(xref.other_key)
                        .or_insert_with(|| AdbEntry::new(match xref.kind {
                            AdbXrefKind::GlobalR | AdbXrefKind::GlobalW => AdbEntryKind::Global,
                            AdbXrefKind::Scene => AdbEntryKind::Scene,
                            _ => AdbEntryKind::Dummy,
                        }))
                        .xrefs
                        .push(AdbXref {
                            other_key: from.to_string(),
                            loc: xref.loc,
                            kind: xref.kind,
                        });
                }
            }

            // Create hierarchy.
            let mut root = NavTree {
                key: "".to_string(),
                kind: "root",
                children: HashMap::new(),
            };
            for (key, entry) in &entries {
                root.add(key.split('.'), entry.describe(key));
            }
            root.add_dummies(&mut Vec::new(), &mut entries);

            // Apply known labels to objects.
            known::apply_known(&mut root, &mut entries);

            // Second pass: output decompiled objects.
            let mut output = output.clone();
            std::fs::create_dir_all(&output).unwrap();

            let res = Resources {
                entries: &entries,
                data: &data,
                do_analyse,
                first_pass: false,
            };
            let mut count_string = 0;
            let mut count_raw = 0;
            let mut count_region = 0;
            let mut count_code = 0;
            let mut count_code_error = 0;
            let mut count_global = 0;
            let mut count_dummy = 0;
            let mut count_scene = 0;
            let entry_filter = filter.map(|pat| regex::Regex::new(&pat).unwrap());
            for (key, entry) in &entries {
                if let Some(re) = entry_filter.as_ref() {
                    if !re.is_match(key) {
                        continue;
                    }
                }
                let mut prefix_full = String::new();
                let key_parts = key.split('.').collect::<Vec<_>>();
                let rendered_breadcrumbs = key_parts.iter()
                    .enumerate()
                    .map(|(i, comp)| {
                        let res = if i == 0 {
                            format!("<a href=\"{prefix_full}{comp}.html\">{comp}</a>")
                        } else {
                            format!(" / <a href=\"{prefix_full}{comp}.html\">{comp}</a>")
                        };
                        prefix_full.push_str(comp);
                        prefix_full.push('.');
                        res
                    })
                    .collect();
                println!("  {key} ({}, {} bytes)", entry.describe(key), entry.size());
                let mut pretty = None;
                let code = match &entry.kind {
                    AdbEntryKind::String { raw, .. } if entry.is_dialogue_text() => {
                        count_string += 1; // TODO
                        dis::analyse_dialogue_text(raw, res).unwrap()
                    }
                    AdbEntryKind::String { raw, decoded, .. } => {
                        count_string += 1;
                        dis::analyse_string(raw, decoded, res).unwrap()
                    }
                    AdbEntryKind::Raw(_) if entry.is_region(key) => {
                        count_region += 1;
                        let (p, code) = dis::analyse_region(entry, res).unwrap();
                        pretty = Some(p);
                        code
                    }
                    AdbEntryKind::Raw(c) => {
                        count_raw += 1;
                        dis::analyse_raw(c, res).unwrap()
                    }
                    AdbEntryKind::Code(c) => {
                        count_code += 1;
                        patcher.with_data(key, c, |c, patches| {
                            let (p, code) = dis::analyse_code(c, res).unwrap();
                            pretty = p;
                            if code.error {
                                println!("    code error!");
                                count_code_error += 1;
                            }
                            code.finalise_with_patches(patches)
                        })
                    }
                    AdbEntryKind::Global => {
                        count_global += 1;
                        dis::analyse_dummy(res).unwrap()
                    }
                    AdbEntryKind::Dummy => {
                        count_dummy += 1;
                        dis::analyse_dummy(res).unwrap()
                    }
                    AdbEntryKind::Scene => {
                        count_scene += 1;
                        dis::analyse_dummy(res).unwrap()
                    }
                };
                let hierarchy = root.get(key_parts[0]).flatten();
                let rendered_hierarchy = hierarchy.render(key, &entries);
                output.push(format!("{key}.html"));
                let mut sorted_xrefs = entry.xrefs.clone();
                sorted_xrefs.sort_by_cached_key(|xref| (xref.other_key.clone(), xref.loc));
                std::fs::write(&output, templates::Bytecode {
                    title: key.to_string(),
                    kind: &entry.kind,
                    rendered_hierarchy: &rendered_hierarchy,
                    rendered_breadcrumbs,
                    code,
                    pretty,
                    xrefs: sorted_xrefs,
                }.render().unwrap()).unwrap();
                output.pop();
            }
            println!("code:    {count_code}, errored: {count_code_error}");
            println!("globals: {count_global}");
            println!("dummy:   {count_dummy}");
            println!("raw:     {count_raw}");
            println!("regions: {count_region}");
            println!("strings: {count_string}");
            println!("scenes:  {count_scene}");
        }
        CliCommand::Patch { output, .. } => {
            assert_ne!(db_path, output);
            std::fs::write(&output, adb::create_patched(db, patcher)).unwrap();
            println!("patched .adb file written to {output:?}");
        }
        _ => unreachable!(),
    }
}
