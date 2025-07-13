use std::env;

#[derive(Debug)]
enum ESize {
    B(i32),
    KB(i32),
    MB(i32),
    GB(i32),
}

struct Size {
    base_size: ESize,
}

impl Size {
    fn new(original: String) -> Size {
        let splitted: Vec<&str> = original.split_whitespace().collect();
        let value: i32 = splitted[0].parse().expect("Invalid number");

        let base_size = match splitted[1] {
            "kb" => ESize::KB(value),
            "mb" => ESize::MB(value),
            "gb" => ESize::GB(value),
            _ => ESize::B(value)
        };

        Size {
            base_size,
        }
    }

    fn to_bytes(&self) {
        let value: i32 = match self.base_size {
            ESize::B(value) => value,
            ESize::KB(value) => value * 1024,
            ESize::MB(value) => value  * 1024 * 1024,
            ESize::GB(value) => value * 1024 * 1024 * 1024
        };
        println!("The value in bytes: {}", value);
    }

    fn to_kilo_bytes(&self) {
        let value: i32 = match self.base_size {
            ESize::B(value) => value / 1024,
            ESize::KB(value) => value,
            ESize::MB(value) => value * 1024,
            ESize::GB(value) => value * 1024 * 1024
        };
        println!("The value in KiloBytes: {}", value);
    }
    fn to_mega_bytes(&self) {
        let value: i32 = match self.base_size {
            ESize::B(value) => value / 1024 / 1024,
            ESize::KB(value) => value / 1024,
            ESize::MB(value) => value,
            ESize::GB(value) => value * 1024
        };
        println!("The value in MegaBytes: {}", value);
    }

    fn to_giga_bytes(&self) {
        let value: i32 = match self.base_size {
            ESize::B(value) => value / 1024 / 1024 / 1024,
            ESize::KB(value) => value / 1024 / 1024,
            ESize::MB(value) => value / 1024 ,
            ESize::GB(value) => value
        };
        println!("The value in GigaBytes: {}", value);
    }

    fn show_sizes(&self) {
        println!("The size: {:?}", self.base_size);
        self.to_bytes();
        self.to_kilo_bytes();
        self.to_mega_bytes();
        self.to_giga_bytes();
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    let size: String = match args.get(1) {
        Some(arg) => {
            arg.clone()
        },
        None => panic!("Required argument")
    };

    validate_args(&size);

    let size_struct: Size = Size::new(size);
    size_struct.show_sizes();
}

fn validate_args(s: &String) {
    let vec_args: Vec<&str> = s.split_whitespace().collect();
    if vec_args.len() != 2 {
        panic!("You shoul include the args like <value metric>");
    }
}