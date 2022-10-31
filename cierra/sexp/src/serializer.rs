use crate::utils::check_atom;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SerializerStyle<'a> {
    pub line_break: &'a str,
    pub indentation: &'a str,
}

/// Serializes into a possibly multi-line string.
///
/// # Example
///
/// Compact (single line) example:
///
/// ```
/// let style = sexp::SerializerStyle { line_break: "\n", indentation: " " };
///
/// let mut result = String::new();
/// let mut serializer = sexp::Serializer::new(style, &mut result);
///
/// serializer.begin_list(usize::MAX);
/// serializer.put_atom("example", usize::MAX);
/// serializer.begin_list(usize::MAX);
/// serializer.put_atom("1", usize::MAX);
/// serializer.put_atom("2", usize::MAX);
/// serializer.put_atom("3", usize::MAX);
/// serializer.end_list();
/// serializer.begin_list(usize::MAX);
/// serializer.put_atom("a", usize::MAX);
/// serializer.put_atom("b", usize::MAX);
/// serializer.put_atom("c", usize::MAX);
/// serializer.end_list();
/// serializer.end_list();
/// serializer.finish(false);
///
/// let expected_result = "(example (1 2 3) (a b c))";
/// assert_eq!(result, expected_result);
/// ```
///
/// Full multi-line example:
///
/// ```
/// let style = sexp::SerializerStyle { line_break: "\n", indentation: " " };
///
/// let mut result = String::new();
/// let mut serializer = sexp::Serializer::new(style, &mut result);
///
/// serializer.begin_list(usize::MAX);
/// serializer.put_atom("example", 0);
/// serializer.begin_list(0);
/// serializer.put_atom("1", 0);
/// serializer.put_atom("2", 0);
/// serializer.put_atom("3", 0);
/// serializer.end_list();
/// serializer.begin_list(0);
/// serializer.put_atom("a", 0);
/// serializer.put_atom("b", 0);
/// serializer.put_atom("c", 0);
/// serializer.end_list();
/// serializer.end_list();
/// serializer.finish(true);
///
/// let expected_result = "(\n example\n (\n  1\n  2\n  3\n )\n (\n  a\n  b\n  c\n )\n)\n";
/// assert_eq!(result, expected_result);
/// ```
///
/// You can write specified nodes in a single line:
///
/// ```
/// let style = sexp::SerializerStyle { line_break: "\n", indentation: " " };
///
/// let mut result = String::new();
/// let mut serializer = sexp::Serializer::new(style, &mut result);
///
/// serializer.begin_list(0);
/// serializer.put_atom("example", usize::MAX);
/// serializer.begin_list(0);
/// // Write the three atoms in a single line.
/// serializer.put_atom("1", usize::MAX);
/// serializer.put_atom("2", usize::MAX);
/// serializer.put_atom("3", usize::MAX);
/// serializer.end_list();
/// serializer.begin_list(0);
/// // Write the three atoms in a single line.
/// serializer.put_atom("a", usize::MAX);
/// serializer.put_atom("b", usize::MAX);
/// serializer.put_atom("c", usize::MAX);
/// serializer.end_list();
/// serializer.end_list();
/// serializer.finish(true);
///
/// let expected_result = "(example\n (1 2 3)\n (a b c)\n)\n";
/// assert_eq!(result, expected_result);
/// ```
pub struct Serializer<'a, 'b> {
    style: SerializerStyle<'a>,
    out: &'b mut String,
    state: State,
}

enum State {
    Beginning,
    Writing(WritingState),
    Finished,
}

struct WritingState {
    stack: Vec<StackItem>,
    list_beginning: bool,
    current_list_line_broken: bool,
    line_len: usize,
}

struct StackItem {
    line_broken: bool,
}

impl<'a, 'b> Serializer<'a, 'b> {
    pub fn new(style: SerializerStyle<'a>, out: &'b mut String) -> Self {
        Self { style, out, state: State::Beginning }
    }

    fn write_indent(indentation: &str, n: usize, out: &mut String) -> usize {
        let prev_len = out.len();
        for _ in 0..n {
            out.push_str(indentation);
        }
        out.len() - prev_len
    }

    pub fn put_atom(&mut self, atom: &str, break_line_at: usize) {
        assert!(check_atom(atom), "invalid atom {:?}", atom);

        match self.state {
            State::Beginning => {
                self.out.push_str(atom);
                self.state = State::Finished;
            }
            State::Writing(ref mut state) => {
                if state.line_len < break_line_at {
                    if !state.list_beginning {
                        self.out.push(' ');
                        state.line_len += 1;
                    }
                    self.out.push_str(atom);
                    state.line_len += atom.len();
                } else {
                    self.out.push_str(self.style.line_break);
                    let indent_len =
                        Self::write_indent(self.style.indentation, state.stack.len() + 1, self.out);
                    self.out.push_str(atom);
                    state.current_list_line_broken = true;
                    state.line_len = indent_len + atom.len();
                }
                state.list_beginning = false;
            }
            State::Finished => panic!("writing already finished"),
        }
    }

    #[allow(clippy::branches_sharing_code)]
    pub fn begin_list(&mut self, break_line_at: usize) {
        match self.state {
            State::Beginning => {
                self.out.push('(');
                self.state = State::Writing(WritingState {
                    stack: Vec::new(),
                    list_beginning: true,
                    current_list_line_broken: false,
                    line_len: 1,
                });
            }
            State::Writing(ref mut state) => {
                if state.line_len < break_line_at {
                    if !state.list_beginning {
                        self.out.push(' ');
                        state.line_len += 1;
                    }
                    self.out.push('(');
                    state.line_len += 1;
                } else {
                    self.out.push_str(self.style.line_break);
                    state.line_len =
                        Self::write_indent(self.style.indentation, state.stack.len() + 1, self.out);
                    self.out.push('(');
                    state.current_list_line_broken = true;
                    state.line_len += 1;
                }

                state.stack.push(StackItem { line_broken: state.current_list_line_broken });
                state.list_beginning = true;
                state.current_list_line_broken = false;
            }
            State::Finished => panic!("writing already finished"),
        }
    }

    pub fn end_list(&mut self) {
        match self.state {
            State::Beginning => panic!("no list to end"),
            State::Writing(ref mut state) => {
                if state.current_list_line_broken {
                    self.out.push_str(self.style.line_break);
                    state.line_len =
                        Self::write_indent(self.style.indentation, state.stack.len(), self.out);
                }
                self.out.push(')');
                state.line_len += 1;

                if let Some(previous) = state.stack.pop() {
                    state.current_list_line_broken |= previous.line_broken;
                    state.list_beginning = false;
                } else {
                    self.state = State::Finished;
                }
            }
            State::Finished => panic!("writing already finished"),
        }
    }

    pub fn finish(self, insert_line_break: bool) {
        match self.state {
            State::Finished => {
                if insert_line_break {
                    self.out.push_str(self.style.line_break);
                }
            }
            _ => panic!("writing not finished yet"),
        }
    }
}
