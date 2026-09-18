use proc_macro::TokenStream;

#[proc_macro]
pub fn bitboard(input: TokenStream) -> TokenStream {
    let string = input.to_string();
    let mut chars = string.chars();
    let file = chars.next().unwrap();
    let rank = chars.next().unwrap();

    format!("crate::engine::FILE_{file} & crate::engine::RANK_{rank}")
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn square(input: TokenStream) -> TokenStream {
    let string = input.to_string();
    let mut chars = string.chars();

    let file = chars.next().unwrap();
    let file = match file {
        'A' => 0u8,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        _ => panic!("Invalid file"),
    };

    let rank = chars.next().unwrap();
    let rank = match rank {
        '1' => 0u8,
        '2' => 8,
        '3' => 16,
        '4' => 24,
        '5' => 32,
        '6' => 40,
        '7' => 48,
        '8' => 56,
        _ => panic!("Invalid rank"),
    };

    format!("{}", rank + file).parse().unwrap()
}
