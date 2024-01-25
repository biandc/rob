// use thirtyfour::prelude::*;
// use tokio;


// #[tokio::main]
// async fn main() -> WebDriverResult<()> {
//     // 创建一个新的 Firefox WebDriver 实例
//     let caps = DesiredCapabilities::firefox();
//     let driver = WebDriver::new("http://localhost:4444", &caps).await?;

//     // 访问一个网址
//     driver.get("https://www.baidu.com").await?;

//     // 关闭 WebDriver
//     driver.quit().await?;

//     Ok(())
// }

use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::{Command, Stdio};

fn main() -> Result<(), Error> {
    // 创建一个 Command 对象，指定要运行的外部命令
    let mut cmd = Command::new("geckodriver");

    // 添加命令行参数
    cmd.args(&["--port", "44444"]);

    // 将子进程的输出重定向到管道
    cmd.stdout(Stdio::piped());

    // 运行命令
    let mut child = cmd.spawn()?;

    // 获取子进程的输出
    let reader = BufReader::new(child.stdout.take().expect(
        "Failed to get the stdout of the command",
    ));

    // 创建一个新的文件，用于存储命令的输出
    let mut output_file = File::create("output.txt")?;

    // 实时打印子进程的输出并将其写入文件
    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
                writeln!(output_file, "{}", line)?;
            }
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }

    // 等待命令完成
    let status = child.wait()?;
    println!("Command exited with status: {}", status);

    Ok(())
}
