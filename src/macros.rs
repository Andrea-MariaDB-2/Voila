pub macro println_on_debug {
    ($($args:tt)*) => {
        if std::env::var("DEBUG").unwrap_or_else(|_| { "false".to_string() }) == "true" ||
           env!("CARGO_PKG_VERSION").ends_with("dev") {
               eprintln!("DEBUG-MSG [{}:{}] {}\n", file!(), line!(), format!{ $($args)* })
        }
    }
}
