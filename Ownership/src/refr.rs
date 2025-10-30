/*pub fn refr(){
    //Borrowing
    let s1 : String = String ::from("hello");
    let len : usize = cal_len(&s1); // It is read content of s1 but it cannot modify.
    println!("The length of {} is {}",s1,len);
}

fn cal_len(s:&String)->usize{
    return s.len();
}
*/

//Mutable References
/*pub fn mutref(){        //You need to mut in order to have a mutable refrence.
    let mut s1 : String = String::from("Hello");
    append_world(&mut s1);
    println!("This is append s1 {}",s1);
}

fn append_world(s:&mut String){
    s.push_str(" Code");
}
*/

//Mutable Refrence Restriction
pub fn mutresref(){
    let mut s = String :: from("Hello");
    let w1 = &mut s;
    w1.push_str(" World");
    println!("{}",w1);

    let w2 = &mut s;
    println!("This is w2 {}",w2);
}