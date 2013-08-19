
use std::vec;

type Discrim<K,V> = &'static fn( ~[(K,V)] ) -> ~[~[V]];

fn dsort<T:Clone>( disc : Discrim<T,T>, xs : ~[T] ) -> ~[T] {
    let n = xs.len();
    let pairs = do vec::build_sized(n) |push| {
        for x in xs.iter() {
            push((x.clone(),x.clone()));
        }
    };
    let groups = disc(pairs);
    let sorted = do vec::build_sized(n) |push| {
        for vs in groups.iter() {
            for v in vs.iter() {
                push(v.clone());
            }
        }
    };
    sorted
}

fn u8_discrim<T>( pairs : ~[(u8,T)] ) -> ~[~[T]] {
    let mut vec = do vec::build_sized(256) |push| {
        for _ in range(0,256) {
            push(~[]);
        }
    };
    for ( k, v ) in pairs.move_iter() {
        vec[k].push(v);
    }
    do vec.retain |vs| { vs.len() > 0 };
    vec
}

fn main () {
    let two_pi = [6u8,2u8,8u8,3u8,1u8];
    println(two_pi.to_str());
}
