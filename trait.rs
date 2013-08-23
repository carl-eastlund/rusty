
use std::vec;

fn vector_map<A,B>( xs : &[A], f : &fn(&A)->B ) -> ~[B] {
    xs.iter().map(f).to_owned_vec()
}

fn vector_collapse_sized<T>( xss : ~[~[T]], n : uint ) -> ~[T] {
    let mut combined = vec::with_capacity(n);
    for xs in xss.move_iter() {
        combined.push_all_move(xs);
    }
    combined
}

trait Discrim<K,V> {

    fn discrim( self, ~[(K,V)] ) -> ~[~[V]];

}

fn discrim_sort<T:Clone,D:Discrim<T,T>>( d : D, xs : &[T] ) -> ~[T] {
    let pairs = do vector_map(xs) |x| { ((*x).clone(), (*x).clone()) };
    let groups = d.discrim(pairs);
    vector_collapse_sized( groups, xs.len() )
}

impl<T> Discrim<u8,T> for () {

    fn discrim( self, pairs : ~[(u8,T)] ) -> ~[~[T]] {
        let mut buckets = do vec::build_sized(256) |push| {
            for _ in range(0,256) {
                push(~[]);
            }
        };
        for (k,v) in pairs.move_iter() {
            buckets[k].push(v);
        }
        do buckets.retain |bucket| { bucket.len() > 0 };
        buckets
    }

}

fn main () {
    let input = ~[3u8, 1u8, 4u8, 1u8, 5u8];
    println( input.to_str() );
    let output = discrim_sort( (), input );
    println( output.to_str() );
}
