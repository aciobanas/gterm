use gtherm_units::dims::Dims;
use gtherm_units::isq::Isq;

#[test]
fn test_expansion_and_pressure_coefficients() {
    // pow(L, -1) * (L / theta) cancels the length, leaving 1/theta
    assert_eq!(Isq::LINEAR_EXPANSION_COEFFICIENT.dims, Dims::THETA.pow(-1));
    assert_eq!(Isq::CUBIC_EXPANSION_COEFFICIENT.dims, Dims::THETA.pow(-1));
    assert_eq!(Isq::RELATIVE_PRESSURE_COEFFICIENT.dims, Dims::THETA.pow(-1));

    assert_eq!(
        Isq::PRESSURE_COEFFICIENT.dims,
        Isq::PRESSURE.dims / Dims::THETA
    );
}

#[test]
fn test_compressibilities_share_compressibility_dimension() {
    // own kind, but dimensionally equal to `compressibility` from mechanics.rs
    assert_eq!(
        Isq::ISOTHERMAL_COMPRESSIBILITY.dims,
        Isq::COMPRESSIBILITY.dims
    );
    assert_eq!(
        Isq::ISENTROPIC_COMPRESSIBILITY.dims,
        Isq::COMPRESSIBILITY.dims
    );
}

#[test]
fn test_energy_and_heat_family() {
    assert_eq!(Isq::INTERNAL_ENERGY.dims, Isq::ENERGY.dims);
    assert_eq!(Isq::HEAT.dims, Isq::INTERNAL_ENERGY.dims);
    assert_eq!(Isq::AMOUNT_OF_HEAT, Isq::HEAT);
    assert_eq!(Isq::LATENT_HEAT.dims, Isq::HEAT.dims);
}

#[test]
fn test_heat_flow_rate_and_density_of_heat_flow_rate() {
    assert_eq!(Isq::HEAT_FLOW_RATE.dims, Isq::HEAT.dims / Dims::T);
    assert_eq!(
        Isq::DENSITY_OF_HEAT_FLOW_RATE.dims,
        Isq::HEAT_FLOW_RATE.dims / Isq::AREA.dims
    );
}

#[test]
fn test_thermal_conductivity_and_heat_transfer_coefficients() {
    let expected_conductivity = Isq::DENSITY_OF_HEAT_FLOW_RATE.dims * (Dims::L / Dims::THETA);
    assert_eq!(Isq::THERMAL_CONDUCTIVITY.dims, expected_conductivity);

    let expected_transfer = Isq::DENSITY_OF_HEAT_FLOW_RATE.dims / Dims::THETA;
    assert_eq!(Isq::COEFFICIENT_OF_HEAT_TRANSFER.dims, expected_transfer);
    assert_eq!(
        Isq::SURFACE_COEFFICIENT_OF_HEAT_TRANSFER.dims,
        expected_transfer
    );

    assert_eq!(
        Isq::THERMAL_INSULANCE.dims,
        Isq::COEFFICIENT_OF_HEAT_TRANSFER.dims.pow(-1)
    );
    assert_eq!(
        Isq::COEFFICIENT_OF_THERMAL_INSULANCE,
        Isq::THERMAL_INSULANCE
    );
}

#[test]
fn test_thermal_resistance_and_conductance() {
    assert_eq!(
        Isq::THERMAL_RESISTANCE.dims,
        Dims::THETA / Isq::HEAT_FLOW_RATE.dims
    );
    assert_eq!(
        Isq::THERMAL_CONDUCTANCE.dims,
        Isq::THERMAL_RESISTANCE.dims.pow(-1)
    );
}

#[test]
fn test_heat_capacity_family() {
    assert_eq!(Isq::HEAT_CAPACITY.dims, Isq::HEAT.dims / Dims::THETA);
    assert_eq!(
        Isq::SPECIFIC_HEAT_CAPACITY.dims,
        Isq::HEAT_CAPACITY.dims / Dims::M
    );
    assert_eq!(
        Isq::SPECIFIC_HEAT_CAPACITY_AT_CONSTANT_PRESSURE.dims,
        Isq::SPECIFIC_HEAT_CAPACITY.dims
    );
    assert_eq!(
        Isq::SPECIFIC_HEAT_CAPACITY_AT_CONSTANT_VOLUME.dims,
        Isq::SPECIFIC_HEAT_CAPACITY.dims
    );
    assert_eq!(
        Isq::SPECIFIC_HEAT_CAPACITY_AT_SATURATED_VAPOUR_PRESSURE.dims,
        Isq::SPECIFIC_HEAT_CAPACITY.dims
    );
}

#[test]
fn test_thermal_diffusivity_dims() {
    let expected = Isq::THERMAL_CONDUCTIVITY.dims
        / (Isq::MASS_DENSITY.dims * Isq::SPECIFIC_HEAT_CAPACITY_AT_CONSTANT_PRESSURE.dims);
    assert_eq!(Isq::THERMAL_DIFFUSIVITY.dims, expected);
}

#[test]
fn test_ratio_and_exponent_quantities_are_dimensionless() {
    assert_eq!(Isq::RATIO_OF_SPECIFIC_HEAT_CAPACITIES.dims, Dims::ZERO);
    assert_eq!(Isq::ISENTROPIC_EXPONENT.dims, Dims::ZERO);
    assert_eq!(Isq::ISENTROPIC_EXPANSION_FACTOR, Isq::ISENTROPIC_EXPONENT);
}

#[test]
fn test_entropy_family() {
    assert_eq!(Isq::ENTROPY.dims, Isq::KINETIC_ENERGY.dims / Dims::THETA);
    assert_eq!(Isq::SPECIFIC_ENTROPY.dims, Isq::ENTROPY.dims / Dims::M);
    assert_eq!(Isq::SPECIFIC_GAS_CONSTANT.dims, Isq::ENTROPY.dims / Dims::M);
}

#[test]
fn test_thermodynamic_potentials_share_energy_dimension() {
    assert_eq!(Isq::ENTHALPY.dims, Isq::INTERNAL_ENERGY.dims);
    assert_eq!(Isq::THERMODYNAMIC_ENERGY, Isq::INTERNAL_ENERGY);

    assert_eq!(Isq::HELMHOLTZ_ENERGY.dims, Isq::INTERNAL_ENERGY.dims);
    assert_eq!(Isq::HELMHOLTZ_FUNCTION, Isq::HELMHOLTZ_ENERGY);

    assert_eq!(Isq::GIBBS_ENERGY.dims, Isq::ENTHALPY.dims);
    assert_eq!(Isq::GIBBS_FUNCTION, Isq::GIBBS_ENERGY);
}

#[test]
fn test_specific_energy_family() {
    assert_eq!(Isq::SPECIFIC_ENERGY.dims, Isq::ENERGY.dims / Dims::M);
    assert_eq!(
        Isq::SPECIFIC_INTERNAL_ENERGY.dims,
        Isq::INTERNAL_ENERGY.dims / Dims::M
    );
    assert_eq!(
        Isq::SPECIFIC_THERMODYNAMIC_ENERGY,
        Isq::SPECIFIC_INTERNAL_ENERGY
    );

    assert_eq!(Isq::SPECIFIC_ENTHALPY.dims, Isq::ENTHALPY.dims / Dims::M);

    assert_eq!(
        Isq::SPECIFIC_HELMHOLTZ_ENERGY.dims,
        Isq::HELMHOLTZ_ENERGY.dims / Dims::M
    );
    assert_eq!(
        Isq::SPECIFIC_HELMHOLTZ_FUNCTION,
        Isq::SPECIFIC_HELMHOLTZ_ENERGY
    );

    assert_eq!(
        Isq::SPECIFIC_GIBBS_ENERGY.dims,
        Isq::GIBBS_ENERGY.dims / Dims::M
    );
    assert_eq!(Isq::SPECIFIC_GIBBS_FUNCTION, Isq::SPECIFIC_GIBBS_ENERGY);
}

#[test]
fn test_massieu_and_planck_functions_and_joule_thomson_coefficient() {
    assert_eq!(
        Isq::MASSIEU_FUNCTION.dims,
        Isq::HELMHOLTZ_ENERGY.dims / Dims::THETA
    );
    assert_eq!(
        Isq::PLANCK_FUNCTION.dims,
        Isq::GIBBS_ENERGY.dims / Dims::THETA
    );
    assert_eq!(
        Isq::JOULE_THOMSON_COEFFICIENT.dims,
        Dims::THETA / Isq::PRESSURE.dims
    );
}

#[test]
fn test_efficiency_quantities_are_dimensionless() {
    // work and heat both reduce to energy dims, so the ratio cancels out
    assert_eq!(Isq::THERMODYNAMIC_EFFICIENCY.dims, Dims::ZERO);
    assert_eq!(Isq::MAXIMUM_EFFICIENCY.dims, Dims::ZERO);
}

#[test]
fn test_mass_concentration_of_water_shares_mass_density_dimension() {
    // own kind, but dimensionally equal to `mass_density` from mechanics.rs
    assert_eq!(
        Isq::MASS_CONCENTRATION_OF_WATER.dims,
        Isq::MASS_DENSITY.dims
    );
    assert_eq!(
        Isq::MASS_CONCENTRATION_OF_WATER_VAPOUR.dims,
        Isq::MASS_DENSITY.dims
    );
}

#[test]
fn test_humidity_related_ratios_are_dimensionless() {
    assert_eq!(Isq::MASS_RATIO_OF_WATER_TO_DRY_MATTER.dims, Dims::ZERO);
    assert_eq!(Isq::MASS_RATIO_OF_WATER_VAPOUR_TO_DRY_GAS.dims, Dims::ZERO);
    assert_eq!(Isq::MASS_FRACTION_OF_WATER.dims, Dims::ZERO);
    assert_eq!(
        Isq::MASS_FRACTION_OF_DRY_MATTER.dims,
        Isq::MASS_FRACTION_OF_WATER.dims
    );
    assert_eq!(Isq::RELATIVE_HUMIDITY.dims, Dims::ZERO);
    assert_eq!(Isq::RELATIVE_MASS_CONCENTRATION_OF_VAPOUR.dims, Dims::ZERO);
    assert_eq!(Isq::RELATIVE_MASS_RATIO_OF_VAPOUR.dims, Dims::ZERO);
}

#[test]
fn test_dew_point_temperature_is_theta() {
    assert_eq!(Isq::DEW_POINT_TEMPERATURE.dims, Dims::THETA);
}

#[test]
fn test_find_same_kind_across_a_term_chain() {
    // latent_heat -> heat -> internal_energy, so `heat` is their nearest common ancestor
    assert_eq!(
        Isq::HEAT.find_same_kind(&Isq::LATENT_HEAT),
        Some(&Isq::HEAT)
    );

    // same dims but neither is a `Term` ancestor of the other
    assert_eq!(
        Isq::COEFFICIENT_OF_HEAT_TRANSFER
            .find_same_kind(&Isq::SURFACE_COEFFICIENT_OF_HEAT_TRANSFER),
        None
    );
}
