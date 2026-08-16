//! Translated from mp-units' `thermodynamics.h`:
//! <https://github.com/mpusz/mp-units/blob/master/src/systems/include/mp-units/systems/isq/thermodynamics.h>
//!
//! Original source is MIT licensed, Copyright (c) 2018 Mateusz Pusz.
//!

use crate::qspec::{QSpec, QSpecEq};
use QSpecEq::{Div, Mul, Pow, Term};

use crate::isq::Isq;

// Celsius_temperature is defined in ISQ but explicitly not supported (depends on SI units, breaks non-negativity)

impl Isq {
    pub const LINEAR_EXPANSION_COEFFICIENT: QSpec = QSpec::new("linear_expansion_coefficient")
        .equation(&Mul(
            &Pow(&Term(&Isq::L), -1),
            &Div(&Term(&Isq::L), &Term(&Isq::THETA)),
        ));

    pub const CUBIC_EXPANSION_COEFFICIENT: QSpec = QSpec::new("cubic_expansion_coefficient")
        .equation(&Mul(
            &Pow(&Term(&Isq::VOLUME), -1),
            &Div(&Term(&Isq::VOLUME), &Term(&Isq::THETA)),
        ));

    pub const RELATIVE_PRESSURE_COEFFICIENT: QSpec = QSpec::new("relative_pressure_coefficient")
        .equation(&Mul(
            &Pow(&Term(&Isq::PRESSURE), -1),
            &Div(&Term(&Isq::PRESSURE), &Term(&Isq::THETA)),
        ));

    pub const PRESSURE_COEFFICIENT: QSpec = QSpec::new("pressure_coefficient")
        .equation(&Div(&Term(&Isq::PRESSURE), &Term(&Isq::THETA)));

    /// Own kind sharing `compressibility`'s dimension. TODO: how to handle the "negative" part?
    pub const ISOTHERMAL_COMPRESSIBILITY: QSpec = QSpec::new("isothermal_compressibility")
        .equation(&Mul(
            &Pow(&Term(&Isq::VOLUME), -1),
            &Div(&Term(&Isq::VOLUME), &Term(&Isq::PRESSURE)),
        ));

    /// Own kind sharing `compressibility`'s dimension. TODO: how to handle the "negative" part?
    pub const ISENTROPIC_COMPRESSIBILITY: QSpec = QSpec::new("isentropic_compressibility")
        .equation(&Mul(
            &Pow(&Term(&Isq::VOLUME), -1),
            &Div(&Term(&Isq::VOLUME), &Term(&Isq::PRESSURE)),
        ));

    // energy definition moved to mechanics

    /// Differs from ISO 80000.
    pub const INTERNAL_ENERGY: QSpec = QSpec::new("internal_energy").equation(&Term(&Isq::ENERGY));

    pub const HEAT: QSpec = QSpec::new("heat").equation(&Term(&Isq::INTERNAL_ENERGY));

    pub const AMOUNT_OF_HEAT: QSpec = Isq::HEAT;

    /// TODO: what is the correct equation here? (kept from the original)
    pub const LATENT_HEAT: QSpec = QSpec::new("latent_heat").equation(&Term(&Isq::HEAT));

    pub const HEAT_FLOW_RATE: QSpec =
        QSpec::new("heat_flow_rate").equation(&Div(&Term(&Isq::HEAT), &Term(&Isq::T)));

    pub const DENSITY_OF_HEAT_FLOW_RATE: QSpec = QSpec::new("density_of_heat_flow_rate")
        .equation(&Div(&Term(&Isq::HEAT_FLOW_RATE), &Term(&Isq::AREA)));

    pub const THERMAL_CONDUCTIVITY: QSpec = QSpec::new("thermal_conductivity").equation(&Mul(
        &Term(&Isq::DENSITY_OF_HEAT_FLOW_RATE),
        &Div(&Term(&Isq::L), &Term(&Isq::THETA)),
    ));

    pub const COEFFICIENT_OF_HEAT_TRANSFER: QSpec = QSpec::new("coefficient_of_heat_transfer")
        .equation(&Div(
            &Term(&Isq::DENSITY_OF_HEAT_FLOW_RATE),
            &Term(&Isq::THETA),
        ));

    pub const SURFACE_COEFFICIENT_OF_HEAT_TRANSFER: QSpec =
        QSpec::new("surface_coefficient_of_heat_transfer").equation(&Div(
            &Term(&Isq::DENSITY_OF_HEAT_FLOW_RATE),
            &Term(&Isq::THETA),
        ));

    pub const THERMAL_INSULANCE: QSpec = QSpec::new("thermal_insulance")
        .equation(&Pow(&Term(&Isq::COEFFICIENT_OF_HEAT_TRANSFER), -1));

    pub const COEFFICIENT_OF_THERMAL_INSULANCE: QSpec = Isq::THERMAL_INSULANCE;

    pub const THERMAL_RESISTANCE: QSpec = QSpec::new("thermal_resistance")
        .equation(&Div(&Term(&Isq::THETA), &Term(&Isq::HEAT_FLOW_RATE)));

    pub const THERMAL_CONDUCTANCE: QSpec =
        QSpec::new("thermal_conductance").equation(&Pow(&Term(&Isq::THERMAL_RESISTANCE), -1));

    pub const HEAT_CAPACITY: QSpec =
        QSpec::new("heat_capacity").equation(&Div(&Term(&Isq::HEAT), &Term(&Isq::THETA)));

    pub const SPECIFIC_HEAT_CAPACITY: QSpec = QSpec::new("specific_heat_capacity")
        .equation(&Div(&Term(&Isq::HEAT_CAPACITY), &Term(&Isq::M)));

    pub const SPECIFIC_HEAT_CAPACITY_AT_CONSTANT_PRESSURE: QSpec =
        QSpec::new("specific_heat_capacity_at_constant_pressure")
            .equation(&Term(&Isq::SPECIFIC_HEAT_CAPACITY));

    pub const SPECIFIC_HEAT_CAPACITY_AT_CONSTANT_VOLUME: QSpec =
        QSpec::new("specific_heat_capacity_at_constant_volume")
            .equation(&Term(&Isq::SPECIFIC_HEAT_CAPACITY));

    pub const SPECIFIC_HEAT_CAPACITY_AT_SATURATED_VAPOUR_PRESSURE: QSpec =
        QSpec::new("specific_heat_capacity_at_saturated_vapour_pressure")
            .equation(&Term(&Isq::SPECIFIC_HEAT_CAPACITY));

    pub const THERMAL_DIFFUSIVITY: QSpec = QSpec::new("thermal_diffusivity").equation(&Div(
        &Term(&Isq::THERMAL_CONDUCTIVITY),
        &Mul(
            &Term(&Isq::MASS_DENSITY),
            &Term(&Isq::SPECIFIC_HEAT_CAPACITY_AT_CONSTANT_PRESSURE),
        ),
    ));

    pub const RATIO_OF_SPECIFIC_HEAT_CAPACITIES: QSpec =
        QSpec::new("ratio_of_specific_heat_capacities").equation(&Div(
            &Term(&Isq::SPECIFIC_HEAT_CAPACITY_AT_CONSTANT_PRESSURE),
            &Term(&Isq::SPECIFIC_HEAT_CAPACITY_AT_CONSTANT_VOLUME),
        ));

    pub const ISENTROPIC_EXPONENT: QSpec = QSpec::new("isentropic_exponent").equation(&Mul(
        &Div(&Term(&Isq::VOLUME), &Term(&Isq::PRESSURE)),
        &Div(&Term(&Isq::PRESSURE), &Term(&Isq::VOLUME)),
    ));

    pub const ISENTROPIC_EXPANSION_FACTOR: QSpec = Isq::ISENTROPIC_EXPONENT;

    pub const ENTROPY: QSpec =
        QSpec::new("entropy").equation(&Div(&Term(&Isq::KINETIC_ENERGY), &Term(&Isq::THETA)));

    pub const SPECIFIC_ENTROPY: QSpec =
        QSpec::new("specific_entropy").equation(&Div(&Term(&Isq::ENTROPY), &Term(&Isq::M)));

    pub const ENTHALPY: QSpec = QSpec::new("enthalpy").equation(&Term(&Isq::INTERNAL_ENERGY));

    pub const THERMODYNAMIC_ENERGY: QSpec = Isq::INTERNAL_ENERGY;

    pub const HELMHOLTZ_ENERGY: QSpec =
        QSpec::new("Helmholtz_energy").equation(&Term(&Isq::INTERNAL_ENERGY));

    pub const HELMHOLTZ_FUNCTION: QSpec = Isq::HELMHOLTZ_ENERGY;

    pub const GIBBS_ENERGY: QSpec = QSpec::new("Gibbs_energy").equation(&Term(&Isq::ENTHALPY));

    pub const GIBBS_FUNCTION: QSpec = Isq::GIBBS_ENERGY;

    pub const SPECIFIC_ENERGY: QSpec =
        QSpec::new("specific_energy").equation(&Div(&Term(&Isq::ENERGY), &Term(&Isq::M)));

    pub const SPECIFIC_INTERNAL_ENERGY: QSpec = QSpec::new("specific_internal_energy")
        .equation(&Div(&Term(&Isq::INTERNAL_ENERGY), &Term(&Isq::M)));

    pub const SPECIFIC_THERMODYNAMIC_ENERGY: QSpec = Isq::SPECIFIC_INTERNAL_ENERGY;

    pub const SPECIFIC_ENTHALPY: QSpec =
        QSpec::new("specific_enthalpy").equation(&Div(&Term(&Isq::ENTHALPY), &Term(&Isq::M)));

    pub const SPECIFIC_HELMHOLTZ_ENERGY: QSpec = QSpec::new("specific_Helmholtz_energy")
        .equation(&Div(&Term(&Isq::HELMHOLTZ_ENERGY), &Term(&Isq::M)));

    pub const SPECIFIC_HELMHOLTZ_FUNCTION: QSpec = Isq::SPECIFIC_HELMHOLTZ_ENERGY;

    pub const SPECIFIC_GIBBS_ENERGY: QSpec = QSpec::new("specific_Gibbs_energy")
        .equation(&Div(&Term(&Isq::GIBBS_ENERGY), &Term(&Isq::M)));

    pub const SPECIFIC_GIBBS_FUNCTION: QSpec = Isq::SPECIFIC_GIBBS_ENERGY;

    pub const MASSIEU_FUNCTION: QSpec = QSpec::new("Massieu_function")
        .equation(&Div(&Term(&Isq::HELMHOLTZ_ENERGY), &Term(&Isq::THETA)));

    pub const PLANCK_FUNCTION: QSpec =
        QSpec::new("Planck_function").equation(&Div(&Term(&Isq::GIBBS_ENERGY), &Term(&Isq::THETA)));

    pub const JOULE_THOMSON_COEFFICIENT: QSpec = QSpec::new("Joule_Thomson_coefficient")
        .equation(&Div(&Term(&Isq::THETA), &Term(&Isq::PRESSURE)));

    pub const THERMODYNAMIC_EFFICIENCY: QSpec =
        QSpec::new("thermodynamic_efficiency").equation(&Div(&Term(&Isq::WORK), &Term(&Isq::HEAT)));

    pub const MAXIMUM_EFFICIENCY: QSpec =
        QSpec::new("maximum_efficiency").equation(&Div(&Term(&Isq::THETA), &Term(&Isq::THETA)));

    pub const SPECIFIC_GAS_CONSTANT: QSpec =
        QSpec::new("specific_gas_constant").equation(&Div(&Term(&Isq::ENTROPY), &Term(&Isq::M)));

    pub const MASS_CONCENTRATION_OF_WATER: QSpec = QSpec::new("mass_concentration_of_water")
        .equation(&Div(&Term(&Isq::M), &Term(&Isq::VOLUME)));

    pub const MASS_CONCENTRATION_OF_WATER_VAPOUR: QSpec =
        QSpec::new("mass_concentration_of_water_vapour")
            .equation(&Div(&Term(&Isq::M), &Term(&Isq::VOLUME)));

    pub const MASS_RATIO_OF_WATER_TO_DRY_MATTER: QSpec =
        QSpec::new("mass_ratio_of_water_to_dry_matter")
            .equation(&Div(&Term(&Isq::M), &Term(&Isq::M)));

    pub const MASS_RATIO_OF_WATER_VAPOUR_TO_DRY_GAS: QSpec =
        QSpec::new("mass_ratio_of_water_vapour_to_dry_gas")
            .equation(&Div(&Term(&Isq::M), &Term(&Isq::M)));

    pub const MASS_FRACTION_OF_WATER: QSpec = QSpec::new("mass_fraction_of_water").equation(&Div(
        &Term(&Isq::MASS_RATIO_OF_WATER_TO_DRY_MATTER),
        &Term(&Isq::MASS_RATIO_OF_WATER_TO_DRY_MATTER),
    ));

    pub const MASS_FRACTION_OF_DRY_MATTER: QSpec =
        QSpec::new("mass_fraction_of_dry_matter").equation(&Term(&Isq::MASS_FRACTION_OF_WATER));

    pub const RELATIVE_HUMIDITY: QSpec = QSpec::new("relative_humidity")
        .equation(&Div(&Term(&Isq::PRESSURE), &Term(&Isq::PRESSURE)));

    pub const RELATIVE_MASS_CONCENTRATION_OF_VAPOUR: QSpec =
        QSpec::new("relative_mass_concentration_of_vapour").equation(&Div(
            &Term(&Isq::MASS_CONCENTRATION_OF_WATER_VAPOUR),
            &Term(&Isq::MASS_CONCENTRATION_OF_WATER_VAPOUR),
        ));

    pub const RELATIVE_MASS_RATIO_OF_VAPOUR: QSpec = QSpec::new("relative_mass_ratio_of_vapour")
        .equation(&Div(
            &Term(&Isq::MASS_RATIO_OF_WATER_VAPOUR_TO_DRY_GAS),
            &Term(&Isq::MASS_RATIO_OF_WATER_VAPOUR_TO_DRY_GAS),
        ));

    pub const DEW_POINT_TEMPERATURE: QSpec =
        QSpec::new("dew_point_temperature").equation(&Term(&Isq::THETA));
}

// some compile-time assertions to ensure that the const functions are working as expected
const _: () = assert!(Isq::HEAT_FLOW_RATE.dims.const_eq(&Isq::HEAT.dims.const_div(&Isq::T.dims)));
const _: () = assert!(Isq::HEAT_CAPACITY.dims.const_eq(&Isq::HEAT.dims.const_div(&Isq::THETA.dims)));
const _: () = assert!(Isq::SPECIFIC_HEAT_CAPACITY.dims.const_eq(&Isq::HEAT_CAPACITY.dims.const_div(&Isq::M.dims)));
const _: () = assert!(Isq::THERMAL_RESISTANCE.dims.const_eq(&Isq::THETA.dims.const_div(&Isq::HEAT_FLOW_RATE.dims)));
const _: () = assert!(Isq::THERMAL_CONDUCTANCE.dims.const_eq(&Isq::THERMAL_RESISTANCE.dims.pow(-1)));
const _: () = assert!(Isq::ENTROPY.dims.const_eq(&Isq::KINETIC_ENERGY.dims.const_div(&Isq::THETA.dims)));

// ratios of like dimensions should reduce to dimensionless
const _: () = assert!(Isq::RATIO_OF_SPECIFIC_HEAT_CAPACITIES.dims.const_eq(&crate::dims::Dims::ZERO));
const _: () = assert!(Isq::MAXIMUM_EFFICIENCY.dims.const_eq(&crate::dims::Dims::ZERO));
const _: () = assert!(Isq::RELATIVE_HUMIDITY.dims.const_eq(&crate::dims::Dims::ZERO));
