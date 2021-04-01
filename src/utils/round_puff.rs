use regex::Regex;
use std::string::ToString;

pub struct Dict {
    pub data: String
}

impl Dict {
    pub fn get(&self, identifier: &str) -> Option<&str> {
        // Create the regex
        let mut id_owned = identifier.to_owned();
        id_owned.push_str(":(.*?);");
        let re = Regex::new(&id_owned).unwrap();
        // Find (or not!) the expression
        let caps = re.captures(&self.data);
        if caps.is_none() {
            // Handle non existent key
            None
        } else {
            Some(caps.unwrap().get(1).unwrap().as_str())
        }
    }

    pub fn new() -> Dict {
        Dict {data: String::from("")}
    }

    pub fn set<T: ToString>(&mut self, identifier: &str, value: T) -> Result<&'static str, &'static str> {
        // Check that the strings do not contain ; or :
        let owned_id = String::from(identifier);
        let owned_value = String::from(&value.to_string());
        if owned_id.contains(";") || owned_id.contains(":") || owned_value.contains(";") || owned_value.contains(":") {
            return Err("key or value contained invalid characters")
        }
        if self.get(identifier).is_none() {
            self.data.push_str(&format!("{}:{};", identifier, value.to_string()));
            Ok("ok")
        } else {
            let old_value = &format!("{}:{}", identifier, self.get(identifier).unwrap());
            let new_value = &format!("{}:{}", identifier, value.to_string());
            self.data = self.data.replace(old_value, new_value);
            Ok("ok")
        }
    }

}