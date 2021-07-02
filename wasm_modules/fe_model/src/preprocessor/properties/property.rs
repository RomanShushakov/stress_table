use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde::Serialize;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum CrossSectionType
{
    Truss,
    Beam,
}


impl CrossSectionType
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            CrossSectionType::Truss => String::from("Truss"),
            CrossSectionType::Beam => String::from("Beam"),
        }
    }

    pub fn create(cross_section_type: &str) -> Result<Self, JsValue>
    {
        if cross_section_type.replace('"', "") ==
            CrossSectionType::Truss.as_str().to_lowercase()
        {
            Ok(CrossSectionType::Truss)
        }
        else if cross_section_type.replace('"', "") ==
            CrossSectionType::Beam.as_str().to_lowercase()
        {
            Ok(CrossSectionType::Beam)
        }
        else
        {
            let error_message = "Properties: Create cross section type action: \
                You input unknown cross section type!";
            Err(JsValue::from(error_message))
        }
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct Property
{
    material_name: String,
    cross_section_name: String,
    cross_section_type: CrossSectionType,
}


impl Property
{
    pub fn create(material_name: &str, cross_section_name: &str,
        cross_section_type: CrossSectionType) -> Self
    {

        Property
        {
            material_name: material_name.to_string(),
            cross_section_name: cross_section_name.to_string(),
            cross_section_type
        }
    }


    pub fn data_same(&self, material_name: &str, cross_section_name: &str,
        cross_section_type: &CrossSectionType) -> bool
    {
        self.material_name == material_name &&
        self.cross_section_name == cross_section_name &&
        self.cross_section_type == *cross_section_type
    }


    pub fn update(&mut self, material_name: &str, cross_section_name: &str,
        cross_section_type: CrossSectionType)
    {
        self.material_name = material_name.to_string();
        self.cross_section_name = cross_section_name.to_string();
        self.cross_section_type = cross_section_type;
    }


    pub fn extract_data(&self) -> (&str, &str, CrossSectionType)
    {
        (&self.material_name, &self.cross_section_name, self.cross_section_type.clone())
    }
}


#[derive(Debug, Clone)]
pub struct DeletedProperty
{
    name: String,
    property: Property,
}


impl DeletedProperty
{
    pub fn create(name: &str, property: Property) -> Self
    {
        DeletedProperty { name: String::from(name), property }
    }


    pub fn extract_name_and_data(&self) -> (&str, &str, &str, CrossSectionType)
    {
        let (material_name, cross_section_name, cross_section_type) =
            self.property.extract_data();
        (&self.name, material_name, cross_section_name, cross_section_type)
    }
}
