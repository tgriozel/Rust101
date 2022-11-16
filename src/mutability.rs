pub fn main() {
    println!("Case 1:");
    {
        let mut writer = vec![1,2,3];
        let reader = &writer;
    
        println!("len: {}", writer.len()); // no error, inactive writer
        //writer.push(4); -> error: cannot borrow `writer` as mutable because it is also borrowed as immutable
        println!("len: {}", reader.len());
    }
    println!("Case 2:");
    {
        let mut writer = vec![1,2,3];
        writer.push(4); 

        let reader = &writer;
        println!("len: {}", reader.len()); // no error, reader is not active because it was borrowed _after_ last writer mutation
    }
    println!("Case 3:");
    {
        let mut writer = vec![1,2,3];
        writer.push(4);
    
        for number in writer.iter() {
            println!("number: {}", number); // no error, implicit borrow occurs after writer mutation
            //writer.push(number + 2); -> error: cannot borrow `writer` as mutable because it is also borrowed as immutable
        }
    }
}
