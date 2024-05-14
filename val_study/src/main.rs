fn main() {
    /*
        cargo 相关命令
            可以使用 cargo new 创建项目。
            可以使用 cargo build 构建项目。
            可以使用 cargo run 一步构建并运行项目。
            可以使用 cargo check 在不生成二进制文件的情况下构建项目来检查错误。
            有别于将构建结果放在与源码相同的目录，Cargo 会将其放到 target/debug 目录。
        使用 Cargo 的一个额外的优点是，不管你使用什么操作系统，其命令都是一样的。
    */

    // 代码示例1:不变变量
    /*
        let hello_val = "hello world!"; //把 “hello_world”赋值给hello_val的过程叫变量绑定
        println!("打印字符串：{hello_val}");
        hello_val = "重新给变量赋值";
        println!("打印字符串：{hello_val}"); // err : cannot assign twice to immutable variable `hello_val`

        Rust 的变量在默认情况下是不可变的，一旦为它绑定值，就不能再修改，重新赋值会报错
    */
    
    //代码示例2：可变变量
    /*
        let mut munber_eg = "abc"; 
        println!("打印字符串：{munber_eg}");
        munber_eg = "bbbb";
        println!("打印字符串：{munber_eg}");

        Rust中，通过 mut 关键字让变量变为可变的，让设计更灵活。
        否让变量可变的最终决定权仍然在你，取决于在某个特定情况下，你是否认为变量可变会让代码更加清晰明了
    */
   
    //代码例子4:变量遮蔽(shadowing)相关
    /*       
        let x = 10;
        let x: u32 = x + 10;
        println!("这个值为 {x}");

        //相同变量可以是不同类型
        let space = "    "; 
    
        let space: usize = space.len();
        println!("这个值为 {space}");

        // let mut space_str = "     "; //如果let后面加 mut就会报错
        // space_str = space_str.len();//报错类型比匹配，定义的是字符串，修改的值是整形，不能改变变量的类型
        // println!("这个值为 {space_str}");
    */

    //代码示例5：常量
    /*
        //常量也是绑定到一个常量名且不允许更改的值,常量可以在任意作用域内声明，包括全局作用域，在声明的作用域内，常量在程序运行的整个过程中都有效。
        对于需要在多处代码共享一个不可变的值时非常有用,在实际使用中，最好将程序中用到的硬编码值都声明为常量，对于代码后续的维护有莫大的帮助.
            1.常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
            2.常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。
        const MAX_POINTS: u32 = 100_000; //再数字中加 _ 便于阅读
        println!("打印常量：{}",MAX_POINTS);
     */

    //代码示例6：使用下划线开头忽略未使用的变量
    /* 
        //你希望告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头
        let _max = 100;
        let mut _val = "abc";
    */

    //代码示例7：变量解构和解构式解析  -后面再补。。。

    println!("hello world!");

}

    
