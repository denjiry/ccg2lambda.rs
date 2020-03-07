use ccg2lambda_rs::lambda::combine;
use ccg2lambda_rs::parse::parse_jigg;

fn main() {
    let mut input = "\\ G.";
    combine(&mut input);
    parse_jigg();
}
