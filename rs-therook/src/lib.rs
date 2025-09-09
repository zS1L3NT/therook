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
pub fn tile(input: TokenStream) -> TokenStream {
    let string = input.to_string();
    let mut chars = string.chars();
    let file = chars.next().unwrap();
    let rank = chars.next().unwrap();

    format!("crate::engine::Tile::try_from(crate::engine::FILE_{file} & crate::engine::RANK_{rank}).unwrap()")
        .parse()
        .unwrap()
}
