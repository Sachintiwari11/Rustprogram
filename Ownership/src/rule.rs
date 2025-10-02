pub fn rule(){
    //rulei();
    //rules();
    //ques();
    //ques2();
}
//Integer is fixed data type and it is stored in stack
/*
fn rulei(){
    let int1 = 11;
    let int2 = int1;
    println!("value1={}",int1);
    println!("value2={}",int2);
}
*/

//String is Dynamic data type ad it is stored in heap
/*
fn rules(){
        let str1 = String::from("Sachin");
        let str2 = str1;//Transfer of ownership.str2 is the new owner of str1 
        // println!("str1={}",str1);    // If you access str1 then this is give error.
        println!("str2={}",str2);
    }
*/

//For Integer
/*
fn ques(){
    let a:u8 = 11;
    process_int(a);
    println!("Value of a in main() is {}",a);
}
fn process_int(a:u8){
    println!("Value of a in process_int() is {}",a);
}
*/

//For String
/*
fn ques2(){
    let a:String = String::from("Hello");
    process_str(a); //transfer of ownership.
    //println!("Value of x in que2 is {}",a);
}
fn process_str(item:String){
    println!("Value of x is process_str is {}",item);
}
*/