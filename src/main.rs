fn main(){
    let x = 5;
    let mut y = 10;

    fn add(a: i32, b:i32) -> i32 {
        a + b
    }

    if x < y{
        println!(" x is less than y:");
    }

    for i in 0..5{
        println!("値: {}", i);
    }
}

