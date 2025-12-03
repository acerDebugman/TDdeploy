use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "bin/csv.pest"]
pub struct CSVParser;

fn main() -> anyhow::Result<()> {
    let successful_parse = CSVParser::parse(Rule::field, "-273.15")?;
    println!("{:?}", successful_parse);

    let unsuccessful_parse = CSVParser::parse(Rule::field, "this is not a number");
    println!("{:?}", unsuccessful_parse);

    Ok(())
}
