
trait Thing<T> {
    fn ignore(self) -> ();
}

impl<T> Thing<T> for () {
    fn ignore(self) -> () { self }
}

fn main () {
    let x = @() as @Thing<u8>;
    x.ignore()
}
