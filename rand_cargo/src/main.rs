use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    let rand_num: u32 = rand::thread_rng().gen_range(1..=101);
    println!("随机数是 {}", rand_num);
    loop {
         // let foo = 1; //let 声明的变量在默认情况下是不可变的，也不能修改值
         println!("猜数！");
         println!("请输入一个数字");
        let mut num_str = String :: new();
        io::stdin().read_line(&mut num_str).expect("无法读取行");
        // let  num_str:u32 = num_str.trim().parse().expect("please type a number");   
        let  num_str:u32 = match num_str.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {num_str}");
        match num_str.cmp(&rand_num){
            Ordering::Less => println!("太小！"),
            Ordering::Greater => println!("太大！"),
            Ordering::Equal => {
                println!("你赢了！");
                break;
            }
        }
    }

}
