use std::ops::RangeBounds;

use nwest_shared_component_library::{AttributeError, IntegerAttribute};

#[test]
fn test_with_min_and_max() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    assert_eq!(attribute.min, 0);
    assert_eq!(attribute.max, 100);
    assert_eq!(attribute.current, 100);
}

#[test]
fn test_set_value() {
    let mut attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    attribute.set_value(50);
    assert_eq!(attribute.current, 50);
}

#[test]
fn test_current_value() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    assert_eq!(attribute.current_value(), 100);
}

#[test]
fn test_current_percentage() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    assert!((attribute.current_percentage() - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_set_max() {
    let mut attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    attribute.set_max(200).expect("Failed to set max");
    assert_eq!(attribute.max, 200);
    assert_eq!(attribute.current, 100);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_set_max_error() {
    let mut attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let result = attribute.set_max(-10);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), AttributeError::MaxLessThanMin(-10, 0));
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_set_min() {
    let mut attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    attribute.set_min(-50).unwrap();
    assert_eq!(attribute.min, -50);
    assert_eq!(attribute.current, 100);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_set_min_error() {
    let mut attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let result = attribute.set_min(200);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        AttributeError::MinGreaterThanMax(200, 100)
    );
}

#[test]
fn test_eq() {
    let attribute1 =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let attribute2 =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    assert_eq!(attribute1, attribute2);
}

#[test]
fn test_eq_i32() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    assert_eq!(attribute, 100);
}

#[test]
fn test_eq_integer_attribute() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    assert_eq!(100, attribute);
}

#[test]
fn test_display() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    assert_eq!(format!("{attribute}"), "100 (1.00%)");
}

#[test]
fn test_from() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let value: i32 = attribute.into();
    assert_eq!(value, 100);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_try_from() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let value: Result<u32, AttributeError> = TryFrom::try_from(attribute);
    assert_eq!(value.unwrap(), 100);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_try_from_error() {
    let attribute =
        IntegerAttribute::with_min_and_max(-100, -1).expect("Failed to create IntegerAttribute");
    let value: Result<u32, AttributeError> = TryFrom::try_from(attribute);
    assert!(value.is_err());
    assert_eq!(
        value.unwrap_err(),
        AttributeError::ConversionError(
            "Current value is negative when trying to convert to u32.".to_string()
        )
    );
}

#[test]
fn test_add() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let result = attribute + 50;
    assert_eq!(result.current, 100);
}

#[test]
fn test_add_assign() {
    let mut attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    attribute += 50;
    assert_eq!(attribute.current, 100);
}

#[test]
fn test_sub() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let result = attribute - 50;
    assert_eq!(result.current, 50);
}

#[test]
fn test_sub_assign() {
    let mut attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    attribute -= 50;
    assert_eq!(attribute.current, 50);
}

#[test]
fn test_mul() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let result = attribute * 2;
    assert_eq!(result.current, 100);
}

#[test]
fn test_mul_assign() {
    let mut attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    attribute *= 2;
    assert_eq!(attribute.current, 100);
}

#[test]
fn test_div() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let result = attribute / 2;
    assert_eq!(result.current, 50);
}

#[test]
fn test_div_assign() {
    let mut attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    attribute /= 2;
    assert_eq!(attribute.current, 50);
}

#[test]
fn test_neg() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let result = -attribute;
    assert_eq!(result.current, 0);
}

#[test]
fn test_rem() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let result = attribute % 30;
    assert_eq!(result.current, 10);
}

#[test]
fn test_rem_assign() {
    let mut attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    attribute %= 30;
    assert_eq!(attribute.current, 10);
}

#[test]
fn test_range_bounds() {
    let attribute =
        IntegerAttribute::with_min_and_max(0, 100).expect("Failed to create IntegerAttribute");
    let start_bound = attribute.start_bound();
    let end_bound = attribute.end_bound();
    assert_eq!(start_bound, std::ops::Bound::Included(&0));
    assert_eq!(end_bound, std::ops::Bound::Included(&100));
}
