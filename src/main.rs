use ccg2lambda_rs::lambda::{combine, string2lambda};
use ccg2lambda_rs::parse::parse_jigg;

fn main() {
    let mut input = "input";
    combine(&mut input);
    string2lambda();
    parse_jigg();
}
