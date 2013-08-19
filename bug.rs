
#[deriving (Clone,ToStr)]
enum List<T> { Empty, Cons( @T, @List<T> ) }

type Discrim<'self,T> = &'self fn(List<T>) -> List<List<T>>;

fn dsort<'self,T:Clone+'static>( _:Discrim<'self,T>, _:List<T> ) -> List<T> {
    Empty
}

fn main () {}
