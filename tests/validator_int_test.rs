//! Tests for the [`Validator`] trait contract and [`BrokerSvc::validate`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_message_broker::{BrokerSvc, Validator};

#[test]
fn test_custom_validator_ok_path() {
    struct AlwaysOk;
    impl Validator for AlwaysOk {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(AlwaysOk.validate().is_ok());
}

#[test]
fn test_custom_validator_error_path() {
    struct AlwaysErr;
    impl Validator for AlwaysErr {
        fn validate(&self) -> Result<(), String> {
            Err("always invalid".into())
        }
    }
    let result = AlwaysErr.validate();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "always invalid");
}

/// @covers: validate — BrokerSvc::validate delegates to Validator::validate
#[test]
fn test_broker_svc_validate_ok_for_valid_type() {
    struct Valid;
    impl Validator for Valid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(BrokerSvc::validate(&Valid).is_ok());
}

/// @covers: validate — BrokerSvc::validate returns err for invalid type
#[test]
fn test_broker_svc_validate_err_for_invalid_type() {
    struct Invalid;
    impl Validator for Invalid {
        fn validate(&self) -> Result<(), String> {
            Err("bad state".into())
        }
    }
    let result = BrokerSvc::validate(&Invalid);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "bad state");
}
