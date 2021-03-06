#![allow(unused)]

pub(crate) mod color_trace;
pub(crate) mod expand_trace;

pub(crate) use self::color_trace::*;
pub(crate) use self::expand_trace::*;

use crate::parser::hir::tokens_iterator::TokensIteratorState;
use crate::traits::ToDebug;

#[derive(Debug)]
pub(crate) enum DebugIteratorToken {
    Seen(String),
    Unseen(String),
    Cursor,
}

pub(crate) fn debug_tokens(state: &TokensIteratorState, source: &str) -> Vec<DebugIteratorToken> {
    let mut out = vec![];

    for (i, token) in state.tokens.iter().enumerate() {
        if state.index == i {
            out.push(DebugIteratorToken::Cursor);
        }

        if state.seen.contains(&i) {
            out.push(DebugIteratorToken::Seen(format!("{}", token.debug(source))));
        } else {
            out.push(DebugIteratorToken::Unseen(format!(
                "{}",
                token.debug(source)
            )));
        }
    }

    out
}
