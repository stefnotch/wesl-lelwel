use codespan_reporting::diagnostic::Label;
use lexer::{Diagnostic, Token, tokenize};

// TODO: add context information to the parser if required
#[derive(Default)]
pub struct Context<'a> {
    marker: std::marker::PhantomData<&'a ()>,
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

impl<'a> ParserCallbacks for Parser<'a> {
    fn create_tokens(source: &str, diags: &mut Vec<Diagnostic>) -> (Vec<Token>, Vec<Span>) {
        tokenize(source, diags)
    }
    fn create_diagnostic(&self, span: Span, message: String) -> Diagnostic {
        Diagnostic::error()
            .with_message(message)
            .with_labels(vec![Label::primary((), span)])
    }
    fn predicate_param_list_1(&self) -> bool {
        todo!()
    }
    fn predicate_arg_list_1(&self) -> bool {
        todo!()
    }
}
