use auid::Uid;

fn main() {
    let int = Uid::new();
    println!("{}", *int);
    println!("{}", int.to_hex());
    println!("{}", int);
}
