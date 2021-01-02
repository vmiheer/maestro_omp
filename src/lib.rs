#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(maestro);

// If lib.rs is used, other files nearby need to be
// imported like below
// This mod declaration is needed by maestro.lalrpop
mod maestro_ast;

#[cfg(test)]
mod tests {
    // tests is seperate module and needs import
    use super::maestro::DirectiveParser;
    #[test]
    fn it_works() {
        println!("{:?}", DirectiveParser::new().parse("hi").ok());
    }
}
