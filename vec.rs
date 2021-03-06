
use std::vec;
use std::rand;

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
            let fact = $factor;
            for ( k , v ) in pairs.move_iter() {
                split.push( ( (k / fact) as $lil, ( (k % fact) as $lil, v ) ) );
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

make_uint_discrim!(u16_discrim,u16,u8_discrim,u8,(1u16 << 8))
make_uint_discrim!(u32_discrim,u32,u16_discrim,u16,(1u32 << 16))
make_uint_discrim!(u64_discrim,u64,u32_discrim,u32,(1u64 << 32))

macro_rules! make_int_discrim {
    ($name:ident, $i:ident, $help:ident, $u:ident, $offset:expr) => {
        fn $name<T>( pairs : ~[($i,T)] ) -> ~[~[T]] {
            let mut u_pairs = vec::with_capacity(pairs.len());
            for (k,v) in pairs.move_iter() {
                u_pairs.push( ( (k+$offset) as $u, v ) );
            }
            $help(u_pairs)
        }
    }
}

make_int_discrim!(i8_discrim,i8,u8_discrim,u8,(1i8 << 7))
make_int_discrim!(i16_discrim,i16,u16_discrim,u16,(1i16 << 15))
make_int_discrim!(i32_discrim,i32,u32_discrim,u32,(1i32 << 31))
make_int_discrim!(i64_discrim,i64,u64_discrim,u64,(1i64 << 63))

macro_rules! make_cast_discrim {
    ($name:ident, $orig:ident, $help:ident, $cast:ident) => {
        fn $name<T>( pairs : ~[($orig,T)] ) -> ~[~[T]] {
            let mut cast_pairs = vec::with_capacity(pairs.len());
            for (k,v) in pairs.move_iter() {
                cast_pairs.push( ( k as $cast, v ) );
            }
            $help(cast_pairs)
        }
    }
}

make_cast_discrim!(int_discrim,int,i64_discrim,i64)
make_cast_discrim!(uint_discrim,uint,u64_discrim,u64)

fn main () {
    let mut numbers = ~[];
    for _ in range(0,rand::random::<u8>()) {
        numbers.push( rand::random::<int>() )
    }
    println( numbers.to_str() );
    let sorted = dsort( int_discrim, numbers );
    println( sorted.to_str() );
}
