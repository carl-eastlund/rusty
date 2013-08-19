
#[deriving (Clone,ToStr)]
enum Type<T> { Constant }

trait Trait<K,V> {
    fn method(&self,Type<(K,V)>) -> ();
}

impl<V:Clone+'static> Trait<u8,V> for () {
    fn method( &self, xs:Type<(u8,V)> ) -> () {
        *self; @xs;
    }
}

fn function<V:Clone+'static>( x:@Trait<V,V> ) -> () {
    x.method(Constant);
}

fn instance<V:Clone+'static>() -> @Trait<u8,V> {
    @() as @Trait<u8,V>
}

fn main () {
    function(instance());
}
