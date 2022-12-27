fn xmas_tree(n: i32) {
    for i in 0..n {
        for _ in (i..n-1).rev() {
            print!(" ");
        }
        for j in (0..i+1).rev() {
            if j == i {print!("*")} else {print!("**")};
        }
        println!();
    }
    for i in 0..(n/5)+1 {
        for _ in 0..n-1 {
            print!(" ");
        }
        if i == (n/5) {println!("v")} else {println!("|")};
    }
}  

fn main() {
    xmas_tree(5);
}