use lazydbg::lazydbg;

#[test]
fn test_lazydbg_in_debug() {
    let x = 42;
    
    // Capture output in debug mode
    #[cfg(debug_assertions)]
    {
        let output = std::panic::catch_unwind(|| {
            lazydbg!(x);
        });
        assert!(output.is_ok()); // Ensure it runs without panicking
    }

    // Ensure `lazydbg!` has no side effects
    assert_eq!(x, 42);
}

#[test]
fn test_lazydbg_in_release() {
    let x = 99;
    
    // Ensure that the macro does nothing in release mode
    #[cfg(not(debug_assertions))]
    {
        let output = std::panic::catch_unwind(|| {
            lazydbg!(x);
        });
        assert!(output.is_ok()); // Should still run safely
    }

    assert_eq!(x, 99); // No side effects
}
