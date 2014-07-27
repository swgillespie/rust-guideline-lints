/// match_dereference provides a simple lint that warns when the expression
/// used in a match expression is a dereference. While perfectly legal and
/// works just fine, this practice is discouraged by rust-guidelines in favor
/// of 'box' patterns. For example, the code
/// ```rust
/// fn main() {
///     let five = box 5i;
///     match *five {
///         5 => println!("Five!"),
///         _ => println!("Not five!")
///     };
///  }
/// ```
/// Although legal, this is considered bad style and this linter warns on
/// this. Instead, the following is preferred:
/// fn main() {
///     let five = box 5i;
///     match five {
///         box 5 => println!("Five!"),
///         _     => println!("Not five!")
///     };
/// }
///
/// See https://github.com/rust-lang/rust-guidelines/blob/master/features/match.md
/// for more information.

use std::gc::Gc;

use syntax::ast;
use rustc::lint;
use rustc::lint::{Context, LintPass, LintArray};
use syntax::codemap::{Span};

declare_lint! {
    MATCH_DEREFERENCE, Warn, "match conditions that do not need to dereference"
}

pub struct MatchDerefLint;

impl LintPass for MatchDerefLint {
    fn get_lints(&self) -> LintArray {
        lint_array!(MATCH_DEREFERENCE)
    }

    fn check_expr(&mut self, cx: &Context, e: &ast::Expr) {
        match e.node {
            ast::ExprMatch(ref cond, ref arms) => check_match(cx, cond, 
                                                              arms.as_slice()),
            _ => ()
        }
    }
}

/// Visit every Match expression. If the top-level expression of the Match
/// is a dereference (ast::UnDeref), then we issue a warning.
fn check_match(cx: &Context, expr: &Gc<ast::Expr>, arms: &[ast::Arm]) {
    match expr.node {
        ast::ExprUnary(ast::UnDeref, _) => {
            if cx.current_level(MATCH_DEREFERENCE) > lint::Allow {
                cx.span_lint(MATCH_DEREFERENCE, expr.span,
                             "Dereferencing in a match expression is discouraged");
                if arms.len() > 0 {
                    let ref first_arm = arms[0];
                    let span = unify_span(first_arm);
                    cx.sess().span_note(span,
                                        "Consider using a 'box' pattern here and all other match arms.");
                }
            }
        },
        _ => ()
    }
}

/// Given a list of patterns in a single arm, unify the spans so that the returned
/// span consists of the start of the first pattern and the end of the final
/// pattern. If the arm consists of a single pattern, then the span is simply
/// the span of that pattern.
fn unify_span(arm: &ast::Arm) -> Span {
    let first_pattern = arm.pats[0];
    let final_pattern = arm.pats.last();
    match final_pattern {
        Some(p) => Span {
            lo: first_pattern.span.lo,
            hi: p.span.hi,
            expn_info: None
        },
        // since arm.pats is non-empty, last() will return either the same as
        // first_pattern or some other pattern. Either way, p.span.hi gives
        // us the BytePos that we want to use.
        None => unreachable!()
    }
}
