extern crate proc_macro;
use syn::{parse_macro_input, DeriveInput, FieldsNamed, Type};

#[macro_export]
macro_rules! nested {
    // input is empty: time to output
    (@munch () -> {$(#[$attr:meta])* struct $name:ident $(($id:ident: $ty:ty))*}) => {
        $(#[$attr])* 
        pub struct $name { 
            pub path: String, 
            $(pub $id: $ty),* 
     //       $(pub $t.to_string().to_case(Case::Snake): $t),* 
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
                    $($id: <$ty>::new(path.clone()),),*
                }
            }
            pub fn path(&self) -> &str {
                &self.path
            }
        }
    };

    // branch off to generate an inner struct
    (@munch ($id:ident: $name:ident {$($inner:tt)*} $($next:tt)*) -> {$(#[$attr:meta])* struct $($output:tt)*}) => {
        nested!(@munch ($($inner)*) -> {$(#[$attr])* struct $name});
        nested!(@munch ($($next)*) -> {$(#[$attr])* struct $($output)* ($id: $name)});
    };
    
    // throw on the last field
    (@munch ($ty:ty) -> {$($output:tt)*}) => {
        nested!(@munch () -> {$($output)* ($id: $ty)});
    };
    
    // throw on another field (not the last one)
    (@munch ($ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        nested!(@munch ($($next)*) -> {$($output)* ($id: $ty)});
    };
    
    // entry point (this is where a macro call starts)
    ($(#[$attr:meta])* $name:ident { $($input:tt)*} ) => {
        nested!(@munch ($($input)*) -> {$(#[$attr])* struct $name});
        //                 ^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                     input       output
    }
}

/*
#[macro_export]
macro_rules! nested {
    // input is empty: time to output
    (@munch () -> {$(#[$attr:meta])* struct $name:ident $(($id:ident: $ty:ty))*}) => {
 //   (@munch () -> {$(#[$attr:meta])* struct $name:ident $($ty:ty)*}) => {
        let child = match ty {
            Type::Verbatim(type_path) => type_path.clone().into_token_stream().to_string().to_case(Case::Snake),
            _ => (),
        };
        $(#[$attr])* 
        pub struct $name { 
            pub path: String, 
            $($id: $ty),*
    //        $(pub quote!{child}: $ty),* 
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
                    $(quote!{ty.into_token_stream().to_string().to_case(Case::Snake)}: <$ty>::new(path.clone()),),*
                }
            }
            pub fn path(&self) -> &str {
                &self.path
            }
        }
    };

    // branch off to generate an inner struct
    (@munch ($id:ident: $name:ident {$($inner:tt)*} $($next:tt)*) -> {$(#[$attr:meta])* struct $($output:tt)*}) => {
        nested!(@munch ($($inner)*) -> {$(#[$attr])* struct $name});
        nested!(@munch ($($next)*) -> {$(#[$attr])* struct $($output)* (quote!{name.into_token_stream().to_string().to_case(Case::Snake)}: $name)});
    };
    
    // throw on the last field
    (@munch ($ty:ty) -> {$($output:tt)*}) => {
        nested!(@munch () -> {$($output)* (quote!{ty.into_token_stream().to_string().to_case(Case::Snake)}: $ty)});
    };
    
    // throw on another field (not the last one)
    (@munch ($ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        nested!(@munch ($($next)*) -> {$($output)* (quote!{ty.into_token_stream().to_string().to_case(Case::Snake)}: $ty)});
    };
    
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
        $(#[$attr])* 
        pub struct $name { 
            pub path: String, 
            $(pub $id: $ty),* 
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
                    $($id: <$ty>::new(path.clone()),),*
                }
            }
            pub fn path(&self) -> &str {
                &self.path
            }
        }
    };

    // branch off to generate an inner struct
    (@munch ($id:ident: $name:ident {$($inner:tt)*} $($next:tt)*) -> {$(#[$attr:meta])* struct $($output:tt)*}) => {
        nested!(@munch ($($inner)*) -> {$(#[$attr])* struct $name});
        nested!(@munch ($($next)*) -> {$(#[$attr])* struct $($output)* ($id: $name)});
    };
    
    // throw on the last field
    (@munch ($ty:ty) -> {$($output:tt)*}) => {
        nested!(@munch () -> {$($output)* ($id: $ty)});
    };
    
    // throw on another field (not the last one)
    (@munch ($ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        nested!(@munch ($($next)*) -> {$($output)* ($id: $ty)});
    };
    
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
        $(#[$attr])* pub struct $name { 
            pub path: String, 
            $(pub $id: $ty),* 
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
                    $($id: <$ty>::new(path.clone()),),*
                }
            }
            pub fn path(&self) -> &str {
                &self.path
            }
        }
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
/*
#[macro_export]
macro_rules! nested {
    // input is empty: time to output
    (@munch () -> {$(#[$attr:meta])* struct $name:ident $(($id:ident: $ty:ty))*}) => {
        $(#[$attr])* pub struct $name { pub path: String, $($id: $ty),* }
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
/*
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
