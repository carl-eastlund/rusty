#[deriving(Clone)]
struct Box<T>(@T);
fn op<T>( b : Box<T> ) -> Box<T> { b.clone() }
fn main (){}
