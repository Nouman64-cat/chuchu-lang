use inkwell::context::Context;

#[test]
fn verifies_empty_llvm_module() {
    let context = Context::create();
    let module = context.create_module("cheeku_test");

    module.verify().expect("Empty LLVM module should be valid");
}
