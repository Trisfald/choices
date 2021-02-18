use choices::Choices;

macro_rules! scalar_config {
    ($($items:meta),*) => {
        $(#[$items])*
        #[derive(Default)]
        pub struct ScalarConfig {
            b: bool,
            c: char,
            int128: i128,
            int16: i16,
            int32: i32,
            int64: i64,
            int8: i8,
            intsize: isize,
            uint128: u128,
            uint16: u16,
            uint32: u32,
            uint64: u64,
            uint8: u8,
            uintsize: usize,
            float: f32,
            double: f64,
        }

        impl ScalarConfig {
            pub fn new() -> Self {
                Self {
                    b: true,
                    c: 'a',
                    int128: -1,
                    int16: -2,
                    int32: -3,
                    int64: -4,
                    int8: -5,
                    intsize: -6,
                    uint128: 1,
                    uint16: 2,
                    uint32: 3,
                    uint64: 4,
                    uint8: 5,
                    uintsize: 6,
                    float: 5.5,
                    double: 3.2,
                }
            }
        }
    };
}

pub mod text {
    use super::*;

    scalar_config! { derive(Choices) }

    #[derive(Choices)]
    pub struct SimpleBoolConfig {
        pub debug: bool,
    }

    #[derive(Choices, Default)]
    pub struct StringConfig {
        pub string: String,
    }

    #[derive(Choices, Default)]
    pub struct OptionConfig {
        pub character: Option<char>,
        pub empty: Option<bool>,
    }
}

pub mod json {
    use super::*;

    scalar_config! { derive(Choices), choices(json) }

    #[derive(Choices, Default)]
    #[choices(json)]
    pub struct StringConfig {
        pub string: String,
    }

    #[derive(Choices, Default)]
    #[choices(json)]
    pub struct OptionConfig {
        pub character: Option<char>,
        pub empty: Option<bool>,
    }

    #[derive(Choices, Default)]
    #[choices(json)]
    pub struct VecConfig {
        pub vector: Vec<u8>,
    }

    impl VecConfig {
        pub fn new() -> Self {
            Self {
                vector: vec![1, 2, 3],
            }
        }
    }
}
