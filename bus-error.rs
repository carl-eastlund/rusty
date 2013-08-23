
trait Discrim {

    fn discrim( &self );

}

struct Unit_Discrim;

impl Discrim for Unit_Discrim {

    fn discrim( &self ) {}

}

struct Iterator_Discrim<D>{ elem: D }

impl<D:Discrim> Discrim for Iterator_Discrim<D> {

    fn discrim( &self ) {
        self.elem.discrim();
    }
    
}

impl<'self,D> Discrim for &'self D {

    fn discrim( &self ) {
        self.discrim()
    }

}

struct Vector_Discrim<D>{ elem: D }

impl<D:Discrim> Discrim for Vector_Discrim<D> {

    fn discrim( &self ) {
        Iterator_Discrim{
            elem: &self.elem
        }.discrim()
    }

}

fn main () {
    println("start");
    let u = Unit_Discrim;
    let v = Vector_Discrim{ elem: u };
    v.discrim();
    println("finish");
}
