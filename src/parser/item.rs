mod fn_;

use crate::Parser;
use fn_::parse_fn;

pub(super) fn parse_item(p: &mut Parser) {
    assert!(p.at(&[crate::TokenKind::Fn]));

    parse_fn(p);
}