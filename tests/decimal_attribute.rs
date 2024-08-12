use nwest_shared_component_library::DecimalAttribute;

#[test]
fn test_display() {
    let mut health = DecimalAttribute::new(20.0);
    health -= 1.0;
    assert!((health.current - 19.0).abs() < f64::EPSILON);
    assert_eq!(health, 19.0);
    let health_display = format!("{health}");
    assert_eq!(health_display, "19.00 (95.00%)");
}

#[test]
fn test_with_min_and_max() {
    let attribute =
        DecimalAttribute::with_min_and_max(0.0, 100.0).expect("Failed to create DecimalAttribute");
    assert!((attribute.min - 0.0).abs() < f64::EPSILON);
    assert!((attribute.max - 100.0).abs() < f64::EPSILON);
    assert!((attribute.current - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_set_value() {
    let mut attribute = DecimalAttribute::new(100.0);
    attribute.set_value(50.0);
    assert!((attribute.current - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_current_value() {
    let attribute = DecimalAttribute::new(100.0);
    assert!((attribute.current_value() - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_current_percentage() {
    let attribute = DecimalAttribute::new(100.0);
    assert!((attribute.current_percentage() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_current_percentage_negative_minimum() {
    let mut attribute = DecimalAttribute::with_min_and_max(-100.0, 100.0)
        .expect("Failed to create DecimalAttribute");
    attribute.set_value(0.0);
    assert!((attribute.current_percentage() - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_current_percentage_positive_minimum() {
    let mut attribute =
        DecimalAttribute::with_min_and_max(50.0, 100.0).expect("Failed to create DecimalAttribute");
    attribute.set_value(75.0);
    assert!((attribute.current_percentage() - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_set_max() {
    let mut attribute = DecimalAttribute::new(100.0);
    attribute.set_max(200.0).expect("Failed to set max value");
    assert!((attribute.max - 200.0).abs() < f64::EPSILON);
}

#[test]
fn test_set_max_fail() {
    let mut attribute = DecimalAttribute::new(100.0);
    let result = attribute.set_max(-50.0);
    assert!(result.is_err());
}

#[test]
fn test_set_min() {
    let mut attribute = DecimalAttribute::new(100.0);
    attribute.set_min(50.0).expect("Failed to set min value");
    assert!((attribute.min - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_set_min_fail() {
    let mut attribute = DecimalAttribute::new(100.0);
    let result = attribute.set_min(200.0);
    assert!(result.is_err());
}

#[test]
fn test_set_min_max() {
    let mut attribute = DecimalAttribute::new(100.0);
    attribute
        .set_min_max(50.0, 200.0)
        .expect("Failed to set min and max value");
    assert!((attribute.min - 50.0).abs() < f64::EPSILON);
    assert!((attribute.max - 200.0).abs() < f64::EPSILON);
}

#[test]
fn test_set_min_max_fail() {
    let mut attribute = DecimalAttribute::new(100.0);
    let result = attribute.set_min_max(200.0, 50.0);
    assert!(result.is_err());
}

#[test]
fn test_add_assign() {
    let mut attribute = DecimalAttribute::new(100.0);
    attribute.set_value(0.0);
    attribute += 50.0;
    assert!((attribute.current - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_sub_assign() {
    let mut attribute = DecimalAttribute::new(100.0);
    attribute -= 50.0;
    assert!((attribute.current - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_mul_assign() {
    let mut attribute = DecimalAttribute::new(100.0);
    attribute.set_value(25.0);
    attribute *= 2.0;
    assert!((attribute.current - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_div_assign() {
    let mut attribute = DecimalAttribute::new(100.0);
    attribute /= 2.0;
    assert!((attribute.current - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_div_assign_0() {
    let mut attribute = DecimalAttribute::new(100.0);
    attribute /= 0.0;
    assert!((attribute.current - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_add() {
    let attribute = DecimalAttribute::new(100.0);
    let result: DecimalAttribute = attribute + 50.0;
    assert!((result.current - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_sub() {
    let attribute = DecimalAttribute::new(100.0);
    let result: DecimalAttribute = attribute - 50.0;
    assert!((result.current - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_mul() {
    let attribute = DecimalAttribute::new(100.0);
    let result: DecimalAttribute = attribute * 2.0;
    assert!((result.current - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_div() {
    let attribute = DecimalAttribute::new(100.0);
    let result: DecimalAttribute = attribute / 2.0;
    assert!((result.current - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_div_0() {
    let attribute = DecimalAttribute::new(100.0);
    let result: DecimalAttribute = attribute / 0.0;
    assert!((result.current - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_neg() {
    let attribute = DecimalAttribute::new(100.0);
    let result = -attribute;
    assert!((result.current - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_eq() {
    let attribute = DecimalAttribute::new(100.0);
    let other = DecimalAttribute::new(100.0);
    assert_eq!(attribute, other);
}

#[test]
fn test_ne() {
    let attribute = DecimalAttribute::new(100.0);
    let other = DecimalAttribute::new(50.0);
    assert_ne!(attribute, other);
}

#[test]
fn test_cmp_gt() {
    let attribute = DecimalAttribute::new(100.0);
    let other = DecimalAttribute::new(50.0);
    assert!(attribute > other);
}

#[test]
fn test_cmp_lt() {
    let attribute = DecimalAttribute::new(50.0);
    let other = DecimalAttribute::new(100.0);
    assert!(attribute < other);
}

#[test]
fn test_min_greater_than_max() {
    let result = DecimalAttribute::with_min_and_max(100.0, 50.0);
    assert!(result.is_err());
}

#[test]
fn test_max_less_than_min() {
    let result = DecimalAttribute::with_min_and_max(50.0, 100.0);
    assert!(result.is_ok());
}

#[test]
fn test_min_greater_than_max_set() {
    let mut attribute = DecimalAttribute::new(100.0);
    let result = attribute.set_min_max(100.0, 50.0);
    assert!(result.is_err());
}

#[test]
fn test_max_less_than_min_set() {
    let mut attribute = DecimalAttribute::new(100.0);
    let result = attribute.set_min_max(50.0, 100.0);
    assert!(result.is_ok());
}

#[test]
fn test_into_f64() {
    let attribute = DecimalAttribute::new(100.0);
    let value: f64 = attribute.into();
    assert!((value - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_into_f32() {
    let attribute = DecimalAttribute::new(100.0);
    let value: f32 = attribute.into();
    assert!((value - 100.0).abs() < f32::EPSILON);
}
