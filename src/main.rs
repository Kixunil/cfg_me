#[macro_use]
extern crate configure_me;
extern crate configure_me_codegen;

use std::path::Path;
use std::process::{Child, ChildStdin};
use std::io::BufWriter;
use configure_me::include_config;

include_config!();

macro_rules! try_or_die {
    ($val:expr, $operation:expr) => {
        $val.unwrap_or_else(|err| {
            eprintln!("Failed to {}: {}", $operation, err);
            std::process::exit(1);
        })
    };
    ($val:expr, $operation:expr, $ignore:expr) => {
        $val.unwrap_or_else(|err| {
            if $ignore {
                std::process::exit(0);
            } else {
                eprintln!("Failed to {}: {}", $operation, err);
                std::process::exit(1);
            }
        })
    };
}

fn spawn_man() -> (Child, BufWriter<ChildStdin>) {
    let man = std::process::Command::new("man")
        .args(&["-l", "-"])
        .stdin(std::process::Stdio::piped())
        .spawn();
    let mut man = try_or_die!(man, "execute `man`");
    let output = man.stdin.take().expect("Bug: missing stdin");
    (man, std::io::BufWriter::new(output))
}

fn create(out_file_name: &Path) -> BufWriter<std::fs::File> {
    let output = try_or_die!(std::fs::File::create(out_file_name), "open output file");
    BufWriter::new(output)
}

fn generate_man_curr_dir<W: std::io::Write>(output: W, ignore_no_dep: bool) {
    use configure_me_codegen::manifest::{LoadManifest, CurrentDir, SpecificationPaths};
    use configure_me_codegen::generate_man;

    let manifest = try_or_die!(CurrentDir.load_manifest(), "load Cargo manfest");
    let ignore = if ignore_no_dep {
        !manifest
            .build_dependencies
            .iter()
            .any(|(name, _)| name == "configure_me_codegen")
    } else {
        false
    };
    let package = try_or_die!(manifest.package.as_ref().ok_or("missing package"), "get package", ignore);
    let metadata = try_or_die!(package.metadata.as_ref().ok_or("missing metadata"), "get package metadata", ignore);
    let configure_me = try_or_die!(metadata.configure_me.as_ref().ok_or("missing configure_me"), "get configure_me metadata", ignore);

    if let SpecificationPaths::Single(path) = &configure_me.spec_paths {
        try_or_die!(generate_man(path, output, &manifest), "generate man page");
    } else {
        eprintln!("Multi-binary crates not supported yet");
        std::process::exit(1);
    }
}

fn generate_man<W: std::io::Write>(input: &Path, output: W) {
    use configure_me_codegen::manifest::CurrentDir;

    try_or_die!(configure_me_codegen::generate_man(input, output, CurrentDir), "generate man page");
}

fn main() {
    let (config, mut args) = Config::including_optional_config_files(std::iter::empty::<&str>()).unwrap_or_exit();

    let subcommand = try_or_die!(args.next().ok_or("missing subcommand"), "run");

    if *subcommand == *"man" {
        match (config.input, config.output) {
            (None, None) => {
                let (mut child, output) = spawn_man();
                generate_man_curr_dir(output, config.ignore_no_dep);
                try_or_die!(child.wait(), "wait for child");
            },
            (Some(input), None) => {
                let (mut child, output) = spawn_man();
                generate_man(&input, output);
                try_or_die!(child.wait(), "wait for child");
            },
            (None, Some(output)) => {
                generate_man_curr_dir(create(&output), config.ignore_no_dep);
            },
            (Some(input), Some(output)) => generate_man(&input, create(&output)),
        }
    } else {
        eprintln!("Failed to run: unknown subcommand {:?}", subcommand);
        std::process::exit(1);
    }
}
