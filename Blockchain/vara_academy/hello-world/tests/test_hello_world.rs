use gtest::{Log, Program, System};

#[test]
fn init_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("INIT MESSAGE"));
    assert!(!res.main_failed());
    assert!(!res.log().is_empty());
}

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("Hello"));

    let expected_log = Log::builder().dest(2).payload(String::from("Hello world"));
    assert!(res.contains(&expected_log));
}
