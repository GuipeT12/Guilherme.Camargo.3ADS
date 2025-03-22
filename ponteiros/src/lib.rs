pub fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;

    for i in 0..len {
        unsafe {
            product *= *ptr.add(i); // Agora estamos desreferenciando corretamente
        }
    }

    product
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24); // Verifica se o produto está correto
    }
}

fn main() {
    println!("Hello, world!");
}