
trait Trait {

    fn method( &self );

}

struct Simple;

impl Trait for Simple {

    fn method( &self ) {}

}

impl<'self,D> Trait for &'self D {

    fn method( &self ) {
        self.method()
    }

}

struct Nested<D>{ elem: D }

impl<D:Trait> Trait for Nested<D> {

    fn method( &self ) {
        self.elem.method();
    }
    
}

fn main () {
    println("start");
    Nested{ elem: &Simple }.method();
    println("finish");
}
