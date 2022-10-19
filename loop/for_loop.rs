fn for_loop(){
    for x in 1..11
    {
        println!("x = {}", x);
    }

    for (index, y) in (30..41).enumerate()
    {
        println!("{}, {}", index, y);
    }
}

fn main()
{
    for_loop();
}