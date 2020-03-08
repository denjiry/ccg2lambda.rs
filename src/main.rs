use ccg2lambda_rs::lambda::cmb;
use ccg2lambda_rs::parse::parse_jigg;

fn main() {
    let mut input = "\\ G.";
    cmb(&mut input);
    parse_jigg();
}
