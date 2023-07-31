use std::ops::Add;

struct Point(i32, i32);
struct Complex(i32, i32);

// 定义一个打印 Trait
trait Print {
    fn print(&self);
}

// 为 Point 实现 Print trait
impl Print for Point {
    fn print(&self) {
        println!("Point: ({}, {})", self.0, self.1); 
    }
}

// 为 Complex 实现 Print trait
impl Print for Complex {
    fn print(&self) {
        println!("Complex: ({}, {})", self.0, self.1);
    }
}

// 为Point实现Add trait
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

// 为Complex实现Add trait
impl Add for Complex {
    type Output = Self;  

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn main() {
    let point = Point(1, 2);
    let complex = Complex(3, 4);

    let objects: Vec<&dyn Print> = vec![&point, &complex];

    for obj in objects {
        obj.print();
    }
}
