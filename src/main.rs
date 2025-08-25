use clap::{Arg, ArgAction, Command, crate_name, crate_version, value_parser};
use clap_complete::aot as completion;
use memory_palace::{exam::Exam, print::Print, select::Select, update::Update};
use std::{collections::HashSet, path::PathBuf};

fn main() {
    flexi_logger::Logger::try_with_env_or_str("error, memory_palace=info")
        .unwrap()
        .use_utc()
        .adaptive_format_for_stderr(flexi_logger::AdaptiveFormat::Detailed)
        .start()
        .unwrap();
    let args = parse_args();
    match args {
        Args::Exam(exam) => {
            exam.gogogo();
        }
        Args::Select(select) => {
            select.gogogo();
        }
        Args::Print(print) => {
            print.gogogo();
        }
        Args::Update(update) => {
            update.gogogo();
        }
    }
}

fn parse_args() -> Args {
    const COMPLETION: &str = "completion";
    const COMPLETION_SHELL: &str = "completion/SHELL";
    const EXAM: &str = "exam";
    const EXAM_FILE_NAME: &str = "exam/FILE_NAME";
    const EXAM_TAKE: &str = "exam/TAKE";
    const EXAM_DRY_RUN: &str = "exam/DRY-RUN";
    const EXAM_SORT: &str = "exam/SORT";
    const SELECT: &str = "select";
    const SELECT_IN: &str = "select/IN-FILE";
    const SELECT_OUT: &str = "select/OUT-FILE";
    const SELECT_TIMEOUT: &str = "select/TIMEOUT";
    const SELECT_TAKE: &str = "select/TAKE";
    const SELECT_TAGS: &str = "select/TAGS";
    const SELECT_SORT: &str = "select/SORT";
    const PRINT: &str = "print";
    const PRINT_TYPST: &str = "typst";
    const PRINT_TYPST_INPUT: &str = "print/typst/INPUT";
    const PRINT_TYPST_OUTPUT: &str = "print/typst/OUTPUT";
    const UPDATE: &str = "update";
    const UPDATE_INTO: &str = "update/INTO";
    const UPDATE_FROM: &str = "update/FROM";

    let mut cmd = Command::new(crate_name!())
        .about("Do an exam in the memory palace.")
        .version(crate_version!())
        .subcommand_required(true)
        .subcommand(
            Command::new(EXAM)
                .about("Do an exam.")
                .arg(
                    Arg::new(EXAM_FILE_NAME)
                        .help("the file of a memory palace.")
                        .action(ArgAction::Set)
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new(EXAM_TAKE)
                        .value_name("N")
                        .help("Take at most <N> items to go over.")
                        .long("take")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(usize)),
                )
                .arg(
                    Arg::new(EXAM_DRY_RUN)
                        .help("Do everything except writing back to disks.")
                        .long("dry-run")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new(EXAM_SORT)
                        .help("Sort items.")
                        .long("sort")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new(SELECT)
                .about("Select some items.")
                .arg(
                    Arg::new(SELECT_IN)
                        .value_name("IN-FILE")
                        .help("the file of a memory palace.")
                        .required(true)
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new(SELECT_OUT)
                        .value_name("OUT-FILE")
                        .help("the file to be appended.")
                        .required(true)
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new(SELECT_TIMEOUT)
                        .help("selects timed out items only.")
                        .long("timeout")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new(SELECT_TAKE)
                        .value_name("N")
                        .help("takes randomly at most <N> items.")
                        .long("take")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(usize)),
                )
                .arg(
                    Arg::new(SELECT_TAGS)
                        .value_name("TAG")
                        .help("selects only items with one of the specified tags.")
                        .long("tag")
                        .num_args(1..)
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new(SELECT_SORT)
                        .help("sorts the selected items.")
                        .long("sort")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new(PRINT)
                .about("Prints a memory palace.")
                .subcommand_required(true)
                .subcommand(
                    Command::new(PRINT_TYPST)
                        .about("Prints a memory palace in typst format.")
                        .arg(
                            Arg::new(PRINT_TYPST_INPUT)
                                .value_name("INPUT")
                                .help("the file of a memory palace.")
                                .required(true)
                                .action(ArgAction::Set)
                                .value_parser(value_parser!(PathBuf)),
                        )
                        .arg(
                            Arg::new(PRINT_TYPST_OUTPUT)
                                .value_name("OUTPUT")
                                .help("the typst file to be printed.")
                                .required(true)
                                .action(ArgAction::Set)
                                .value_parser(value_parser!(PathBuf)),
                        ),
                ),
        )
        .subcommand(
            Command::new(UPDATE)
                .about("Updates several memory-palace files from one.")
                .arg(
                    Arg::new(UPDATE_INTO)
                        .value_name("INTO")
                        .help("the files to be merged into.")
                        .long("into")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new(UPDATE_FROM)
                        .value_name("FROM")
                        .help("the file to merge from.")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new(COMPLETION)
                .about("Generate the completion file.")
                .arg(
                    Arg::new(COMPLETION_SHELL)
                        .value_name("SHELL")
                        .help("Which shell is the completion file for.")
                        .action(ArgAction::Set)
                        .required(true)
                        .value_parser(value_parser!(completion::Shell)),
                ),
        );
    let matches = cmd.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches(COMPLETION) {
        if let Some(sh) = matches
            .get_one::<completion::Shell>(COMPLETION_SHELL)
            .copied()
        {
            let cmd_name = cmd.get_name().to_string();
            completion::generate(sh, &mut cmd, cmd_name, &mut std::io::stdout());
            std::process::exit(0);
        }
    }
    if let Some(matches) = matches.subcommand_matches(EXAM) {
        let file_name = matches.get_one::<PathBuf>(EXAM_FILE_NAME).unwrap().clone();
        let take = matches.get_one::<usize>(EXAM_TAKE).copied();
        let dry_run = matches.get_flag(EXAM_DRY_RUN);
        let sort = matches.get_flag(EXAM_SORT);
        return Args::Exam(Exam {
            file_name,
            dry_run,
            take,
            sort,
        });
    }
    if let Some(matches) = matches.subcommand_matches(SELECT) {
        let input = matches.get_one::<PathBuf>(SELECT_IN).unwrap().clone();
        let output = matches.get_one::<PathBuf>(SELECT_OUT).unwrap().clone();
        let take = matches.get_one::<usize>(SELECT_TAKE).copied();
        let timeout = matches.get_flag(SELECT_TIMEOUT);
        let tags = matches
            .get_many(SELECT_TAGS)
            .map(|tags| tags.cloned().collect::<HashSet<_>>());
        let sort = matches.get_flag(SELECT_SORT);
        return Args::Select(Select {
            input,
            output,
            take,
            timeout,
            tags,
            sort,
        });
    }
    if let Some(matches) = matches.subcommand_matches(PRINT) {
        if let Some(matches) = matches.subcommand_matches(PRINT_TYPST) {
            let input = matches
                .get_one::<PathBuf>(PRINT_TYPST_INPUT)
                .unwrap()
                .clone();
            let output = matches
                .get_one::<PathBuf>(PRINT_TYPST_OUTPUT)
                .unwrap()
                .clone();
            return Args::Print(Print::Typst { input, output });
        }
    }
    if let Some(matches) = matches.subcommand_matches(UPDATE) {
        let into: Vec<_> = matches
            .get_many::<PathBuf>(UPDATE_INTO)
            .unwrap()
            .cloned()
            .collect();
        let from = matches.get_one::<PathBuf>(UPDATE_FROM).unwrap().clone();
        return Args::Update(Update { into, from });
    }
    unreachable!()
}

enum Args {
    Exam(Exam),
    Select(Select),
    Print(Print),
    Update(Update),
}
