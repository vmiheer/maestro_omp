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
    use super::maestro::{DirectiveParser, DirectiveStackParser, NumParser, VaridParser};
    use maestro_ast::Directive;
    use std::str::FromStr;
    use std::borrow::Borrow;
    use std::iter::{Iterator, Enumerate};
    #[test]
    fn it_works() {
        assert!(NumParser::new().parse("123").unwrap() == 123u32);
        assert!(VaridParser::new().parse("x").unwrap() == String::from_str("x").unwrap());
        // assert!(DirectiveParser::new().parse("hi").is_ok());
        if let Directive::SpacialMap {
            dimention,
            size,
            offset,
        } = DirectiveParser::new().parse("SpatialMap (3,3)x;").unwrap()
        {
            assert!(dimention == "x".to_string() && size == 3u32 && offset == 3u32);
        } else {
            panic!()
        }
        if let Directive::TemporalMap {
            dimention,
            size,
            offset,
        } = DirectiveParser::new().parse("TemporalMap (3,3)x;").unwrap()
        {
            assert!(dimention == "x".to_string() && size == 3u32 && offset == 3u32)
        } else {
            panic!()
        }
        if let Directive::Cluster { size } =
            DirectiveParser::new().parse("Cluster (32,P);").unwrap()
        {
            assert!(size == 32u32)
        } else {
            panic!()
        }
    }
    #[test]
    fn directive_stack_works() {
        let res = DirectiveStackParser::new()
            .parse("SpatialMap (3,3)x;TemporalMap (2,2)y;")
            .unwrap();
        assert_eq!(res.len(), 2usize);
        for x in res.iter().enumerate() {
            if let Directive::SpacialMap {
                dimention,
                size,
                offset,
            } = x.1.borrow() {
                assert!(x.0 == 0);
                assert!(dimention.borrow() == "x".to_string());
            } else if let Directive::TemporalMap {
                dimention,
                size,
                offset
            } = x.1.borrow() {
                assert!(x.0 == 1);
                assert!(dimention.borrow() == "y".to_string());
            }
        }
        if let Directive::SpacialMap {
            dimention,
            size,
            offset,
        } = res[0].borrow() {
            assert!(dimention.borrow() == "x".to_string());
        } else {
            panic!();
        }
    }
}
