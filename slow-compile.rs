
use std::vec;

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

struct UnitDisc;

impl<T> Disc<(),T> for UnitDisc {

    fn disc( &self, pairs : ~[((),T)] ) -> ~[~[T]] {
        ~[ vec_map_move( pairs, |(_,x)| x ) ]
    }

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

    }
    
}

impl<'self,K,V,D:Disc<K,V>> Disc<K,V> for &'self D {

    fn disc( &self, pairs : ~[(K,V)] ) -> ~[~[V]] {
        (**self).disc(pairs)
    }

}

struct OwnedVecDisc<D>{ elem: D }

impl<K,V,D:Disc<K,(vec::MoveIterator<K>,V)>>
Disc<~[K],V> for OwnedVecDisc<D> {

    fn disc( &self, pairs : ~[(~[K],V)] ) -> ~[~[V]] {
        let iter_disc = IterDisc{ elem: &self.elem };
        let vec_iter = |v:~[K]| v.move_iter();
        let vec_disc = MapDisc{ key: vec_iter, disc: iter_disc };
        vec_disc.disc(pairs)
    }

}

#[deriving (Clone,ToStr)]
enum Tree {
    Unit,
    Node(~[Tree])
}

struct TreeDisc;

impl<T> Disc<Tree,T> for TreeDisc {

    fn disc( &self, pairs : ~[(Tree,T)] ) -> ~[~[T]] {
        let mut units = ~[];
        let mut nodes = ~[];
        for (k,v) in pairs.move_iter() {
            match k {
                Unit => { units.push( ((),v) ) }
                Node(vec) => { nodes.push( (vec,v) ) }
            }
        }
        let mut sorted = ~[];
        sorted.push_all_move( UnitDisc.disc( units ) );
        sorted.push_all_move( OwnedVecDisc{ elem: TreeDisc }.disc( nodes ) );
        sorted
    }

}

fn u() -> Tree { Unit }
fn n( n : ~[Tree] ) -> Tree { Node(n) }

fn main () {
    let input =
        ~[n(~[u(), u()]),
          u(),
          n(~[u(), u(), u()]),
          u(),
          n(~[u()])];
    println( input.to_str() );
    let output = disc_sort( TreeDisc, input );
    println( output.to_str() );
}