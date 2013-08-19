
#[deriving (Clone,ToStr)]
enum List<E> { Empty, Cons( @E, @List<E> ) }

fn is_empty<E>( xs:List<E> ) -> bool { match xs { Empty => true, _ => false } }
fn empty<E>() -> List<E> { Empty }
fn cons<E>( x:@E, xs:List<E> ) -> List<E> { Cons( x, @xs ) }

fn foldl<E,R>( f:&fn(@E,R)->R, mut r:R, mut xs:List<E> ) -> R {
    loop {
        match xs {
            Empty => { return r }
            Cons( e, ys ) => {
                r = f(e, r);
                xs = (*ys).clone();
                loop
            }
        }
    }
}

fn revapp<E>( xs:List<E>, ys:List<E> ) -> List<E> {
    foldl( cons, ys, xs )
}

fn reverse<E>( xs:List<E> ) -> List<E> {
    revapp( xs, empty() )
}

fn foldr<E,R>( f:&fn(@E,R)->R, v:R, xs:List<E> ) -> R {
    foldl( f, v, reverse(xs) )
}

fn append<E>( xs:List<E>, ys:List<E> ) -> List<E> {
    foldr( cons, ys, xs )
}

fn append_lists<E>( xss:List<List<E>> ) -> List<E> {
    foldr( append, empty(), xss )
}

fn map<A,B>( f:&fn(@A)->@B, xs:List<A> ) -> List<B> {
    foldr( |x,ys| Cons(f(x),@ys), empty(), xs )
}

type Discrim<K,V> = &'static fn(List<(K,V)>) -> List<List<V>>;

fn dsort<E>( d:Discrim<E,E>, xs:List<E> ) -> List<E> {
    append_lists( d( map( |x| ( x.clone(), x.clone() ), xs ) ) )
}

fn main () {
    let fwd : List<u8> = cons(1,cons(2,empty()));
    println(fwd.to_str());
    let rev : List<u8> = reverse(fwd);
    println(rev.to_str());
    let add : List<u8> = map(|x|x-1,rev);
    println(add.to_str());
}
