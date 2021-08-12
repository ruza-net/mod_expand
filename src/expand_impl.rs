/// Accepts either `$expr`, or `use { $path, $path, ... } in $expr`.
/// Expands the namespaces of `$path`s inside `$expr`.
#[macro_export]
macro_rules! expand {
    ( $( use { $( $name:path ),* } in )? $val:expr ) => ({
        $($(
            use $name::*;
        )*)?

        $val
    });
}
