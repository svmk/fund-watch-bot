use crate::sec_gov::model::form_13f_component::Form13FComponent;

#[derive(Debug)]
pub struct Form13FComponentTable {
    components: Vec<Form13FComponent>,
}

impl Form13FComponentTable {
    pub fn new() -> Form13FComponentTable {
        return Form13FComponentTable {
            components: Vec::new(),
        };
    }

    pub fn push_component(&mut self, component: Form13FComponent) {
        self.components.push(component);
    }

    pub fn iter_components(&self) -> impl Iterator<Item=&Form13FComponent> {
        return self.components.iter();
    }
}