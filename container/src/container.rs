use super::mem::{Address, Span};

#[derive(Clone)]
pub struct Container {
    id: ContainerID,
    metadata: ContainerMetadata,
    span: Option<Span>,
}

impl Container {
    pub fn new(memory_size: usize) -> Self {
        Self {
            id: ContainerID::new(),
            metadata: ContainerMetadata { memory_size },
            span: None,
        }
    }
    pub fn set_span(&mut self, start: Address, end: Address) {
        self.span = Some(Span::new(start, end));
    }

    pub const fn id(&self) -> &ContainerID {
        &self.id
    }

    pub const fn metadata(&self) -> &ContainerMetadata {
        &self.metadata
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
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

#[derive(Clone)]
pub struct ContainerMetadata {
    pub memory_size: usize,
}
