pub mod first { //一层
    pub fn print_chars() {
        for c in b'a'..=b'z' {
            println!("{}", c as char);
        }
        for c in b'A'..=b'Z' {
            println!("{}", c as char);
        }
        println!("一层");
    }

    pub mod second {//二层
        pub fn print_chars() {//排除掉那种[]^_`字符
            for c in b'A'..=b'Z' {
                println!("{}", c as char);
            }
            for c in b'a'..=b'z' {
                println!("{}", c as char);
            }
            println!("二层");
        }
    }
}

fn main() {
    first::print_chars();
    first::second::print_chars();
}
