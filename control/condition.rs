fn if_statement()
{
    let temp = 5;

    if temp > 30{
        println!("over 30.");
    }
    else if temp > 10
    {
        println!("over 10");
    }
    else {
        println!("less than 30.");
    }


    let day = if temp > 20 {"over 20"} else {"less than 20"};
    println!("{}", day);


    println!{"is it {}", 
        if temp > 20 {"over 20"} else if temp > 10 {"over 10"} else {"less than 10"};
    }
}

fn main(){
    if_statement();
}
