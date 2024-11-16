

#[cfg(test)]
mod tests {
    use whitespace::bs::Bs;

    #[test]
    fn test_main() -> () {
        let b = Bs::new("111");
        dbg!(b.post_sync("33333","333").unwrap());
    }
}
