#[macro_export]
macro_rules! start_profiler {
    () => {{
        unsafe { PROFILER.start() }
    }};
}

#[macro_export]
macro_rules! end_profiler {
    () => {{
        unsafe { PROFILER.end() }
    }};
}

#[macro_export]
macro_rules! get_profiler_metrics {
    () => {{
        unsafe { PROFILER.get_output() }
    }};
}

#[cfg(feature = "disable")]
#[macro_export]
macro_rules! profile {
    ($slot:expr, $block:block) => {{
        $block
    }};
}

#[cfg(not(feature = "disable"))]
#[macro_export]
macro_rules! profile {
    ($slot:expr, $block:block) => {{
        let _scope = ProfileScope::new($slot, $slot, unsafe { &mut PROFILER });
        let output = { $block };
        output
    }};
}

#[macro_export]
macro_rules! define_slots {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug)]
        pub enum $name {
            $($variant),+
        }

        impl Into<&'static str> for $name {
            fn into(self) -> &'static str {
                match self {
                    $(
                        Self::$variant => stringify!($variant),
                    )+
                }
            }
        }

        impl Into<usize> for $name {
            fn into(self) -> usize {
               self as usize + 1
            }
        }

        impl TryFrom<usize> for $name {
            type Error = &'static str;
            fn try_from(value: usize) -> Result<Self, Self::Error> {
                let value = value + 1; // keep zero as sentinel
                if value == 999999999 {
                    unreachable!()
                }
                $(
                    else if Self::$variant as usize == value {
                        Ok(Self::$variant)
                    }
                )+
                else {
                    Err("No variant for this value")
                }
            }
        }
    }
}
