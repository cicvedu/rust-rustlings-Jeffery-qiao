// drive4.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.



// This execrise shares build.rs with the previous exercise.
// You need to add some code to build.rs to make both this exercise and
// the previous one work.
use std::env;

fn main() {
    // 设置 TEST_FOO 环境变量为所需范围内的值
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let test_foo_value = timestamp + 5; // 设置为当前时间戳 + 5，可根据需求进行调整

    // 将 TEST_FOO 环境变量设置为所需值
    env::set_var("TEST_FOO", test_foo_value.to_string());

    // 检查是否设置了 feature "pass"，如果设置了，则返回，否则继续执行下面的代码
    #[cfg(not(feature = "pass"))]
    {
        // 添加你想要执行的其他构建操作代码
        // ...

        // 示例：设置另一个环境变量
        env::set_var("ANOTHER_VARIABLE", "some value");

        // 示例：执行其他构建操作
        // ...
    }
}