    #[test]
    fn test_zero_hour_format() {
        // Test that "0" is valid for both HH and H formats
        assert!(validate_time_input("0").is_ok(), "Single '0' should be valid for HH format");
        assert!(validate_time_input("0").is_ok(), "Single '0' should be valid for H format");
    }