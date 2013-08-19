
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
    let mut sorted = vec::with_capacity(n);
    for vs in groups.iter() {
        sorted.push_all(*vs);
    }
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

macro_rules! make_uint_discrim {
    ($name:ident, $big:ident, $help:ident, $lil:ident, $factor:expr) => {
        fn $name<T>( pairs : ~[($big,T)] ) -> ~[~[T]] {
            let n = pairs.len();
            let mut split = vec::with_capacity(n);
            let x = $factor;
            for ( k , v ) in pairs.move_iter() {
                split.push( ( (k / x) as $lil, ( (k % x) as $lil, v ) ) );
            }
            let groups = $help(split);
            let mut result = ~[];
            for lil_pairs in groups.move_iter() {
                result.push_all_move($help(lil_pairs));
            }
            result
        }
    }
}

make_uint_discrim!(u16_discrim,u16,u8_discrim,u8,256)

fn i16_discrim<T>( pairs : ~[(i16,T)] ) -> ~[~[T]] {
    let mut u16_pairs = vec::with_capacity(pairs.len());
    for (k,v) in pairs.move_iter() {
        u16_pairs.push( ( (k+32768) as u16, v ) );
    }
    u16_discrim(u16_pairs)
}

fn main () {
    let numbers =
        ~[-1025i16, 1i16, 256i16, 3i16, -1024i16, 2i16, 512i16, -1026i16];
    println( numbers.to_str() );
    let sorted = dsort( i16_discrim, numbers );
    println( sorted.to_str() );
}
