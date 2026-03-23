#[macro_export]
macro_rules! with_builder {
    (
        $vis:vis struct $name:ident {
            $($fvis:vis $key:ident : $(#$kind:meta)? $value:path,)*
        }
    ) => { paste::paste! {
        $vis struct $name {
            $($fvis $key : $crate::with_builder!(@repr $($kind)? $value),)*
        }

        struct [<$name Builder>] {
            $($key: Option<$value>,)*
        }

        impl [<$name Builder>] {
            fn new() -> Self {
                Self {$( $key: None, )*}
            }

            $(
                fn [<set_ $key:lower>](&mut self, value: $value) -> &mut Self {
                    self.$key = self.$key.replace(value);
                    self
                }
            )*

            fn finalize<E: snafu::FromString>(self) -> Result<$name, E> {
                Ok($name {$(
                    $key: $crate::with_builder!(@finalize $($kind)? self.$key),
                )*})
            }
        }

    }};

    (@repr required $value:ty) => { $value };
    (@repr $value:ty) => { Option<$value> };

    (@finalize required $self:ident.$key:ident) => {
        $self.$key.whatever_context(
            &format!("Required key {} not found",
            stringify!($key))
        )?
    };

    (@finalize $self:ident.$key:ident) => { $self.$key };
}

// if we ever decide to make this a common utility
// implement a way to have exhaustive enums
#[macro_export]
macro_rules! strenum {
    (
        $(#[$($attributes:tt)*])?
        $vis:vis $name:ident {
            $($field:ident $(= $value:literal)?,)*
        }
    ) => {
        $(#[$($attributes)*])?
        $vis enum $name {
            $($field,)*
            Other(String)
        }

        impl std::str::FromStr for $name {
            type Err = std::convert::Infallible;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $(
                        $crate::strenum!(@field_string $field $($value)?)
                        => Self::$field,
                    )*
                    other => Self::Other(other.to_string()),
                })
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$field => f.write_str(
                        $crate::strenum!(@field_string $field $($value)?)
                    ),)*
                    Self::Other(other) => f.write_str(&other),
                }
            }
        }
    };

    (@field_string $field:ident $value:literal) => { $value };
    (@field_string $field:ident) => { paste::paste! { stringify!([<$field:lower>]) } };
}
