#![deny(warnings)]

mod ex01_helloworld;
mod ex02_primitives;
mod ex03_custom_types;
mod ex04_variable_bindings;
mod ex05_types;
mod ex06_conversion;
mod ex07_expressions;

fn main() {
    // ex01_helloworld:
    ex01_helloworld::ex00_main::example();
    ex01_helloworld::ex01_comments::example();
    ex01_helloworld::ex02_01_debug::example();
    ex01_helloworld::ex02_01_pretty_print_debug::example();
    ex01_helloworld::ex02_02_01_testcase_list::example();
    ex01_helloworld::ex02_02_display::example();
    ex01_helloworld::ex02_02_display_activity::example();
    ex01_helloworld::ex02_03_formatted_print::example();
    ex01_helloworld::ex02_03_print_formatting::example();

    // ex02_primitives:
    ex02_primitives::ex00_main::example();
    ex02_primitives::ex01_literals::example();
    ex02_primitives::ex02_tuples::example();
    ex02_primitives::ex03_array_and_slices::example();

    // ex03_custom_types:
    ex03_custom_types::ex01_structures::example();
    ex03_custom_types::ex02_01_enums_use::example();
    ex03_custom_types::ex02_02_enums_c_like::example();
    ex03_custom_types::ex02_03_enums_linked_list::example();
    ex03_custom_types::ex02_enums::example();
    ex03_custom_types::ex02_enums_type_aliases::example();
    ex03_custom_types::ex03_constants::example();

    // ex04_variable_bindings:
    ex04_variable_bindings::ex00_main::example();
    ex04_variable_bindings::ex01_mutability::example();
    ex04_variable_bindings::ex02_scope::example();
    ex04_variable_bindings::ex02_shadowing::example();
    ex04_variable_bindings::ex03_declare_first::example();
    ex04_variable_bindings::ex04_freezing::example();

    // ex05_types:
    ex05_types::ex01_casting::example();
    ex05_types::ex02_literals::example();
    ex05_types::ex03_inference::example();
    ex05_types::ex04_aliasing::example();

    // ex06_conversion:
    ex06_conversion::ex01_from_custom::example();
    ex06_conversion::ex01_from_string::example();
    ex06_conversion::ex01_into::exmaple();
    ex06_conversion::ex02_tryfrom_tryinto::example();
    ex06_conversion::ex03_from_string::example();
    ex06_conversion::ex03_to_string::example();

    // ex07_expressions:
    ex07_expressions::ex00_blocks::example();
    ex07_expressions::ex00_expr::example();
}
