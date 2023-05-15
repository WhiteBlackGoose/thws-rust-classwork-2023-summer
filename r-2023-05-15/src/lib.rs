mod students;

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn no_valid_file() {
        super::students::read_students("aaa");
    }
}
