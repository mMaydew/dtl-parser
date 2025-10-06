fn main() {
    let single = plus(10, vec![10]);
    read_plus_return(single);

    let multiple = plus(10, vec![1, 2, 3]);
    read_plus_return(multiple);
}

fn read_plus_return(event: PlusReturn) {
    match event {
        PlusReturn::Num(x) => println!("Single: {}", x),
        PlusReturn::NumArray(x) => println!("Multi: {:?}", x),
    }
}

#[derive(Debug)]
enum PlusReturn {
    Num(i32),
    NumArray(Vec<i32>),
}

fn plus(increment: i32, values: Vec<i32>) -> PlusReturn {
    println!("\ninc: {}\nval: {:?}", increment, values);
    let mut result = vec![];
    for v in values {
        result.push(increment + v);
    }

    if result.len() == 1 {
        PlusReturn::Num(result[0])
    } else {
        PlusReturn::NumArray(result)
    }
}
