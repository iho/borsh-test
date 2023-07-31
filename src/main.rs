use borsh_derive::{BorshDeserialize, BorshSerialize};
#[derive(BorshSerialize, BorshDeserialize, Debug)]
// #[use_discriminant = false]
enum UnitEnum {
    A,
    B,
    C,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct A {
    a: u8,
    b: i32,
    c: u64,
    d: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct B {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
#[repr(u8)]
#[borsh(use_discriminant = false)]
enum TupleEnum {
    A,
    B,
    Tuple(A, B),
    C = 11u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
#[repr(u8)]
#[borsh(use_discriminant = false)]
enum StructEnum {
    A,
    Struct { a: u8, b: u8, c: u8, d: u8 },
    C = 20,
}

// vec![A::Unit, A::Tuple(...), A::Struct{..}]

fn main() {
    print!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use borsh::{from_slice, BorshSerialize};
    #[test]
    fn test_unit_enum() {
        let values = vec![UnitEnum::A, UnitEnum::B, UnitEnum::C];

        for value in values.iter() {
            let encoded = value.try_to_vec().unwrap();
            println!("encoded: {:?}", encoded);
            let decoded = from_slice::<UnitEnum>(&encoded).unwrap();
            insta::assert_debug_snapshot!(decoded);
            insta::assert_debug_snapshot!(encoded);
        }
    }
    #[test]
    fn test_tuple_enum() {
        let values = vec![
            TupleEnum::A,
            TupleEnum::B,
            TupleEnum::Tuple(
                A {
                    a: 1,
                    b: 2,
                    c: 3,
                    d: "hello".to_string(),
                },
                B {
                    a: 1,
                    b: 2,
                    c: 3,
                    d: 4,
                },
            ),
            TupleEnum::C,
        ];
        for value in values.iter() {
            let encoded = value.try_to_vec().unwrap();
            println!("encoded: {:?}", encoded);
            let decoded = from_slice::<TupleEnum>(&encoded).unwrap();
            insta::assert_debug_snapshot!(decoded);
            insta::assert_debug_snapshot!(encoded);
        }
    }
    #[test]
    fn test_struct_enum() {
        let values = vec![
            StructEnum::A,
            StructEnum::Struct {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
            },
            StructEnum::C,
        ];
        for value in values.iter() {
            let encoded = value.try_to_vec().unwrap();
            println!("encoded: {:?}", encoded);
            let decoded = from_slice::<StructEnum>(&encoded).unwrap();
            insta::assert_debug_snapshot!(decoded);
            insta::assert_debug_snapshot!(encoded);
        }
    }
}
