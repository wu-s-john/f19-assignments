 mod part1;
 mod part2;


 fn first_word(s: &mut String) -> &str {
     let bytes = s.as_bytes();

     for (i, &item) in bytes.iter().enumerate() {
         if item == b' ' {
             return &s[0..i];
         }
     }

     &s[..]
 }

fn main() {

    let s1 = String::from("hello");


    println!("{}, world!", s1);
    let vec = [1, 2, 3];



}
