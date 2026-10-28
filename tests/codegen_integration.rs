use chuchu::ast::{BinaryOperator, Expr, ExprKind};
use chuchu::codegen::{CodegenError, generate_expression, generate_module};
use chuchu::lexer::tokenize;
use chuchu::parser::Parser;
use chuchu::token::Span;
use inkwell::context::Context;
use inkwell::values::InstructionOpcode;

fn parse_expression(source: &str) -> Expr {
    let tokens = tokenize(source).expect("Expression should tokenize");
    Parser::new(tokens)
        .parse_expression()
        .expect("Expression should parse")
}

fn assert_generated_addition(expression: &Expr, expected: i64) {
    let context = Context::create();
    let module = context.create_module("addition_test");
    let builder = context.create_builder();
    let function = module.add_function("test", context.i64_type().fn_type(&[], false), None);
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    let value =
        generate_expression(&context, &builder, expression).expect("Addition should generate");
    assert_eq!(value.get_type(), context.i64_type());
    // LLVM may fold constant-only additions instead of retaining add instructions.
    assert_eq!(value.get_sign_extended_constant(), Some(expected));
    builder
        .build_return(Some(&value))
        .expect("Return should build");
    module.verify().expect("Addition module should verify");
    let ir = module.print_to_string().to_string();
    assert!(!ir.contains("nsw"));
    assert!(!ir.contains("nuw"));
}

#[test]
fn generates_integer_addition() {
    assert_generated_addition(&parse_expression("10 + 20"), 30);
}

#[test]
fn generates_chained_additions() {
    assert_generated_addition(&parse_expression("10 + 20 + 30"), 60);
}

#[test]
fn generates_nested_additions() {
    // Right-nested addition is constructed directly; parentheses are not Chuchu syntax.
    let expression = Expr {
        kind: ExprKind::Binary {
            left: Box::new(parse_expression("10")),
            operator: BinaryOperator::Add,
            right: Box::new(parse_expression("20 + 30 + 40")),
        },
        span: Span { start: 0, end: 17 },
    };
    assert_generated_addition(&expression, 100);
}

#[test]
fn propagates_integer_errors_inside_additions() {
    let context = Context::create();
    let module = context.create_module("invalid_addition_test");
    let builder = context.create_builder();
    let function = module.add_function("test", context.i64_type().fn_type(&[], false), None);
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    for source in [
        "9223372036854775808 + 10",
        "10 + 9223372036854775808",
        "10 + 20 + 9223372036854775808",
    ] {
        let expression = parse_expression(source);
        let start = source
            .find("9223372036854775808")
            .expect("Overflowing literal should exist");
        assert!(
            matches!(generate_expression(&context, &builder, &expression),
            Err(CodegenError::InvalidIntegerLiteral { literal, span })
                if literal == "9223372036854775808" && span == Span { start, end: start + 19 })
        );
    }

    for invalid_on_left in [true, false] {
        let invalid = Expr {
            kind: ExprKind::Integer("bad".to_owned()),
            span: Span { start: 5, end: 8 },
        };
        let valid = parse_expression("10");
        let (left, right) = if invalid_on_left {
            (invalid, valid)
        } else {
            (valid, invalid)
        };
        let expression = Expr {
            kind: ExprKind::Binary {
                left: Box::new(left),
                operator: BinaryOperator::Add,
                right: Box::new(right),
            },
            span: Span { start: 0, end: 8 },
        };
        assert!(
            matches!(generate_expression(&context, &builder, &expression),
            Err(CodegenError::InvalidIntegerLiteral { literal, span })
                if literal == "bad" && span == Span { start: 5, end: 8 })
        );
    }

    builder
        .build_return(Some(&context.i64_type().const_zero()))
        .expect("Return should build");
    module
        .verify()
        .expect("Module should remain valid after errors");
}

#[test]
fn propagates_unsupported_identifier_errors_inside_additions() {
    let context = Context::create();
    let module = context.create_module("unsupported_addition_test");
    let builder = context.create_builder();
    let function = module.add_function("test", context.i64_type().fn_type(&[], false), None);
    builder.position_at_end(context.append_basic_block(function, "entry"));

    for source in ["unknown + 10", "10 + unknown", "10 + 20 + unknown"] {
        let expression = parse_expression(source);
        let start = source.find("unknown").expect("Identifier should exist");
        assert!(
            matches!(generate_expression(&context, &builder, &expression),
            Err(CodegenError::UnsupportedExpression { span }) if span == Span { start, end: start + 7 })
        );
    }

    builder
        .build_return(Some(&context.i64_type().const_zero()))
        .expect("Return should build");
    module
        .verify()
        .expect("Module should remain valid after errors");
}

#[test]
fn rejects_addition_without_valid_insertion_point() {
    let context = Context::create();
    let builder = context.create_builder();
    let expression = parse_expression("10 + 20");
    assert!(
        matches!(generate_expression(&context, &builder, &expression),
        Err(CodegenError::InvalidInsertionPoint { span }) if span == expression.span)
    );

    let module = context.create_module("terminated_block_test");
    let function = module.add_function("test", context.i64_type().fn_type(&[], false), None);
    builder.position_at_end(context.append_basic_block(function, "entry"));
    builder
        .build_return(Some(&context.i64_type().const_zero()))
        .expect("Return should build");

    assert!(
        matches!(generate_expression(&context, &builder, &expression),
        Err(CodegenError::InvalidInsertionPoint { span }) if span == expression.span)
    );
    module
        .verify()
        .expect("No instruction should be emitted after return");
}

#[test]
fn generates_integer_literal_42() {
    let context = Context::create();
    let builder = context.create_builder();
    let value = generate_expression(&context, &builder, &parse_expression("42"))
        .expect("Integer literal should generate");

    assert_eq!(value.get_sign_extended_constant(), Some(42));
    assert_eq!(value.get_type(), context.i64_type());
}

#[test]
fn generates_integer_literal_zero() {
    let context = Context::create();
    let builder = context.create_builder();
    let value = generate_expression(&context, &builder, &parse_expression("0"))
        .expect("Zero literal should generate");

    assert_eq!(value.get_sign_extended_constant(), Some(0));
    assert_eq!(value.get_type(), context.i64_type());
}

#[test]
fn generates_largest_signed_integer_literal() {
    let context = Context::create();
    let builder = context.create_builder();
    let value = generate_expression(&context, &builder, &parse_expression("9223372036854775807"))
        .expect("Largest signed i64 literal should generate");

    assert_eq!(value.get_sign_extended_constant(), Some(i64::MAX));
    assert_eq!(value.get_type(), context.i64_type());
}

#[test]
fn rejects_out_of_range_integer_literals() {
    let context = Context::create();
    let builder = context.create_builder();
    for literal in [
        "9223372036854775808",
        "18446744073709551615",
        "999999999999999999999999999999",
    ] {
        let expression = parse_expression(&format!("  {literal}"));
        let error = generate_expression(&context, &builder, &expression)
            .expect_err("Literal should overflow i64");

        assert!(
            matches!(error, CodegenError::InvalidIntegerLiteral { literal: actual, span }
            if actual == literal && span == expression.span)
        );
        assert_eq!(
            expression.span,
            Span {
                start: 2,
                end: 2 + literal.len()
            }
        );
    }
}

#[test]
fn rejects_invalid_integer_ast_literals() {
    let context = Context::create();
    let builder = context.create_builder();
    // Malformed integer text cannot come from the parser, so construct it directly.
    for literal in ["", "abc", "12.5", "1_000"] {
        let span = Span {
            start: 3,
            end: 3 + literal.len(),
        };
        let expression = Expr {
            kind: ExprKind::Integer(literal.to_owned()),
            span,
        };

        assert!(
            matches!(generate_expression(&context, &builder, &expression),
            Err(CodegenError::InvalidIntegerLiteral { literal: actual, span: actual_span })
                if actual == literal && actual_span == span)
        );
    }
}

#[test]
fn rejects_unsupported_expression_kinds() {
    let context = Context::create();
    let builder = context.create_builder();
    for source in ["  total", "unknown"] {
        let expression = parse_expression(source);
        assert!(
            matches!(generate_expression(&context, &builder, &expression),
            Err(CodegenError::UnsupportedExpression { span }) if span == expression.span)
        );
    }
}

#[test]
fn generates_main_returning_zero() {
    let context = Context::create();
    let module = generate_module(&context).expect("Module generation should succeed");
    let main = module
        .get_function("main")
        .expect("Main function should exist");

    assert_eq!(
        main.get_type().get_return_type(),
        Some(context.i32_type().into())
    );
    assert_eq!(main.count_params(), 0);
    assert_eq!(main.count_basic_blocks(), 1);
    let entry = main
        .get_first_basic_block()
        .expect("Main should have an entry block");
    assert_eq!(
        entry
            .get_name()
            .to_str()
            .expect("Block name should be UTF-8"),
        "entry"
    );
    let terminator = entry
        .get_terminator()
        .expect("Entry should end with a terminator");
    assert_eq!(terminator.get_opcode(), InstructionOpcode::Return);
    assert!(module.print_to_string().to_string().contains("ret i32 0"));

    module
        .verify()
        .expect("Generated LLVM module should be valid");
}
