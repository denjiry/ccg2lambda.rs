use lambda_calculus::{parse, Classic};

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
    let s = "λL.λR.λQ.λN.λF. forall x.(Q(λG.N(λy.(G(y) & (x = y))), λy.True) -> F(x))".to_string();
    let replaced = space2lam(s);
    let p = parse(&replaced, Classic);
    println!("{:#?}", p);
}
