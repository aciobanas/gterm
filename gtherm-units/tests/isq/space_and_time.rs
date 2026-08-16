use gtherm_units::dims::Dims;
use gtherm_units::isq::Isq;
use gtherm_units::qcharacter::QCharacter;

#[test]
fn test_altitude_depth_height_share_length_dimension() {
    assert_eq!(Isq::ALTITUDE.dims, Dims::L);
    assert_eq!(Isq::DEPTH, Isq::ALTITUDE);
    assert_eq!(Isq::HEIGHT.dims, Dims::L);
}

#[test]
fn test_thickness_and_diameter_share_width_dimension() {
    assert_eq!(Isq::THICKNESS.dims, Isq::WIDTH.dims);
    assert_eq!(Isq::DIAMETER.dims, Isq::WIDTH.dims);
}

#[test]
fn test_distance_and_radial_distance_dims() {
    assert_eq!(Isq::DISTANCE.dims, Dims::L);
    assert_eq!(Isq::RADIAL_DISTANCE.dims, Dims::L);
}

#[test]
fn test_displacement_is_a_length_vector() {
    assert_eq!(Isq::DISPLACEMENT.dims, Dims::L);
    assert_eq!(Isq::DISPLACEMENT.character, QCharacter::REAL_VECTOR);
}

#[test]
fn test_position_vector_dims() {
    assert_eq!(Isq::POSITION_VECTOR.dims, Isq::DISPLACEMENT.dims);
}

#[test]
fn test_radius_of_curvature_and_curvature_dims() {
    assert_eq!(Isq::RADIUS_OF_CURVATURE.dims, Dims::L);
    assert_eq!(Isq::CURVATURE.dims, Dims::L.pow(-1));
}

#[test]
fn test_volume_is_length_cubed() {
    assert_eq!(Isq::VOLUME.dims, Dims::L.pow(3));
}

#[test]
fn test_rotational_and_angular_displacement_are_dimensionless() {
    // path_length / radius, both length, cancels out
    assert_eq!(Isq::ROTATIONAL_DISPLACEMENT.dims, Dims::ZERO);
    assert_eq!(Isq::ANGULAR_DISPLACEMENT, Isq::ROTATIONAL_DISPLACEMENT);
}

#[test]
fn test_phase_angle_is_dimensionless() {
    assert_eq!(Isq::PHASE_ANGLE.dims, Dims::ZERO);
}

#[test]
fn test_speed_and_velocity_are_length_over_time() {
    assert_eq!(Isq::SPEED.dims, Dims::L / Dims::T);
    assert_eq!(Isq::VELOCITY.dims, Isq::DISPLACEMENT.dims / Dims::T);
}

#[test]
fn test_acceleration_dims_and_free_fall_alias() {
    assert_eq!(Isq::ACCELERATION.dims, Isq::VELOCITY.dims / Dims::T);
    assert_eq!(Isq::ACCELERATION_OF_FREE_FALL.dims, Isq::ACCELERATION.dims);
}

#[test]
fn test_angular_velocity_and_acceleration_are_inverse_powers_of_time() {
    assert_eq!(Isq::ANGULAR_VELOCITY.dims, Dims::T.pow(-1));
    assert_eq!(Isq::ANGULAR_ACCELERATION.dims, Dims::T.pow(-2));
}

#[test]
fn test_time_constant_is_time() {
    assert_eq!(Isq::TIME_CONSTANT.dims, Dims::T);
}

#[test]
fn test_rotation_is_dimensionless_and_rotational_frequency_is_inverse_time() {
    assert_eq!(Isq::ROTATION.dims, Dims::ZERO);
    assert_eq!(Isq::ROTATIONAL_FREQUENCY.dims, Dims::T.pow(-1));
}

#[test]
fn test_angular_frequency_is_inverse_time() {
    assert_eq!(Isq::ANGULAR_FREQUENCY.dims, Dims::T.pow(-1));
}

#[test]
fn test_repetency_and_wavenumber_are_inverse_wavelength() {
    assert_eq!(Isq::WAVELENGTH.dims, Dims::L);
    assert_eq!(Isq::REPETENCY.dims, Dims::L.pow(-1));
    assert_eq!(Isq::WAVENUMBER, Isq::REPETENCY);
    assert_eq!(Isq::ANGULAR_REPETENCY.dims, Dims::L.pow(-1));
    assert_eq!(Isq::ANGULAR_WAVENUMBER, Isq::ANGULAR_REPETENCY);
}

#[test]
fn test_wave_vector_is_a_vector() {
    assert_eq!(Isq::WAVE_VECTOR.dims, Isq::ANGULAR_REPETENCY.dims);
    assert_eq!(Isq::WAVE_VECTOR.character, QCharacter::REAL_VECTOR);
}

#[test]
fn test_phase_speed_and_group_speed_dims() {
    let expected = Isq::ANGULAR_FREQUENCY.dims / Isq::ANGULAR_REPETENCY.dims;
    assert_eq!(Isq::PHASE_SPEED.dims, expected);
    assert_eq!(Isq::GROUP_SPEED.dims, expected);
}

#[test]
fn test_damping_coefficient_and_logarithmic_decrement_dims() {
    assert_eq!(Isq::DAMPING_COEFFICIENT.dims, Isq::TIME_CONSTANT.dims.pow(-1));
    // damping_coefficient (T^-1) * period_duration (T) cancels out to dimensionless
    assert_eq!(Isq::LOGARITHMIC_DECREMENT.dims, Dims::ZERO);
}

#[test]
fn test_attenuation_and_extinction_alias() {
    assert_eq!(Isq::ATTENUATION.dims, Isq::DISTANCE.dims.pow(-1));
    assert_eq!(Isq::EXTINCTION, Isq::ATTENUATION);
}

#[test]
fn test_phase_coefficient_and_propagation_coefficient_dims() {
    assert_eq!(Isq::PHASE_COEFFICIENT.dims, Isq::PHASE_ANGLE.dims / Isq::PATH_LENGTH.dims);
    assert_eq!(Isq::PROPAGATION_COEFFICIENT.dims, Dims::L.pow(-1));
}

#[test]
fn test_find_same_kind_across_a_term_chain() {
    // diameter and thickness are both directly rooted in width
    assert_eq!(Isq::DIAMETER.find_same_kind(&Isq::THICKNESS), Some(&Isq::WIDTH));

    // height and altitude share the closer `altitude` ancestor
    assert_eq!(Isq::HEIGHT.find_same_kind(&Isq::ALTITUDE), Some(&Isq::ALTITUDE));
}
