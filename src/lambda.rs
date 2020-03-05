extern crate combine;
extern crate combine_language;
use combine::char::{alpha_num, letter, string};
use combine::error::ParseError;
use combine::{chainl1, many1, parser, satisfy, ParseResult, Parser, Stream, StreamOnce};
use combine_language::{Identifier, LanguageDef, LanguageEnv};
use lambda_calculus::{parse, Classic};

#[test]
fn test_combine() {
    // assert_eq!(result, Ok(((Borrowed("identifier"), 42), "")));
    let mut input = "input";
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
    // pub fn combine() -> () {
    let env = calc_env::<'a, I>();
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
    // let parenthesized = env.parens(parser(term));
    let mut name = env.identifier().map(|name| Box::new(Term::Name(name)));
    name.parse_stream(input)
    // parenthesized.or(name).parse_stream(input)
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

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct LambdaTerm {
//     bind: Vec<String>,
//     formula: Box<Term>,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Name(String),
    // Lambda(LambdaTerm),
    // Forall(LambdaTerm),
    // Exists(LambdaTerm),
    // Apply(Box<Term>, Box<Term>),
    Equal(Box<Term>, Box<Term>),
    Imply(Box<Term>, Box<Term>),
    And(Box<Term>, Box<Term>),
    Negate(Box<Term>),
}

#[test]
fn test() {
    let result = parse(&"λa.a", Classic);
    assert_eq!(result, parse(&"λa.a", Classic));
}

#[test]
fn test_space() {
    assert_eq!(
        space2lam(
            "λL R Q N F. forall x.(Q(λG.N(λy.(G(y) & (x = y))), λy.True) -> F(x))".to_string()
        ),
        "λL.λR.λQ.λN.λF. forall x.(Q(λG.N(λy.(G(y) & (x = y))), λy.True) -> F(x))".to_string()
    );
}

fn space2lam(mut input: String) -> String {
    let offset = input.find('.').unwrap_or(input.len());
    let t: String = input.drain(..offset).collect();
    let tt = t.replace(" ", ".λ");
    tt + &input
}

pub fn string2lambda() {
    let s = "λL R Q N F. forall x.(Q(λG.N(λy.(G(y) & (x = y))), λy.True) -> F(x))".to_string();
    let replaced = space2lam(s);
    let p = parse(&replaced, Classic);
    println!("{:#?}", p);
}
