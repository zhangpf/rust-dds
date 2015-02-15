
macro_rules! lazy_static {
	(static mut ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        lazy_static!(PRIV static MUT ref $N : $T = $e; $($t)*);
    };
    (pub static mut ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        lazy_static!(PUB static MUT ref $N : $T = $e; $($t)*);
    };
    (static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        lazy_static!(PRIV static IMMUT ref $N : $T = $e; $($t)*);
    };
    (pub static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        lazy_static!(PUB static IMMUT ref $N : $T = $e; $($t)*);
    };
    ($VIS:ident static ref $MUT:ident $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        lazy_static!(MAKE TY $VIS $MUT $N);
        impl ::std::ops::Deref for $N {
            type Target = $T;
            fn deref<'a>(&'a self) -> &'a $T {
                use std::sync::{Once, ONCE_INIT};
                use std::mem::transmute;

                #[inline(always)]
                fn require_sync<T: Sync>(_: &T) { }

                unsafe {
                    static mut __static: *const $T = 0 as *const $T;
                    static mut __ONCE: Once = ONCE_INIT;
                    __ONCE.call_once(|| {
                        __static = transmute::<Box<$T>, *const $T>(box() ($e));
                    });
                    let static_ref = &*__static;
                    require_sync(static_ref);
                    static_ref
                }
            }
        }
        lazy_static!($($t)*);
    };
    (MAKE TY PUB IMMUT $N:ident) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct $N {__private_field: ()}
        pub static $N: $N = $N {__private_field: ()};
    };
    (MAKE TY PUB MUT $N:ident) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct $N {__private_field: ()}
        pub static mut $N: $N = $N {__private_field: ()};
    };
    (MAKE TY PRIV IMMUT $N:ident) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct $N {__private_field: ()}
        static $N: $N = $N {__private_field: ()};
    };
    (MAKE TY PRIV MUT $N:ident) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct $N {__private_field: ()}
        static mut $N: $N = $N {__private_field: ()};
    };
    () => ()
}