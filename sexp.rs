
#[deriving (ToStr)]
enum Sexp {
    List(@Sexp_List),
    Number(int),
    String(@str)
}

#[deriving (ToStr)]
enum Sexp_List {
    Null,
    Cons(@Sexp,@Sexp_List)
}

fn main () {
    let sexp12 = l([n(1), s(@"two")]);
    write_sexp( sexp12 );
    println("");
    println(sexp12.to_str());

    let sexp1234 = l([n(1), l([s(@"two"), l([]), n(3)]), s(@"four")]);
    write_sexp( sexp1234 );
    println("");
    println(sexp1234.to_str());
}

fn s(st:@str) -> Sexp { String(st) }
fn n(x:int) -> Sexp { Number(x) }
fn l(xs:&[Sexp]) -> Sexp {
    let mut lst : @Sexp_List = @Null;
    for x in xs.rev_iter() {
        lst = @Cons(@*x, lst);
    }
    List(lst)
}

fn write_sexp (sexp : Sexp) {
    match sexp {
        List(list) => { write_list(*list); }
        Number(num) => { print(std::int::to_str(num)); }
        String(st) => {
            print("\"");
            for c in st.iter() {
                match c {
                    '\\' => print("\\\\"),
                    '\"' => print("\\\""),
                    '\n' => print("\\n"),
                    '\r' => print("\\r"),
                    '\t' => print("\\t"),
                    _ => print(std::str::from_char(c)),
                }
            }
            print("\"");
        }
    }
}

fn write_list (list : Sexp_List) {
    match list {
        Null => { print ("()") }
        Cons(hd,tl) => {
            print("(");
            write_sexp(*hd);
            let mut rest = tl;
            loop {
                match *rest {
                    Null => { break; }
                    Cons(hd2,tl2) => {
                        print(" ");
                        write_sexp(*hd2);
                        rest = tl2;
                        loop;
                    }
                }
            }
            print(")");
        }
    }
}
