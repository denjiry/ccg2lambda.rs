use combine;
use combine::char::{alpha_num, letter, string};
use combine::error::{Consumed::Consumed, ParseError};
use combine::{between, chainl1, many1, parser, satisfy, ParseResult, Parser, Stream, StreamOnce};
use combine_language;
use combine_language::{Identifier, LanguageDef, LanguageEnv};

#[test]
fn test_combine() {
    let mut input = "G";
    let result = cmb(&mut input);
    println!("{:?}", result);
}

pub fn cmb<'a, I>(input: &mut I) -> ParseResult<Box<Term>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    I: 'a,
    <I as StreamOnce>::Error: std::fmt::Debug,
{
    let mut parser = parser(term);
    let result = parser.parse_stream(input);
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
            start: satisfy(|c| "=-&".chars().any(|x| x == c)),
            rest: satisfy(|c| "=>-".chars().any(|x| x == c)),
            reserved: ["=", "->", "&", "-"].iter().map(|x| (*x).into()).collect(),
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
    let var = env.identifier().map(|var: String| Box::new(Term::Var(var)));
    // let lambda = parser(lambda_term);
    parenthesized.or(var).parse_stream(input)
}

fn binop<I>(input: &mut I) -> ParseResult<Box<Term>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let env = calc_env();
    let op = env.reserved_op("=").or(env.reserved_op("->")).map(|op| {
        move |lhs, rhs| {
            if op == "=" {
                Box::new(Term::Equal(lhs, rhs))
            } else if op == "->" {
                Box::new(Term::Imply(lhs, rhs))
            } else {
                unreachable!()
            }
        }
    });
    chainl1(parser(term), op).parse_stream(input)
}

#[test]
fn test_bind() {
    let mut bindin = "\\ A b c dd.";
    let r = bind(&mut bindin);
    let expected: Vec<String> = ["A", "b", "c", "dd"].iter().map(|&s| s.into()).collect();
    assert_eq!(r, Ok((expected, Consumed(()))));
}

pub fn bind<I>(input: &mut I) -> ParseResult<Vec<String>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let env = calc_env();
    let var = env.identifier();
    let vars = many1(var);
    let mut bind = between(env.lex(string("\\")), env.lex(string(".")), vars);
    bind.parse_stream(input)
}

// fn lambda_term<I>(input: &mut I) -> ParseResult<Box<Term>, I>
// where
//     I: Stream<Item = char>,
//     I::Error: ParseError<I::Item, I::Range, I::Position>,
// {
//     let formula = term;
//     bind.and_then(formula).parse_stream(input)
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(String),
    Lambda(Vec<String>, Box<Term>),
    Forall(String, Box<Term>),
    Exists(String, Box<Term>),
    Apply(Box<Term>, Box<Term>),
    Equal(Box<Term>, Box<Term>),
    Imply(Box<Term>, Box<Term>),
    And(Box<Term>, Box<Term>),
    Negate(Box<Term>),
}
