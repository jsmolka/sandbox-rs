use std::{ops::Deref, rc::Rc};

#[derive(Default, Debug, PartialEq)]
struct Wrapper {
    value: usize,
}

impl Wrapper {
    fn new() -> Self {
        Self::default()
    }
}

impl Deref for Wrapper {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

enum Enum {
    A { name: String },
    B { name: String },
}

impl Enum {
    fn switch(&mut self) {
        *self = match self {
            Enum::A { name } => Enum::B {
                name: std::mem::take(name),
            },
            Enum::B { name } => Enum::A {
                name: std::mem::take(name),
            },
        };
    }
}

fn main() {
    // Coerce
    {
        fn coerce(string: &str) -> usize {
            string.len()
        }

        coerce("test");
        let string = "test".to_owned();
        coerce(&string);
    }
    {
        fn coerce(slice: &[i32]) -> usize {
            slice.len()
        }

        let vector = vec![1, 2, 3];
        coerce(&vector);
    }
    {
        fn coerce(wrapper: &Wrapper) -> usize {
            wrapper.value
        }

        let a = Wrapper::default();
        let b = Box::new(Wrapper::default());
        coerce(&a);
        coerce(&b);
    }
    {
        fn coerce(value: &usize) -> &usize {
            value
        }

        let a = Wrapper::default();
        let b = Box::new(Wrapper::default());
        coerce(&a);
        coerce(&b);
    }

    // Constructor
    {
        let a = Wrapper::new();
        let b = Wrapper::default();
        assert_eq!(a, b);
    }

    // Finalization
    {
        fn implicit() -> Result<(), String> {
            Err("implicit".into())
        }

        fn run() -> Result<(), String> {
            struct Guard;

            impl Drop for Guard {
                fn drop(&mut self) {
                    println!("exit");
                }
            }

            let _exit = Guard;

            implicit()?;

            Ok(())
        }

        let _ = run();
    }

    // Memmory
    {
        let mut a = Enum::A { name: "a".into() };
        a.switch();

        assert!(match a {
            Enum::A { .. } => false,
            Enum::B { name } => name == "a",
        })
    }

    // Option iterator
    {
        let maybe = Some(4);
        let maybe_not: Option<i32> = None;
        let mut values = vec![1, 2, 3];
        values.extend(maybe);
        values.extend(maybe_not);
        assert_eq!(values, [1, 2, 3, 4]);
    }

    // Selective capture
    {
        let a = Rc::new(1);
        let b = Rc::new(2);
        let c = Rc::new(3);
        let closure = {
            // Move a
            let b = b.as_ref();
            let c = c.as_ref();
            move || *a + *b + *c
        };
        assert_eq!(closure(), 6);
    }

    // Temporary mutability
    {
        fn make() -> Vec<i32> {
            vec![1, 3, 2]
        }

        let data = {
            let mut data = make();
            data.sort();
            data
        };
        assert_eq!(data, [1, 2, 3]);
    }
}
