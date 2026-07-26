extern crate slint_build;
fn main() {
    slint_build::compile("src/ui/a-main.slint").expect("Slint build failed");
}