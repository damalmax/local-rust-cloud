pub(crate) fn is_valid_input(input: &str, valid_characters: &[char]) -> bool {
    input.chars().into_iter().all(|ch| valid_characters.contains(&ch))
}
