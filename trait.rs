
use std::vec;

fn vector_map<A,B>( xs : &[A], f : &fn(&A)->B ) -> ~[B] {
    let mut ys = vec::with_capacity(xs.len());
    for x in xs.iter() {
        ys.push(f(x));
    }
    ys
}

fn vector_map_move<A,B>( xs : ~[A], f : &fn(A)->B ) -> ~[B] {
    let mut ys = vec::with_capacity(xs.len());
    for x in xs.move_iter() {
        ys.push(f(x));
    }
    ys
}

fn vector_collapse_sized<T>( xss : ~[~[T]], n : uint ) -> ~[T] {
    let mut combined = vec::with_capacity(n);
    for xs in xss.move_iter() {
        combined.push_all_move(xs);
    }
    combined
}

trait Discrim<K,V> {

    fn discrim( &self, ~[(K,V)] ) -> ~[~[V]];

}

fn discrim_sort<T:Clone,D:Discrim<T,T>>( d : D, xs : &[T] ) -> ~[T] {
    let pairs = do vector_map(xs) |x| { ((*x).clone(), (*x).clone()) };
    let groups = d.discrim(pairs);
    vector_collapse_sized( groups, xs.len() )
}

struct Pair_Discrim<A,B>(A,B);

impl<K1,K2,V,D1:Discrim<K1,(K2,V)>,D2:Discrim<K2,V>>
Discrim<(K1,K2),V> for Pair_Discrim<D1,D2> {

    fn discrim( &self, pairs : ~[((K1,K2),V)] ) -> ~[~[V]] {
        match self {
            &Pair_Discrim(ref first, ref second) => {
                let n = pairs.len();
                let nested = do vector_map_move(pairs) |((k1,k2),v)| {
                    (k1,(k2,v))
                };
                let pass1 = first.discrim(nested);
                let pass2 = do vector_map_move(pass1) |group| {
                    second.discrim(group)
                };
                vector_collapse_sized( pass2, n )
            }
        }
    }

}

struct Map_Discrim<'self,A,B,D>{ key : &'self fn(A)->B, discrim : D }

impl<'self,A,B,V,D:Discrim<B,V>> Discrim<A,V> for Map_Discrim<'self,A,B,D> {

    fn discrim( &self, pairs : ~[(A,V)] ) -> ~[~[V]] {
        let mapped = do vector_map_move(pairs) |(k,v)| { ((self.key)(k),v) };
        self.discrim.discrim(mapped)
    }

}

macro_rules! make_uint_split {
    ($name:ident,$big:ident,$lil:ident,$factor:expr) => {
        fn $name( x : $big ) -> ($lil,$lil) {
            let (a,b) = x.div_rem(&$factor);
            (a as $lil, b as $lil)
        }
    }
}

macro_rules! make_uint_discrim {
    ($name:ident,$big:ident,$split:ident,$help:ident) => {
        impl<T> Discrim<$big,T> for $name {
            fn discrim( &self, pairs : ~[($big,T)] ) -> ~[~[T]] {
                Map_Discrim{
                    key : $split,
                    discrim : Pair_Discrim($help,$help)
                }.discrim(pairs)
            }
        }
    }
}

struct U8_Discrim;
struct U16_Discrim;
struct U32_Discrim;
struct U64_Discrim;

make_uint_split!(split_u16,u16,u8,(1u16 << 8))
make_uint_split!(split_u32,u32,u16,(1u32 << 16))
make_uint_split!(split_u64,u64,u32,(1u64 << 32))

make_uint_discrim!(U16_Discrim,u16,split_u16,U8_Discrim)
make_uint_discrim!(U32_Discrim,u32,split_u32,U16_Discrim)
make_uint_discrim!(U64_Discrim,u64,split_u64,U32_Discrim)

impl<T> Discrim<u8,T> for U8_Discrim {

    fn discrim( &self, pairs : ~[(u8,T)] ) -> ~[~[T]] {
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
    let input = ~[257u64, 65536u64, 1u64, 0u64, 65537u64, 256u64];
    println( input.to_str() );
    let output = discrim_sort( U64_Discrim, input );
    println( output.to_str() );
}
