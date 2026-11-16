

pub trait Different<Other> {}

pub trait NotContains<X> {}
impl<X> NotContains<X> for () {}
impl<X, H, T> NotContains<X> for (H, T)
where
    H: Different<X>,
    T: NotContains<X>,
{
}

pub trait NonEmpty {}
impl<H, T> NonEmpty for (H, T) {}

#[macro_export]
macro_rules! define_email_builder {
    (
        $(
            $variant:ident => $marker:ident => $data:ty => $method:ident
        ),* $(,)?
    ) => {
        $( #[derive(Debug)] pub struct $marker; )*

        define_email_builder!(@different [$($marker),*]);

        pub struct EmailSenderConfigBuilder<State = ()> {
            enabled_services: Vec<EnabledService>,
            _state: PhantomData<State>,
        }

        impl EmailSenderConfigBuilder<()> {
            pub fn new() -> Self {
                Self { enabled_services: Vec::new(), _state: PhantomData }
            }
        }
        impl Default for EmailSenderConfigBuilder<()> {
    fn default() -> Self {
        Self::new()
    }
}

        impl<State> EmailSenderConfigBuilder<State> {
            $(
                pub fn $method(self, data: $data)
                    -> EmailSenderConfigBuilder<($marker, State)>
                where
                    State: $crate::NotContains<$marker>,
                {
                    let mut svcs = self.enabled_services;
                    svcs.push(EnabledService::$variant(data));
                    EmailSenderConfigBuilder { enabled_services: svcs, _state: PhantomData }
                }
            )*

            pub fn build(self) -> EmailSenderConfig
            where
                State: $crate::NonEmpty,
            {
                EmailSenderConfig { enabled_services: self.enabled_services }
            }
        }
    };


    (@different [$head:ident $(, $rest:ident)*]) => {
        $( define_email_builder!(@pair $head, $rest); )*
        define_email_builder!(@different [$($rest),*]);
    };
    (@different []) => {};
    (@pair $a:ident, $b:ident) => {
        impl $crate::Different<$b> for $a {}
        impl $crate::Different<$a> for $b {}
    };
}
