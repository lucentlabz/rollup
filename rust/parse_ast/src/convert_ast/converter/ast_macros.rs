// This file is generated by scripts/generate-ast-macros.js.
// Do not edit this file directly.

#[macro_export]
macro_rules! store_assignment_expression {
  ($self:expr, span => $span:expr, operator => $operator_value:expr, left => [$left_value:expr, $left_converter:ident], right => [$right_value:expr, $right_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&5u32.to_ne_bytes(), &$span, 16, false);
    // operator
    let operator_position = end_position + 4;
    $self.buffer[operator_position..operator_position + 4].copy_from_slice($operator_value);
    // left
    $self.update_reference_position(end_position + 8);
    $self.$left_converter(&$left_value);
    // right
    $self.update_reference_position(end_position + 12);
    $self.$right_converter(&$right_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_await_expression {
  ($self:expr, span => $span:expr, argument => [$argument_value:expr, $argument_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&7u32.to_ne_bytes(), &$span, 8, false);
    // argument
    $self.update_reference_position(end_position + 4);
    $self.$argument_converter(&$argument_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_break_statement {
  ($self:expr, span => $span:expr, label => [$label_value:expr, $label_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&10u32.to_ne_bytes(), &$span, 8, false);
    // label
    if let Some(value) = $label_value.as_ref() {
      $self.update_reference_position(end_position + 4);
      $self.$label_converter(value);
    }
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_conditional_expression {
  ($self:expr, span => $span:expr, test => [$test_value:expr, $test_converter:ident], consequent => [$consequent_value:expr, $consequent_converter:ident], alternate => [$alternate_value:expr, $alternate_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&17u32.to_ne_bytes(), &$span, 16, false);
    // test
    $self.update_reference_position(end_position + 4);
    $self.$test_converter(&$test_value);
    // consequent
    $self.update_reference_position(end_position + 8);
    $self.$consequent_converter(&$consequent_value);
    // alternate
    $self.update_reference_position(end_position + 12);
    $self.$alternate_converter(&$alternate_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_continue_statement {
  ($self:expr, span => $span:expr, label => [$label_value:expr, $label_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&18u32.to_ne_bytes(), &$span, 8, false);
    // label
    if let Some(value) = $label_value.as_ref() {
      $self.update_reference_position(end_position + 4);
      $self.$label_converter(value);
    }
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_debugger_statement {
  ($self:expr, span => $span:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&19u32.to_ne_bytes(), &$span, 4, false);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_decorator {
  ($self:expr, span => $span:expr, expression => [$expression_value:expr, $expression_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&20u32.to_ne_bytes(), &$span, 8, false);
    // expression
    $self.update_reference_position(end_position + 4);
    $self.$expression_converter(&$expression_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_directive {
  ($self:expr, span => $span:expr, directive => $directive_value:expr, expression => [$expression_value:expr, $expression_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&21u32.to_ne_bytes(), &$span, 12, false);
    // directive
    $self.convert_string($directive_value, end_position + 4);
    // expression
    $self.update_reference_position(end_position + 8);
    $self.$expression_converter(&$expression_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_do_while_statement {
  ($self:expr, span => $span:expr, body => [$body_value:expr, $body_converter:ident], test => [$test_value:expr, $test_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&22u32.to_ne_bytes(), &$span, 12, false);
    // body
    $self.update_reference_position(end_position + 4);
    $self.$body_converter(&$body_value);
    // test
    $self.update_reference_position(end_position + 8);
    $self.$test_converter(&$test_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_empty_statement {
  ($self:expr, span => $span:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&23u32.to_ne_bytes(), &$span, 4, false);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_export_specifier {
  ($self:expr, span => $span:expr, local => [$local_value:expr, $local_converter:ident], exported => [$exported_value:expr, $exported_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&27u32.to_ne_bytes(), &$span, 12, false);
    // local
    $self.update_reference_position(end_position + 4);
    $self.$local_converter(&$local_value);
    // exported
    if let Some(value) = $exported_value.as_ref() {
      $self.update_reference_position(end_position + 8);
      $self.$exported_converter(value);
    }
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_expression_statement {
  ($self:expr, span => $span:expr, expression => [$expression_value:expr, $expression_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&28u32.to_ne_bytes(), &$span, 8, false);
    // expression
    $self.update_reference_position(end_position + 4);
    $self.$expression_converter(&$expression_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_for_in_statement {
  ($self:expr, span => $span:expr, left => [$left_value:expr, $left_converter:ident], right => [$right_value:expr, $right_converter:ident], body => [$body_value:expr, $body_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&29u32.to_ne_bytes(), &$span, 16, false);
    // left
    $self.update_reference_position(end_position + 4);
    $self.$left_converter(&$left_value);
    // right
    $self.update_reference_position(end_position + 8);
    $self.$right_converter(&$right_value);
    // body
    $self.update_reference_position(end_position + 12);
    $self.$body_converter(&$body_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_for_of_statement {
  ($self:expr, span => $span:expr, await => $await_value:expr, left => [$left_value:expr, $left_converter:ident], right => [$right_value:expr, $right_converter:ident], body => [$body_value:expr, $body_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(
      &30u32.to_ne_bytes(),
      &$span,
      20,
      false,
    );
    // flags
    store_for_of_statement_flags!($self, end_position, await => $await_value);
    // left
    $self.update_reference_position(end_position + 8);
    $self.$left_converter(&$left_value);
    // right
    $self.update_reference_position(end_position + 12);
    $self.$right_converter(&$right_value);
    // body
    $self.update_reference_position(end_position + 16);
    $self.$body_converter(&$body_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_for_statement {
  ($self:expr, span => $span:expr, init => [$init_value:expr, $init_converter:ident], test => [$test_value:expr, $test_converter:ident], update => [$update_value:expr, $update_converter:ident], body => [$body_value:expr, $body_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&31u32.to_ne_bytes(), &$span, 20, false);
    // init
    if let Some(value) = $init_value.as_ref() {
      $self.update_reference_position(end_position + 4);
      $self.$init_converter(value);
    }
    // test
    if let Some(value) = $test_value.as_ref() {
      $self.update_reference_position(end_position + 8);
      $self.$test_converter(value);
    }
    // update
    if let Some(value) = $update_value.as_ref() {
      $self.update_reference_position(end_position + 12);
      $self.$update_converter(value);
    }
    // body
    $self.update_reference_position(end_position + 16);
    $self.$body_converter(&$body_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_if_statement {
  ($self:expr, span => $span:expr, test => [$test_value:expr, $test_converter:ident], consequent => [$consequent_value:expr, $consequent_converter:ident], alternate => [$alternate_value:expr, $alternate_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&35u32.to_ne_bytes(), &$span, 16, false);
    // test
    $self.update_reference_position(end_position + 4);
    $self.$test_converter(&$test_value);
    // consequent
    $self.update_reference_position(end_position + 8);
    $self.$consequent_converter(&$consequent_value);
    // alternate
    if let Some(value) = $alternate_value.as_ref() {
      $self.update_reference_position(end_position + 12);
      $self.$alternate_converter(value);
    }
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_import_default_specifier {
  ($self:expr, span => $span:expr, local => [$local_value:expr, $local_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&38u32.to_ne_bytes(), &$span, 8, false);
    // local
    $self.update_reference_position(end_position + 4);
    $self.$local_converter(&$local_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_import_namespace_specifier {
  ($self:expr, span => $span:expr, local => [$local_value:expr, $local_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&40u32.to_ne_bytes(), &$span, 8, false);
    // local
    $self.update_reference_position(end_position + 4);
    $self.$local_converter(&$local_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_import_specifier {
  ($self:expr, span => $span:expr, imported => [$imported_value:expr, $imported_converter:ident], local => [$local_value:expr, $local_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&41u32.to_ne_bytes(), &$span, 12, false);
    // imported
    if let Some(value) = $imported_value.as_ref() {
      $self.update_reference_position(end_position + 4);
      $self.$imported_converter(value);
    }
    // local
    $self.update_reference_position(end_position + 8);
    $self.$local_converter(&$local_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_labeled_statement {
  ($self:expr, span => $span:expr, label => [$label_value:expr, $label_converter:ident], body => [$body_value:expr, $body_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&56u32.to_ne_bytes(), &$span, 12, false);
    // label
    $self.update_reference_position(end_position + 4);
    $self.$label_converter(&$label_value);
    // body
    $self.update_reference_position(end_position + 8);
    $self.$body_converter(&$body_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_literal_big_int {
  ($self:expr, span => $span:expr, bigint => $bigint_value:expr, raw => $raw_value:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&57u32.to_ne_bytes(), &$span, 12, false);
    // bigint
    $self.convert_string($bigint_value, end_position + 4);
    // raw
    $self.convert_string($raw_value, end_position + 8);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_literal_boolean {
  ($self:expr, span => $span:expr, value => $value_value:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(
      &58u32.to_ne_bytes(),
      &$span,
      8,
      false,
    );
    // flags
    store_literal_boolean_flags!($self, end_position, value => $value_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_literal_null {
  ($self:expr, span => $span:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&59u32.to_ne_bytes(), &$span, 4, false);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_literal_number {
  ($self:expr, span => $span:expr, raw => $raw_value:expr, value => $value_value:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&60u32.to_ne_bytes(), &$span, 16, false);
    // raw
    if let Some(value) = $raw_value.as_ref() {
      $self.convert_string(value, end_position + 4);
    }
    // value
    let value_position = end_position + 8;
    $self.buffer[value_position..value_position + 8].copy_from_slice(&$value_value.to_le_bytes());
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_literal_reg_exp {
  ($self:expr, span => $span:expr, flags => $flags_value:expr, pattern => $pattern_value:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&61u32.to_ne_bytes(), &$span, 12, false);
    // flags
    $self.convert_string($flags_value, end_position + 4);
    // pattern
    $self.convert_string($pattern_value, end_position + 8);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_literal_string {
  ($self:expr, span => $span:expr, value => $value_value:expr, raw => $raw_value:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&62u32.to_ne_bytes(), &$span, 12, false);
    // value
    $self.convert_string($value_value, end_position + 4);
    // raw
    if let Some(value) = $raw_value.as_ref() {
      $self.convert_string(value, end_position + 8);
    }
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_object_expression {
  ($self:expr, span => $span:expr, properties => [$properties_value:expr, $properties_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&68u32.to_ne_bytes(), &$span, 8, false);
    // properties
    $self.convert_item_list(
      &$properties_value,
      end_position + 4,
      |ast_converter, node| {
        ast_converter.$properties_converter(node);
        true
      },
    );
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_object_pattern {
  ($self:expr, span => $span:expr, properties => [$properties_value:expr, $properties_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&69u32.to_ne_bytes(), &$span, 8, false);
    // properties
    $self.convert_item_list(
      &$properties_value,
      end_position + 4,
      |ast_converter, node| {
        ast_converter.$properties_converter(node);
        true
      },
    );
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_private_identifier {
  ($self:expr, span => $span:expr, name => $name_value:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&70u32.to_ne_bytes(), &$span, 8, false);
    // name
    $self.convert_string($name_value, end_position + 4);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_return_statement {
  ($self:expr, span => $span:expr, argument => [$argument_value:expr, $argument_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&75u32.to_ne_bytes(), &$span, 8, false);
    // argument
    if let Some(value) = $argument_value.as_ref() {
      $self.update_reference_position(end_position + 4);
      $self.$argument_converter(value);
    }
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_sequence_expression {
  ($self:expr, span => $span:expr, expressions => [$expressions_value:expr, $expressions_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&76u32.to_ne_bytes(), &$span, 8, false);
    // expressions
    $self.convert_item_list(
      &$expressions_value,
      end_position + 4,
      |ast_converter, node| {
        ast_converter.$expressions_converter(node);
        true
      },
    );
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_static_block {
  ($self:expr, span => $span:expr, body => [$body_value:expr, $body_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&78u32.to_ne_bytes(), &$span, 8, false);
    // body
    $self.convert_item_list(&$body_value, end_position + 4, |ast_converter, node| {
      ast_converter.$body_converter(node);
      true
    });
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_super_element {
  ($self:expr, span => $span:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&79u32.to_ne_bytes(), &$span, 4, false);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_switch_case {
  ($self:expr, span => $span:expr, test => [$test_value:expr, $test_converter:ident], consequent => [$consequent_value:expr, $consequent_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&80u32.to_ne_bytes(), &$span, 12, false);
    // test
    if let Some(value) = $test_value.as_ref() {
      $self.update_reference_position(end_position + 4);
      $self.$test_converter(value);
    }
    // consequent
    $self.convert_item_list(
      &$consequent_value,
      end_position + 8,
      |ast_converter, node| {
        ast_converter.$consequent_converter(node);
        true
      },
    );
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_switch_statement {
  ($self:expr, span => $span:expr, discriminant => [$discriminant_value:expr, $discriminant_converter:ident], cases => [$cases_value:expr, $cases_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&81u32.to_ne_bytes(), &$span, 12, false);
    // discriminant
    $self.update_reference_position(end_position + 4);
    $self.$discriminant_converter(&$discriminant_value);
    // cases
    $self.convert_item_list(&$cases_value, end_position + 8, |ast_converter, node| {
      ast_converter.$cases_converter(node);
      true
    });
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_tagged_template_expression {
  ($self:expr, span => $span:expr, tag => [$tag_value:expr, $tag_converter:ident], quasi => [$quasi_value:expr, $quasi_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&82u32.to_ne_bytes(), &$span, 12, false);
    // tag
    $self.update_reference_position(end_position + 4);
    $self.$tag_converter(&$tag_value);
    // quasi
    $self.update_reference_position(end_position + 8);
    $self.$quasi_converter(&$quasi_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_template_element {
  ($self:expr, span => $span:expr, tail => $tail_value:expr, cooked => $cooked_value:expr, raw => $raw_value:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(
      &83u32.to_ne_bytes(),
      &$span,
      16,
      false,
    );
    // flags
    store_template_element_flags!($self, end_position, tail => $tail_value);
    // cooked
    if let Some(value) = $cooked_value.as_ref() {
      $self.convert_string(value, end_position + 8);
    }
    // raw
    $self.convert_string($raw_value, end_position + 12);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_this_expression {
  ($self:expr, span => $span:expr) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&85u32.to_ne_bytes(), &$span, 4, false);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_throw_statement {
  ($self:expr, span => $span:expr, argument => [$argument_value:expr, $argument_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&86u32.to_ne_bytes(), &$span, 8, false);
    // argument
    $self.update_reference_position(end_position + 4);
    $self.$argument_converter(&$argument_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_unary_expression {
  ($self:expr, span => $span:expr, operator => $operator_value:expr, argument => [$argument_value:expr, $argument_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&88u32.to_ne_bytes(), &$span, 12, false);
    // operator
    let operator_position = end_position + 4;
    $self.buffer[operator_position..operator_position + 4].copy_from_slice($operator_value);
    // argument
    $self.update_reference_position(end_position + 8);
    $self.$argument_converter(&$argument_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_update_expression {
  ($self:expr, span => $span:expr, prefix => $prefix_value:expr, operator => $operator_value:expr, argument => [$argument_value:expr, $argument_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(
      &89u32.to_ne_bytes(),
      &$span,
      16,
      false,
    );
    // flags
    store_update_expression_flags!($self, end_position, prefix => $prefix_value);
    // operator
    let operator_position = end_position + 8;
    $self.buffer[operator_position..operator_position + 4].copy_from_slice($operator_value);
    // argument
    $self.update_reference_position(end_position + 12);
    $self.$argument_converter(&$argument_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_while_statement {
  ($self:expr, span => $span:expr, test => [$test_value:expr, $test_converter:ident], body => [$body_value:expr, $body_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(&92u32.to_ne_bytes(), &$span, 12, false);
    // test
    $self.update_reference_position(end_position + 4);
    $self.$test_converter(&$test_value);
    // body
    $self.update_reference_position(end_position + 8);
    $self.$body_converter(&$body_value);
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_yield_expression {
  ($self:expr, span => $span:expr, delegate => $delegate_value:expr, argument => [$argument_value:expr, $argument_converter:ident]) => {
    let _: &mut AstConverter = $self;
    let end_position = $self.add_type_and_start(
      &93u32.to_ne_bytes(),
      &$span,
      12,
      false,
    );
    // flags
    store_yield_expression_flags!($self, end_position, delegate => $delegate_value);
    // argument
    if let Some(value) = $argument_value.as_ref() {
      $self.update_reference_position(end_position + 8);
      $self.$argument_converter(value);
    }
    // end
    $self.add_end(end_position, &$span);
  };
}

#[macro_export]
macro_rules! store_arrow_function_expression_flags {
  ($self:expr, $end_position:expr, async => $async_value:expr, expression => $expression_value:expr, generator => $generator_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $async_value {
      flags |= 1;
    }
    if $expression_value {
      flags |= 2;
    }
    if $generator_value {
      flags |= 4;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_call_expression_flags {
  ($self:expr, $end_position:expr, optional => $optional_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $optional_value {
      flags |= 1;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_for_of_statement_flags {
  ($self:expr, $end_position:expr, await => $await_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $await_value {
      flags |= 1;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_function_declaration_flags {
  ($self:expr, $end_position:expr, async => $async_value:expr, generator => $generator_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $async_value {
      flags |= 1;
    }
    if $generator_value {
      flags |= 2;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_j_s_x_opening_element_flags {
  ($self:expr, $end_position:expr, selfClosing => $selfClosing_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $selfClosing_value {
      flags |= 1;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_literal_boolean_flags {
  ($self:expr, $end_position:expr, value => $value_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $value_value {
      flags |= 1;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_member_expression_flags {
  ($self:expr, $end_position:expr, computed => $computed_value:expr, optional => $optional_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $computed_value {
      flags |= 1;
    }
    if $optional_value {
      flags |= 2;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_method_definition_flags {
  ($self:expr, $end_position:expr, static => $static_value:expr, computed => $computed_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $static_value {
      flags |= 1;
    }
    if $computed_value {
      flags |= 2;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_property_flags {
  ($self:expr, $end_position:expr, method => $method_value:expr, shorthand => $shorthand_value:expr, computed => $computed_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $method_value {
      flags |= 1;
    }
    if $shorthand_value {
      flags |= 2;
    }
    if $computed_value {
      flags |= 4;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_property_definition_flags {
  ($self:expr, $end_position:expr, static => $static_value:expr, computed => $computed_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $static_value {
      flags |= 1;
    }
    if $computed_value {
      flags |= 2;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_template_element_flags {
  ($self:expr, $end_position:expr, tail => $tail_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $tail_value {
      flags |= 1;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_update_expression_flags {
  ($self:expr, $end_position:expr, prefix => $prefix_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $prefix_value {
      flags |= 1;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}

#[macro_export]
macro_rules! store_yield_expression_flags {
  ($self:expr, $end_position:expr, delegate => $delegate_value:expr) => {
    let _: &mut AstConverter = $self;
    let _: usize = $end_position;
    let mut flags = 0u32;
    if $delegate_value {
      flags |= 1;
    }
    let flags_position = $end_position + 4;
    $self.buffer[flags_position..flags_position + 4].copy_from_slice(&flags.to_ne_bytes());
  };
}
