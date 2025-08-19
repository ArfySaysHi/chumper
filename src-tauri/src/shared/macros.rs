#[macro_export]
macro_rules! builder_setters {
    ($($field:ident: $type:ty),* $(,)?) => {
        $(
            pub fn $field<T: Into<$type>>(mut self, $field: T) -> Self {
                self.$field = $field.into();
                self
            }
        )*
    };
}
