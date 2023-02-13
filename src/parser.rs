// use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "kotlin.pest"]
pub struct KotlinParser;
