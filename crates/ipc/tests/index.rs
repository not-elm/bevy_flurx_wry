#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/test01_construct_ipc_handler.rs");
    t.pass("tests/test02_with_inputs.rs");
    t.pass("tests/test03_ipc_handlers_macro.rs");
}