fn main() {
    //流程控制
    /*
        1. if else 所有语言都有 见_funif 
        2. loop 
            loop 就是一个简单的无限循环，你可以在内部实现逻辑通过 break 关键字来控制循环何时结束
            _loop_demo();
        3. while
            每次执行循环体之前都要判断一次，直达条件不满足才退出
            _while_demo();
        4. for
            todo... 等符合类型搞完补充


     */
    
     _for_demo();
    println!("Hello, world!");
}

//若 condition 的值为 true，则执行 A 代码，否则执行 B 代码。
fn _funif(){
    let bool_val = true;
    if bool_val == true {
        // A...
    } else {
        // B...
    }
}
/*
if 语句块是表达式，这里我们使用 if 表达式的返回值来给 number 进行赋值：number 的值是 5
用 if 来赋值时，要保证每个分支返回的类型一样(事实上，这种说法不完全准确，见这里)，此处返回的 5 和 6 就是同一个类型，如果返回类型不一致就会报错
*/
fn _funif2() {
    let bool_val = true;
    let number = if bool_val {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

fn _loop_demo() {
    let mut counts = 0;
    let res = loop {
        counts += 2;
        if counts % 9 == 0 {
            break counts-10
        }
    };
    println!("count = {}, res = {}", counts, res);
}

fn _while_demo(){
    let mut nums = 0;
    while nums < 11 {
        println!("nums = {}", nums);
        nums += 3;
    }
}

fn _for_demo(){
    for item in (1..4){
        println!("item= {item}");
    }

    for item in 1..=4{
        println!("item= {item}");
    }
}
