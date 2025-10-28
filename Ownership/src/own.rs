pub fn own(){
    let s1:String = String :: from("hello");    //s1 Owner
    /*let s2:usize = calculate_length(s1);   //Ownership transfer
    println!("The length is {}",s2);*/
    //For Tupple
    /*let (s2,len) = calculate_length_tup(s1);  //Making tupple
    println!("The length of {}, length {}",s2,len);*/

    //For Clone
    let len = calculate_length_clone(s1.clone());
    println!("The length is {}",len);
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
fn calculate_length_clone(s:String)->usize{
    let length:usize = s.len(); 
    return length;
}