#![feature(trace_macros)]   
pub mod nested;
pub mod data_tree;
use std::env;
use data_tree::*;
use tree_macro::add_field;
//

/*
#[add_field]
#[derive (Debug)]
struct Ship { 
    a: String,
}
*/


fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();


 //   let tree = Foo{ bar: Bar{}, a: "a".to_string() };
   let tree = Foo{ bar: Bar{} };
    println!("tree: {:#?}", tree);

 //   let ship = Ship{ a: "a".to_string() };
 //   println!("ship: {:#?}", ship);

 /*   let tree = Foo::new("");
    println!("tree: {:#?}", tree);
    println!("tree.path: {:#?}", tree.path());
    println!("tree.bar.path: {:#?}", tree.bar.path());
    println!("tree.bar.foobar.path: {:#?}", tree.bar.foobar.path());
*/
  /*  println!("{:#?}", 
        Foo {  
            path: "/Foo".to_owned(), 
            bar: Bar { 
                path: "/Bar".to_owned(),  
                foobar: Foobar {
                    path: "/Foobar".to_owned(), 
                } 
            } 
        }  
    );
*/
 /*   let tree = Parent::new("");
    println!("tree: {:#?}", tree);
    println!("tree.path: {:#?}", tree.path());
    println!("tree.child.path: {:#?}", tree.child.path());
  */ 
/*
    let tree = Ship::new("");
    println!("tree: {:#?}", tree);
    println!("tree.path: {:#?}", tree.path());
    println!("tree.body.path: {:#?}", tree.child.path());
 */   
}


/*

use std::env;
use nestify::nest;
use struct_iterable::Iterable;

/*
macro_rules! nested {
    //
    // input is empty: time to output
    (@munch () -> {$(#[$attr:meta])* $name:ident $(($ty:ty))*}) => {
        $(#[$attr])*
        struct $name {
            path: String,
            $(child: $ty,),*
            // $($id: $ty,),*
        }
    };
    //    
    // branch off to generate an inner struct
    (@munch ($name:ident {$($inner:tt)*} $($next:tt)*) -> {$(#[$attr:meta])* struct $($output:tt)*}) => {
        nested!(@munch ($($inner)*) -> {$(#[$attr])* $name});
        nested!(@munch ($($next)*) -> {$(#[$attr])* $($output)* ($name)});
    };
    //
    // throw on the last field
    (@munch ($ty:ty) -> {$($output:tt)*}) => {
        nested!(@munch () -> {$($output)* ($ty)});
    };
    //
    // throw on another field (not the last one)
    (@munch ($ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        nested!(@munch ($($next)*) -> {$($output)* ($id: $ty)});
    };
    //
    // entry point (this is where a macro call starts)
    ($(#[$attr:meta])* $name:ident { $($input:tt)*} ) => {
        nested!(@munch ($($input)*) -> {$(#[$attr])* struct $name});
        //                 ^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                     input       output
    }
}
*/
/*
macro_rules! nested {
    // input is empty: time to output
    (@munch () -> {$(#[$attr:meta])* struct $name:ident $(($id:ident: $ty:ty))*}) => {
        $(#[$attr])* struct $name { path: String, $($id: $ty),* }
    };

    // branch off to generate an inner struct
    (@munch ($id:ident: struct $name:ident {$($inner:tt)*} $($next:tt)*) -> {$(#[$attr:meta])* struct $($output:tt)*}) => {
        nested!(@munch ($($inner)*) -> {$(#[$attr])* struct $name});
        nested!(@munch ($($next)*) -> {$(#[$attr])* struct $($output)* ($id: $name)});
    };
    
    // throw on the last field
    (@munch ($id:ident: $ty:ty) -> {$($output:tt)*}) => {
        nested!(@munch () -> {$($output)* ($id: $ty)});
    };
    
    // throw on another field (not the last one)
    (@munch ($id:ident: $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        nested!(@munch ($($next)*) -> {$($output)* ($id: $ty)});
    };
    
    // entry point (this is where a macro call starts)
    ($(#[$attr:meta])* struct $name:ident { $($input:tt)*} ) => {
        nested!(@munch ($($input)*) -> {$(#[$attr])* struct $name});
        //                 ^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                     input       output
    }
}
*/

///
/// Build tree
nested!{
    #[derive(Debug)]
    struct Foo {
        bar: struct Bar {
            foobar: struct Foobar {
            }
        }
    }
}





fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    println!("{:#?}", 
        Foo {  
            path: "/Foo".to_owned(), 
            bar: Bar { 
                path: "/Bar".to_owned(),  
                foobar: Foobar {
                    path: "/Foobar".to_owned(), 
                } 
            } 
        }  
    );

    // let thing = Parent { child: Child {} };
    // let name = name_struct!(thing.child);
    // println!("{name}"); // on playground prints: "playground::Parent"

    // let tree = Parent::from(Child {});
    // Child, };

/*    let tree = Parent {
        path: "/parent".to_owned(),
        child: Child {
            path: "/child".to_owned(),
        }
    };
    println!("tree: {:#?}", tree);
    println!("tree.path: {:#?}", tree.path);
    println!("tree.child.path: {:#?}", tree.child.path);*/
}

*/