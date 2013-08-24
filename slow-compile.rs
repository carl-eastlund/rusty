
struct Data(~Option<Data>);

fn generic<T>( _ : ~[(Data,T)] ) {
    let rec : ~[(Data,(bool,T))] = ~[];
    generic( rec );
}

fn main () {
    let input : ~[(Data,())] = ~[];
    generic(input);
}
