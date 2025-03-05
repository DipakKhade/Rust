#[test] fn model_single_field() { run_reformat_test("model_single_field.prisma"); }
#[test] fn type_aliases() { run_reformat_test("type_aliases.prisma"); }
#[test] fn relations_back_relation_fields_must_be_added() { run_reformat_test("relations/back_relation_fields_must_be_added.prisma"); }
#[test] fn relations_native_types_in_missing_relation_fields() { run_reformat_test("relations/native_types_in_missing_relation_fields.prisma"); }
#[test] fn relations_forward_relation_fields_must_be_added() { run_reformat_test("relations/forward_relation_fields_must_be_added.prisma"); }
#[test] fn relations_back_relation_fields_and_attribute_must_be_added_even_when_attribute_is_missing() { run_reformat_test("relations/back_relation_fields_and_attribute_must_be_added_even_when_attribute_is_missing.prisma"); }
#[test] fn trailing_comments_allowed_in_configuration_blocks() { run_reformat_test("trailing_comments_allowed_in_configuration_blocks.prisma"); }
#[test] fn regression_add_relation_attribute_on_field_with_multi_byte_trailing_comment() { run_reformat_test("regression_add_relation_attribute_on_field_with_multi_byte_trailing_comment.prisma"); }
#[test] fn block_attribute_comments_are_preserved() { run_reformat_test("block_attribute_comments_are_preserved.prisma"); }
#[test] fn catch_all_in_a_block_must_not_influence_table_layout() { run_reformat_test("catch_all_in_a_block_must_not_influence_table_layout.prisma"); }
#[test] fn model_block_attributes_ordering() { run_reformat_test("model_block_attributes_ordering.prisma"); }
#[test] fn optional_list_fields() { run_reformat_test("optional_list_fields.prisma"); }
#[test] fn model_field_attributes_ordering() { run_reformat_test("model_field_attributes_ordering.prisma"); }
