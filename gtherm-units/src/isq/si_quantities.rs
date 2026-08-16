//! Translated from mp-units' `si_quantities.h`:
//! <https://github.com/mpusz/mp-units/blob/master/src/systems/include/mp-units/systems/isq/si_quantities.h>
//!
//! Original source is MIT licensed, Copyright (c) 2018 Mateusz Pusz.
//!
//! Kinds are modeled implicitly here via [`QSpec::find_same_kind`], so the original's `is_kind`
//! tag needs no explicit marker. The `non_negative`/`possibly_negative` sign tags aren't modeled.

use crate::qcharacter::QCharacter;
use crate::qspec::{QSpec, QSpecEq};

use crate::isq::Isq;

impl Isq {
    // space and time

    pub const WIDTH: QSpec = QSpec::new("width").equation(&QSpecEq::Term(&Isq::L));
    pub const BREADTH: QSpec = Isq::WIDTH;
    /// Differs from ISO 80000: a child of `width`, not of `length` directly.
    pub const RADIUS: QSpec = QSpec::new("radius").equation(&QSpecEq::Term(&Isq::WIDTH));
    pub const PATH_LENGTH: QSpec = QSpec::new("path_length").equation(&QSpecEq::Term(&Isq::L));
    pub const ARC_LENGTH: QSpec = Isq::PATH_LENGTH;
    pub const AREA: QSpec = QSpec::new("area").equation(&QSpecEq::Pow(&QSpecEq::Term(&Isq::L), 2));
    pub const ANGULAR_MEASURE: QSpec = QSpec::new("angular_measure").equation(&QSpecEq::Div(
        &QSpecEq::Term(&Isq::ARC_LENGTH),
        &QSpecEq::Term(&Isq::RADIUS),
    ));
    pub const SOLID_ANGULAR_MEASURE: QSpec = QSpec::new("solid_angular_measure").equation(&QSpecEq::Div(
        &QSpecEq::Term(&Isq::AREA),
        &QSpecEq::Pow(&QSpecEq::Term(&Isq::RADIUS), 2),
    ));
    pub const PERIOD_DURATION: QSpec = QSpec::new("period_duration").equation(&QSpecEq::Term(&Isq::T));
    pub const PERIOD: QSpec = Isq::PERIOD_DURATION;
    pub const FREQUENCY: QSpec =
        QSpec::new("frequency").equation(&QSpecEq::Pow(&QSpecEq::Term(&Isq::PERIOD_DURATION), -1));

    // mechanics

    /// Differs from ISO 80000.
    pub const ENERGY: QSpec = QSpec::new("energy").equation(&QSpecEq::Div(
        &QSpecEq::Mul(&QSpecEq::Term(&Isq::M), &QSpecEq::Pow(&QSpecEq::Term(&Isq::L), 2)),
        &QSpecEq::Pow(&QSpecEq::Term(&Isq::T), 2),
    ));
    pub const FORCE: QSpec = QSpec::new("force")
        .equation(&QSpecEq::Div(
            &QSpecEq::Mul(&QSpecEq::Term(&Isq::M), &QSpecEq::Term(&Isq::L)),
            &QSpecEq::Pow(&QSpecEq::Term(&Isq::T), 2),
        ))
        .character(QCharacter::REAL_VECTOR);
    pub const PRESSURE: QSpec = QSpec::new("pressure")
        .equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::FORCE), &QSpecEq::Term(&Isq::AREA)))
        .character(QCharacter::REAL_SCALAR);

    // electromagnetism

    pub const ELECTRIC_POTENTIAL: QSpec = QSpec::new("electric_potential")
        .equation(&QSpecEq::Div(
            &QSpecEq::Term(&Isq::ENERGY),
            &QSpecEq::Mul(&QSpecEq::Term(&Isq::I), &QSpecEq::Term(&Isq::T)),
        ))
        .character(QCharacter::REAL_SCALAR);
    pub const CAPACITANCE: QSpec = QSpec::new("capacitance").equation(&QSpecEq::Div(
        &QSpecEq::Mul(&QSpecEq::Term(&Isq::I), &QSpecEq::Term(&Isq::T)),
        &QSpecEq::Term(&Isq::ELECTRIC_POTENTIAL),
    ));
    pub const IMPEDANCE: QSpec = QSpec::new("impedance")
        .equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::ELECTRIC_POTENTIAL), &QSpecEq::Term(&Isq::I)))
        .character(QCharacter::COMPLEX_SCALAR);
    pub const ADMITTANCE: QSpec = QSpec::new("admittance")
        .equation(&QSpecEq::Pow(&QSpecEq::Term(&Isq::IMPEDANCE), -1))
        .character(QCharacter::COMPLEX_SCALAR);
    pub const MAGNETIC_FLUX_DENSITY: QSpec = QSpec::new("magnetic_flux_density")
        .equation(&QSpecEq::Div(
            &QSpecEq::Term(&Isq::M),
            &QSpecEq::Mul(&QSpecEq::Term(&Isq::I), &QSpecEq::Pow(&QSpecEq::Term(&Isq::T), 2)),
        ))
        .character(QCharacter::REAL_VECTOR);

    // light_and_radiation

    pub const LUMINOUS_FLUX: QSpec = QSpec::new("luminous_flux").equation(&QSpecEq::Mul(
        &QSpecEq::Term(&Isq::J),
        &QSpecEq::Term(&Isq::SOLID_ANGULAR_MEASURE),
    ));
    pub const ILLUMINANCE: QSpec = QSpec::new("illuminance")
        .equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::LUMINOUS_FLUX), &QSpecEq::Term(&Isq::AREA)));

    // physical_chemistry

    pub const CATALYTIC_ACTIVITY: QSpec = QSpec::new("catalytic_activity")
        .equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::N), &QSpecEq::Term(&Isq::T)));

    // atomic_and_nuclear_physics

    pub const ACTIVITY: QSpec = QSpec::new("activity").equation(&QSpecEq::Pow(&QSpecEq::Term(&Isq::T), -1));
    pub const ABSORBED_DOSE: QSpec = QSpec::new("absorbed_dose")
        .equation(&QSpecEq::Div(&QSpecEq::Term(&Isq::ENERGY), &QSpecEq::Term(&Isq::M)));
    pub const IONIZING_RADIATION_QUALITY_FACTOR: QSpec = QSpec::new("ionizing_radiation_quality_factor");
    pub const DOSE_EQUIVALENT: QSpec = QSpec::new("dose_equivalent").equation(&QSpecEq::Mul(
        &QSpecEq::Term(&Isq::ABSORBED_DOSE),
        &QSpecEq::Term(&Isq::IONIZING_RADIATION_QUALITY_FACTOR),
    ));

    // not in ISO 80000 but central for mechanics, thermodynamics and chemistry

    pub const POWER: QSpec = QSpec::new("power").equation(&QSpecEq::Div(
        &QSpecEq::Mul(&QSpecEq::Term(&Isq::M), &QSpecEq::Pow(&QSpecEq::Term(&Isq::L), 2)),
        &QSpecEq::Pow(&QSpecEq::Term(&Isq::T), 3),
    ));
    pub const ENERGY_DENSITY: QSpec = QSpec::new("energy_density").equation(&QSpecEq::Div(
        &QSpecEq::Term(&Isq::M),
        &QSpecEq::Mul(&QSpecEq::Term(&Isq::L), &QSpecEq::Pow(&QSpecEq::Term(&Isq::T), 2)),
    ));
}