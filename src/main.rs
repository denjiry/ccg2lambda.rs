use ccg2lambda_rs::lambda::{bind, cmb};
use ccg2lambda_rs::parse::parse_jigg;

fn main() {
    let mut input = "\\G F.A&G->F";
    println!("{:?}", cmb(&mut input));
    parse_jigg();
}
