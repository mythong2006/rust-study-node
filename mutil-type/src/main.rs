
fn main() {
    //复合类型
    /*
    复合类型：
        字符串与切片
            切片: 引用集合中部分连续的元素序列，而不是引用整个集合,对于字符串而言，切片就是对 String 类型中某一部分的引用
            字符串: 是由字符组成的连续集合.
                Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)
                str 类型是硬编码进可执行文件，也无法被修改，但是 String 则是一个可增长、可改变且具有所有权的 UTF-8 编码字符串，
                当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码

        元组
        枚举
        数组
     */

    //示例1：字符串传值报错  
    /*
        编译器提示 greet 函数需要一个 String 类型的字符串，却传入了一个 &str 类型的字符串
    
        let my_name = "Pascal";
        greet(my_name);
        
        fn greet(name: String) {
            println!("Hello, {}!", name);
        }
    */

    // 示例2：切片基本操作
    /* 
        let my_name = "hello world"; //字符串字面量 my_name的类型是  &str
        //创建切片的语法，使用方括号包括的一个序列：[开始索引..终止索引]
        let hello = &my_name[0..5]; //range序列 这种还可以简写为 let hello = &my_name[..5];
    
        let str_len = my_name.len(); //获得字符串的总长度 
        let world = &my_name[6..11];
        let world2 = &my_name[6..str_len];
        let slice1 = &my_name[..]; //获取全部字符串
        println!("Hello, world!,hello = {}, world = {}, world2 = {} slice = {}",hello, world,world2,slice1);
    */
     //示例3：切片的使用-获得首字母
    /*
        let my_name = String::from("hello world"); //字符串类型
        let first_word: &str = great(&my_name); //传递不可变引用类型
        //my_name.clear();
        println!("first_world = {first_word} {}", my_name);
        fn great(str: &String) -> &str {
            &str[..1]
        }
     */
    //示例：String 与 &str 的转换
    /* 
        //&str 类型生成 String 类型
        let _s1 = String::from("hello,world");
        let _s2 = "hello,world".to_string();
        //String类型转换为 &str 类型
        let s = String::from("hello,world");
        say_hello(&s);
        say_hello(&s[..]);
        say_hello(s.as_str());
        
        fn say_hello(str: &str){
            println!("{}",str);
        }
          
    */
    
    //字符串操作
    //追加:原有的字符串上追加，并不会返回新的字符串,字符串变量必须由 mut 关键字修饰
    let mut str1 = String::from("hello ");
    
    str1.push_str("string");//追加字符串字面量
    str1.push('!');//追加字符 char
    println!("追加字符串 push_str -> {}", str1);
    
    //插入 字符串变量必须由 mut 关键字修饰
    let mut str2 = String::from("hello ");
    str2.insert(5, ','); //插入单个字符 char
    str2.insert_str(7, "string");//插入字符串字面量
    println!("插入字符串 insert -> {}", str2);
    
    //替换
    //replace 该方法可适用于 String 和 &str 类型。
    //replace() 方法接收两个参数，第一个参数是要被替换的字符串，第二个参数是新的字符串。
    //该方法会替换所有匹配到的字符串。该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = String::from("I like rust. Learning rust is my favorite!");   
    let new_str = string_replace.replace("rust", "RUST");
    println!("替换字符串1 replace -> {}", new_str);
    // dbg!(new_str);

    //replacen 该方法可适用于 String 和 &str 类型。
    //replacen() 方法接收三个参数，前两个参数与 replace() 方法一样，第三个参数则表示替换的个数。
    //该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = String::from("I like rust. Learning rust is my favorite!");   
    let new_str = string_replace.replacen("rust", "RUST", 1);
    println!("替换字符串2 replacen -> {}", new_str);
    
    //删除
    // 1.pop 删除并返回字符串的最后一个字符,直接操作原来的字符串。但是存在返回值，其返回值是一个 Option 类型，如果字符串为空，则返回 None
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    //2.remove —— 删除并返回字符串中指定位置的字符,直接操作原来的字符串
    //存在返回值，其返回值是删除位置的字符串，只接收一个参数，表示该字符起始索引位置。
    //remove() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误
    let mut string_remove = String::from("测试remove方法");
    string_remove.remove(3); //删除 ‘试‘
    dbg!(string_remove);
    
    //3、truncate —— 删除字符串中从指定位置开始到结尾的全部字符 直接操作原来的字符串。无返回值
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
    
    //4.clear —— 清空字符串,直接操作原来的字符串。
    // 调用后，删除字符串中的所有字符，相当于 truncate() 方法参数为 0 的时候
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    //连接
    //1.使用 + 或者 += 连接字符串
    //使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    let res = string_append + &string_rust;
    let mut res = res + "!"; // `result + "!"` 中的 `result` 是不可变的
    res += "!!!";
    println!("连接字符串 + -> {}", res);

    //2、使用 format! 连接字符串,适用于 String 和 &str 
    let s1: &str = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);

    /*
        栈
            -栈按照顺序存储值并以相反顺序取出值，这也被称作后进先出。
            -增加数据叫做进栈，移出数据则叫做出栈。
            -栈中的所有数据都必须占用已知且固定大小的内存空间，假设数据大小是未知的，那么在取出数据时，你将无法取到你想要的数据。

        堆
            -与栈不同，对于大小未知或者可能变化的数据，我们需要将它存储在堆上。
            -当向堆上放入数据时，需要请求一定大小的内存空间。操作系统在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的指针, 
            该过程被称为在堆上分配内存，有时简称为 “分配”(allocating)。
        
        性能区别
            -在栈上分配内存比在堆上分配内存要快，因为入栈时操作系统无需进行函数调用（或更慢的系统调用）来分配新的空间，只需要将新数据放入栈顶即可。
            -相比之下，在堆上分配内存则需要更多的工作，这是因为操作系统必须首先找到一块足够存放数据的内存空间，接着做一些记录为下一次分配做准备，
            -如果当前进程分配的内存页不足时，还需要进行系统调用来申请更多内存。 
            -因此，处理器在栈上分配数据会比在堆上分配数据更加高效。

        所有权与堆栈
            当你的代码调用一个函数时，传递给函数的参数（包括可能指向堆上数据的指针和函数的局部变量）依次被压入栈中，当函数调用结束时，这些值将被从栈中按照相反的顺序依次移除。
            因为堆上的数据缺乏组织，因此跟踪这些数据何时分配和释放是非常重要的，否则堆上的数据将产生内存泄漏 —— 这些数据将永远无法被回收。

        所有权原则谨记以下规则：
            Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
            一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
            当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

        借用规则总结
            总的来说，借用规则如下：
                同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
                引用必须总是有效的           
    */
 
    //示例4：传递可变类型
    /*
        //可变引用同时只能存在一个
        let mut my_name = String::from("hello");
        let s1 = &mut my_name;
        //let s2: &mut String = &mut my_name; //同一作用域，特定数据只能有一个可变引用：
        push_strings(s1);
        fn push_strings(str: &mut String){
            str.push_str(", world");
        }
        println!("my_name = {my_name}");
        my_name.push_str("!");
        println!("my_name = {my_name}");
     */
   
    //示例5:深浅拷贝- 克隆（深拷贝）
    /*
    深拷贝
        -Rust 永远也不会自动创建数据的 “深拷贝”,任何自动的复制都不是深拷贝
        -用 clone函数可以深度复制 String 中堆上的数据
    浅拷贝
        -浅拷贝只发生在栈上，因此性能很高 
    copy注意事项：
        Rust 有一个叫做 Copy 的特征，可以用在类似整型这样在栈中存储的类型。如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用，也就是赋值的过程即是拷贝的过程。    
        任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。如下是一些 Copy 的类型：
            -所有整数类型，比如 u32
            -布尔类型，bool，它的值是 true 和 false
            -所有浮点数类型，比如 f64
            -字符类型，char
            -元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
            -可变引用 &mut T 是不可以 Copy的
    */

    //深拷贝
    /*
        let mut my_name = String::from("hello");
        let mut s1 = my_name.clone();

        s1.push_str("demo");
        my_name.push_str("test");
        println!("my_name = {my_name} s1 = {s1}");
        
        //浅拷贝
        //像整型这样的基本类型在编译时是已知大小的，会被存储在栈上，所以拷贝其实际的值是快速的。
        let x = 5;
        let y = x; //没有深浅拷贝的区别，因此调用 clone 并不会与通常的浅拷贝有什么不同，可以理解成在栈上做了深拷贝。

        println!("x = {}, y = {}", x, y);
     */
}
