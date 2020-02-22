extern crate combine;
extern crate combine_language;
use combine::char::{alpha_num, letter, string};
use combine::combinator::many;
use combine::easy::Errors;
use combine::stream::PointerOffset;
use combine::{satisfy, Parser};
use combine_language::{Identifier, LanguageDef, LanguageEnv};
use lambda_calculus::{parse, Classic};

#[test]
fn test_combine() {
    // assert_eq!(result, Ok(((Borrowed("identifier"), 42), "")));
    let result = combine();
    assert_eq!(result, Ok(((String::from("forall"), 42), "")));
}

pub fn combine() -> Result<((String, i64), &'static str), Errors<char, &'static str, PointerOffset>>
{
    // pub fn combine() -> () {
    let env = LanguageEnv::new(LanguageDef {
        ident: Identifier {
            start: letter(),
            rest: alpha_num(),
            reserved: ["for"].iter().map(|x| (*x).into()).collect(),
        },
        op: Identifier {
            start: satisfy(|c| "+-*/".chars().any(|x| x == c)),
            rest: satisfy(|c| "+-*/".chars().any(|x| x == c)),
            reserved: ["+", "-", "*", "/"].iter().map(|x| (*x).into()).collect(),
        },
        comment_start: string("/*").map(|_| ()),
        comment_end: string("*/").map(|_| ()),
        comment_line: string("//").map(|_| ()),
    });
    let id = env.identifier(); //An identifier parser
    let integer = env.integer(); //An integer parser
    let result = (many(id), integer).easy_parse("forall x  42");
    println!("{:?}", result);
    result
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
