#[cfg(test)]
mod tests {
    trait Foo {
        async fn foo(&self) -> i32;
    }

    struct Bar;

    impl Foo for Bar {
        async fn foo(&self) -> i32 {
            42
        }
    }

    #[tokio::test]
    async fn test_foo() {
        let bar = Bar;
        assert_eq!(42, bar.foo().await);
    }

    // Test async without tokio runtime
    #[test]
    fn test_foo_manual() {
        // create runtime
        let rt = tokio::runtime::Runtime::new().unwrap();

        // run async function
        let result = rt.block_on(async {
            let bar = Bar;
            bar.foo().await
        });

        assert_eq!(42, result);
    }
}
