
enum List<E> { Empty, Cons( @E, @List<E> ) }

impl<E> Iterator<@E> for List<E> {

    fn next(&mut self) -> Option<@E> {
        match *self {
            Empty => { None }
            Cons( x, xs ) => { *self = *xs; Some( x ) }
        }
    }

}

impl<E> List<E> {

    fn foldl<R>( &self, mut r : R, f : &fn(@E,R)->R ) -> R {
        let mut xs = *self;
        for x in xs { r = f(x,r) };
        return r
    }

}

impl<E:'static> List<E> {

    fn revapp( &self, xs : List<E> ) -> List<E> {
        do self.foldl(xs) |x,ys| {
            Cons(x,@ys)
        }
    }

}

fn main () {}
