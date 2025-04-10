mod parser;

use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::{self, Config};
use parser::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        std::process::exit(1);
    }

    let source = std::fs::read_to_string(&args[1])?;
    let mut diags = vec![];
    let cst = Parser::parse(&source, &mut diags);
    println!("{cst}");

    let file = SimpleFile::new(&args[1], &source);
    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = Config::default();
    for diag in diags.iter() {
        term::emit(&mut writer.lock(), &config, &file, diag).unwrap();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    use codespan_reporting::files::SimpleFile;
    use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
    use codespan_reporting::term::{self, Config};
    use std::io::Write;

    fn parse_file(source: &str) {
        let mut diags = vec![];
        let cst = Parser::parse(&source, &mut diags);
        let mut writer = StandardStream::stderr(ColorChoice::Auto);
        writeln!(writer, "{cst}").unwrap();
        let file = SimpleFile::new("test file", &source);
        let config = Config::default();
        for diag in diags.iter() {
            term::emit(&mut writer.lock(), &config, &file, diag).unwrap();
        }
    }

    #[test]
    fn nya() {
        parse_file(
            "fn foo () {
        let a = 100;
        let b = c();
        }",
        );
    }
}
