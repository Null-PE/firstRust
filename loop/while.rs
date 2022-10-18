fn loop_func(){
    let mut x = 1;
    while x < 1000
    {
        x *= 2;
        if x == 64 {
            continue;
        }
        println!{"x = {}", x};
    }
}

fn loop_true()
{
    let mut y = 1;
    loop{
        y *= 2;
        println!{"y is {}", y};
        if y == 1<<10 {break;}
    }
}

fn main(){
    //loop_func();
    loop_true();
}