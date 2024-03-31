use std::{f32::consts::E, fs::File, io::ErrorKind};

fn main() {
    // 在实践中有两种方法造成 panic：执行会造成代码 panic 的操作（比如访问超过数组结尾的内容）或者显式调用 panic! 宏。这两种情况都会使程序 panic。通常情况下这些 panic 会打印出一个错误信息，展开并清理栈数据，然后退出。
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let greeting_file_name = String::from("hello.txt");
    let greeting_file_result = File::open(&greeting_file_name);
    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&greeting_file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("fail creating the file '{}': {:#?}", &greeting_file_name, e),
            },
            other_error => panic!("fail opening file '{}': {:#?}", &greeting_file_name, other_error),
        } 
    };
}
