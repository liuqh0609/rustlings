// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)


fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // shadowing可以进行类型覆盖，如果是mut的话，是需要类型一致的
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}
