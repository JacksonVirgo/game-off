#[macro_export]
macro_rules! plugin {
    ($name:ident, |$app_ident:ident| $body:block) => {
        pub struct $name;

        impl bevy::prelude::Plugin for $name {
            fn build(&self, app: &mut bevy::prelude::App) {
                let $app_ident = app;
                $body
            }
        }
    };
}
