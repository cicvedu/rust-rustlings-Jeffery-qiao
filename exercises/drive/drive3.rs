// drive3.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.


// We look for an environment variable and expect it to fall in a range.
// look into the testcase to find out the details.
// You should not modify this file. Modify `build.rs` to pass this exercise.
use std::env;

fn main() {
    // 设置 TEST_FOO 环境变量为所需范围内的值
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let test_foo_value = timestamp + 5; // 设置为当前时间戳 + 5，可根据需求进行调整

    // 将 TEST_FOO 环境变量设置为所需值
    env::set_var("TEST_FOO", test_foo_value.to_string());
}