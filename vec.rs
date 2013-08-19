
use std::vec;

type Discrim<'self,K,V> = &'self fn( ~[(K,V)] ) -> ~[~[V]];

fn dsort<'self,T>( disc : Discrim<'self,T,T>, xs : ~[T] ) -> ~[T] {
    let n = xs.len();
    let pairs = do vec::build_sized(n) |push| {
        for x in xs.iter() {
            push((x,x));
        }
    };
    let groups = disc(pairs);
    let sorted = do vec::build_sized(n) |push| {
        for vs in groups.iter() {
            for v in vs.iter() {
                push(v);
            }
        }
    };
    sorted
}

fn main () {
    let two_pi = [6u8,2u8,8u8,3u8,1u8];
    println(two_pi.to_str());
}
