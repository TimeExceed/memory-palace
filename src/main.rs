use chrono::prelude::*;
use clap::Parser;
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
    let args = Args::parse();
    let items = read_file(&args.file_name);
    let now = Utc::now();
    let selected = Rc::new(RefCell::new(Selected::new(items, &now, args.take)));
    gui::App::start(&args.file_name, selected.clone());
    let items = selected.borrow_mut().feedback(&now);
    if args.dry_run {
        debug!("dry run!");
    } else {
        write_out(&args.file_name, items);
    }
}

/// Pick up something in the memory palace.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the file of a memory palace.
    #[arg()]
    file_name: String,

    /// Take at most `TAKE` things to remember.
    #[arg(long)]
    take: Option<usize>,

    /// Do everything except writing back.
    #[arg(long)]
    dry_run: bool,
}
