//別ファイルの関数を利用するためにmod filenameとする;
mod array;
mod control_syntax;
mod structure;
mod variable;

fn main() {
    variable::var_num();
    variable::var_string();
    variable::var_str();
    array::arrays();
    control_syntax::loop1();
    control_syntax::if_match();
    control_syntax::let_if_match();
    structure::structure();
    structure::tuple_like();
    structure::enum_1();
}
