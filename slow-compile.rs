
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

struct MapDisc<'self,A,B,D>{ key : &'self fn(A)->B, disc : D }

impl<'self,A,B,V,D:Disc<B,V>> Disc<A,V> for MapDisc<'self,A,B,D> {

    fn disc( &self, pairs : ~[(A,V)] ) -> ~[~[V]] {
        let mapped = do vec_map_move(pairs) |(k,v)| { ((self.key)(k),v) };
        self.disc.disc(mapped)
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
    Node(~[Tree])
}

struct TreeDisc;

impl<T> Disc<Tree,T> for TreeDisc {

    fn disc( &self, pairs : ~[(Tree,T)] ) -> ~[~[T]] {
        let nodes = do vec_map_move(pairs) |(Node(vec),v)| { (vec,v) };
        OwnedVecDisc{ elem: TreeDisc }.disc(nodes)
    }

}

fn main () {
    let input : ~[Tree] = ~[];
    println( input.to_str() );
    let output = disc_sort( TreeDisc, input );
    println( output.to_str() );
}
