
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

trait Disc<K,V> {

    fn disc( &self, ~[(K,V)] ) -> ~[~[V]];

}

fn disc_sort<T:Clone,D:Disc<T,T>>( d : D, xs : &[T] ) -> ~[T] {
    let pairs = do vec_map(xs) |x| { ((*x).clone(), (*x).clone()) };
    let groups = d.disc(pairs);
    vec_collapse_move( groups )
}

struct Pair_Disc<A,B>(A,B);

impl<K1,K2,V,D1:Disc<K1,(K2,V)>,D2:Disc<K2,V>>
Disc<(K1,K2),V> for Pair_Disc<D1,D2> {

    fn disc( &self, pairs : ~[((K1,K2),V)] ) -> ~[~[V]] {
        match self {
            &Pair_Disc(ref first, ref second) => {
                let nested = do vec_map_move(pairs) |((k1,k2),v)| {
                    (k1,(k2,v))
                };
                let pass1 = first.disc(nested);
                let pass2 = do vec_map_move(pass1) |group| {
                    second.disc(group)
                };
                vec_collapse_move( pass2 )
            }
        }
    }

}

struct Map_Disc<'self,A,B,D>{ key : &'self fn(A)->B, disc : D }

impl<'self,A,B,V,D:Disc<B,V>> Disc<A,V> for Map_Disc<'self,A,B,D> {

    fn disc( &self, pairs : ~[(A,V)] ) -> ~[~[V]] {
        let mapped = do vec_map_move(pairs) |(k,v)| { ((self.key)(k),v) };
        self.disc.disc(mapped)
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

macro_rules! make_uint_disc {
    ($name:ident,$big:ident,$help:ident,$lil:ident,$factor:expr) => {
        impl<T> Disc<$big,T> for $name {
            fn disc( &self, pairs : ~[($big,T)] ) -> ~[~[T]] {
                Map_Disc{
                    key: |x:$big| ((x/$factor) as $lil, (x%$factor) as $lil),
                    disc: Pair_Disc($help,$help)
                }.disc(pairs)
            }
        }
    }
}

macro_rules! make_int_disc {
    ($name:ident,$i:ident,$help:ident,$u:ident) => {
        impl<T> Disc<$i,T> for $name {
            fn disc( &self, pairs : ~[($i,T)] ) -> ~[~[T]] {
                Map_Disc{
                    key: |x:$i| (x - $i::min_value) as $u,
                    disc: $help
                }.disc(pairs)
            }
        }
    }
}

macro_rules! make_cast_disc {
    ($name:ident,$from:ident,$help:ident,$to:ident) => {
        impl<T> Disc<$from,T> for $name {
            fn disc( &self, pairs : ~[($from,T)] ) -> ~[~[T]] {
                Map_Disc{
                    key: |x:$from| x as $to,
                    disc: $help
                }.disc(pairs)
            }
        }
    }
}

struct U8_Disc;
struct U16_Disc;
struct U32_Disc;
struct U64_Disc;
struct UInt_Disc;

struct I8_Disc;
struct I16_Disc;
struct I32_Disc;
struct I64_Disc;
struct Int_Disc;

impl<T> Disc<u8,T> for U8_Disc {

    fn disc( &self, pairs : ~[(u8,T)] ) -> ~[~[T]] {
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

make_uint_disc!(U16_Disc,u16,U8_Disc,u8,1u16<<8)
make_uint_disc!(U32_Disc,u32,U16_Disc,u16,1u32<<16)
make_uint_disc!(U64_Disc,u64,U32_Disc,u32,1u64<<32)

make_int_disc!(I8_Disc,i8,U8_Disc,u8)
make_int_disc!(I16_Disc,i16,U16_Disc,u16)
make_int_disc!(I32_Disc,i32,U32_Disc,u32)
make_int_disc!(I64_Disc,i64,U64_Disc,u64)

make_cast_disc!(Int_Disc,int,I64_Disc,i64)
make_cast_disc!(UInt_Disc,uint,U64_Disc,u64)

struct Iter_Disc<D>{ elem: D }

impl<K,V,I:Iterator<K>,D:Disc<K,(I,V)>>
Disc<I,V> for Iter_Disc<D> {

    fn disc( &self, pairs : ~[(I,V)] ) -> ~[~[V]] {
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
                if( done_group.len() > 0 ) {
                    done.push(done_group);
                }
                let todo_groups = self.elem.disc(work_group);
                new_todo.push_all_move( todo_groups );
            }
            todo = new_todo;
        }
        done
    }
    
}

impl<'self,K,V,D:Disc<K,V>> Disc<K,V> for &'self D {

    fn disc( &self, pairs : ~[(K,V)] ) -> ~[~[V]] {
        (**self).disc(pairs)
    }

}

struct Owned_Vec_Disc<D>{ elem: D }

impl<K,V,D:Disc<K,(vec::MoveIterator<K>,V)>>
Disc<~[K],V> for Owned_Vec_Disc<D> {

    fn disc( &self, pairs : ~[(~[K],V)] ) -> ~[~[V]] {
        let iter_disc = Iter_Disc{ elem: &self.elem };
        let vec_iter = |v:~[K]| v.move_iter();
        let vec_disc = Map_Disc{ key: vec_iter, disc: iter_disc };
        vec_disc.disc(pairs)
    }

}

struct Vec_Disc<D>{ elem: D }

impl<'self,K:Clone,V,D:Disc<K,(vec::MoveIterator<K>,V)>>
Disc<&'self[K],V> for Vec_Disc<D> {

    fn disc( &self, pairs : ~[(&[K],V)] ) -> ~[~[V]] {
        Map_Disc{
            key: |x:&[K]| x.to_owned(),
            disc: Owned_Vec_Disc{ elem: &self.elem }
        }.disc( pairs )
    }

}

fn main () {
    let input = ~[&[3,1,4], &[], &[3,1,4,1,5], &[1,2,3]];
    println( input.to_str() );
    let output = disc_sort( Vec_Disc{ elem: Int_Disc }, input );
    println( output.to_str() );
}
