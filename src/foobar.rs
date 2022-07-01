pub fn foobar() {
    println!("foobar");
}

pub fn add(x: i32, y: i32) -> i32{
    let result = x + y;
    result
}

#[cfg(test)]
mod tests {
    use crate::foobar::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
