#[derive(Debug, PartialEq)]
#[repr(C)]
struct Cat<'a> {
    kawaii: bool,
    name: &'a str,
    age: u8,
}
#[derive(Debug, PartialEq)]
#[repr(C)]
struct Dog<'a> {
    dangerous: bool,
    last_name: &'a str,
    weight: u8,
}
#[derive(Debug, PartialEq)]
#[repr(C)]
struct Human<'a> {
    vegan: bool,
    first_name: &'a str,
    nationality: [u8; 8],
}

trait Transmutable {
    type A;
    type B;
    fn transmute_a(self) -> Self::A;
    fn transmute_b(self) -> Self::B;
}

impl<'a> Transmutable for Dog<'a> {
    type A = Cat<'a>;

    type B = Human<'a>;

    fn transmute_a(self) -> Self::A {
        unsafe { std::mem::transmute(self) }
    }

    fn transmute_b(self) -> Self::B {
        unsafe { std::mem::transmute(self) }
    }
}
impl<'a> Transmutable for Cat<'a> {
    type A = Dog<'a>;

    type B = Human<'a>;

    fn transmute_a(self) -> Self::A {
        unsafe { std::mem::transmute(self) }
    }

    fn transmute_b(self) -> Self::B {
        unsafe { std::mem::transmute(self) }
    }
}
impl<'a> Transmutable for Human<'a> {
    type A = Dog<'a>;

    type B = Cat<'a>;

    fn transmute_a(self) -> Self::A {
        unsafe { std::mem::transmute(self) }
    }

    fn transmute_b(self) -> Self::B {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(test)]
mod test {
    use crate::dog_becomes_a_cat::{Human, Transmutable};

    use super::{Cat, Dog};

    #[test]
    fn dog_to_cat_to_human() {
        let dog = Dog {
            dangerous: true,
            last_name: "Snoop Doggy",
            weight: 25,
        };
        let cat = dog.transmute_a();
        assert_eq!(
            &Cat {
                kawaii: true,
                name: "Snoop Doggy",
                age: 25
            },
            &cat
        );
        let human = cat.transmute_b();
        assert_eq!(
            &Human {
                vegan: true,
                first_name: "Snoop Doggy",
                nationality: [25, 0, 0, 0, 0, 0, 0, 0],
            },
            &human
        );
    }
    #[test]
    fn human_to_cat_to_dog() {
        let flag_be: [u8; 8] = [0xf0, 0x9f, 0x87, 0xa7, 0xf0, 0x9f, 0x87, 0xaa];
        let human = Human {
            vegan: true,
            first_name: "Eden Hazard",
            nationality: flag_be,
        };
        println!(
            "{} from {}",
            &human.first_name,
            std::str::from_utf8(&human.nationality).unwrap()
        );

        let cat = human.transmute_b();
        assert_eq!(
            &Cat {
                kawaii: true,
                name: "Eden Hazard",
                age: 240
            },
            &cat
        );
        let dog = cat.transmute_a();
        assert_eq!(
            &Dog {
                dangerous: true,
                last_name: "Eden Hazard",
                weight: 240
            },
            &dog
        );
    }
}
