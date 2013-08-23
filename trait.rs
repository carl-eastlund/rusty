
use std::vec;
use std::i8;
use std::i16;
use std::i32;
use std::i64;

fn vec_map<A,B>( xs : &[A], f : &fn(&A)->B ) -> ~[B] {
    let mut ys = vec::with_capacity(xs.len());
    for x in xs.iter() {
        ys.push(f(x));
    }
    ys
}

fn vec_map_move<A,B>( xs : ~[A], f : &fn(A)->B ) -> ~[B] {
    let mut ys = vec::with_capacity(xs.len());
    for x in xs.move_iter() {
        ys.push(f(x));
    }
    ys
}

fn vec_collapse_move<T>( xss : ~[~[T]] ) -> ~[T] {
    let mut combined = ~[];
    for xs in xss.move_iter() {
        combined.push_all_move(xs);
    }
    combined
}

fn vec_partition_move<T>( xs : ~[T], f : &fn(&T)->bool ) -> (~[T],~[T]) {
    let mut pass = ~[];
    let mut fail = ~[];
    for x in xs.move_iter() {
        if f(&x) {
            pass.push(x);
        } else {
            fail.push(x);
        }
    }
    (pass,fail)
}

trait Discrim<K,V> {

    fn discrim( &self, ~[(K,V)] ) -> ~[~[V]];

}

fn discrim_sort<T:Clone,D:Discrim<T,T>>( d : D, xs : &[T] ) -> ~[T] {
    let pairs = do vec_map(xs) |x| { ((*x).clone(), (*x).clone()) };
    let groups = d.discrim(pairs);
    vec_collapse_move( groups )
}

struct Pair_Discrim<A,B>(A,B);

impl<K1,K2,V,D1:Discrim<K1,(K2,V)>,D2:Discrim<K2,V>>
Discrim<(K1,K2),V> for Pair_Discrim<D1,D2> {

    fn discrim( &self, pairs : ~[((K1,K2),V)] ) -> ~[~[V]] {
        match self {
            &Pair_Discrim(ref first, ref second) => {
                let nested = do vec_map_move(pairs) |((k1,k2),v)| {
                    (k1,(k2,v))
                };
                let pass1 = first.discrim(nested);
                let pass2 = do vec_map_move(pass1) |group| {
                    second.discrim(group)
                };
                vec_collapse_move( pass2 )
            }
        }
    }

}

struct Map_Discrim<'self,A,B,D>{ key : &'self fn(A)->B, discrim : D }

impl<'self,A,B,V,D:Discrim<B,V>> Discrim<A,V> for Map_Discrim<'self,A,B,D> {

    fn discrim( &self, pairs : ~[(A,V)] ) -> ~[~[V]] {
        let mapped = do vec_map_move(pairs) |(k,v)| { ((self.key)(k),v) };
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
    ($name:ident,$big:ident,$help:ident,$lil:ident,$factor:expr) => {
        impl<T> Discrim<$big,T> for $name {
            fn discrim( &self, pairs : ~[($big,T)] ) -> ~[~[T]] {
                Map_Discrim{
                    key: |x:$big| ((x/$factor) as $lil, (x%$factor) as $lil),
                    discrim: Pair_Discrim($help,$help)
                }.discrim(pairs)
            }
        }
    }
}

macro_rules! make_int_discrim {
    ($name:ident,$i:ident,$help:ident,$u:ident) => {
        impl<T> Discrim<$i,T> for $name {
            fn discrim( &self, pairs : ~[($i,T)] ) -> ~[~[T]] {
                Map_Discrim{
                    key: |x:$i| (x - $i::min_value) as $u,
                    discrim: $help
                }.discrim(pairs)
            }
        }
    }
}

macro_rules! make_cast_discrim {
    ($name:ident,$from:ident,$help:ident,$to:ident) => {
        impl<T> Discrim<$from,T> for $name {
            fn discrim( &self, pairs : ~[($from,T)] ) -> ~[~[T]] {
                Map_Discrim{
                    key: |x:$from| x as $to,
                    discrim: $help
                }.discrim(pairs)
            }
        }
    }
}

struct U8_Discrim;
struct U16_Discrim;
struct U32_Discrim;
struct U64_Discrim;
struct UInt_Discrim;

struct I8_Discrim;
struct I16_Discrim;
struct I32_Discrim;
struct I64_Discrim;
struct Int_Discrim;

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

make_uint_discrim!(U16_Discrim,u16,U8_Discrim,u8,1u16<<8)
make_uint_discrim!(U32_Discrim,u32,U16_Discrim,u16,1u32<<16)
make_uint_discrim!(U64_Discrim,u64,U32_Discrim,u32,1u64<<32)

make_int_discrim!(I8_Discrim,i8,U8_Discrim,u8)
make_int_discrim!(I16_Discrim,i16,U16_Discrim,u16)
make_int_discrim!(I32_Discrim,i32,U32_Discrim,u32)
make_int_discrim!(I64_Discrim,i64,U64_Discrim,u64)

make_cast_discrim!(Int_Discrim,int,I64_Discrim,i64)
make_cast_discrim!(UInt_Discrim,uint,U64_Discrim,u64)

struct Iterator_Discrim<D>{ elem: D }

impl<K,V,I:Iterator<K>,D:Discrim<K,(I,V)>>
Discrim<I,V> for Iterator_Discrim<D> {

    fn discrim( &self, pairs : ~[(I,V)] ) -> ~[~[V]] {
        let mut done = ~[];
        let mut todo = ~[pairs];
        while( todo.len() > 0 ) {
            let mut new_todo = ~[];
            for group in todo.move_iter() {
                let mut done_group = ~[];
                let mut work_group = ~[];
                for (i0,v) in group.move_iter() {
                    let mut i = i0;
                    match i.next() {
                        None => { done_group.push(v) }
                        Some(e) => { work_group.push( (e,(i,v)) ) }
                    }
                }
                if( done.len() > 0 ) {
                    done.push(done_group);
                }
                let todo_groups = self.elem.discrim(work_group);
                new_todo.push_all_move( todo_groups );
            }
            todo = new_todo;
        }
        done
    }
    
}

impl<'self,K,V,D:Discrim<K,V>> Discrim<K,V> for &'self D {

    fn discrim( &self, pairs : ~[(K,V)] ) -> ~[~[V]] {
        (**self).discrim(pairs)
    }

}

struct Vector_Discrim<D>{ elem: D }

impl<K:Clone,V,D:Discrim<K,(vec::MoveIterator<K>,V)>>
Discrim<~[K],V> for Vector_Discrim<D> {

    fn discrim( &self, pairs : ~[(~[K],V)] ) -> ~[~[V]] {
        let iter_disc = Iterator_Discrim{ elem: &self.elem };
        let vec_iter = |v:~[K]| v.move_iter();
        let vec_disc = Map_Discrim{ key: vec_iter, discrim: iter_disc };
        vec_disc.discrim(pairs)
    }

}

fn main () {
    let input = ~[~[3,1,4], ~[], ~[3,1,4,1,5], ~[1,2,3]];
    println( input.to_str() );
    let output = discrim_sort( Vector_Discrim{ elem: Int_Discrim }, input );
    println( input.to_str() );
    println( output.to_str() );
}
