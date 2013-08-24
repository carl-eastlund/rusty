
use std::vec;

trait Disc<K,V> {

    fn disc( &self, ~[(K,V)] ) -> ~[~[V]];

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

#[deriving (Clone,ToStr)]
enum Tree { Node(~[Tree]) }

struct TreeDisc;

impl<T> Disc<Tree,T> for TreeDisc {

    fn disc( &self, pairs : ~[(Tree,T)] ) -> ~[~[T]] {
        let mut ys = vec::with_capacity(pairs.len());
        for (Node(k),v) in pairs.move_iter() {
            ys.push( (k.move_iter(),v) );
        }
        IterDisc{ elem: TreeDisc }.disc(ys)
    }

}

fn main () {
    let input : ~[(Tree,())] = ~[];
    println( input.to_str() );
    let output = TreeDisc.disc(input);
    println( output.to_str() );
}
