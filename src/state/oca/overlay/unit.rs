use crate::state::{attribute::Attribute, oca::Overlay};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use std::any::Any;
use std::collections::HashMap;

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

impl std::str::FromStr for MeasurementSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "metric" => Ok(MeasurementSystem::Metric),
            "imperial" => Ok(MeasurementSystem::Imperial),
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

impl Serialize for UnitOverlay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use std::collections::BTreeMap;

        let mut state = serializer.serialize_struct("UnitOverlay", 5)?;
        state.serialize_field("said", &self.said)?;
        state.serialize_field("type", &self.overlay_type)?;
        state.serialize_field("capture_base", &self.capture_base)?;
        state.serialize_field("measurement_system", &self.measurement_system)?;
        let sorted_attribute_units: BTreeMap<_, _> = self.attribute_units.iter().collect();
        state.serialize_field("attribute_units", &sorted_attribute_units)?;
        state.end()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UnitOverlay {
    capture_base: String,
    said: String,
    #[serde(rename = "type")]
    overlay_type: String,
    pub measurement_system: MeasurementSystem,
    pub attribute_units: HashMap<String, MeasurementUnit>,
}

impl Overlay for UnitOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&self) -> &String {
        &self.capture_base
    }
    fn capture_base_mut(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn said(&self) -> &String {
        &self.said
    }
    fn said_mut(&mut self) -> &mut String {
        &mut self.said
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
            capture_base: String::new(),
            said: String::from("############################################"),
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
