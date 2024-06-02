use qoolang::lex::lex;
use qoolang::parse::parse;

fn main()
{
        let source = "value : 10 + 20 - 13";
        println!("-- LEXICAL ANALYSIS --");
        let source = source.chars().peekable();
        let tokens = lex(source);
        println!("-- PARSING --");
        let tokens = tokens.into_iter().peekable();
        let tree = parse(tokens);
        println!("{tree:#?}");
}
