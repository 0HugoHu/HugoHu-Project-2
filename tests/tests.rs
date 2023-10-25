// tests.rs

#[cfg(test)]
mod tests {
    extern crate mysql;

    #[test]
    fn test_crud_operations() {
        // Run the CRUD operations
        let result = project2::run("mysql://root:qwerty9870@localhost:3306/mydb");

        assert!(result.is_err());
    }
}
