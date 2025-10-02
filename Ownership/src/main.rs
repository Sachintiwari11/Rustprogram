//mod rule;
mod test;

// understanding of scoped variable
//const OTHER:u8 = 11;    // Global constant variable it thought accessable in program 
fn main() {
    //rule::rule(); // Calling another file
    test::test();
    // let out_var = 10;   // Accessable in this function
    // {
    //     let in_var = 5;
    //     println!("{}",in_var);
    //     println!("{}",out_var);

    // }// end of in_var scope
    // println!("{}",out_var);
    // printval();
}

// fn printval(){
//     println!("{}",OTHER);
// }

