extern crate proc_macro;

#[macro_export]
macro_rules! nested {
    //
    // input is empty: time to output
    (@munch () -> {$(#[$attr:meta])* $name:ident $(($ty:ty))*}) => {
        $(#[$attr])*
  //      #[derive(NewMacro)]    
        pub struct $name {
            path: String,
            $(pub child: $ty,),*
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
            pub fn path(&self) -> &str {
                &self.path
            }
        }
    };
    //    
    // branch off to generate an inner struct
    (@munch ($name:ident {$($inner:tt)*} $($next:tt)*) -> {$(#[$attr:meta])* pub struct $($output:tt)*}) => {
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
        nested!(@munch ($($input)*) -> {$(#[$attr])* pub struct $name});
        //                 ^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                     input       output
    }
}