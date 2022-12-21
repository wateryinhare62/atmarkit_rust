macro_rules! empty {
    () => {}
}
  
fn main() {
    empty!();
    empty![];
    empty!{};
}
