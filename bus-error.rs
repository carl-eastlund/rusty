
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

trait Discrim<K,V> {

    fn discrim( &self, ~[(K,V)] ) -> ~[~[V]];

}

struct Unit_Discrim;

impl<T> Discrim<(),T> for Unit_Discrim {

    fn discrim( &self, pairs : ~[((),T)] ) -> ~[~[T]] {
        ~[ vec_map_move( pairs, |(_,x)| x ) ]
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
        Iterator_Discrim{
            elem: &self.elem
        }.discrim( do vec_map_move(pairs) |(k,v)| { (k.move_iter(),v) } )
    }

}

fn main () {
    println("start");
    let input : ~[(~[()],())] = ~[];
    Vector_Discrim{ elem: Unit_Discrim }.discrim(input);
    println("finish");
}
