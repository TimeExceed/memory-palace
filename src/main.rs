use chrono::prelude::*;
use clap::{crate_name, crate_version, value_parser, Arg, ArgAction, Command};
use clap_complete::aot as completion;
use log::*;
use memory_palace::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    flexi_logger::Logger::try_with_env_or_str("error, memory_palace=info")
        .unwrap()
        .use_utc()
        .adaptive_format_for_stderr(flexi_logger::AdaptiveFormat::Detailed)
        .start()
        .unwrap();
    let args = parse_args();
    let items = read_file(&args.file_name);
    let now = Utc::now();
    let selected = if let Some(ref selected_in) = args.selected_in {
        Selected::read_back(items, selected_in)
    } else {
        Selected::new(items, &now, args.take)
    };
    let selected = Rc::new(RefCell::new(selected));
    gui::App::start(&args.file_name, selected.clone());
    if let Some(ref selected_out) = args.selected_out {
        selected.borrow().write_out(selected_out);
    }
    let items = selected.borrow_mut().feedback(&now);
    if args.dry_run {
        debug!("dry run!");
    } else {
        write_out(&args.file_name, items);
    }
}

fn parse_args() -> Args {
    const COMPLETION: &str = "COMPLETION";
    const FILE_NAME: &str = "FILE_NAME";
    const TAKE: &str = "TAKE";
    const DRY_RUN: &str = "DRY-RUN";
    const SELECTED_IN: &str = "SELECTED-IN";
    const SELECTED_OUT: &str = "SELECTED-OUT";
    let mut cmd = Command::new(crate_name!())
        .about("Pick up something in the memory palace.")
        .version(crate_version!())
        .arg(
            Arg::new(COMPLETION)
                .value_name("SHELL")
                .help("Generate the completion file.")
                .long("completion")
                .action(ArgAction::Set)
                .value_parser(value_parser!(completion::Shell)),
        )
        .arg(
            Arg::new(FILE_NAME)
                .help("the file of a memory palace.")
                .action(ArgAction::Set)
                .required_unless_present(COMPLETION),
        )
        .arg(
            Arg::new(TAKE)
                .value_name("N")
                .help("Take at most <N> things to remember.")
                .long("take")
                .action(ArgAction::Set)
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new(DRY_RUN)
                .help("Do everything except writing back.")
                .long("dry-run")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(SELECTED_IN)
                .value_name("SELECTED-FILE")
                .help("Read back the selected items.")
                .long("selected-in")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new(SELECTED_OUT)
                .value_name("SELECTED-FILE")
                .help("Write out the selected items.")
                .long("selected-out")
                .action(ArgAction::Set),
        );
    let matches = cmd.clone().get_matches();
    if let Some(gen) = matches.get_one::<completion::Shell>(COMPLETION).copied() {
        let cmd_name = cmd.get_name().to_string();
        completion::generate(gen, &mut cmd, cmd_name, &mut std::io::stdout());
        std::process::exit(0);
    }
    let file_name = matches.get_one::<String>(FILE_NAME).unwrap().clone();
    let take = matches.get_one::<usize>(TAKE).copied();
    let dry_run = matches.get_flag(DRY_RUN);
    let selected_in = matches.get_one::<String>(SELECTED_IN).cloned();
    let selected_out = matches.get_one::<String>(SELECTED_OUT).cloned();
    Args {
        file_name,
        dry_run,
        take,
        selected_in,
        selected_out,
    }
}

struct Args {
    /// the file of a memory palace.
    file_name: String,

    /// Take at most `TAKE` things to remember.
    take: Option<usize>,

    /// Do everything except writing back.
    dry_run: bool,

    /// Write out the selected items.
    selected_out: Option<String>,

    /// Read back the selected items.
    selected_in: Option<String>,
}
