use ccg2lambda_rs::lambda::cmb;
use ccg2lambda_rs::parse::parse_jigg;

fn main() {
    let mut input = "\\G F.G( G(A&B) -> C)";
    let mut input = "G(F(G))";
    println!("{:?}", cmb(&mut input));
    parse_jigg();
}
