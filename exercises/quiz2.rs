// quiz2.rs
// 这是一个关于以下部分的小测验
// - 向量（Vecs）
// - 移动语义（Move semantics）
// - 模块（模块）
// - 枚举（枚举）
// //
// 让我们构建一个小型机器，它是一个函数的形式。作为输入，我们将提供一个字符串和命令的列表。这些命令确定对字符串要执行的操作。它可以是：
// - 将字符串转换为大写
// - 去除字符串的空白部分
// - 将字符串附加指定次数的 “bar”
// 具体形式如下：
// - 输入将是一个二元组的向量，长度为2，第一个元素是字符串，第二个元素是命令。
// - 输出元素将是一个字符串的向量。
// //


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => {
                    output.push(string.to_uppercase());
                }
                Command::Trim => {
                    output.push(string.trim().to_string());
                }
                Command::Append(n) => {
                    let appended_string = string.clone() + &"bar".repeat(*n);
                    output.push(appended_string);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
