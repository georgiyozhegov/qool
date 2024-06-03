use qoolang::lex::lex;
use qoolang::parse::parse;

fn main()
{
        let source = "mutable value : (2 + 12)";
        let source = source.chars().peekable();
        let tokens = lex(source);
        let tokens = tokens.into_iter().peekable();
        let tree = parse(tokens);
        println!("{tree:?}");
}
