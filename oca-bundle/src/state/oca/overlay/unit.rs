use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};
use std::any::Any;
use std::collections::HashMap;
use said::{sad::SAD, sad::SerializationFormats, derivation::HashFunctionCode};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
pub enum MeasurementSystem {
    Metric,
    Imperial,
}

pub struct AttributeUnit {
    pub measurement_system: MeasurementSystem,
    pub unit: MeasurementUnit,
}
#[derive(Eq, Hash, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MeasurementUnit {
    Metric(MetricUnit),
    Imperial(ImperialUnit),
}

#[derive(Eq, Hash, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MetricUnit {
    Kilogram,
    Gram,
    Milligram,
    Liter,
    Milliliter,
    Meter,
    Centimeter,
    Millimeter,
    Inch,
    Foot,
    Yard,
    Mile,
    Celsius,
    Fahrenheit,
    Kelvin,
    Percent,
    Count,
    Other,
}

#[derive(Eq, Hash, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ImperialUnit {
    Pound,
    Ounce,
    Gallon,
    Quart,
    Pint,
    FluidOunce,
    Inch,
    Foot,
    Yard,
    Mile,
    Celsius,
    Fahrenheit,
    Kelvin,
    Percent,
    Count,
    Other,
}

impl std::str::FromStr for MetricUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kilogram" => Ok(MetricUnit::Kilogram),
            "kg" => Ok(MetricUnit::Kilogram),
            "gram" => Ok(MetricUnit::Gram),
            "g" => Ok(MetricUnit::Gram),
            "milligram" => Ok(MetricUnit::Milligram),
            "mg" => Ok(MetricUnit::Milligram),
            "liter" => Ok(MetricUnit::Liter),
            "l" => Ok(MetricUnit::Liter),
            "meter" => Ok(MetricUnit::Meter),
            "m" => Ok(MetricUnit::Meter),
            "milliliter" => Ok(MetricUnit::Milliliter),
            "ml" => Ok(MetricUnit::Milliliter),
            "centimeter" => Ok(MetricUnit::Centimeter),
            "cm" => Ok(MetricUnit::Centimeter),
            "millimeter" => Ok(MetricUnit::Millimeter),
            "mm" => Ok(MetricUnit::Millimeter),
            "inch" => Ok(MetricUnit::Inch),
            "in" => Ok(MetricUnit::Inch),
            "foot" => Ok(MetricUnit::Foot),
            "ft" => Ok(MetricUnit::Foot),
            "yard" => Ok(MetricUnit::Yard),
            "yd" => Ok(MetricUnit::Yard),
            "mile" => Ok(MetricUnit::Mile),
            "mi" => Ok(MetricUnit::Mile),
            "celsius" => Ok(MetricUnit::Celsius),
            "c" => Ok(MetricUnit::Celsius),
            "fahrenheit" => Ok(MetricUnit::Fahrenheit),
            "f" => Ok(MetricUnit::Fahrenheit),
            "kelvin" => Ok(MetricUnit::Kelvin),
            "k" => Ok(MetricUnit::Kelvin),
            "percent" => Ok(MetricUnit::Percent),
            "%" => Ok(MetricUnit::Percent),
            "count" => Ok(MetricUnit::Count),
            "other" => Ok(MetricUnit::Other),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for ImperialUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pound" => Ok(ImperialUnit::Pound),
            "lb" => Ok(ImperialUnit::Pound),
            "ounce" => Ok(ImperialUnit::Ounce),
            "oz" => Ok(ImperialUnit::Ounce),
            "gallon" => Ok(ImperialUnit::Gallon),
            "gal" => Ok(ImperialUnit::Gallon),
            "quart" => Ok(ImperialUnit::Quart),
            "qt" => Ok(ImperialUnit::Quart),
            "pint" => Ok(ImperialUnit::Pint),
            "pt" => Ok(ImperialUnit::Pint),
            "fluid ounce" => Ok(ImperialUnit::FluidOunce),
            "fl oz" => Ok(ImperialUnit::FluidOunce),
            "inch" => Ok(ImperialUnit::Inch),
            "in" => Ok(ImperialUnit::Inch),
            "foot" => Ok(ImperialUnit::Foot),
            "ft" => Ok(ImperialUnit::Foot),
            "yard" => Ok(ImperialUnit::Yard),
            "yd" => Ok(ImperialUnit::Yard),
            "mile" => Ok(ImperialUnit::Mile),
            "mi" => Ok(ImperialUnit::Mile),
            "celsius" => Ok(ImperialUnit::Celsius),
            "c" => Ok(ImperialUnit::Celsius),
            "fahrenheit" => Ok(ImperialUnit::Fahrenheit),
            "f" => Ok(ImperialUnit::Fahrenheit),
            "kelvin" => Ok(ImperialUnit::Kelvin),
            "k" => Ok(ImperialUnit::Kelvin),
            "percent" => Ok(ImperialUnit::Percent),
            "%" => Ok(ImperialUnit::Percent),
            "count" => Ok(ImperialUnit::Count),
            "other" => Ok(ImperialUnit::Other),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for MeasurementSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "metric" => Ok(MeasurementSystem::Metric),
            "si" => Ok(MeasurementSystem::Metric),
            "imperial" => Ok(MeasurementSystem::Imperial),
            "iu" => Ok(MeasurementSystem::Imperial),
            _ => Err(()),
        }
    }
}

pub trait Unit {
    fn set_unit(&mut self, attr_unit: AttributeUnit) -> ();
}

impl Unit for Attribute {
    fn set_unit(&mut self, attr_unit: AttributeUnit) -> () {
        match self.units {
            Some(ref mut units) => {
                units.insert(attr_unit.measurement_system, attr_unit.unit);
            }
            None => {
                let mut units = HashMap::new();
                units.insert(attr_unit.measurement_system, attr_unit.unit);
                self.units = Some(units);
            }
        }
    }
}

pub fn serialize_attributes<S>(attributes: &HashMap<String, MeasurementUnit>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use std::collections::BTreeMap;

    let mut ser = s.serialize_map(Some(attributes.len()))?;
    let sorted_attributes: BTreeMap<_, _> = attributes.iter().collect();
    for (k, v) in sorted_attributes {
        ser.serialize_entry(k, v)?;
    }
    ser.end()
}

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct UnitOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: String,
    capture_base: Option<said::SelfAddressingIdentifier>,
    pub measurement_system: MeasurementSystem,
    #[serde(serialize_with = "serialize_attributes")]
    pub attribute_units: HashMap<String, MeasurementUnit>,
}

impl Overlay for UnitOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.capture_base
    }
    fn set_capture_base(&mut self, said: &said::SelfAddressingIdentifier) {
        self.capture_base = Some(said.clone());
    }
    fn said(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.said
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }

    fn attributes(&self) -> Vec<&String> {
        self.attribute_units.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(units) = &attribute.units {
            if let Some(unit) = units.get(&self.measurement_system) {
                self.attribute_units
                    .insert(attribute.name.clone(), unit.clone());
            }
        }
    }
}
impl UnitOverlay {
    pub fn new(measurement_system: MeasurementSystem) -> UnitOverlay {
        UnitOverlay {
            capture_base: None,
            said: None,
            overlay_type: "spec/overlays/unit/1.0".to_string(),
            measurement_system,
            attribute_units: HashMap::new(),
        }
    }

    pub fn measurement_system(&self) -> Option<&MeasurementSystem> {
        Some(&self.measurement_system)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::state::attribute::Attribute;
    use crate::state::oca::overlay::unit::MeasurementSystem;

    #[test]
    fn test_set_unit() {
        let attribute = cascade! {
            Attribute::new("test".to_string());
            ..set_unit(AttributeUnit { measurement_system: MeasurementSystem::Metric, unit: MeasurementUnit::Metric(MetricUnit::Kilogram)});
            ..set_unit(AttributeUnit { measurement_system: MeasurementSystem::Imperial, unit: MeasurementUnit::Imperial(ImperialUnit::Pound) });
        };

        // assert eq units
        assert_eq!(
            attribute.units,
            Some({
                let mut units = HashMap::new();
                units.insert(MeasurementSystem::Metric, MeasurementUnit::Metric(MetricUnit::Kilogram));
                units.insert(MeasurementSystem::Imperial, MeasurementUnit::Imperial(ImperialUnit::Pound));
                units
            })
        );

    }
}
