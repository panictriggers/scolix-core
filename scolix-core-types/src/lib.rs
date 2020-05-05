
pub struct Person {
    id: i64,
    name: String,
    sur_name: String
}

impl Person{
    pub fn new(id: i64, name: String, sur_name: String) -> Person{
        Person{id, name, sur_name}
	}
    
    pub fn id(&self) -> &i64 {
        &self.id
	}
    pub fn name(&self) -> &String {
        &self.name
	}
    pub fn sur_name(&self) -> &String {
        &self.sur_name
	}
}