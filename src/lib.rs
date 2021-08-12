#[macro_use]
mod expand_impl;


#[cfg(test)]
mod tests {
    macro_rules! test_expand {
        ( { $( $name:path ),* } => $body:expr => $expected:expr ) => ({
            let expanded =
            expand! { use { $( $name ),* } in $body };

            assert_eq![ expanded, $expected ];
        });
    }

    #[test]
    fn empty() {
        expand! { () }
    }

    #[test]
    fn expand_constants() {
        for original in 0..100 {
            test_expand! { {} => &original => &original }
        }
    }

    #[test]
    fn expands_names() {
        #[derive(Debug)]
        enum Foo {
            A,
            B,
        }

        test_expand! { { Foo } => format!["{:?}", A] => "A" }
        test_expand! { { Foo } => format!["{:?}", B] => "B" }
    }
}
