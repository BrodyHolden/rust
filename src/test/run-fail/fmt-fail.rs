// error-pattern:meh
// no-valgrind
use std;
import std::str;

fn main() { let str_var: str = "meh"; fail #fmt["%s", str_var]; }
