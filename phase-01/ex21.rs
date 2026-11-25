fn main(){
    let x :&str = "hello";
    {
        let x:i32 = 20;
        println!("{}",x);
    }
    println!("{}",x);
}
// scope 
/*
  scope is the one which determines the lifetime of variable or constant
  constants have global and local scope 
  variable can be block scope 
  In this code variable declared in the scope gets destroyed when the block ends
  */
