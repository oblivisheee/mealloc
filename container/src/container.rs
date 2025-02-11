use super::mem::Span;

pub struct Container {
    id: ContainerID,
    memory: Span,
}

impl Container {
    pub fn new() -> Self {
        Self {
            id: ContainerID::new(),
            memory: Span::empty(),
        }
    }
}

pub struct ContainerID {
    id: uuid::Uuid,
}
impl ContainerID {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
        }
    }
}
impl core::ops::Deref for ContainerID {
    type Target = uuid::Uuid;
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl core::ops::DerefMut for ContainerID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.id
    }
}
