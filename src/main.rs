mod chapter4;
mod chapter5;

fn main() {
    let x = String::from("Hello world");
    println!("Hello, BEGIN CHAPTER 4!");

    chapter4::hello_references();
    println!("{}", chapter4::first_word(&x));

    chapter4::assert_slice_int();
    
    chapter5::calculate();
    chapter5::check_holding();
}
