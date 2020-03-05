extern crate combine;
extern crate combine_language;
use combine::char::{alpha_num, letter, string};
use combine::error::ParseError;
use combine::{parser, satisfy, ParseResult, Parser, Stream, StreamOnce};
use combine_language::{Identifier, LanguageDef, LanguageEnv};

#[test]
fn test_combine() {
    // assert_eq!(result, Ok(((Borrowed("identifier"), 42), "")));
    let mut input = "(input)";
    let result = combine(&mut input);
    println!("{:?}", result);
}

pub fn combine<'a, I>(input: &mut I) -> ParseResult<Box<Term>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    I: 'a,
    <I as StreamOnce>::Error: std::fmt::Debug,
{
    let mut parser = parser(term);
    let result = parser.parse_stream(input);
    println!("{:?}", result);
    result
}

fn calc_env<'a, I>() -> LanguageEnv<'a, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    I: 'a,
{
    LanguageEnv::new(LanguageDef {
        ident: Identifier {
            start: letter(),
            rest: alpha_num(),
            reserved: ["forall"].iter().map(|x| (*x).into()).collect(),
        },
        op: Identifier {
            start: satisfy(|c| "=-/".chars().any(|x| x == c)),
            rest: satisfy(|c| "=>\\-".chars().any(|x| x == c)),
            reserved: ["=", "->", "/\\", "-"]
                .iter()
                .map(|x| (*x).into())
                .collect(),
        },
        comment_start: string("/*").map(|_| ()),
        comment_end: string("*/").map(|_| ()),
        comment_line: string("//").map(|_| ()),
    })
}

fn term<I>(input: &mut I) -> ParseResult<Box<Term>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let env = calc_env();
    let parenthesized = env.parens(parser(term));
    let name = env.identifier().map(|name| Box::new(Term::Name(name)));
    // name.parse_stream(input)
    parenthesized.or(name).parse_stream(input)
}

// fn lambda_term<I>(input: I) -> ParseResult<Box<Term>, I>
// where
//     I: Stream<Item = char>,
//     I::Error: ParseError<I::Item, I::Range, I::Position>,
// {
//     let env = calc_env();
//     let parenthesized = env.parens(parser(term));
//     parenthesized.parse_stream(term)
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaTerm {
    bind: Vec<String>,
    formula: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Name(String),
    Lambda(LambdaTerm),
    Forall(LambdaTerm),
    Exists(LambdaTerm),
    Apply(Box<Term>, Box<Term>),
    Equal(Box<Term>, Box<Term>),
    Imply(Box<Term>, Box<Term>),
    And(Box<Term>, Box<Term>),
    Negate(Box<Term>),
}
