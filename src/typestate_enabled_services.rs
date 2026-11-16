#[derive(Debug)]
pub enum BuildError {
    NoServicesEnabled,
}
impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoServicesEnabled => f.write_str("no email services enabled at runtime"),
        }
    }
}
impl std::error::Error for BuildError {}


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
            $variant:ident => $marker:ident => $data:ty => $method:ident => $method_opt:ident
        ),* $(,)?
    ) => {
        $( #[derive(Debug)] pub struct $marker; )*
        $crate::define_email_builder!(@different [$($marker),*]);

        pub struct EmailSenderConfigBuilder<State = ()> {
            enabled_services: Vec<$crate::EnabledService>,
            _state: ::core::marker::PhantomData<State>,
        }

        impl EmailSenderConfigBuilder<()> {
            pub fn new() -> Self {
                Self { enabled_services: Vec::new(), _state: ::core::marker::PhantomData }
            }
        }
        impl ::core::default::Default for EmailSenderConfigBuilder<()> {
            fn default() -> Self { Self::new() }
        }

        impl<State> EmailSenderConfigBuilder<State> {
            $(
                // Unconditional — same as before.
                pub fn $method(self, data: $data)
                    -> $crate::EmailSenderConfigBuilder<($marker, State)>
                where State: $crate::NotContains<$marker>
                {
                    let mut svcs = self.enabled_services;
                    svcs.push($crate::EnabledService::$variant(data));
                    $crate::EmailSenderConfigBuilder { enabled_services: svcs, _state: ::core::marker::PhantomData }
                }

                // Conditional — marker still advances, data pushed only if Some.
                pub fn $method_opt(self, data: Option<$data>)
                    -> $crate::EmailSenderConfigBuilder<($marker, State)>
                where State: $crate::NotContains<$marker>
                {
                    let mut svcs = self.enabled_services;
                    if let Some(data) = data {
                        svcs.push($crate::EnabledService::$variant(data));
                    }
                    $crate::EmailSenderConfigBuilder { enabled_services: svcs, _state: ::core::marker::PhantomData }
                }
            )*

            pub fn build(self) -> Result<$crate::EmailSenderConfig, $crate::BuildError>
            where State: $crate::NonEmpty,
            {
                if self.enabled_services.is_empty() {
                    return Err($crate::BuildError::NoServicesEnabled);
                }
                Ok($crate::EmailSenderConfig { enabled_services: self.enabled_services })
            }
        }
    };

    (@different [$head:ident $(, $rest:ident)*]) => {
        $( $crate::define_email_builder!(@pair $head, $rest); )*
        $crate::define_email_builder!(@different [$($rest),*]);
    };
    (@different []) => {};
    (@pair $a:ident, $b:ident) => {
        impl $crate::Different<$b> for $a {}
        impl $crate::Different<$a> for $b {}
    };
}