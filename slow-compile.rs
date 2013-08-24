
enum Tree { Node(~[Tree]) }

struct MacGuffin;

impl<T> MacGuffin {

    fn disc( &self, _ : ~[(Tree,T)] ) {
        let todo_elems : ~[(Tree,(Option<Tree>,T))] = ~[];
        MacGuffin.disc( todo_elems );
    }

}

fn main () {
    let input : ~[(Tree,())] = ~[]; MacGuffin.disc(input);
}
