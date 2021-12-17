#![deny(warnings)]

mod ex01_helloworld;
mod ex02_primitives;
mod ex03_custom_types;
mod ex04_variable_bindings;
mod ex05_types;
mod ex06_conversion;
mod ex07_expressions;
mod ex08_flow_of_control;
mod ex09_functions;
mod ex10_modules;
mod ex11_crates;
mod ex13_attributes;
mod ex14_generics;

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

  // ex08_flow_of_control:
  ex08_flow_of_control::ex01_if_else::example();
  ex08_flow_of_control::ex02_01_nesting_labels::example();
  ex08_flow_of_control::ex02_02_returning_from_loop::example();
  ex08_flow_of_control::ex02_loop::example();
  ex08_flow_of_control::ex03_while::example();
  ex08_flow_of_control::ex04_for_and_range_exclusive::example();
  ex08_flow_of_control::ex04_for_and_range_inclusive::example();
  ex08_flow_of_control::ex04_for_into_iter::example();
  ex08_flow_of_control::ex04_for_iter::example();
  ex08_flow_of_control::ex04_for_iter_mut::example();
  ex08_flow_of_control::ex05_01_01_match_tuples::example();
  ex08_flow_of_control::ex05_01_02_match_enums::example();
  ex08_flow_of_control::ex05_01_03_match_pointers::example();
  ex08_flow_of_control::ex05_01_04_match_structs::example();
  ex08_flow_of_control::ex05_02_match_guards::example();
  ex08_flow_of_control::ex05_03_match_binding_destructure::example();
  ex08_flow_of_control::ex05_03_match_binding_sigil::example();
  ex08_flow_of_control::ex05_match::example();
  ex08_flow_of_control::ex06_if_let::example();
  ex08_flow_of_control::ex06_if_let_any_enum::example();
  ex08_flow_of_control::ex06_if_let_match_awkward::example();
  ex08_flow_of_control::ex06_if_let_nonparametrized::example();
  ex08_flow_of_control::ex07_while_let::example();
  ex08_flow_of_control::ex07_while_let_match_awkward::example();

  // ex09_functions:
  ex09_functions::ex00_main::example();
  ex09_functions::ex01_methods::example();
  ex09_functions::ex02_01_capturing::example();
  ex09_functions::ex02_01_capturing_move::example();
  ex09_functions::ex02_02_capturing_as_input::example();
  ex09_functions::ex02_03_type_anonimity::example();
  ex09_functions::ex02_04_input::example();
  ex09_functions::ex02_05_output::example();
  ex09_functions::ex02_06_01_iterator_any::example();
  ex09_functions::ex02_06_02_position::example();
  ex09_functions::ex02_06_02_through_iterators::example();
  ex09_functions::ex02_closures::example();
  ex09_functions::ex03_higher_order_functions::example();
  ex09_functions::ex04_diverging_functions::example();
  ex09_functions::ex04_diverging_no_return::example();

  // ex10_modules:
  ex10_modules::ex01_visibility::example();
  ex10_modules::ex02_struct_visibility::example();
  ex10_modules::ex03_as_use::example();
  ex10_modules::ex04_super_and_self::example();
  ex10_modules::split::example();

  // ex11_crates:
  ex11_crates::ex01_creating_library::example();
  ex11_crates::ex02_using_library::example();

  // ex13_attributes:
  ex13_attributes::ex01_dead_code::example();
  ex13_attributes::ex02_crates::example();
  ex13_attributes::ex03_01_custom::example();
  ex13_attributes::ex03_cfg::example();

  // ex14_generics:
  ex14_generics::ex00_main::example();
  ex14_generics::ex01_functions::example();
  ex14_generics::ex02_impl::example();
  ex14_generics::ex02_implementation::example();
  ex14_generics::ex03_traits::example();
  ex14_generics::ex04_01_empty_bounds::example();
  ex14_generics::ex04_bounds::example();
  ex14_generics::ex05_multiple_bounds::example();
  ex14_generics::ex06_where::example();
  ex14_generics::ex07_new_type_idiom::example();
  ex14_generics::ex08_01_assiociated_problem::example();
  ex14_generics::ex08_01_associated_types::example();
  ex14_generics::ex09_01_phantom_types_clarification::example();
  ex14_generics::ex09_phantom_types::example();
}
