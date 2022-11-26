use getter_derive_example::Point;

fn main() {
    let some_var: Point = Default::default();
    let haha = some_var.get_x();
    println!("{}", haha);
}
