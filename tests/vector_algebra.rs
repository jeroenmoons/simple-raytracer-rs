use srt::math::vector::Vec3;

// Not the most useful test, more to try out how integration tests fit into a project
#[test]
fn test_vector_manipulations() {
    let a = Vec3::new(2., 3., 4.);
    let b = Vec3::new(5., 1., 3.);
    let c = Vec3::new(1., 0., 2.);

    let expected = Vec3::new(8., -7., -4.);

    let result = a + b;
    let result = result.cross(c);

    assert_eq!(result.x(), expected.x());
    assert_eq!(result.y(), expected.y());
    assert_eq!(result.z(), expected.z());
}
