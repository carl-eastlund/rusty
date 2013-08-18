
#[deriving (Clone)]
enum List<E> { Null, Cons( E, @List<E> ) }

fn null<E:'static>() -> List<E> { Null }
fn cons<E:'static>( x : E, xs : List<E> ) -> List<E> { Cons( x, @xs ) }

fn foldl<E:Clone+'static,R>( f:&fn(E,R)->R, mut r:R, mut xs:List<E> ) -> R {
    loop {
        match xs {
            Null => { return r }
            Cons( e, ys ) => {
                r = f(e, r);
                xs = (*ys).clone();
                loop
            }
        }
    }
}

fn revapp<E:Clone+'static>( xs:List<E>, ys:List<E> ) -> List<E> {
    foldl( cons, ys, xs )
}

fn reverse<E:Clone+'static>( xs:List<E> ) -> List<E> {
    revapp( xs, null() )
}

fn foldr<E:Clone+'static,R>( f:&fn(E,R)->R, v:R, xs:List<E> ) -> R {
    foldl( f, v, reverse(xs) )
}

fn append<E:Clone+'static>( xs:List<E>, ys:List<E> ) -> List<E> {
    foldr( cons, ys, xs )
}

fn append_lists<E:Clone+'static>( xss:List<List<E>> ) -> List<E> {
    foldr( append, null(), xss )
}

fn main () {}
