#[macro_export]
macro_rules! pr {
    ( $( $x:expr ),* ) => {
        {
            $(
                print!("{} ", $x);
            )*
            println!();
        }
    };
}
