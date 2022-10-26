use crate::{
    subst::Substitutable,
    taurus::{logic::ArithTerm, Symbol},
};

pub fn mk_subst<T>(y: ArithTerm, x: Symbol) -> impl Fn(&T) -> T + 'static
where
    T: Substitutable,
{
    move |t| t.subst(y.clone(), x)
}

pub fn boxed<T>(b: T) -> Box<T> {
    Box::new(b)
}

#[macro_export]
macro_rules! vect {
    (@push $v:ident, ..$e:expr, $($tt:tt)+) => {
        {
            $v.extend($e);
            vect!(@push $v, $($tt)+)
        }
    };
    (@push $v:ident, $e:expr, $($tt:tt)+) => {
        {
            #[allow(clippy::vec_init_then_push)]
            $v.push($e);
            vect!(@push $v, $($tt)+)
        }
    };
    (@push $v:ident, .. $e:expr) => {
        $v.extend($e)
    };
    (@push $v:ident, $e:expr) => {
        {
            #[allow(clippy::vec_init_then_push)]
            $v.push($e)
        }
    };
    ([$($tt:tt)+]) => {
        {
            let mut v = Vec::new();
            vect!(@push v, $($tt)+);
            v
        }
    };
}
