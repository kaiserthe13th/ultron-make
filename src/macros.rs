#[macro_export]
macro_rules! map {
    ( $($k:expr => $v:expr),* ) => {
        {
            use std::collections::HashMap;
            
            let mut map = HashMap::new();
        
            $(map.insert($k, $v);)*
            
            map
        }
    }
}
