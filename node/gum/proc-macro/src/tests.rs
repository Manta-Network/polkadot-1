use super::*;

use quote::quote;

#[test]
fn x() {
    let ts = impl_gum(quote!{
        x = Foo::default(),
        z = ?Game::new(),
        "Foo {p} x {q}",
        p,
        q,
    }).unwrap();
    assert_eq!(ts, quote!{

    });
}
