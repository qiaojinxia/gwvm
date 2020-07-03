#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            let s = format!($($arg)*);
            println!("{}", ansi_term::Colour::White.dimmed().paint(s));
        }
    }
}

#[macro_export]
macro_rules! when_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $($arg)*;
        }
    };
}
#[macro_export]
macro_rules! try_eq {
    ($expr:expr) => {{
        if !$expr {
            return None;
        }
    }};
}
//判断是否相等 宏
#[macro_export]
macro_rules! try_eq {
    ($expr:expr) => {{
        if !$expr {
            return None;
        }
    }};
}