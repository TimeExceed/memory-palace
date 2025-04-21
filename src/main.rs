use clap::{Arg, ArgAction, Command, crate_name, crate_version, value_parser};
use clap_complete::aot as completion;
use memory_palace::{exam::Exam, print::Print, select::Select};
use std::collections::HashSet;

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
    }
}

fn parse_args() -> Args {
    const COMPLETION: &str = "COMPLETION";
    const EXAM_FILE_NAME: &str = "exam/FILE_NAME";
    const EXAM_TAKE: &str = "exam/TAKE";
    const EXAM_DRY_RUN: &str = "exam/DRY-RUN";
    const SELECT_IN: &str = "select/IN-FILE";
    const SELECT_OUT: &str = "select/OUT-FILE";
    const SELECT_TIMEOUT: &str = "select/TIMEOUT";
    const SELECT_TAKE: &str = "select/TAKE";
    const SELECT_TAGS: &str = "select/TAGS";
    const PRINT_TYPST_INPUT: &str = "print/typst/INPUT";
    const PRINT_TYPST_OUTPUT: &str = "print/typst/OUTPUT";

    let mut cmd = Command::new(crate_name!())
        .about("Do an exam in the memory palace.")
        .version(crate_version!())
        .subcommand_required(true)
        .subcommand(
            Command::new("exam")
                .about("Do an exam.")
                .arg(
                    Arg::new(EXAM_FILE_NAME)
                        .help("the file of a memory palace.")
                        .action(ArgAction::Set)
                        .required(true),
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
                ),
        )
        .subcommand(
            Command::new("select")
                .about("Select some items.")
                .arg(
                    Arg::new(SELECT_IN)
                        .value_name("IN-FILE")
                        .help("the file of a memory palace.")
                        .required(true)
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new(SELECT_OUT)
                        .value_name("OUT-FILE")
                        .help("the file to be appended.")
                        .required(true)
                        .action(ArgAction::Set),
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
                ),
        )
        .subcommand(
            Command::new("print")
                .about("Prints a memory palace.")
                .subcommand_required(true)
                .subcommand(
                    Command::new("typst")
                        .about("Prints a memory palace in typst format.")
                        .arg(
                            Arg::new(PRINT_TYPST_INPUT)
                                .value_name("INPUT")
                                .help("the file of a memory palace.")
                                .required(true)
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new(PRINT_TYPST_OUTPUT)
                                .value_name("OUTPUT")
                                .help("the typst file to be printed.")
                                .required(true)
                                .action(ArgAction::Set),
                        ),
                ),
        )
        .subcommand(
            Command::new("complete")
                .about("Generate the completion file.")
                .arg(
                    Arg::new(COMPLETION)
                        .value_name("SHELL")
                        .help("Which shell is the completion file for.")
                        .action(ArgAction::Set)
                        .required(true)
                        .value_parser(value_parser!(completion::Shell)),
                ),
        );
    let matches = cmd.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches("complete") {
        if let Some(sh) = matches.get_one::<completion::Shell>(COMPLETION).copied() {
            let cmd_name = cmd.get_name().to_string();
            completion::generate(sh, &mut cmd, cmd_name, &mut std::io::stdout());
            std::process::exit(0);
        }
    }
    if let Some(matches) = matches.subcommand_matches("exam") {
        let file_name = matches.get_one::<String>(EXAM_FILE_NAME).unwrap().clone();
        let take = matches.get_one::<usize>(EXAM_TAKE).copied();
        let dry_run = matches.get_flag(EXAM_DRY_RUN);
        return Args::Exam(Exam {
            file_name,
            dry_run,
            take,
        });
    }
    if let Some(matches) = matches.subcommand_matches("select") {
        let input = matches.get_one::<String>(SELECT_IN).unwrap().clone();
        let output = matches.get_one::<String>(SELECT_OUT).unwrap().clone();
        let take = matches.get_one::<usize>(SELECT_TAKE).copied();
        let timeout = matches.get_flag(SELECT_TIMEOUT);
        let tags = matches
            .get_many(SELECT_TAGS)
            .map(|tags| tags.cloned().collect::<HashSet<_>>());
        return Args::Select(Select {
            input,
            output,
            take,
            timeout,
            tags,
        });
    }
    if let Some(matches) = matches.subcommand_matches("print") {
        if let Some(matches) = matches.subcommand_matches("typst") {
            let input = matches
                .get_one::<String>(PRINT_TYPST_INPUT)
                .unwrap()
                .clone();
            let output = matches
                .get_one::<String>(PRINT_TYPST_OUTPUT)
                .unwrap()
                .clone();
            return Args::Print(Print::Typst { input, output });
        }
    }
    unreachable!()
}

enum Args {
    Exam(Exam),
    Select(Select),
    Print(Print),
}
