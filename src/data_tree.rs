use crate::nested;

// Build tree
nested!{
    #[derive(Debug)]
    Parent {
        Child {
        }
    }
}