use crate::{
    data::{Id, PSP34Event},
    PSP34Error,
};
use ink::{prelude::{string::String, vec::Vec, vec}, storage::Mapping};

#[ink::storage_item]
#[derive(Default, Debug)]
pub struct Data {
    attributes: Mapping<(Id, Vec<u8>), Vec<u8>>,
    attribute_count: u32,
    attribute_names: Mapping<u32, Vec<u8>>,
    is_attribute: Mapping<String, bool>,
}

impl Data {
    pub fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
        self.attributes.get((&id, &key))
    }

    pub fn set_attribute(
        &mut self,
        id: Id,
        key: Vec<u8>,
        value: Vec<u8>,
    ) -> Result<Vec<PSP34Event>, PSP34Error> {
        self.attributes.insert((&id, &key), &value);
        Ok(vec![PSP34Event::AttributeSet {
            id,
            key,
            data: value,
        }])
    }

    pub fn get_attribute_count(&self) -> u32 {
        self.attribute_count
    }

    pub fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error> {
        self.set_attribute(Id::U8(0), String::from("baseURI").into_bytes(), uri.into_bytes());
        Ok(())
    }

    pub fn get_attribute_name(&self, index: u32) -> String {
        let attribute = self.attribute_names.get(&index);

        if let Some(value_in_bytes) = attribute {
            if let Ok(value_in_string) = String::from_utf8(value_in_bytes) {
                return value_in_string;
            } else {
                return String::from("");    
            }                   
        } else {
            return String::from(""); 
        }
    }

    pub fn token_uri(&self, token_id: u64) -> String {
        let value = self.get_attribute(Id::U8(0), String::from("baseURI").into_bytes());
        let mut token_uri = String::from("");

        if let Some(value_in_bytes) = value {
            if let Ok(value_in_string) = String::from_utf8(value_in_bytes) {
                token_uri = value_in_string;
            }                 
        }

        token_uri = token_uri + &String::from("1") + &String::from(".json");
        token_uri
    }

    pub fn set_multiple_attributes(
        &mut self,
        token_id: Id,
        metadata: Vec<(String, String)>
    ) -> Result<(), PSP34Error> {
        if token_id == Id::U64(0){
            return Err(PSP34Error::InvalidInput)
        }
        for (attribute, value) in &metadata {
            self.add_attribute_name(&attribute.clone().into_bytes());
            self.set_attribute(token_id.clone(), attribute.clone().into_bytes(), value.clone().into_bytes());
        }
        Ok(())
    }

    pub fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String> {
        let length = attributes.len();
        let mut ret = Vec::<String>::new();
        for i in 0..length {
            let attribute = attributes[i].clone();
            let value = self.get_attribute(token_id.clone(), attribute.into_bytes());
            
            if let Some(value_in_bytes) = value {
                if let Ok(value_in_string) = String::from_utf8(value_in_bytes) {
                    ret.push(value_in_string);
                } else {
                    ret.push(String::from(""));    
                }                   
            } else {
                ret.push(String::from(""));
            }
        }
        ret
    }

    fn add_attribute_name(&mut self, attribute_input: &Vec<u8>) -> Result<(), PSP34Error> {
        if let Ok(attr_input) = String::from_utf8((*attribute_input).clone()) {
            let exist: bool = self.is_attribute.get(&attr_input).is_some();
    
            if !exist {
                if let Some(attribute_count) = self.attribute_count.checked_add(1) {
                    self.attribute_count = attribute_count;
                    self.attribute_names.insert(&self.attribute_count, attribute_input);
                    self.is_attribute.insert(&attr_input, &true);
                    return Ok(());
                } else {
                    return Err(PSP34Error::Custom(String::from("Fail to increase attribute count"))); 
                }
            } else {
                return Err(PSP34Error::Custom(String::from("Attribute input exists"))); 
            } 
        } else {
            return Err(PSP34Error::Custom(String::from("Attribute input error")));
        }
    }
}
