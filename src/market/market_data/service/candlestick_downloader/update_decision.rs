use std::ops::Deref;

#[derive(Debug)]
pub struct UpdateDecision<T> {
    value: T,
    is_need_update: bool,
}

impl <T>UpdateDecision<T> {
    pub fn update(value: T) -> Self {
        return UpdateDecision {
            value,
            is_need_update: true,
        }
    }

    pub fn nothing(value: T) -> Self {
        return UpdateDecision {
            value,
            is_need_update: false,
        }
    }

    pub fn as_ref(&self) -> &T {
        return &self.value;
    }

    pub fn as_mut(&mut self) -> &mut T {
        self.is_need_update = true;
        return &mut self.value;
    }

    pub fn is_need_update(&self) -> bool {
        return self.is_need_update;
    }
}

impl <T>Deref for UpdateDecision<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}