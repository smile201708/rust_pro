macro_rules! say_hi {
    ($val:expr) => {
        println!("Hi, I am {}!", $val);
    };
}

fn main() {
    say_hi!("Smile");
    say_hi!("Lily");
}

/*1. 代码结构
- macro_rules! 定义了一个名为say_hi的声明宏
- $val:expr 定义了一个名为val的表达式参数
- 使用print!宏输出一个say hi 语句
2. 编译过程
- 预处理阶段:编译器处理宏,展开say_hi!("Smile")
- 替换阶段:用"Smile"替换$val,生成print!宏语句
- 解析阶段:解析展开后的println语句
- 代码生成:生成最终的可执行代码
- 运行时:打印输出“Hi, I am Smile!”
3. 特点
- 编译期展开:宏展开发生在编译期
- 语法扩展:自定义say_hi!类似语法
- 参数化:基于$val参数生成不同代码
- 抽象:抽象出println细节代码
4. 好处
- 提高重用性:避免重复 println 语句
- 提高抽象能力:用声明式方式生成代码
- 扩展语言:增加say_hi!自定义语法
综上,声明宏通过在编译期代码生成,实现了代码重用和抽象的能力,扩展了Rust语言表达能力*/