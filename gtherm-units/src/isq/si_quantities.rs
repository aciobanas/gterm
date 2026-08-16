//! Translated from mp-units' `si_quantities.h`:
//! <https://github.com/mpusz/mp-units/blob/master/src/systems/include/mp-units/systems/isq/si_quantities.h>
//!
//! Original source is MIT licensed, Copyright (c) 2018 Mateusz Pusz.
//!

use crate::qcharacter::QCharacter;
use crate::qspec::{QSpec, QSpecEq};
use QSpecEq::{Div, Mul, Pow, Term};

use crate::isq::Isq;

impl Isq {
    // space and time

    pub const WIDTH: QSpec = QSpec::new("width").equation(&Term(&Isq::L));

    pub const BREADTH: QSpec = Isq::WIDTH;

    /// Differs from ISO 80000: a child of `width`, not of `length` directly.
    pub const RADIUS: QSpec = QSpec::new("radius").equation(&Term(&Isq::WIDTH));

    pub const PATH_LENGTH: QSpec = QSpec::new("path_length").equation(&Term(&Isq::L));

    pub const ARC_LENGTH: QSpec = Isq::PATH_LENGTH;

    pub const AREA: QSpec = QSpec::new("area").equation(&Pow(&Term(&Isq::L), 2));

    pub const ANGULAR_MEASURE: QSpec =
        QSpec::new("angular_measure").equation(&Div(&Term(&Isq::ARC_LENGTH), &Term(&Isq::RADIUS)));

    pub const SOLID_ANGULAR_MEASURE: QSpec = QSpec::new("solid_angular_measure")
        .equation(&Div(&Term(&Isq::AREA), &Pow(&Term(&Isq::RADIUS), 2)));

    pub const PERIOD_DURATION: QSpec = QSpec::new("period_duration").equation(&Term(&Isq::T));

    pub const PERIOD: QSpec = Isq::PERIOD_DURATION;

    pub const FREQUENCY: QSpec =
        QSpec::new("frequency").equation(&Pow(&Term(&Isq::PERIOD_DURATION), -1));

    // mechanics

    /// Differs from ISO 80000.
    pub const ENERGY: QSpec = QSpec::new("energy").equation(&Div(
        &Mul(&Term(&Isq::M), &Pow(&Term(&Isq::L), 2)),
        &Pow(&Term(&Isq::T), 2),
    ));

    pub const FORCE: QSpec = QSpec::new("force")
        .equation(&Div(
            &Mul(&Term(&Isq::M), &Term(&Isq::L)),
            &Pow(&Term(&Isq::T), 2),
        ))
        .character(QCharacter::REAL_VECTOR);

    // override to scalar
    pub const PRESSURE: QSpec = QSpec::new("pressure")
        .equation(&Div(&Term(&Isq::FORCE), &Term(&Isq::AREA)))
        .character(QCharacter::REAL_SCALAR);

    // electromagnetism

    pub const ELECTRIC_POTENTIAL: QSpec = QSpec::new("electric_potential").equation(&Div(
        &Term(&Isq::ENERGY),
        &Mul(&Term(&Isq::I), &Term(&Isq::T)),
    ));

    pub const CAPACITANCE: QSpec = QSpec::new("capacitance").equation(&Div(
        &Mul(&Term(&Isq::I), &Term(&Isq::T)),
        &Term(&Isq::ELECTRIC_POTENTIAL),
    ));

    pub const IMPEDANCE: QSpec = QSpec::new("impedance")
        .equation(&Div(&Term(&Isq::ELECTRIC_POTENTIAL), &Term(&Isq::I)))
        .character(QCharacter::COMPLEX_SCALAR);

    pub const ADMITTANCE: QSpec =
        QSpec::new("admittance").equation(&Pow(&Term(&Isq::IMPEDANCE), -1));

    pub const MAGNETIC_FLUX_DENSITY: QSpec = QSpec::new("magnetic_flux_density")
        .equation(&Div(
            &Term(&Isq::M),
            &Mul(&Term(&Isq::I), &Pow(&Term(&Isq::T), 2)),
        ))
        .character(QCharacter::REAL_VECTOR);

    // light_and_radiation

    pub const LUMINOUS_FLUX: QSpec = QSpec::new("luminous_flux")
        .equation(&Mul(&Term(&Isq::J), &Term(&Isq::SOLID_ANGULAR_MEASURE)));

    pub const ILLUMINANCE: QSpec =
        QSpec::new("illuminance").equation(&Div(&Term(&Isq::LUMINOUS_FLUX), &Term(&Isq::AREA)));

    // physical_chemistry

    pub const CATALYTIC_ACTIVITY: QSpec =
        QSpec::new("catalytic_activity").equation(&Div(&Term(&Isq::N), &Term(&Isq::T)));

    // atomic_and_nuclear_physics

    pub const ACTIVITY: QSpec = QSpec::new("activity").equation(&Pow(&Term(&Isq::T), -1));

    pub const ABSORBED_DOSE: QSpec =
        QSpec::new("absorbed_dose").equation(&Div(&Term(&Isq::ENERGY), &Term(&Isq::M)));

    pub const IONIZING_RADIATION_QUALITY_FACTOR: QSpec =
        QSpec::new("ionizing_radiation_quality_factor");

    pub const DOSE_EQUIVALENT: QSpec = QSpec::new("dose_equivalent").equation(&Mul(
        &Term(&Isq::ABSORBED_DOSE),
        &Term(&Isq::IONIZING_RADIATION_QUALITY_FACTOR),
    ));

    // not in ISO 80000 but central for mechanics, thermodynamics and chemistry

    pub const POWER: QSpec = QSpec::new("power").equation(&Div(
        &Mul(&Term(&Isq::M), &Pow(&Term(&Isq::L), 2)),
        &Pow(&Term(&Isq::T), 3),
    ));

    pub const ENERGY_DENSITY: QSpec = QSpec::new("energy_density").equation(&Div(
        &Term(&Isq::M),
        &Mul(&Term(&Isq::L), &Pow(&Term(&Isq::T), 2)),
    ));
}
