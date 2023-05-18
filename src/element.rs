pub struct Element {
    pub id: u8,
    pub wins_against: u8,
    pub loses_against: u8,
}

impl Element {
   pub fn game_result(&self, other_id: u8) -> bool {
        if other_id == self.wins_against && other_id != self.loses_against {
            return true;
        } else {
            return false;
        }
    }
}