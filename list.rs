
enum List<E> { Empty, Cons( @E, @List<E> ) }

impl<E> Iterator<@E> for List<E> {

    fn next(&mut self) -> Option<@E> {
        match *self {
            Empty => { None }
            Cons( x, xs ) => { *self = *xs; Some( x ) }
        }
    }

}

impl<E> List<E> {

    fn foldl<R>( &self, mut r : R, f : &fn(@E,R)->R ) -> R {
        let mut xs = *self;
        for x in xs { r = f(x,r) };
        return r
    }

    fn map<F:'static>( &self, f : &fn(@E)->@F ) -> List<F> {
        let rev = do self.foldl(Empty) |x,ys| {
            Cons(f(x),@ys)
        };
        rev.reverse()
    }

}

impl<E:'static> List<E> {

    fn revapp( &self, xs : List<E> ) -> List<E> {
        do self.foldl(xs) |x,ys| {
            Cons(x,@ys)
        }
    }

    fn reverse( &self ) -> List<E> {
        self.revapp(Empty)
    }

    fn foldr<R>( &self, r : R, f : &fn(@E,R)->R ) -> R {
        self.reverse().foldl(r,f)
    }

    fn append( &self, xs : List<E> ) -> List<E> {
        self.reverse().revapp(xs)
    }

}

impl<E:'static> List<List<E>> {

    fn append_all( &self ) -> List<E> {
        do self.foldr(Empty) |xs,ys| {
            xs.append(ys)
        }
    }

}

type Discrim<'self,K,V> = &'self fn( List<(K,V)> ) -> List<List<V>>;

impl<'self, E:Clone+'static> List<E> {

    fn dsort( &self, d : Discrim<'self,E,E> ) -> List<E> {
        let pairs = do self.map |x| { @((*x).clone(),(*x).clone()) };
        d(pairs).append_all()
    }

}

fn main () {}
