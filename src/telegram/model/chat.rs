use crate::telegram::model::chat_id::ChatId;
use crate::repository::model::entity::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    #[serde(rename="id")]
    id: ChatId,
}

impl Chat {
    pub fn new(id: ChatId) -> Chat {
        return Chat {
            id,
        }
    }
}

impl Entity<ChatId> for Chat {
    fn get_entity_id(&self) -> &ChatId {
        return &self.id;
    }
}