
enum Tree { Node(~[Tree]) }

struct TreeDisc;

impl<T> TreeDisc {

    fn disc( &self, pairs : ~[(Tree,T)] ) {
        let mut ys = ~[];
        for (Node(k),v) in pairs.move_iter() {
            ys.push( (k.move_iter(),v) );
        }
        let mut sorted = ~[];
        let mut todo_stack = ~[ys];
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
            TreeDisc.disc( todo_elems );
        }
    }

}

fn main () {
    let input : ~[(Tree,())] = ~[]; TreeDisc.disc(input);
}
