use codespan_reporting::diagnostic::Label;
use logos::Logos;

// TODO: change definition and all uses if codespan_reporting is not used
pub type Diagnostic = codespan_reporting::diagnostic::Diagnostic<()>;
type Span = core::ops::Range<usize>;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LexerError {
    #[default]
    Invalid,
    // TODO: add more errors if required
}

impl LexerError {
    pub fn into_diagnostic(self, span: Span) -> Diagnostic {
        match self {
            Self::Invalid => Diagnostic::error()
                .with_message("invalid token")
                .with_labels(vec![Label::primary((), span)]),
        }
    }
}

// TODO: implement lexer
#[allow(clippy::upper_case_acronyms)]
#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(error = LexerError)]
pub enum Token {
    EOF,
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("->")]
    Arrow,
    #[token("(")]
    LPar,
    #[token(")")]
    RPar,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(";")]
    Semi,
    #[token("=")]
    Asn,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[regex(r#"([_\p{XID_Start}][\p{XID_Continue}]+)|([\p{XID_Start}])"#)]
    Name,
    #[regex(r#"0|[1-9]\d*"#)]
    #[regex(r#"0[xX][\da-fA-F]+"#)]
    Int,
    #[regex(r"[\s\u0085\u200e\u200f\u2028\u2029]+")]
    Whitespace,
    Error,
}

// TODO: extend tokenization (e.g. check for mismatched parentheses)
pub fn tokenize(source: &str, diags: &mut Vec<Diagnostic>) -> (Vec<Token>, Vec<Span>) {
    let lexer = Token::lexer(source);
    let mut tokens = vec![];
    let mut spans = vec![];

    for (token, span) in lexer.spanned() {
        match token {
            Ok(token) => {
                tokens.push(token);
            }
            Err(err) => {
                diags.push(err.into_diagnostic(span.clone()));
                tokens.push(Token::Error);
            }
        }
        spans.push(span);
    }
    (tokens, spans)
}
