use combine::char::{alpha_num, letter, string};
use combine::error::ParseError;
use combine::{
    attempt, between, chainl1, many1, parser, satisfy, ParseResult, Parser, Stream, StreamOnce,
};
use combine_language::{Identifier, LanguageDef, LanguageEnv};

#[cfg(test)]
use combine::error::Consumed::Consumed;

// EXPR = "\", Vec<String>, ".", BINOP | BINOP
// BINOP = combine::chainl1(UNIOP, "=" | "->" | "&")
// UNIOP = "-", APPLY | APPLY
// APPLY = TERM, TERM | TERM
// TERM = Var(String) | "(", EXPR, ")"

#[test]
fn test_combine() {
    let mut input = "\\G F.F2&((A&B)->C)";
    let result = cmb(&mut input);
    let aandb = Box::new(Term::And(
        Box::new(Term::Var("A".to_string())),
        Box::new(Term::Var("B".to_string())),
    ));
    let aandb_implyc = Box::new(Term::Imply(aandb, Box::new(Term::Var("C".to_string()))));
    let body = Box::new(Term::And(
        Box::new(Term::Var("F2".to_string())),
        aandb_implyc,
    ));
    let binds: Vec<String> = ["G", "F"].iter().map(|&s| s.into()).collect();
    let expected = Box::new(Term::Lambda(binds, body));
    assert_eq!(result, Ok((expected, Consumed(()))));
}

pub fn cmb<'a, I>(input: &mut I) -> ParseResult<Box<Term>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    I: 'a,
    <I as StreamOnce>::Error: std::fmt::Debug,
{
    let mut parser = parser(expr);
    parser.parse_stream(input)
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
    let paren = env.parens(parser(expr));
    let var = env.identifier().map(|var: String| Box::new(Term::Var(var)));
    var.or(paren).parse_stream(input)
}

fn apply<I>(input: &mut I) -> ParseResult<Box<Term>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let apply = parser(term)
        .and(parser(term))
        .map(|(a, b)| Box::new(Term::Apply(a, b)));
    attempt(apply).or(parser(term)).parse_stream(input)
}

fn negate<I>(input: &mut I) -> ParseResult<Box<Term>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let env = calc_env();
    let apply = parser(apply);
    let mut negate = env
        .reserved_op("-")
        .and(apply)
        .map(|(_negate, apply)| Box::new(Term::Negate(apply)));
    negate.parse_stream(input)
}

fn uniop<I>(input: &mut I) -> ParseResult<Box<Term>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let negate = parser(negate);
    let apply = parser(apply);
    negate.or(apply).parse_stream(input)
}

#[test]
fn test_binop() {
    let mut bindin = "(A&B)->C";
    let r = binop(&mut bindin);
    let aandb = Term::And(
        Box::new(Term::Var("A".to_string())),
        Box::new(Term::Var("B".to_string())),
    );
    let expected = Box::new(Term::Imply(
        Box::new(aandb),
        Box::new(Term::Var("C".to_string())),
    ));
    assert_eq!(r, Ok((expected, Consumed(()))));
}

fn binop<I>(input: &mut I) -> ParseResult<Box<Term>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let env = calc_env();
    let binop = env
        .reserved_op("=")
        .or(env.reserved_op("->"))
        .or(env.reserved_op("&"))
        .map(|op| {
            move |lhs, rhs| {
                if op == "=" {
                    Box::new(Term::Equal(lhs, rhs))
                } else if op == "->" {
                    Box::new(Term::Imply(lhs, rhs))
                } else if op == "&" {
                    Box::new(Term::And(lhs, rhs))
                } else {
                    unreachable!()
                }
            }
        });
    chainl1(parser(uniop), binop).parse_stream(input)
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

#[test]
fn test_lambda_term() {
    let mut input = "\\ A B . (A)";
    let r = lambda_term(&mut input);
    let bind: Vec<String> = ["A", "B"].iter().map(|&s| s.into()).collect();
    let term = Box::new(Term::Var("A".to_string()));
    let expected = Box::new(Term::Lambda(bind, term));
    assert_eq!(r, Ok((expected, Consumed(()))));
}

fn lambda_term<I>(input: &mut I) -> ParseResult<Box<Term>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let binop = parser(binop);
    parser(bind)
        .and(binop)
        .map(|(bind, binop): (Vec<String>, Box<Term>)| Box::new(Term::Lambda(bind, binop)))
        .parse_stream(input)
}

fn expr<I>(input: &mut I) -> ParseResult<Box<Term>, I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let binop = parser(binop);
    parser(lambda_term).or(binop).parse_stream(input)
}

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
