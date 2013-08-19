
enum Maybe<V> { Yes(@V), No }

fn wrapped<V>( m : Maybe<V> ) -> Maybe<V> {
    let mut v : [Maybe<V>, ..8] = [No, ..8];
    v[1] = m;
    v[0]
}

fn main () {}
