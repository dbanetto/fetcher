#[cfg(not(feature = "serde_macros"))]
mod inner {
    extern crate syntex;
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        for (src, dst) in vec![
            ("src/fetcher/clients/mod.rs.in", "client.mod.rs"),
            ("src/fetcher/settings.rs.in", "settings.rs")
        ] {
            let src = Path::new(src);
            let dst = Path::new(&out_dir).join(dst);

            let mut registry = syntex::Registry::new();

            serde_codegen::register(&mut registry);
            registry.expand("", &src, &dst).unwrap();
        }
    }
}

#[cfg(feature = "serde_macros")]
mod inner {
    pub fn main() {}
}

fn main() {
    inner::main();
}
