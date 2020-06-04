use {
    pest_derive::Parser,
    pest::Parser as _,
};


#[derive(Parser)]
#[grammar = "http.pest"]
pub struct HttpParser;
