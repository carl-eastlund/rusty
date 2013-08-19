
#[deriving (Clone,ToStr)]
enum List<E> { Empty }

trait Discrim<K,V> {
    fn discrim(&self,List<(K,V)>) -> ();
}

impl<V:Clone+'static> Discrim<u8,V> for () {
    fn discrim( &self, xs:List<(u8,V)> ) -> () {
        *self; @xs;
    }
}

fn dsort<V:Clone+'static>( d:@Discrim<V,V> ) -> () {
    d.discrim(Empty);
}

fn u8_disc<V:Clone+'static>() -> @Discrim<u8,V> {
    @() as @Discrim<u8,V>
}

fn main () {
    dsort(u8_disc());
}
