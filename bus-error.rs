
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

trait Discrim<K,V> {

    fn discrim( &self, ~[(K,V)] ) -> ~[~[V]];

}

fn discrim_sort<T:Clone,D:Discrim<T,T>>( d : D, xs : &[T] ) -> ~[T] {
    let pairs = do vec_map(xs) |x| { ((*x).clone(), (*x).clone()) };
    let groups = d.discrim(pairs);
    vec_collapse_move( groups )
}

struct Map_Discrim<'self,A,B,D>{ key : &'self fn(A)->B, discrim : D }

impl<'self,A,B,V,D:Discrim<B,V>> Discrim<A,V> for Map_Discrim<'self,A,B,D> {

    fn discrim( &self, pairs : ~[(A,V)] ) -> ~[~[V]] {
        let mapped = do vec_map_move(pairs) |(k,v)| { ((self.key)(k),v) };
        self.discrim.discrim(mapped)
    }

}

struct U8_Discrim;

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
        self.discrim(pairs)
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
    let input = ~[];
    println("start");
    discrim_sort( Vector_Discrim{ elem: U8_Discrim }, input );
    println("finish");
}
