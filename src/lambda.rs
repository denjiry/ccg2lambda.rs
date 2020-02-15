use lambda_calculus::{parse, Classic};

#[test]
fn test() {
    let result = parse(&"λa.a", Classic);
    assert_eq!(result, parse(&"λa.a", Classic));
    let mut ss: String =
        "λL R Q N F. forall x.(Q(λG.N(λy.(G(y) & (x = y))), λy.True) -> F(x))".to_string();
    let offset = ss.find('.').unwrap_or(ss.len());
    let t: String = ss.drain(..offset).collect();
    let tt = t.replace(" ", ".λ");
    let rrr = tt + &ss;
    let s = "λL.λR.λQ.λN.λF. forall x.(Q(λG.N(λy.(G(y) & (x = y))), λy.True) -> F(x))".to_string();
    assert_eq!(rrr, s);
}

pub fn string2lambda() {
    let s = &"λL.λR.λQ.λN.λF. forall x.(Q(λG.N(λy.(G(y) & (x = y))), λy.True) -> F(x))";
    let p = parse(s, Classic);
    println!("{:#?}", p);
}
