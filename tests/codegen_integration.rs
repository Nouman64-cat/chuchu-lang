use chuchu::codegen::generate_module;
use inkwell::context::Context;
use inkwell::values::InstructionOpcode;

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
