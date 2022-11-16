use crate::shared::Shared;

pub struct Instance {}

impl Shared for Instance {
    fn shared(&self) {}
}
