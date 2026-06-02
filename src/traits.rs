mod traits {
    trait GetMut {
        fn get_mut(&mut self, field: &str) -> Option<&mut isize>;
    }

    macro_rules! impl_get_mut {
    ($struct:ty, { $($field:ident),* }, custom: [$($vec_field:ident),*]) => {
        impl GetMut for $struct {
            fn get_mut(&mut self, field: &str) -> Option<&mut isize> {
                match field {
                    $(stringify!($field) => Some(&mut self.$field),)*
                    _ => {
                        $(
                            if let Some(s) = self.$vec_field.iter_mut().find(|s| s.name == field) {
                                return Some(&mut s.value);
                            }
                        )*
                        None
                    }
                }
            }
        }
    };
}
}
