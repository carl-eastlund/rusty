
#[deriving (Clone,ToStr)]
enum Type<T> { Empty, Cons( @T, @Type<T> ) }

type Fun<'self,T> = &'self fn(Type<T>) -> Type<Type<T>>;

fn op<'self,T:Clone+'static>( _:Fun<'self,T>, _:Type<T> ) -> Type<T> {
    Empty
}

fn main () {}
