pub fn own(){
    //let s1:String = String :: from("helloooo");    //s1 Owner
    /*let s2:usize = calculate_length(s1);   //Ownership transfer
    println!("The length is {}",s2);*/
    //For Tupple
    /*let (s2,len) = calculate_length_tup(s1);  //Making tupple
    println!("The length of {}, length {}",s2,len);*/

    //For Clone
    /*let len = calculate_length_clone(s1.clone());
    println!("The length is {}",len);
    */

    //For Borrow
    /*let len:usize = calculate_borrow(&s1);    //It can read the content of s1 but it can't be modify it.
    println!("The length is {}",len);
    */

    //Changes in borrow data
    let mut bor:String = String ::from("Hello");
    append_string(&mut bor);
    println!("The new string is {}",bor);
}
// fn calculate_length(s:String)->usize{
//     return s.len(); //return 5
// }

//Using Tupple
/*fn calculate_length_tup(s:String)->(String,usize){
    let length:usize = s.len();
    return (s,length);  //return ownership transfer to s2
}*/

//With Clone
/*fn calculate_length_clone(s:String)->usize{
    let length:usize = s.len(); 
    return length;
}*/

//For Borrow
/*fn calculate_borrow(s2:&String)->usize{
    return s2.len();
}*/

//For Changes in borrow data
fn append_string(bor:&mut String){
    bor.push_str("World!");
}