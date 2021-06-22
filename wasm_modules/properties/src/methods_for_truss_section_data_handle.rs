use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::{Properties, TrussSection, DeletedTrussSection};
use crate::{log, dispatch_custom_event};
use crate::
{
    EVENT_TARGET, ADD_TRUSS_SECTION_EVENT_NAME,
    UPDATE_TRUSS_SECTION_EVENT_NAME, DELETE_TRUSS_SECTION_EVENT_NAME
};


#[wasm_bindgen]
impl Properties
{
    pub fn add_truss_section(&mut self, action_id: u32, name: &str, area: f64,
        area2: Option<f64>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);

        if self.truss_sections.contains_key(&name.to_owned())
        {
            let error_message = &format!("Properties: Add truss section action: \
                Truss section with name {} does already exist!", name);
            return Err(JsValue::from(error_message));
        }
        if self.truss_sections.values().position(|truss_section|
            truss_section.data_same(area, area2)).is_some()
        {
            let error_message = &format!("Properties: Add truss section action: \
                Truss section with Area {} and Area 2 {:?} does already exist!",
                    area, area2);
            return Err(JsValue::from(error_message));
        }
        let truss_section = TrussSection::create(area, area2);
        self.truss_sections.insert(name.to_owned(), truss_section);
        let detail = json!({ "truss_section_data": { "name": name, "area": area,
            "area2": area2 },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_TRUSS_SECTION_EVENT_NAME,
            EVENT_TARGET)?;
        log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                truss sections: {:?}, deleted truss sections: {:?}, \
                beam sections: {:?}, deleted beam sections: {:?}, \
                properties: {:?}, deleted properties: {:?}",
                self.materials, self.deleted_materials,
                self.truss_sections, self.deleted_truss_sections,
                self.beam_sections, self.deleted_beam_sections,
                self.properties, self.deleted_properties)
            );
        Ok(())
    }


    pub fn update_truss_section(&mut self, action_id: u32, name: &str, area: f64,
        area2: Option<f64>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);

        if self.truss_sections.values().position(|truss_section|
            truss_section.data_same(area, area2)).is_some()
        {
            let error_message = &format!("Properties: Update truss section action: \
                Truss section with Area {} and Area 2 {:?} does already exist!",
                    area, area2);
            return Err(JsValue::from(error_message));
        }
        if let Some(truss_section) = self.truss_sections.get_mut(name)
        {
            truss_section.update(area, area2);
            let detail = json!({ "truss_section_data": { "name": name,
                "area": area, "area2": area2 },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_TRUSS_SECTION_EVENT_NAME,
                EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                truss sections: {:?}, deleted truss sections: {:?}, \
                beam sections: {:?}, deleted beam sections: {:?}, \
                properties: {:?}, deleted properties: {:?}",
                self.materials, self.deleted_materials,
                self.truss_sections, self.deleted_truss_sections,
                self.beam_sections, self.deleted_beam_sections,
                self.properties, self.deleted_properties)
            );
            Ok(())
        }
        else
        {
             let error_message = format!("Properties: Update truss section action: \
                The truss section with name {} could not be updated because it does not exist!",
                name);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn delete_truss_section(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);

        if let Some((truss_section_name, truss_section)) =
            self.truss_sections.remove_entry(&name.to_owned())
        {
            let deleted_truss_section =
                DeletedTrussSection::create(&truss_section_name, truss_section);
            self.deleted_truss_sections.insert(action_id, deleted_truss_section);
            let detail = json!({ "truss_section_data": { "name": name },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_TRUSS_SECTION_EVENT_NAME,
                EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                truss sections: {:?}, deleted truss sections: {:?}, \
                beam sections: {:?}, deleted beam sections: {:?}, \
                properties: {:?}, deleted properties: {:?}",
                self.materials, self.deleted_materials,
                self.truss_sections, self.deleted_truss_sections,
                self.beam_sections, self.deleted_beam_sections,
                self.properties, self.deleted_properties)
            );
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Delete truss section action: \
                Truss section with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_truss_section(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_truss_section) =
            self.deleted_truss_sections.remove(&action_id)
        {
            let (deleted_truss_section_name, area, area2) =
                deleted_truss_section.extract_name_and_data();
            if deleted_truss_section_name != name
            {
                let error_message = &format!("Properties: Restore truss section \
                    action: Truss section with name {} does not exist!", name);
                return Err(JsValue::from(error_message));
            }
            self.truss_sections.insert(deleted_truss_section_name.to_owned(),
               TrussSection::create(area, area2));
            let detail = json!({ "truss_section_data": { "name": deleted_truss_section_name,
                    "area": area, "area2": area2 },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_TRUSS_SECTION_EVENT_NAME,
                EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                truss sections: {:?}, deleted truss sections: {:?}, \
                beam sections: {:?}, deleted beam sections: {:?}, \
                properties: {:?}, deleted properties: {:?}",
                self.materials, self.deleted_materials,
                self.truss_sections, self.deleted_truss_sections,
                self.beam_sections, self.deleted_beam_sections,
                self.properties, self.deleted_properties)
            );
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Restore truss section action: \
                Truss section with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }
}
