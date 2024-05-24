#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/test01_construct_ipc_handler.rs");
    t.pass("tests/test02_with_inputs.rs");
    t.pass("tests/test03_ipc_handlers_macro.rs");
    t.pass("tests/test04_async_command.rs");
    t.pass("tests/test05_async_command_with_inputs.rs");
    t.pass("tests/test06_custom_id.rs");
}