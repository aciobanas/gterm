//! Translated from mp-units' `mechanics.h`:
//! <https://github.com/mpusz/mp-units/blob/master/src/systems/include/mp-units/systems/isq/mechanics.h>
//!
//! Original source is MIT licensed, Copyright (c) 2018 Mateusz Pusz.
//!

use crate::qcharacter::QCharacter;
use crate::qspec::{QSpec, QSpecEq};
use QSpecEq::{Div, Mul, Pow, Term};

use crate::isq::Isq;

impl Isq {
    pub const MASS_DENSITY: QSpec = QSpec::new("mass_density").equation(&Div(&Term(&Isq::M), &Term(&Isq::VOLUME)));

    pub const DENSITY: QSpec = Isq::MASS_DENSITY;

    pub const SPECIFIC_VOLUME: QSpec = QSpec::new("specific_volume").equation(&Pow(&Term(&Isq::MASS_DENSITY), -1));

    pub const RELATIVE_MASS_DENSITY: QSpec = QSpec::new("relative_mass_density")
        .equation(&Div(&Term(&Isq::MASS_DENSITY), &Term(&Isq::MASS_DENSITY)));

    pub const RELATIVE_DENSITY: QSpec = Isq::RELATIVE_MASS_DENSITY;

    pub const SURFACE_MASS_DENSITY: QSpec = QSpec::new("surface_mass_density")
        .equation(&Div(&Term(&Isq::M), &Term(&Isq::AREA)));

    pub const SURFACE_DENSITY: QSpec = Isq::SURFACE_MASS_DENSITY;

    pub const LINEAR_MASS_DENSITY: QSpec = QSpec::new("linear_mass_density")
        .equation(&Div(&Term(&Isq::M), &Term(&Isq::L)));

    pub const LINEAR_DENSITY: QSpec = Isq::LINEAR_MASS_DENSITY;

    pub const MOMENTUM: QSpec = QSpec::new("momentum")
        .equation(&Mul(&Term(&Isq::M), &Term(&Isq::VELOCITY)));

    pub const WEIGHT: QSpec = QSpec::new("weight")
        .equation(&Mul(&Term(&Isq::M), &Term(&Isq::ACCELERATION_OF_FREE_FALL)));

    pub const STATIC_FRICTION_FORCE: QSpec = QSpec::new("static_friction_force")
        .equation(&Term(&Isq::FORCE));

    pub const STATIC_FRICTION: QSpec = Isq::STATIC_FRICTION_FORCE;

    pub const KINETIC_FRICTION_FORCE: QSpec = QSpec::new("kinetic_friction_force")
        .equation(&Term(&Isq::FORCE));

    pub const DYNAMIC_FRICTION_FORCE: QSpec = Isq::KINETIC_FRICTION_FORCE;

    pub const ROLLING_RESISTANCE: QSpec = QSpec::new("rolling_resistance")
        .equation(&Term(&Isq::FORCE));

    pub const ROLLING_DRAG: QSpec = Isq::ROLLING_RESISTANCE;

    pub const ROLLING_FRICTION_FORCE: QSpec = Isq::ROLLING_RESISTANCE;

    /// Vector: inherited from `force` in the original.
    pub const DRAG_FORCE: QSpec = QSpec::new("drag_force")
        .equation(&Term(&Isq::FORCE));

    pub const IMPULSE: QSpec = QSpec::new("impulse")
        .equation(&Mul(&Term(&Isq::FORCE), &Term(&Isq::T)));

    pub const ANGULAR_MOMENTUM: QSpec = QSpec::new("angular_momentum")
        .equation(&Mul(&Term(&Isq::POSITION_VECTOR), &Term(&Isq::MOMENTUM)))
        .character(QCharacter::REAL_VECTOR); // cross product of `position_vector` and `momentum`

    pub const MOMENT_OF_INERTIA: QSpec = QSpec::new("moment_of_inertia")
        .equation(&Div(&Term(&Isq::ANGULAR_MOMENTUM), &Term(&Isq::ANGULAR_VELOCITY)))
        .character(QCharacter::REAL_TENSOR);

    pub const MOMENT_OF_FORCE: QSpec = QSpec::new("moment_of_force")
        .equation(&Mul(&Term(&Isq::POSITION_VECTOR), &Term(&Isq::FORCE)))
        .character(QCharacter::REAL_VECTOR); // cross product of `position_vector` and `force`

    pub const TORQUE: QSpec = QSpec::new("torque").equation(&Term(&Isq::MOMENT_OF_FORCE));

    pub const ANGULAR_IMPULSE: QSpec = QSpec::new("angular_impulse")
        .equation(&Mul(&Term(&Isq::MOMENT_OF_FORCE), &Term(&Isq::T)));

    pub const GAUGE_PRESSURE: QSpec = QSpec::new("gauge_pressure").equation(&Term(&Isq::PRESSURE));

    pub const STRESS: QSpec = QSpec::new("stress")
        .equation(&Term(&Isq::PRESSURE))
        .character(QCharacter::REAL_TENSOR);

    pub const NORMAL_STRESS: QSpec = QSpec::new("normal_stress").equation(&Term(&Isq::STRESS));

    pub const SHEAR_STRESS: QSpec = QSpec::new("shear_stress").equation(&Term(&Isq::STRESS));

    pub const STRAIN: QSpec = QSpec::new("strain").character(QCharacter::REAL_TENSOR);

    pub const RELATIVE_LINEAR_STRAIN: QSpec = QSpec::new("relative_linear_strain")
        .equation(&Div(&Term(&Isq::L), &Term(&Isq::L)));

    pub const SHEAR_STRAIN: QSpec = QSpec::new("shear_strain")
        .equation(&Div(&Term(&Isq::DISPLACEMENT), &Term(&Isq::THICKNESS)));

    pub const RELATIVE_VOLUME_STRAIN: QSpec = QSpec::new("relative_volume_strain")
        .equation(&Div(&Term(&Isq::VOLUME), &Term(&Isq::VOLUME)));

    pub const POISSON_NUMBER: QSpec = QSpec::new("Poisson_number")
        .equation(&Div(&Term(&Isq::WIDTH), &Term(&Isq::L)));

    pub const MODULUS_OF_ELASTICITY: QSpec = QSpec::new("modulus_of_elasticity")
        .equation(&Div(&Term(&Isq::NORMAL_STRESS), &Term(&Isq::RELATIVE_LINEAR_STRAIN)));

    pub const YOUNG_MODULUS: QSpec = Isq::MODULUS_OF_ELASTICITY;

    pub const MODULUS_OF_RIGIDITY: QSpec = QSpec::new("modulus_of_rigidity")
        .equation(&Div(&Term(&Isq::SHEAR_STRESS), &Term(&Isq::SHEAR_STRAIN)));

    pub const SHEAR_MODULUS: QSpec = Isq::MODULUS_OF_RIGIDITY;

    pub const MODULUS_OF_COMPRESSION: QSpec = QSpec::new("modulus_of_compression")
        .equation(&Div(&Term(&Isq::PRESSURE), &Term(&Isq::RELATIVE_VOLUME_STRAIN)));

    pub const BULK_MODULUS: QSpec = Isq::MODULUS_OF_COMPRESSION;

    pub const COMPRESSIBILITY: QSpec = QSpec::new("compressibility").equation(
        &Mul(
            &Pow(&Term(&Isq::VOLUME), -1),
            &Div(&Term(&Isq::VOLUME), &Term(&Isq::PRESSURE)),
        )
    );

    pub const SECOND_AXIAL_MOMENT_OF_AREA: QSpec = QSpec::new("second_axial_moment_of_area").equation(
        &Mul(
            &Pow(&Term(&Isq::RADIAL_DISTANCE), 2),
            &Term(&Isq::AREA),
        )
    );

    pub const SECOND_POLAR_MOMENT_OF_AREA: QSpec = QSpec::new("second_polar_moment_of_area").equation(
        &Mul(
            &Pow(&Term(&Isq::RADIAL_DISTANCE), 2),
            &Term(&Isq::AREA),
        )
    );

    pub const SECTION_MODULUS: QSpec = QSpec::new("section_modulus")
        .equation(&Div(&Term(&Isq::SECOND_AXIAL_MOMENT_OF_AREA), &Term(&Isq::RADIAL_DISTANCE)));

    pub const STATIC_FRICTION_COEFFICIENT: QSpec = QSpec::new("static_friction_coefficient")
        .equation(&Div(&Term(&Isq::STATIC_FRICTION_FORCE), &Term(&Isq::FORCE)));

    pub const STATIC_FRICTION_FACTOR: QSpec = Isq::STATIC_FRICTION_COEFFICIENT;

    pub const COEFFICIENT_OF_STATIC_FRICTION: QSpec = Isq::STATIC_FRICTION_COEFFICIENT;

    pub const KINETIC_FRICTION_FACTOR: QSpec = QSpec::new("kinetic_friction_factor")
        .equation(&Div(&Term(&Isq::KINETIC_FRICTION_FORCE), &Term(&Isq::FORCE)));

    pub const DYNAMIC_FRICTION_FACTOR: QSpec = Isq::KINETIC_FRICTION_FACTOR;

    pub const ROLLING_RESISTANCE_FACTOR: QSpec = QSpec::new("rolling_resistance_factor")
        .equation(&Div(&Term(&Isq::FORCE), &Term(&Isq::FORCE)));

    pub const DRAG_COEFFICIENT: QSpec = QSpec::new("drag_coefficient").equation(
        &Div(
            &Term(&Isq::DRAG_FORCE),
            &Mul(&Mul(&Term(&Isq::MASS_DENSITY), &Pow(&Term(&Isq::SPEED), 2)), &Term(&Isq::AREA)),
        )
    );

    pub const DRAG_FACTOR: QSpec = Isq::DRAG_COEFFICIENT;

    pub const DYNAMIC_VISCOSITY: QSpec = QSpec::new("dynamic_viscosity")
        .equation(&Div(&Mul(&Term(&Isq::SHEAR_STRESS), &Term(&Isq::L)), &Term(&Isq::VELOCITY)));

    pub const KINEMATIC_VISCOSITY: QSpec = QSpec::new("kinematic_viscosity")
        .equation(&Div(&Term(&Isq::DYNAMIC_VISCOSITY), &Term(&Isq::MASS_DENSITY)));

    // TODO: what is the correct equation here? (kept from the original)
    pub const SURFACE_TENSION: QSpec = QSpec::new("surface_tension")
        .equation(&Div(&Term(&Isq::FORCE), &Term(&Isq::L)));

    pub const MECHANICAL_POWER: QSpec = QSpec::new("mechanical_power")
        .equation(&Mul(&Term(&Isq::FORCE), &Term(&Isq::VELOCITY)));

    pub const MECHANICAL_WORK: QSpec = QSpec::new("mechanical_work")
        .equation(&Mul(&Term(&Isq::FORCE), &Term(&Isq::DISPLACEMENT)));

    pub const WORK: QSpec = Isq::MECHANICAL_WORK;

    /// Differs from ISO 80000.
    pub const MECHANICAL_ENERGY: QSpec = QSpec::new("mechanical_energy").equation(
        &Div(
            &Mul(&Term(&Isq::M), &Pow(&Term(&Isq::L), 2)),
            &Pow(&Term(&Isq::T), 2),
        )
    );

    /// Differs from ISO 80000.
    pub const POTENTIAL_ENERGY: QSpec = QSpec::new("potential_energy").equation(&Term(&Isq::MECHANICAL_ENERGY));

    /// Differs from ISO 80000.
    pub const KINETIC_ENERGY: QSpec = QSpec::new("kinetic_energy")
        .equation(&Mul(&Term(&Isq::M), &Pow(&Term(&Isq::SPEED), 2)));

    pub const MECHANICAL_EFFICIENCY: QSpec = QSpec::new("mechanical_efficiency")
        .equation(&Div(&Term(&Isq::MECHANICAL_POWER), &Term(&Isq::MECHANICAL_POWER)));

    /// Vector: inherited from `velocity` in the original.
    pub const MASS_FLOW: QSpec = QSpec::new("mass_flow")
        .equation(&Mul(&Term(&Isq::MASS_DENSITY), &Term(&Isq::VELOCITY)))
        .character(QCharacter::REAL_VECTOR);

    pub const MASS_FLOW_RATE: QSpec = QSpec::new("mass_flow_rate")
        .equation(&Mul(&Term(&Isq::MASS_FLOW), &Term(&Isq::AREA)));

    pub const MASS_CHANGE_RATE: QSpec = QSpec::new("mass_change_rate")
        .equation(&Div(&Term(&Isq::M), &Term(&Isq::T)));

    pub const VOLUME_FLOW_RATE: QSpec = QSpec::new("volume_flow_rate")
        .equation(&Mul(&Term(&Isq::VELOCITY), &Term(&Isq::AREA)));

    pub const ACTION: QSpec = QSpec::new("action").equation(&Mul(&Term(&Isq::ENERGY), &Term(&Isq::T)));
}
