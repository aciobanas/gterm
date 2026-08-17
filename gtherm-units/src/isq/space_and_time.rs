//! Translated from mp-units' `space_and_time.h`:
//! <https://github.com/mpusz/mp-units/blob/master/src/systems/include/mp-units/systems/isq/space_and_time.h>
//!
//! Original source is MIT licensed, Copyright (c) 2018 Mateusz Pusz.
//!

use crate::dims::Dims;
use crate::qcharacter::{QCharacter, TensorOrder};
use crate::qspec::{QSpec, QSpecEq};
use QSpecEq::{Div, Mul, Pow, Term};

use crate::isq::Isq;

impl Isq {
    pub const ALTITUDE: QSpec = QSpec::new("altitude").equation(&Term(&Isq::L));

    pub const DEPTH: QSpec = Isq::ALTITUDE;

    pub const HEIGHT: QSpec = QSpec::new("height").equation(&Term(&Isq::ALTITUDE));

    pub const THICKNESS: QSpec = QSpec::new("thickness").equation(&Term(&Isq::WIDTH));

    pub const DIAMETER: QSpec = QSpec::new("diameter").equation(&Term(&Isq::WIDTH));

    pub const DISTANCE: QSpec = QSpec::new("distance").equation(&Term(&Isq::PATH_LENGTH));

    pub const RADIAL_DISTANCE: QSpec =
        QSpec::new("radial_distance").equation(&Term(&Isq::DISTANCE));

    pub const DISPLACEMENT: QSpec = QSpec::new("displacement")
        .equation(&Term(&Isq::L))
        .character(QCharacter::REAL_VECTOR);

    pub const POSITION_VECTOR: QSpec =
        QSpec::new("position_vector").equation(&Term(&Isq::DISPLACEMENT));

    pub const RADIUS_OF_CURVATURE: QSpec =
        QSpec::new("radius_of_curvature").equation(&Term(&Isq::RADIUS));

    pub const CURVATURE: QSpec =
        QSpec::new("curvature").equation(&Pow(&Term(&Isq::RADIUS_OF_CURVATURE), -1));

    pub const VOLUME: QSpec = QSpec::new("volume").equation(&Pow(&Term(&Isq::L), 3));

    pub const ROTATIONAL_DISPLACEMENT: QSpec = QSpec::new("rotational_displacement")
        .equation(&Div(&Term(&Isq::PATH_LENGTH), &Term(&Isq::RADIUS)));

    pub const ANGULAR_DISPLACEMENT: QSpec = Isq::ROTATIONAL_DISPLACEMENT;

    pub const PHASE_ANGLE: QSpec = QSpec::new("phase_angle").equation(&Term(&Isq::ANGULAR_MEASURE));

    /// Differs from ISO 80000.
    pub const SPEED: QSpec = QSpec::new("speed").equation(&Div(&Term(&Isq::L), &Term(&Isq::T)));

    pub const VELOCITY: QSpec =
        QSpec::new("velocity").equation(&Div(&Term(&Isq::DISPLACEMENT), &Term(&Isq::T)));

    pub const ACCELERATION: QSpec =
        QSpec::new("acceleration").equation(&Div(&Term(&Isq::VELOCITY), &Term(&Isq::T)));

    /// Not in ISO 80000. Vector: inherited from `acceleration` in the original.
    pub const ACCELERATION_OF_FREE_FALL: QSpec =
        QSpec::new("acceleration_of_free_fall").equation(&Term(&Isq::ACCELERATION));

    pub const ANGULAR_VELOCITY: QSpec = QSpec::new("angular_velocity")
        .equation(&Div(&Term(&Isq::ANGULAR_DISPLACEMENT), &Term(&Isq::T)));

    /// Vector: inherited from `angular_velocity` in the original.
    pub const ANGULAR_ACCELERATION: QSpec = QSpec::new("angular_acceleration")
        .equation(&Div(&Term(&Isq::ANGULAR_VELOCITY), &Term(&Isq::T)));

    pub const TIME_CONSTANT: QSpec = QSpec::new("time_constant").equation(&Term(&Isq::T));

    pub const ROTATION: QSpec = QSpec::new("rotation");

    pub const ROTATIONAL_FREQUENCY: QSpec =
        QSpec::new("rotational_frequency").equation(&Div(&Term(&Isq::ROTATION), &Term(&Isq::T)));

    pub const ANGULAR_FREQUENCY: QSpec =
        QSpec::new("angular_frequency").equation(&Div(&Term(&Isq::PHASE_ANGLE), &Term(&Isq::T)));

    pub const WAVELENGTH: QSpec = QSpec::new("wavelength").equation(&Term(&Isq::L));

    pub const REPETENCY: QSpec =
        QSpec::new("repetency").equation(&Pow(&Term(&Isq::WAVELENGTH), -1));

    pub const WAVENUMBER: QSpec = Isq::REPETENCY;

    pub const ANGULAR_REPETENCY: QSpec =
        QSpec::new("angular_repetency").equation(&Pow(&Term(&Isq::WAVELENGTH), -1));

    pub const ANGULAR_WAVENUMBER: QSpec = Isq::ANGULAR_REPETENCY;

    // ISO 80000-3 defines the wave vector twice (item 3-21 roots it in repetency, item 3-22 in
    // angular repetency); ISO 80000-12 item 12-9.1 and k = p/ħ side with 3-22, so it's rooted here.
    pub const WAVE_VECTOR: QSpec = QSpec::new("wave_vector")
        .equation(&Term(&Isq::ANGULAR_REPETENCY))
        .character(QCharacter::REAL_VECTOR);

    pub const PHASE_SPEED: QSpec = QSpec::new("phase_speed").equation(&Div(
        &Term(&Isq::ANGULAR_FREQUENCY),
        &Term(&Isq::ANGULAR_REPETENCY),
    ));

    pub const GROUP_SPEED: QSpec = QSpec::new("group_speed").equation(&Div(
        &Term(&Isq::ANGULAR_FREQUENCY),
        &Term(&Isq::ANGULAR_REPETENCY),
    ));

    pub const DAMPING_COEFFICIENT: QSpec =
        QSpec::new("damping_coefficient").equation(&Pow(&Term(&Isq::TIME_CONSTANT), -1));

    pub const LOGARITHMIC_DECREMENT: QSpec = QSpec::new("logarithmic_decrement").equation(&Mul(
        &Term(&Isq::DAMPING_COEFFICIENT),
        &Term(&Isq::PERIOD_DURATION),
    ));

    pub const ATTENUATION: QSpec =
        QSpec::new("attenuation").equation(&Pow(&Term(&Isq::DISTANCE), -1));

    pub const EXTINCTION: QSpec = Isq::ATTENUATION;

    pub const PHASE_COEFFICIENT: QSpec = QSpec::new("phase_coefficient")
        .equation(&Div(&Term(&Isq::PHASE_ANGLE), &Term(&Isq::PATH_LENGTH)));

    /// γ = α + iβ where α is attenuation and β the phase coefficient of a plane wave.
    pub const PROPAGATION_COEFFICIENT: QSpec =
        QSpec::new("propagation_coefficient").equation(&Pow(&Term(&Isq::L), -1));
}

// some compile-time assertions to ensure that the const functions are working as expected
const _: () = assert!(Isq::VOLUME.dims.const_eq(&Dims::L.pow(3)));
const _: () = assert!(Isq::VELOCITY.dims.const_eq(&Dims::L.const_div(&Dims::T)));
const _: () = assert!(
    Isq::ACCELERATION
        .dims
        .const_eq(&Isq::VELOCITY.dims.const_div(&Dims::T))
);
const _: () = assert!(
    Isq::ANGULAR_VELOCITY
        .dims
        .const_eq(&Isq::ANGULAR_DISPLACEMENT.dims.const_div(&Dims::T))
);
const _: () = assert!(Isq::REPETENCY.dims.const_eq(&Dims::L.pow(-1)));
const _: () = assert!(Isq::CURVATURE.dims.const_eq(&Dims::L.pow(-1)));
const _: () = assert!(Isq::ATTENUATION.dims.const_eq(&Isq::DISTANCE.dims.pow(-1)));

// character overrides should stick regardless of what the equation's operands would otherwise combine to
const _: () = assert!(matches!(
    Isq::DISPLACEMENT.character.tensor_order,
    Some(TensorOrder::Vector)
));
const _: () = assert!(matches!(
    Isq::WAVE_VECTOR.character.tensor_order,
    Some(TensorOrder::Vector)
));
