#![allow(clippy::missing_panics_doc, clippy::module_name_repetitions)]

pub use serializer::{Serializer, SerializerStyle};

mod serializer;
mod utils;

pub trait SexpDisplay {
    fn fmt(&self, s: &mut Serializer);
    fn display(&self, style: SerializerStyle) -> String {
        let mut buf = String::new();
        let mut s = Serializer::new(style, &mut buf);
        self.fmt(&mut s);
        s.finish(true);
        buf
    }
    fn pretty_display(&self) -> String {
        self.display(SerializerStyle { line_break: "\n", indentation: "  " })
    }
    fn plain_display(&self) -> String {
        self.display(SerializerStyle { line_break: "", indentation: "" })
    }
}

impl<T> SexpDisplay for &T
where
    T: SexpDisplay,
{
    fn fmt(&self, s: &mut Serializer) {
        SexpDisplay::fmt(&**self, s);
    }
}

impl<T> SexpDisplay for &mut T
where
    T: SexpDisplay,
{
    fn fmt(&self, s: &mut Serializer) {
        SexpDisplay::fmt(&**self, s);
    }
}

impl<T> SexpDisplay for Box<T>
where
    T: SexpDisplay,
{
    fn fmt(&self, s: &mut Serializer) {
        SexpDisplay::fmt(&**self, s);
    }
}

impl SexpDisplay for &dyn SexpDisplay {
    fn fmt(&self, s: &mut Serializer) {
        SexpDisplay::fmt(&**self, s);
    }
}

impl SexpDisplay for Box<dyn SexpDisplay> {
    fn fmt(&self, s: &mut Serializer) {
        SexpDisplay::fmt(&**self, s);
    }
}
