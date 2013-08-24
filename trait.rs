
use std::vec;
use std::i8;
use std::i16;
use std::i32;
use std::i64;

type MvIter<T> = vec::MoveIterator<T>;

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
    if( xs.len() == 0 ) {
        return ~[]
    }
    let pairs = do vec_map(xs) |x| { ((*x).clone(), (*x).clone()) };
    let groups = d.disc(pairs);
    vec_collapse_move( groups )
}

struct PairDisc<A,B>(A,B);

impl<K1,K2,V,D1:Disc<K1,(K2,V)>,D2:Disc<K2,V>>
Disc<(K1,K2),V> for PairDisc<D1,D2> {

    fn disc( &self, pairs : ~[((K1,K2),V)] ) -> ~[~[V]] {
        match self {
            &PairDisc(ref first, ref second) => {
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

struct MapDisc<'self,A,B,D>{ key : &'self fn(A)->B, disc : D }

impl<'self,A,B,V,D:Disc<B,V>> Disc<A,V> for MapDisc<'self,A,B,D> {

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
                MapDisc{
                    key: |x:$big| ((x/$factor) as $lil, (x%$factor) as $lil),
                    disc: PairDisc($help,$help)
                }.disc(pairs)
            }
        }
    }
}

macro_rules! make_int_disc {
    ($name:ident,$i:ident,$help:ident,$u:ident) => {
        impl<T> Disc<$i,T> for $name {
            fn disc( &self, pairs : ~[($i,T)] ) -> ~[~[T]] {
                MapDisc{
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
                MapDisc{
                    key: |x:$from| x as $to,
                    disc: $help
                }.disc(pairs)
            }
        }
    }
}

struct U8Disc;
struct U16Disc;
struct U32Disc;
struct U64Disc;
struct UIntDisc;

struct I8Disc;
struct I16Disc;
struct I32Disc;
struct I64Disc;
struct IntDisc;

impl<T> Disc<u8,T> for U8Disc {

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

make_uint_disc!(U16Disc,u16,U8Disc,u8,1u16<<8)
make_uint_disc!(U32Disc,u32,U16Disc,u16,1u32<<16)
make_uint_disc!(U64Disc,u64,U32Disc,u32,1u64<<32)

make_int_disc!(I8Disc,i8,U8Disc,u8)
make_int_disc!(I16Disc,i16,U16Disc,u16)
make_int_disc!(I32Disc,i32,U32Disc,u32)
make_int_disc!(I64Disc,i64,U64Disc,u64)

make_cast_disc!(IntDisc,int,I64Disc,i64)
make_cast_disc!(UIntDisc,uint,U64Disc,u64)

struct IterDisc<D>{ elem: D }

impl<K,V,I:Iterator<K>,D:Disc<K,(I,V)>>
Disc<I,V> for IterDisc<D> {

    fn disc( &self, pairs : ~[(I,V)] ) -> ~[~[V]] {

        let mut sorted = ~[];
        let mut todo_stack = ~[pairs];
        while( todo_stack.len() > 0 ) {
            let todo_iters = todo_stack.pop();
            let mut done = ~[];
            let mut todo_elems = ~[];
            for (k,v) in todo_iters.move_iter() {
                let mut i = k;
                match i.next() {
                    None => { done.push(v) }
                    Some(e) => { todo_elems.push( (e,(i,v)) ) }
                }
            }
            if( done.len() > 0 ) {
                sorted.push( done );
            }
            let todo_groups = self.elem.disc( todo_elems );
            for todo_group in todo_groups.move_rev_iter() {
                todo_stack.push(todo_group);
            }
        }
        sorted

        // let mut done = ~[];
        // let mut todo = ~[pairs];
        // while( todo.len() > 0 ) {
        //     let mut new_todo = ~[];
        //     for group in todo.move_iter() {
        //         let mut done_group = ~[];
        //         let mut work_group = ~[];
        //         for (i0,v) in group.move_iter() {
        //             let mut i = i0;
        //             match i.next() {
        //                 None => { done_group.push(v) }
        //                 Some(e) => { work_group.push( (e,(i,v)) ) }
        //             }
        //         }
        //         if( done_group.len() > 0 ) {
        //             done.push(done_group);
        //         }
        //         let todo_groups = self.elem.disc(work_group);
        //         new_todo.push_all_move( todo_groups );
        //     }
        //     todo = new_todo;
        // }
        // done

    }
    
}

impl<'self,K,V,D:Disc<K,V>> Disc<K,V> for &'self D {

    fn disc( &self, pairs : ~[(K,V)] ) -> ~[~[V]] {
        (**self).disc(pairs)
    }

}

struct OwnedVecDisc<D>{ elem: D }

impl<K,V,D:Disc<K,(MvIter<K>,V)>>
Disc<~[K],V> for OwnedVecDisc<D> {

    fn disc( &self, pairs : ~[(~[K],V)] ) -> ~[~[V]] {
        let iter_disc = IterDisc{ elem: &self.elem };
        let vec_iter = |v:~[K]| v.move_iter();
        let vec_disc = MapDisc{ key: vec_iter, disc: iter_disc };
        vec_disc.disc(pairs)
    }

}

struct VecDisc<D>{ elem: D }

impl<'self,K:Clone,V,D:Disc<K,(MvIter<K>,V)>>
Disc<&'self[K],V> for VecDisc<D> {

    fn disc( &self, pairs : ~[(&[K],V)] ) -> ~[~[V]] {
        MapDisc{
            key: |x:&[K]| x.to_owned(),
            disc: OwnedVecDisc{ elem: &self.elem }
        }.disc( pairs )
    }

}

struct CharDisc;
struct StrDisc;
struct OwnedStrDisc;

impl<T> Disc<char,T> for CharDisc {

    fn disc( &self, pairs : ~[(char,T)] ) -> ~[~[T]] {
        MapDisc{
            key: |c:char| c as u32,
            disc: U32Disc
        }.disc( pairs )
    }

}

impl<'self,V> Disc<&'self str,V> for StrDisc {

    fn disc( &self, pairs : ~[(&str,V)] ) -> ~[~[V]] {
        let iter_disc = IterDisc{ elem: CharDisc };
        let str_iter = |s:&str| s.iter();
        let map_disc = MapDisc{ key: str_iter, disc: iter_disc };
        map_disc.disc( pairs )
    }

}

impl<V> Disc<~str,V> for OwnedStrDisc {

    fn disc( &self, pairs : ~[(~str,V)] ) -> ~[~[V]] {
        let mut strs = ~[];
        let mut vals = ~[];
        for (k,v) in pairs.move_iter() {
            strs.push(k);
            vals.push(v);
        }
        let mut slices = ~[];
        for (k,v) in strs.iter().zip(vals.move_iter()) {
            slices.push( (k.as_slice(), v) );
        }
        StrDisc.disc( slices )
    }

}

#[deriving (Clone,ToStr)]
enum Tree {
    Integer(int),
    String(~str),
    Node(~[Tree])
}

struct TreeDisc;
struct PushTreeDisc;
struct NestTreeDisc;

type Push<T> = (MvIter<Tree>,Nest<T>);
type Nest<T> = (~[MvIter<Tree>],T);

impl<T> Disc<Tree,T> for TreeDisc {

    fn disc( &self, pairs : ~[(Tree,T)] ) -> ~[~[T]] {
        let nested = do vec_map_move( pairs ) |(k,v)| { (k,(~[],v)) };
        let sorted = NestTreeDisc.disc( nested );
        let lifted = do vec_map_move( sorted ) |vs| {
            do vec_map_move( vs ) |(_,v)| { v }
        };
        lifted
    }

}

impl<T> Disc<Tree,Push<T>> for PushTreeDisc {

    fn disc( &self, pairs : ~[(Tree,Push<T>)] ) -> ~[~[Push<T>]] {
        let nested = do vec_map_move( pairs ) |(k,(i,(is,v)))| {
            let mut iters = is;
            iters.push(i);
            (k,(iters,v))
        };
        let sorted = NestTreeDisc.disc( nested );
        let lifted = do vec_map_move( sorted ) |vs| {
            do vec_map_move( vs ) |(is,v)| {
                let mut iters = is;
                let i = iters.pop();
                (i,(iters,v))
            }
        };
        lifted
    }

}

impl<T> Disc<Tree,Nest<T>> for NestTreeDisc {

    fn disc( &self, pairs : ~[(Tree,Nest<T>)] ) -> ~[~[Nest<T>]] {
        let mut ints = ~[];
        let mut strs = ~[];
        let mut nodes = ~[];
        for (k,v) in pairs.move_iter() {
            match k {
                Integer(i) => { ints.push( (i,v) ) }
                String(s) => { strs.push( (s,v) ) }
                Node(vec) => { nodes.push( (vec,v) ) }
            }
        }
        let mut sorted = ~[];
        if( ints.len() > 0 ) {
            sorted.push_all_move( IntDisc.disc(ints) );
        }
        if( strs.len() > 0 ) {
            sorted.push_all_move( OwnedStrDisc.disc(strs) );
        }
        if( nodes.len() > 0 ) {
            sorted.push_all_move
                ( OwnedVecDisc{ elem: PushTreeDisc }.disc(nodes) );
        }
        sorted
    }

}

fn i( i : int ) -> Tree { Integer(i) }
fn s( s : ~str ) -> Tree { String(s) }
fn n( n : ~[Tree] ) -> Tree { Node(n) }

fn main () {
    let input =
        ~[i(3),
          n(~[i(3), s(~"point"), i(1), i(4)]),
          s(~"."),
          n(~[i(3), s(~"."), i(1), i(4)]),
          i(1),
          n(~[i(3), s(~"."), i(1), i(4), i(5), i(6)]),
          i(4)];
    println( input.to_str() );
    let output = disc_sort( TreeDisc, input );
    println( output.to_str() );
}
