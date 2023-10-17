#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use parsec::config;

fn main() {
    let cfg = config();

    let mut session = cfg.session_from_args();

    session.start()
}
