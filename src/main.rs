#![deny(warnings)]

mod ex01_helloworld;
mod ex02_primitives;
mod ex03_custom_types;

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
}
