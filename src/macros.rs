#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

#[macro_export]
macro_rules! crate_description {
    () => {
        env!("CARGO_PKG_DESCRIPTION")
    };
}

#[macro_export]
macro_rules! crate_authors {
    ($sep:expr) => {{
        static CACHED: clap::__macro_refs::once_cell::sync::Lazy<String> =
            clap::__macro_refs::once_cell::sync::Lazy::new(|| {
                env!("CARGO_PKG_AUTHORS").replace(':', $sep)
            });

        let s: &'static str = &*CACHED;
        s
    }};
    () => {
        env!("CARGO_PKG_AUTHORS")
    };
}

#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}
