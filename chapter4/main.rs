                      // s is not valid here, it’s not yet declared
fn main() {          
  // string literals are immutable            
  let s1 = "hello";    // s1 is valid from this point forward
                      // do stuff with s1

  let mut s2 = String::from("hello"); // creating String(mutable) from string literal

  s2.push_str(", world!"); // push_str() appends a literal to a String

  println!("{s2}"); // This will print `hello, world!`

}                     // this scope is now over, and s1 is no longer valid
