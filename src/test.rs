fn main() {
    let s1 = String::from("I am smile");
  let s = s1;
  println!("{}", s);//可以正常打印 I am smile
 // println!("{}", s1);//此处会有错误，因为s1现在不可用,所有权已转移，s1已经无效

  //let s=s1.clone(); 如果上面用的是这行代码 那么是可以打印s1的

    let value = String::from("Hello 不可变");
    let reference = &value;  // 创建一个不可变引用
    // value.push_str(", world!");  // 错误！不可变引用存在，不能修改原值
    println!("Reference: {}", reference);  // 输出Hello 不可变

    // 可变引用示例
    let mut value = String::from("Hello");
    let reference = &mut value;  // 创建一个可变引用
    reference.push_str(", world!");  // 正确！可变引用可以修改原值
    println!("Reference: {}", reference);  // 正常输出Hello world

    // 借用示例
    let mut value = String::from("Hello");
    let reference1 = &value;  // 不可变引用
    // let reference2 = &mut value;  // 错误！不可变引用存在，不能创建可变引用
    // println!("{} {}", reference1, reference2);  // 错误！不可变引用和可变引用不能同时存在
    println!("Reference: {}", reference1);  // 正确！打印不可变引用的值


//总结
 //所有权相关
 //1、 每个值都有一个所有者
//2、 当所有者离开作用域,该值会被drop
//3、 赋值会转移所有权,原变量不再是所有者

//不可变引用
//1、不可变引用允许借用值的引用，但不允许修改被引用的值
//2、可以同时存在多个不可变引用，但不能与可变引用同时存在
//3、不可变引用具有读取共享数据的能力，但不能修改数据

//可变引用
//1、可变引用允许借用值的引用，并允许修改被引用的值。
//2、在给定作用域中，只能有一个可变引用，以防止数据竞争。
//3、不能与其他引用（不可变引用或可变引用）同时存在（遵循借用规则）。


}  // 所有值的作用域结束，内存将自动释放


