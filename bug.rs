
#[deriving (Clone,ToStr)]
enum List<E> { Empty, Cons( @E, @List<E> ) }

fn is_empty<E>( xs:List<E> ) -> bool { match xs { Empty => true, _ => false } }
fn empty<E>() -> List<E> { Empty }
fn cons<E:'static>( x:E, xs:List<E> ) -> List<E> { Cons( @x, @xs ) }

fn foldl<E:Clone+'static,R>( f:&fn(E,R)->R, mut r:R, mut xs:List<E> ) -> R {
    loop {
        match xs {
            Empty => { return r }
            Cons( e, ys ) => {
                r = f((*e).clone(), r);
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
    revapp( xs, empty() )
}

fn foldr<E:Clone+'static,R>( f:&fn(E,R)->R, v:R, xs:List<E> ) -> R {
    foldl( f, v, reverse(xs) )
}

fn append<E:Clone+'static>( xs:List<E>, ys:List<E> ) -> List<E> {
    foldr( cons, ys, xs )
}

fn append_lists<E:Clone+'static>( xss:List<List<E>> ) -> List<E> {
    foldr( append, empty(), xss )
}

fn map<A:Clone+'static,B:'static>( f:&fn(A)->B, xs:List<A> ) -> List<B> {
    foldr( |x,ys| cons(f(x),ys), empty(), xs )
}

type Discrim<'self,K,V> = &'self fn(List<(K,V)>) -> List<List<V>>;

fn dsort<'self,E:Clone+'static>( d:Discrim<'self,E,E>, xs:List<E> ) -> List<E> {
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
