#![feature(plugin_registrar, phase)]

#[phase(plugin, link)] extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;
use match_dereference::MatchDerefLint;

mod match_dereference;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box MatchDerefLint);
}
