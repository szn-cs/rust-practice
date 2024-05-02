trait Trait {
    type Item;

    fn dummy(self);
}

#[cfg(test)]
mod tests {
    use super::Trait as TTT;

    #[test]
    fn generic_associated_trait() {
        struct Struct<T> {
            field: T,
        }

        impl<T> TTT for Struct<T> {
            type Item = T;

            fn dummy(self) {
                println!("{}", 10);
            }
        }

        let i = Struct { field: 10 };
        i.dummy();
    }
}
