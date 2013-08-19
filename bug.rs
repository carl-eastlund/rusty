
#[deriving (Clone,ToStr)]
enum List<E> { Empty, Cons( @E, @List<E> ) }

trait Discrim<K,V> {
    fn discrim(&self,List<(K,V)>) -> List<List<V>>;
}

impl<V:Clone+'static> Discrim<u8,V> for () {
    fn discrim( &self, xs:List<(u8,V)> ) -> List<List<V>> {
        *self; @xs; Empty
    }
}

fn dsort<V:Clone+'static>( d:@Discrim<V,V> ) -> List<V> {
    d.discrim(Empty); Empty
}

fn u8_disc<V:Clone+'static>() -> @Discrim<u8,V> {
    @() as @Discrim<u8,V>
}

fn main () {
    dsort(u8_disc());
}
