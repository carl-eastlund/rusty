
trait Thing<T> {
    fn ignore(self) -> ();
}

impl<T> Thing<T> for () {
    fn ignore(self) -> () { self }
}

fn main () {
    let x : @Thing<u8> = ();
    x.ignore()
}
