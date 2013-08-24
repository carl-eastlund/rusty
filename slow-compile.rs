
// If the definition of Data is simpler, the compiler attempts to inline
// generic<T>, fails, and produces an error.
struct Data(~Option<Data>);

fn generic<T>( _ : ~[(Data,T)] ) {
    // The important detail is substituting (bool,T) for T on each
    // recursive call so the number of representations necessary to compile
    // this function is infinite.
    let rec : ~[(Data,(bool,T))] = ~[];
    generic( rec );
}

fn main () {
    // Use generic<T> at least once to trigger instantiation.
    let input : ~[(Data,())] = ~[];
    generic(input);
}
