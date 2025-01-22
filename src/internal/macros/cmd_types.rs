#[macro_export]
macro_rules! cmd_types {
    ($($key: path),*) => {
        {
            use ::std::collections::HashSet;
            let mut hs = HashSet::<&str>::new();


            $ (
                hs.insert($key);
            )*

            hs
       }
    };
}
