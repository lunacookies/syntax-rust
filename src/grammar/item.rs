mod fn_;
mod struct_;

use crate::Parser;
use fn_::parse_fn;

pub(crate) fn parse_item(p: &mut Parser) {
    assert!(p.at(&[crate::TokenKind::Fn]));

    parse_fn(p);
}
