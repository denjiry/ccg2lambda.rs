use ccg2lambda_rs::lambda::{bind, cmb};
use ccg2lambda_rs::parse::parse_jigg;

fn main() {
    let mut bindin = "\\ A b c.";
    println!("{:?}", bind(&mut bindin));
    let mut input = "\\ G.G";
    println!("{:?}", cmb(&mut input));
    parse_jigg();
}
