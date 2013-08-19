
#[deriving (Clone,ToStr)]
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

fn map<A:Clone+'static,B:'static>( f:&fn(A)->B, xs:List<A> ) -> List<B> {
    foldr( |x,ys| cons(f(x),ys), null(), xs )
}

trait Discrim<K,V> {
    fn discrim(&self,List<(K,V)>) -> List<List<V>>;
}

impl<V:Clone+'static> Discrim<u8,V> for () {
    fn discrim( &self, mut xs:List<(u8,V)> ) -> List<List<V>> {
        let mut vec = [Null, ..256];
        loop {
            match xs {
                Null => {}
                Cons( (k,v), ys ) => {
                    vec[k] = cons(v,vec[k]);
                    xs = *ys;
                    loop
                }
            }
        }
        let mut vss = Null;
        for i in vec.rev_iter() {
            match *i {
                Null => {}
                vs => { vss = cons(reverse(vs), vss) }
            }
        }
        vss
    }
}

fn dsort<V:Clone+'static>( d:@Discrim<V,V>, xs:List<V> ) -> List<V> {
    append_lists(d.discrim(map(|x|(x.clone(),x.clone()),xs)))
}

fn main () {
    let fwd : List<u8> = cons(1,cons(2,null()));
    println(fwd.to_str());
    let rev : List<u8> = reverse(fwd);
    println(rev.to_str());
    let add : List<u8> = map(|x|x-1,rev);
    println(add.to_str());
    let ord : List<u8> = dsort((),add);
}
