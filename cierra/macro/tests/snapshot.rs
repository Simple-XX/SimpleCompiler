use insta::assert_display_snapshot;
use sexp::{Serializer, SerializerStyle, SexpDisplay};
use sexp_macro::sexp;

struct Plain<F>(F)
where
    F: FnOnce(&mut Serializer);

impl<F> SexpDisplay for Plain<F>
where
    F: FnOnce(&mut Serializer),
{
    fn fmt(&self, s: &mut Serializer) {
        sexp!(s, (assert 1 2));
    }
}

macro_rules! plain {
    ($($tt: tt)*) => {
        Plain(|s| {sexp!(s, $($tt)*);})
    }
}

#[test]
fn snapshot() {
    snapshot_sexp("simple", |s| {
        sexp!(s, (assert 1 2));
    });
    snapshot_sexp("symbol_ex", |s| {
        sexp!(s, (= y (+ x 1)));
    });
    let p = plain!((not (= x 0)));
    let q = plain!((> y 1));
    snapshot_sexp("nested", |s| {
        sexp!(s, (=> %p %q));
    });
    snapshot_sexp("bracket_1", |s| {
        sexp!(s, (forall [x y z] (=> (and (<= x y) (<= y z)) (<= x z))));
    });
    snapshot_sexp("bracket_2", |s| {
        sexp!(s, (forall [x] (= x x)));
    });
    let l: [Box<dyn SexpDisplay>; 2] = [Box::new(plain!((not (= x 0)))), Box::new(plain!((> y 1)))];
    snapshot_sexp("expand", |s| {
        sexp!(s, (test ...(&l)));
    });
    snapshot_sexp("long", |s| {
        sexp!(s, (assert 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20));
    });
    snapshot_sexp("long_nested", |s| {
        sexp!(s, (assert (not (=> (choice (+ (choice (8 9 10)) (assume (- (choice (8 (choice (8 9 10)) (choice (+ (assume (- (choice (8 (choice (+ (assume (- (choice (8 9 10)))))) 10)))))) 10))))))))));
    });
}

fn snapshot_sexp(name: &str, f: impl FnOnce(&mut Serializer)) {
    let mut buf = String::new();
    let mut s = Serializer::new(SerializerStyle { line_break: "\n", indentation: "  " }, &mut buf);
    f(&mut s);
    s.finish(true);
    assert_display_snapshot!(name, buf);
}
