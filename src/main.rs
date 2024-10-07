use std::env;
extern crate proc_macro;

macro_rules! nested {
    //
    // input is empty: time to output
    (@munch () -> {$(#[$attr:meta])* $name:ident $(($ty:ty))*}) => {
        $(#[$attr])*
  //      #[derive(NewMacro)]    
        struct $name {
            path: String,
            $(child: $ty,),*
            // $($id: $ty,),*
        }
        impl $name {
            pub fn new(parent: impl Into<String>) -> Self {
                let path = format!("{}/{}", parent.into(), stringify!($name));
                let path = path.chars().map(|s| {
                    if s.is_uppercase() {
                        format!("_{}", s.to_lowercase())
                    } else {
                        s.to_string()
                    }
                })               
                .collect::<String>()
                .replace("/_", "/");                
                Self {
                    path: path.clone(),
                    $(child: <$ty>::new(path.clone()),),* 
                }
            }
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
// Build tree
nested!{
    #[derive(Debug)]
    Parent {
        Child {
        }
    }
}

//
fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // let thing = Parent { child: Child {} };
    // let name = name_struct!(thing.child);
    // println!("{name}"); // on playground prints: "playground::Parent"

    // let tree = Parent::from(Child {});
    // Child, };

    let tree = Parent::new("".to_string());
    println!("tree: {:#?}", tree);
    println!("tree.path: {:#?}", tree.path);
    println!("tree.child.path: {:#?}", tree.child.path);
}