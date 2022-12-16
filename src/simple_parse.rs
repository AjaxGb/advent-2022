/// Extract values from a string with a fixed pattern.
#[macro_export]
macro_rules! simple_parse {
    ($s:expr $(, $parsed:expr)* => $full:expr $(,)?) => {
        if $s == $full {
            Some((
                $($parsed,)*
            ))
        } else {
            None
        }
    };
    ($s:expr $(, $parsed:expr)* => $prefix:expr $(, $($rest:tt)*)?) => {
        if let Some(s) = $s.strip_prefix($prefix) {
            simple_parse!(s $(, $parsed)* => $($($rest)*)?)
        } else {
            None
        }
    };
    ($s:expr $(, $parsed:expr)* => @ $($type:ty)?, $suffix:expr $(,)?) => {
        if let Some(val) = $s.strip_suffix($suffix) {
            if let Ok(val) = val.parse$(::<$type>)?() {
                Some((
                    $($parsed,)*
                    val,
                ))
            } else {
                None
            }
        } else {
            None
        }
    };
    ($s:expr $(, $parsed:expr)* => @ $($type:ty)?, $infix:expr, $($rest:tt)+) => {
        if let Some((val, s)) = $s.split_once($infix) {
            if let Ok(val) = val.parse$(::<$type>)?() {
                simple_parse!(s $(, $parsed)*, val => $($rest)+)
            } else {
                None
            }
        } else {
            None
        }
    };
    ($s:expr $(, $parsed:expr)* => @ $($type:ty)? $(,)?) => {
        if let Ok(val) = $s.parse$(::<$type>)?() {
            Some((
                $($parsed,)*
                val,
            ))
        } else {
            None
        }
    };
    ($s:expr $(, $parsed:expr)* => $(,)?) => {
        if $s.is_empty() {
            Some((
                $($parsed,)*
            ))
        } else {
            None
        }
    };
}
