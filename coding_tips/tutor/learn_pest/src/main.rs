use pest::Parser;



#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    simple_test1();
}

pub fn simple_test1() {
    let parse_result = Parser::parse(Rule::sum, "1773 + 1362").unwrap();
    let tokens = parse_result.tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
