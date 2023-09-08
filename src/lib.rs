use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[wasm_bindgen]
pub fn sub(left: i32, right: i32) -> i32 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
