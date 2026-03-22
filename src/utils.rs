#[macro_export]
macro_rules! with_builder {
    (
        $vis:vis struct $name:ident {
            $($fvis:vis $key:ident : #$kind:meta $value:path,)*
        }
    ) => { paste::paste! {
        $vis struct $name {
            $($fvis $key : $crate::with_builder!(@repr $kind $value),)*
        }

        struct [<$name Builder>] {
            $($key: Option<$value>,)*
        }

        impl [<$name Builder>] {
            fn new() -> Self {
                Self {$(
                    $key: None,
                )*}
            }

            $(
                fn [<set_ $key:lower>](&mut self, value: $value) -> &mut Self {
                    self.$key = self.$key.replace(value);
                    self
                }
            )*

            fn finalize<E: snafu::FromString>(self) -> Result<$name, E> {
                Ok($name {$(
                    $key: $crate::with_builder!(@finalize $kind self.$key),
                )*})
            }
        }

    }};

    (@repr required $value:ty) => { $value };
    (@repr optional $value:ty) => { Option<$value> };

    (@finalize required $self:ident.$key:ident) => {
        $self.$key.whatever_context(
            &format!("Required key {} not found",
            stringify!($key))
        )?
    };

    (@finalize optional $self:ident.$key:ident) => { $self.$key };

}
