#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(maestro);

// If lib.rs is used, other files nearby need to be
// imported like below
// This mod declaration is needed by maestro.lalrpop
mod maestro_ast;

#[cfg(test)]
mod tests {
    use crate::maestro_ast;

    // tests is seperate module and needs import
    use super::maestro::{DirectiveParser, NumParser, VaridParser};
    use maestro_ast::Directive;
    use std::borrow::Borrow;
    use std::convert::AsRef;
    use std::str::FromStr;
    #[test]
    fn it_works() {
        assert!(NumParser::new().parse("123").unwrap() == 123u32);
        assert!(VaridParser::new().parse("x").unwrap() == String::from_str("x").unwrap());
        // assert!(DirectiveParser::new().parse("hi").is_ok());
        match DirectiveParser::new()
            .parse("SpatialMap (3,3)x;")
            .unwrap()
            .as_ref()
        {
            Directive::SpacialMap {
                dimention,
                size,
                offset,
            } => assert!(dimention.borrow() == "x".to_string() && *size == 3u32 && *offset == 3u32),
            _ => panic!(),
        }
    }
}
