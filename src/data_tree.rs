use tree_macro::add_field;

use crate::nested;


#[derive(Debug)]
pub struct Bar {
}

trace_macros!(true);
//nested!{
    #[add_field]   
    #[derive(Debug)] 
        pub struct Foo {
            pub bar: Bar,
        }
//}
trace_macros!(false);

/*
nested!{
    #[derive(Debug)]
    Foo {
        bar: Bar {
            foobar: Foobar {
            }
        }
    }
}
*/
/*
nested!{
    #[derive(Debug)]
    Foo {
        Bar {
            Foobar {
            }
        }
    }
}
*/
/*
nested!{
    #[derive(Debug)]
    Foo {
        bar: Bar {
            foobar: Foobar {
            }
        }
    }
}
*/
/*
nested!{
    #[derive(Debug)]
    struct Foo {
        bar: struct Bar {
            foobar: struct Foobar {
            }
        }
    }
}
*/
/*
nested!{
    #[derive(Debug)]
    Ship { 
    }
}
*/
/*
nested!{
    #[derive(Debug)]
    Ship { // 1.1.1
        Body { // 2.2.2 
            Mass { // 3.3.3
            }
        }
        Cargo { // 4.4.4
            Mass { // 5.5.5
            }
            Volume { // 6.6.6
            }
        }
    }
}
*/
/*
nested!{
    #[derive(Debug)]
    Ship {
        Body {
            Mass {

            }
            Length {

            }
            Width {

            }
        }
        Cargo {            
        }
    }
}
*/