use std::io;
fn main() {
    let mut s1 = String::from("hello");
    update_data(&mut s1);
    

    println!("{}", s1);

}

fn update_data(s: &mut String){
    s.push_str(", nigger");
}
