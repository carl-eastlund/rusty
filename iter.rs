fn imap<A,B>( i : ~Iterator<A>, f : &fn(A)->B ) { i.map(f) }
fn main () {}

// struct Empty();

// impl<T> Iterator<T> for Empty {

//     fn next( &mut self ) -> Option<T> { None }

// }

// type Discrim<K,V> = &'static fn( ~Iterator<(K,V)> ) -> ~Iterator<~Iterator<V>>;

// fn dsort<T:Clone>( disc : Discrim<T,T>, xs : ~Iterator<T> ) -> ~Iterator<T> {
//     let pairs = do xs.map |x| { (x.clone(), x.clone()) };
//     let groups = disc(pairs);
//     do groups.fold(Empty()) |xs,ys| {
//         xs.chain(ys)
//     }
// }
