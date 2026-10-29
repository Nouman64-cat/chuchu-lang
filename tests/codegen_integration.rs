use std::collections::HashMap;

use chuchu::ast::{BinaryOperator, Expr, ExprKind, Stmt};
use chuchu::codegen::{
    CodegenError, generate_declaration, generate_expression, generate_module, generate_program,
};
use chuchu::lexer::tokenize;
use chuchu::parser::Parser;
use chuchu::token::Span;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{InstructionOpcode, IntValue};

fn parse_statement(source: &str) -> Stmt {
    let tokens = tokenize(source).expect("Statement should tokenize");
    Parser::new(tokens)
        .parse_statement()
        .expect("Statement should parse")
}

#[test]
fn generates_complete_program_with_shared_printf() {
    let context = Context::create();
    let statements = Parser::new(
        tokenize("assign x to 10\nassign y to x + 20\nshow y\nshow 42\nshow x + y").unwrap(),
    )
    .parse_program()
    .unwrap();
    let module = generate_program(&context, &statements).expect("Program should generate");
    module.verify().expect("Program should verify");
    let main = module.get_function("main").expect("Main should exist");
    assert_eq!(
        main.get_type().get_return_type(),
        Some(context.i32_type().into())
    );
    let printf = module
        .get_function("printf")
        .expect("Printf should be declared");
    assert!(printf.get_type().is_var_arg());
    assert_eq!(printf.count_params(), 1);
    assert_eq!(
        module
            .get_functions()
            .filter(|function| function.get_name().to_bytes() == b"printf")
            .count(),
        1
    );
    assert_eq!(
        module
            .get_globals()
            .filter(|global| global.get_name().to_bytes() == b".cheeku.format.i64")
            .count(),
        1
    );
    let ir = module.print_to_string().to_string();
    assert_eq!(ir.matches("call i32 (ptr, ...) @printf").count(), 3);
    assert!(ir.contains("%lld\\0A\\00"));
    assert!(ir.contains("ret i32 0"));
    // Program locals now use entry-block storage to support reassignment and control flow.
    assert!(ir.contains("alloca i64"));
}

#[test]
fn generates_empty_program_without_output_runtime() {
    let context = Context::create();
    let module = generate_program(&context, &[]).expect("Empty program should generate");
    assert!(module.get_function("printf").is_none());
    module.verify().expect("Empty program should verify");
}

#[test]
fn storage_is_allocated_only_in_function_entries() {
    let context = Context::create();
    let statements = Parser::new(tokenize("function f(n: int) -> int {\nassign i to 0\nwhile i < n {\nassign a to [1, 2] * [3, 4]\nreassign i to i + 1\n}\nreturn i\n}\nshow f(2)").unwrap()).parse_program().unwrap();
    let module = generate_program(&context, &statements).expect("Program should generate");
    for function in module
        .get_functions()
        .filter(|function| function.count_basic_blocks() > 0)
    {
        assert!(function.verify(false));
        let entry = function.get_first_basic_block().unwrap();
        for block in function.get_basic_blocks() {
            for instruction in block.get_instructions() {
                if instruction.get_opcode() == InstructionOpcode::Alloca {
                    assert_eq!(block, entry);
                }
            }
        }
    }
    let ir = module.print_to_string().to_string();
    assert!(ir.contains("llvm.sadd.with.overflow.i64"));
    assert!(ir.contains("llvm.smul.with.overflow.i64"));
    assert!(!ir.contains("nsw"));
    assert!(!ir.contains("nuw"));
}

#[test]
fn codegen_rejects_invalid_types_without_panicking() {
    let context = Context::create();
    for source in [
        "show true + 1",
        "show [1][false]",
        "show missing(1)",
        "function f() -> int { show 1 }",
    ] {
        let statements = Parser::new(tokenize(source).unwrap())
            .parse_program()
            .unwrap();
        assert!(generate_program(&context, &statements).is_err(), "{source}");
    }
}

#[test]
fn program_generation_propagates_declaration_errors() {
    let context = Context::create();
    let statements = Parser::new(tokenize("assign x to 1\nassign x to 2").unwrap())
        .parse_program()
        .unwrap();
    assert!(
        matches!(generate_program(&context, &statements), Err(CodegenError::DuplicateDeclaration { name, .. }) if name == "x")
    );
}

fn binding_test_function(context: &Context) -> (Module<'_>, Builder<'_>) {
    let module = context.create_module("binding_test");
    let builder = context.create_builder();
    let function = module.add_function("test", context.i64_type().fn_type(&[], false), None);
    builder.position_at_end(context.append_basic_block(function, "entry"));
    (module, builder)
}

fn verify_binding_function<'ctx>(
    module: &Module<'ctx>,
    builder: &Builder<'ctx>,
    value: IntValue<'ctx>,
) {
    builder
        .build_return(Some(&value))
        .expect("Return should build");
    module.verify().expect("Binding module should verify");
    assert!(!module.print_to_string().to_string().contains("alloca"));
}

#[test]
fn generates_variable_declaration() {
    let context = Context::create();
    let (module, builder) = binding_test_function(&context);
    let mut bindings = HashMap::new();

    generate_declaration(
        &context,
        &builder,
        &mut bindings,
        &parse_statement("assign x to 10"),
    )
    .expect("Declaration should generate");

    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings["x"].get_type(), context.i64_type());
    assert_eq!(bindings["x"].get_sign_extended_constant(), Some(10));
    verify_binding_function(&module, &builder, bindings["x"]);
}

#[test]
fn resolves_declared_identifier_to_same_llvm_value() {
    let context = Context::create();
    let (module, builder) = binding_test_function(&context);
    let mut bindings = HashMap::new();
    generate_declaration(
        &context,
        &builder,
        &mut bindings,
        &parse_statement("assign x to 10"),
    )
    .expect("Declaration should generate");

    let value = generate_expression(&context, &builder, &bindings, &parse_expression("x"))
        .expect("Identifier should resolve");
    assert_eq!(value, bindings["x"]);
    verify_binding_function(&module, &builder, value);
}

#[test]
fn generates_addition_with_previously_declared_variable() {
    let context = Context::create();
    let (module, builder) = binding_test_function(&context);
    let mut bindings = HashMap::new();
    generate_declaration(
        &context,
        &builder,
        &mut bindings,
        &parse_statement("assign x to 10"),
    )
    .expect("Declaration should generate");

    let value = generate_expression(&context, &builder, &bindings, &parse_expression("x + 20"))
        .expect("Addition should generate");
    assert_eq!(value.get_type(), context.i64_type());
    assert_eq!(value.get_sign_extended_constant(), Some(30));
    verify_binding_function(&module, &builder, value);
}

#[test]
fn generates_declarations_referencing_earlier_bindings() {
    let context = Context::create();
    let (module, builder) = binding_test_function(&context);
    let mut bindings = HashMap::new();

    for source in [
        "assign x to 10",
        "assign y to x + 20",
        "assign z to x + y + 20",
        "assign alias to z",
    ] {
        generate_declaration(&context, &builder, &mut bindings, &parse_statement(source))
            .expect("Declaration should generate");
    }
    assert_eq!(bindings.len(), 4);
    assert_eq!(bindings["y"].get_sign_extended_constant(), Some(30));
    assert_eq!(bindings["z"].get_sign_extended_constant(), Some(60));
    assert_eq!(bindings["alias"], bindings["z"]);
    verify_binding_function(&module, &builder, bindings["alias"]);
}

#[test]
fn rejects_duplicate_bindings_without_overwriting() {
    let context = Context::create();
    let (module, builder) = binding_test_function(&context);
    let mut bindings = HashMap::new();
    generate_declaration(
        &context,
        &builder,
        &mut bindings,
        &parse_statement("assign x to 10"),
    )
    .expect("First declaration should generate");
    let original = bindings.clone();

    for source in ["  assign x to 20", "assign x to unknown"] {
        let statement = parse_statement(source);
        assert!(
            matches!(generate_declaration(&context, &builder, &mut bindings, &statement),
            Err(CodegenError::DuplicateDeclaration { name, span })
                if name == "x" && span == statement.span)
        );
        assert_eq!(bindings, original);
    }
    verify_binding_function(&module, &builder, bindings["x"]);
}

#[test]
fn failed_initializers_do_not_register_bindings() {
    let context = Context::create();
    let (module, builder) = binding_test_function(&context);
    let mut bindings = HashMap::new();
    generate_declaration(
        &context,
        &builder,
        &mut bindings,
        &parse_statement("assign prior to 10"),
    )
    .expect("Earlier declaration should generate");
    let original = bindings.clone();

    for (source, name, span) in [
        (
            "assign x to unknown",
            "unknown",
            Span { start: 12, end: 19 },
        ),
        ("assign x to x + 20", "x", Span { start: 12, end: 13 }),
        (
            "assign x to prior + unknown",
            "unknown",
            Span { start: 20, end: 27 },
        ),
    ] {
        assert!(
            matches!(generate_declaration(&context, &builder, &mut bindings, &parse_statement(source)),
            Err(CodegenError::UndefinedVariable { name: actual, span: actual_span })
                if actual == name && actual_span == span)
        );
        assert_eq!(bindings, original);
    }
    assert!(matches!(
        generate_declaration(
            &context,
            &builder,
            &mut bindings,
            &parse_statement("assign x to 9223372036854775808")
        ),
        Err(CodegenError::InvalidIntegerLiteral {
            span: Span { start: 12, end: 31 },
            ..
        })
    ));
    assert_eq!(bindings, original);

    generate_declaration(
        &context,
        &builder,
        &mut bindings,
        &parse_statement("assign x to prior"),
    )
    .expect("Failed declarations should not reserve a name");
    verify_binding_function(&module, &builder, bindings["x"]);
}

#[test]
fn binding_environments_are_independent() {
    let context = Context::create();
    let (module, builder) = binding_test_function(&context);
    let mut first = HashMap::new();
    let mut second = HashMap::new();
    generate_declaration(
        &context,
        &builder,
        &mut first,
        &parse_statement("assign x to 10"),
    )
    .expect("First environment should accept declaration");
    assert!(
        matches!(generate_expression(&context, &builder, &second, &parse_expression("x")),
        Err(CodegenError::UndefinedVariable { name, .. }) if name == "x")
    );
    generate_declaration(
        &context,
        &builder,
        &mut second,
        &parse_statement("assign x to 20"),
    )
    .expect("Second environment should accept same name");
    assert_eq!(first["x"].get_sign_extended_constant(), Some(10));
    assert_eq!(second["x"].get_sign_extended_constant(), Some(20));
    verify_binding_function(&module, &builder, second["x"]);
}

#[test]
fn rejects_show_in_declaration_codegen() {
    let context = Context::create();
    let (module, builder) = binding_test_function(&context);
    let mut bindings = HashMap::new();
    let statement = parse_statement("show 42");
    assert!(
        matches!(generate_declaration(&context, &builder, &mut bindings, &statement),
        Err(CodegenError::UnsupportedStatement { span }) if span == statement.span)
    );
    assert!(bindings.is_empty());
    verify_binding_function(&module, &builder, context.i64_type().const_zero());
}

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

    let value = generate_expression(&context, &builder, &HashMap::new(), expression)
        .expect("Addition should generate");
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
            matches!(generate_expression(&context, &builder, &HashMap::new(), &expression),
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
            matches!(generate_expression(&context, &builder, &HashMap::new(), &expression),
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
fn propagates_undefined_identifier_errors_inside_additions() {
    let context = Context::create();
    let module = context.create_module("unsupported_addition_test");
    let builder = context.create_builder();
    let function = module.add_function("test", context.i64_type().fn_type(&[], false), None);
    builder.position_at_end(context.append_basic_block(function, "entry"));

    for source in ["unknown + 10", "10 + unknown", "10 + 20 + unknown"] {
        let expression = parse_expression(source);
        let start = source.find("unknown").expect("Identifier should exist");
        assert!(
            matches!(generate_expression(&context, &builder, &HashMap::new(), &expression),
            Err(CodegenError::UndefinedVariable { name, span }) if name == "unknown" && span == Span { start, end: start + 7 })
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
        matches!(generate_expression(&context, &builder, &HashMap::new(), &expression),
        Err(CodegenError::InvalidInsertionPoint { span }) if span == expression.span)
    );

    let module = context.create_module("terminated_block_test");
    let function = module.add_function("test", context.i64_type().fn_type(&[], false), None);
    builder.position_at_end(context.append_basic_block(function, "entry"));
    builder
        .build_return(Some(&context.i64_type().const_zero()))
        .expect("Return should build");

    assert!(
        matches!(generate_expression(&context, &builder, &HashMap::new(), &expression),
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
    let value = generate_expression(&context, &builder, &HashMap::new(), &parse_expression("42"))
        .expect("Integer literal should generate");

    assert_eq!(value.get_sign_extended_constant(), Some(42));
    assert_eq!(value.get_type(), context.i64_type());
}

#[test]
fn generates_integer_literal_zero() {
    let context = Context::create();
    let builder = context.create_builder();
    let value = generate_expression(&context, &builder, &HashMap::new(), &parse_expression("0"))
        .expect("Zero literal should generate");

    assert_eq!(value.get_sign_extended_constant(), Some(0));
    assert_eq!(value.get_type(), context.i64_type());
}

#[test]
fn generates_largest_signed_integer_literal() {
    let context = Context::create();
    let builder = context.create_builder();
    let value = generate_expression(
        &context,
        &builder,
        &HashMap::new(),
        &parse_expression("9223372036854775807"),
    )
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
        let error = generate_expression(&context, &builder, &HashMap::new(), &expression)
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
            matches!(generate_expression(&context, &builder, &HashMap::new(), &expression),
            Err(CodegenError::InvalidIntegerLiteral { literal: actual, span: actual_span })
                if actual == literal && actual_span == span)
        );
    }
}

#[test]
fn rejects_undefined_identifiers() {
    let context = Context::create();
    let builder = context.create_builder();
    for source in ["  total", "unknown"] {
        let expression = parse_expression(source);
        assert!(
            matches!(generate_expression(&context, &builder, &HashMap::new(), &expression),
            Err(CodegenError::UndefinedVariable { name, span }) if name == source.trim() && span == expression.span)
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
