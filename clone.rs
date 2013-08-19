
fn main (){}

#[deriving(Clone)]
struct Box<T>{unbox:@T}

fn op<T>( b : Box<T> ) -> Box<T> {

    /* This version works: */
    // Box{ unbox : b.unbox.clone() }

    /* This version doesn't: */
    b.clone()

}
