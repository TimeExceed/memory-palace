use crate::*;
use std::{fmt::Write, path::Path};

pub(super) fn typst(input: &Path, output: &Path) {
    let items = read_file(input);
    let mut buf = String::new();
    writeln!(
        &mut buf,
        "\
#table(
    columns: (1cm, auto, auto),
    table.header([], [*Q*], [*A*]),
"
    )
    .unwrap();
    for (i, item) in items.iter().enumerate() {
        writeln!(
            &mut buf,
            "    [{}], [{}], [{}],",
            i + 1,
            item.question.trim().replace("\n", "\\\n"),
            item.answer.trim().replace("\n", "\\\n")
        )
        .unwrap();
    }
    writeln!(&mut buf, ")").unwrap();
    std::fs::write(output, buf).unwrap();
}
