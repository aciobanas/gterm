//! Translated from mp-units' `space_and_time.h`:
//! <https://github.com/mpusz/mp-units/blob/master/src/systems/include/mp-units/systems/isq/space_and_time.h>
//!
//! Original source is MIT licensed, Copyright (c) 2018 Mateusz Pusz.
//!
//! As in `si_quantities.rs`, the `non_negative`/`possibly_negative`/`is_kind` tags aren't modeled
//! here. Additionally, `QSpec::equation` doesn't propagate `QCharacter` from its operands the way
//! mp-units' character inference does, so quantities that are vectors only because their C++
//! parent is a vector (not because of an explicit `quantity_tensor_order` tag) get an explicit
//! `.character(QCharacter::REAL_VECTOR)` call here, noted per-constant below.

use crate::qcharacter::QCharacter;
use crate::qspec::{QSpec, QSpecEq};

use crate::isq::Isq;

impl Isq {
    // WORKAROUND for V2 in the original: altitude/depth are signed vertical coordinates while
    // height is their unsigned magnitude. The "correct" V3 hierarchy (length -> height ->
    // point_for<height> {altitude, depth}) needs `point_for<>`, unavailable in V2, so the
    // hierarchy is temporarily reversed: altitude is a child of length, height a child of
    // altitude. See the original source for the full rationale.
    /// possibly_negative in the original; sign is not yet modeled here.
    pub const ALTITUDE: QSpec = QSpec::new("altitude").equation(&QSpecEq::Term(&Isq::L));
    // `depth` is intentionally not modeled: it is an axis-inverted `altitude` (measured downward),
    // and axis direction belongs to the coordinate frame, not to the quantity itself.
    /// non_negative in the original; not yet modeled here.
    pub const HEIGHT: QSpec = QSpec::new("height").equation(&QSpecEq::Term(&Isq::ALTITUDE));

    pub const THICKNESS: QSpec = QSpec::new("thickness").equation(&QSpecEq::Term(&Isq::WIDTH));
    pub const DIAMETER: QSpec = QSpec::new("diameter").equation(&QSpecEq::Term(&Isq::WIDTH));
    pub const DISTANCE: QSpec = QSpec::new("distance").equation(&QSpecEq::Term(&Isq::PATH_LENGTH));
    pub const RADIAL_DISTANCE: QSpec = QSpec::new("radial_distance").equation(&QSpecEq::Term(&Isq::DISTANCE));
    pub const DISPLACEMENT: QSpec = QSpec::new("displacement")
        .equation(&QSpecEq::Term(&Isq::L))
        .character(QCharacter::REAL_VECTOR);
    /// Vector: inherited from `displacement` in the original.
    pub const POSITION_VECTOR: QSpec = QSpec::new("position_vector")
        .equation(&QSpecEq::Term(&Isq::DISPLACEMENT))
        .character(QCharacter::REAL_VECTOR);
    pub const RADIUS_OF_CURVATURE: QSpec = QSpec::new("radius_of_curvature").equation(&QSpecEq::Term(&Isq::RADIUS));
    /// non_negative in the original; not yet modeled here.
    pub const CURVATURE: QSpec = QSpec::new("curvature")
        .equation(&QSpecEq::Pow(&QSpecEq::Term(&Isq::RADIUS_OF_CURVATURE), -1));
    /// non_negative in the original; not yet modeled here.
    pub const VOLUME: QSpec = QSpec::new("volume").equation(&QSpecEq::Pow(&QSpecEq::Term(&Isq::L), 3));
    /// Kind: `angular_measure` in the original (dropped here in favor of the defining equation).
    pub const ROTATIONAL_DISPLACEMENT: QSpec = QSpec::new("rotational_displacement").equation(&QSpecEq::Div(
        &QSpecEq::Term(&Isq::PATH_LENGTH),
        &QSpecEq::Term(&Isq::RADIUS),
    ));
    pub const ANGULAR_DISPLACEMENT: QSpec = Isq::ROTATIONAL_DISPLACEMENT;
    /// is_kind in the original; not yet modeled here.
    pub const PHASE_ANGLE: QSpec = QSpec::new("phase_angle").equation(&QSpecEq::Term(&Isq::ANGULAR_MEASURE));
    /// Differs from ISO 80000. non_negative in the original; not yet modeled here.
    pub const SPEED: QSpec = QSpec::new("speed").equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::L), &QSpecEq::Term(&Isq::T)));
    /// Differs from ISO 80000. Vector: inherited from `displacement` in the original.
    pub const VELOCITY: QSpec = QSpec::new("velocity")
        .equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::DISPLACEMENT), &QSpecEq::Term(&Isq::T)))
        .character(QCharacter::REAL_VECTOR);
    /// Vector: inherited from `velocity` in the original.
    pub const ACCELERATION: QSpec = QSpec::new("acceleration")
        .equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::VELOCITY), &QSpecEq::Term(&Isq::T)))
        .character(QCharacter::REAL_VECTOR);
    /// Not in ISO 80000. Vector: inherited from `acceleration` in the original.
    pub const ACCELERATION_OF_FREE_FALL: QSpec = QSpec::new("acceleration_of_free_fall")
        .equation(&QSpecEq::Term(&Isq::ACCELERATION))
        .character(QCharacter::REAL_VECTOR);
    pub const ANGULAR_VELOCITY: QSpec = QSpec::new("angular_velocity")
        .equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::ANGULAR_DISPLACEMENT), &QSpecEq::Term(&Isq::T)))
        .character(QCharacter::REAL_VECTOR);
    /// Vector: inherited from `angular_velocity` in the original.
    pub const ANGULAR_ACCELERATION: QSpec = QSpec::new("angular_acceleration")
        .equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::ANGULAR_VELOCITY), &QSpecEq::Term(&Isq::T)))
        .character(QCharacter::REAL_VECTOR);
    pub const TIME_CONSTANT: QSpec = QSpec::new("time_constant").equation(&QSpecEq::Term(&Isq::T));
    /// Kind: `dimensionless` in the original; no defining equation.
    pub const ROTATION: QSpec = QSpec::new("rotation");
    pub const ROTATIONAL_FREQUENCY: QSpec = QSpec::new("rotational_frequency")
        .equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::ROTATION), &QSpecEq::Term(&Isq::T)));
    pub const ANGULAR_FREQUENCY: QSpec = QSpec::new("angular_frequency")
        .equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::PHASE_ANGLE), &QSpecEq::Term(&Isq::T)));
    pub const WAVELENGTH: QSpec = QSpec::new("wavelength").equation(&QSpecEq::Term(&Isq::L));
    /// non_negative in the original; not yet modeled here.
    pub const REPETENCY: QSpec =
        QSpec::new("repetency").equation(&QSpecEq::Pow(&QSpecEq::Term(&Isq::WAVELENGTH), -1));
    pub const WAVENUMBER: QSpec = Isq::REPETENCY;
    /// non_negative in the original; not yet modeled here.
    pub const ANGULAR_REPETENCY: QSpec =
        QSpec::new("angular_repetency").equation(&QSpecEq::Pow(&QSpecEq::Term(&Isq::WAVELENGTH), -1));
    pub const ANGULAR_WAVENUMBER: QSpec = Isq::ANGULAR_REPETENCY;
    // ISO 80000-3 defines the wave vector twice (item 3-21 roots it in repetency, item 3-22 in
    // angular repetency); ISO 80000-12 item 12-9.1 and k = p/ħ side with 3-22, so it's rooted here.
    pub const WAVE_VECTOR: QSpec = QSpec::new("wave_vector")
        .equation(&QSpecEq::Term(&Isq::ANGULAR_REPETENCY))
        .character(QCharacter::REAL_VECTOR);
    pub const PHASE_SPEED: QSpec = QSpec::new("phase_speed").equation(&QSpecEq::Div(
        &QSpecEq::Term(&Isq::ANGULAR_FREQUENCY),
        &QSpecEq::Term(&Isq::ANGULAR_REPETENCY),
    ));
    pub const GROUP_SPEED: QSpec = QSpec::new("group_speed").equation(&QSpecEq::Div(
        &QSpecEq::Term(&Isq::ANGULAR_FREQUENCY),
        &QSpecEq::Term(&Isq::ANGULAR_REPETENCY),
    ));
    /// non_negative in the original; not yet modeled here.
    pub const DAMPING_COEFFICIENT: QSpec =
        QSpec::new("damping_coefficient").equation(&QSpecEq::Pow(&QSpecEq::Term(&Isq::TIME_CONSTANT), -1));
    pub const LOGARITHMIC_DECREMENT: QSpec = QSpec::new("logarithmic_decrement").equation(&QSpecEq::Mul(
        &QSpecEq::Term(&Isq::DAMPING_COEFFICIENT),
        &QSpecEq::Term(&Isq::PERIOD_DURATION),
    ));
    /// non_negative in the original; not yet modeled here.
    pub const ATTENUATION: QSpec =
        QSpec::new("attenuation").equation(&QSpecEq::Pow(&QSpecEq::Term(&Isq::DISTANCE), -1));
    pub const EXTINCTION: QSpec = Isq::ATTENUATION;
    pub const PHASE_COEFFICIENT: QSpec = QSpec::new("phase_coefficient").equation(&QSpecEq::Div(
        &QSpecEq::Term(&Isq::PHASE_ANGLE),
        &QSpecEq::Term(&Isq::PATH_LENGTH),
    ));
    /// non_negative in the original; not yet modeled here. γ = α + iβ where α is attenuation and
    /// β the phase coefficient of a plane wave.
    pub const PROPAGATION_COEFFICIENT: QSpec =
        QSpec::new("propagation_coefficient").equation(&QSpecEq::Pow(&QSpecEq::Term(&Isq::L), -1));
}
