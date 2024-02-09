pub enum OpenState {
    Open,
    Close,
}

pub enum OpenResult {
    Safe,
    Bomb,
}

pub enum FlagState {
    Flag,
    NoFlag,
}

// pub enum Panel {
//     Blank(BlankPanel),
//     Bomb(BombPanel),
//     Boarder(BoarderPanel),
// }

pub trait Panel {
    fn flag(&mut self) {
        match self.get_flag_state() {
            FlagState::Flag => self.set_flag_state(FlagState::NoFlag),
            FlagState::NoFlag => self.set_flag_state(FlagState::Flag)
        }
    }
    fn open(&mut self) -> OpenResult;
    fn get_open_state(&self) -> &OpenState;
    fn set_open_state(&mut self, state:OpenState);
    fn get_flag_state(&self) -> &FlagState;
    fn set_flag_state(&mut self, state:FlagState);
    fn to_str(&self) -> String;
    fn set_bomb_value(&mut self, _:u8){
    }
    fn get_bomb_value(&self) -> Option<u8>{
        None
    }
    fn is_bomb(&self) -> bool{
        false
    }
    fn is_blank(&self) -> bool{
        false
    }
}

pub struct BombPanel {
    open_state: OpenState,
    flag_state: FlagState,
}

impl Panel for BombPanel{
    // Common Accessor
    fn get_open_state(&self) -> &OpenState {
        &self.open_state
    }
    fn set_open_state(&mut self, state:OpenState) {
        self.open_state = state;
    }
    fn get_flag_state(&self) -> &FlagState {
        &self.flag_state
    }
    fn set_flag_state(&mut self, state:FlagState) {
        self.flag_state = state;
    }
    // Struct Specific Functions
    fn open(&mut self) -> OpenResult {
        match self.flag_state{
            FlagState::Flag => OpenResult::Safe,
            FlagState::NoFlag => {
                self.open_state = OpenState::Open;
                OpenResult::Bomb
            }
        }
    }
    fn to_str(&self) -> String {
        match self.flag_state {
            FlagState::Flag => "F".to_string(),
            FlagState::NoFlag => {
                match self.open_state {
                    OpenState::Open => "B".to_string(),
                    OpenState::Close => "#".to_string()
                }
            }
        }
    }
    fn is_bomb(&self) -> bool {
        true
    }
}

pub fn new_bomb_panel() -> BombPanel{
    BombPanel {
        open_state: OpenState::Close,
        flag_state: FlagState::NoFlag,
    }
}

pub struct BlankPanel {
    open_state: OpenState,
    flag_state: FlagState,
    bomb_value: u8,
}

impl Panel for BlankPanel{
    // Common Accessor
    fn get_open_state(&self) -> &OpenState {
        &self.open_state
    }
    fn set_open_state(&mut self, state:OpenState) {
        self.open_state = state;
    }
    fn get_flag_state(&self) -> &FlagState {
        &self.flag_state
    }
    fn set_flag_state(&mut self, state:FlagState) {
        self.flag_state = state;
    }
    // Struct Specific Functions
    fn open(&mut self) -> OpenResult {
        self.open_state = OpenState::Open;
        OpenResult::Safe
    }
    fn to_str(&self) -> String {
        match self.flag_state {
            FlagState::Flag => "F".to_string(),
            FlagState::NoFlag => {
                match self.open_state {
                    OpenState::Close => "#".to_string(),
                    OpenState::Open => {
                        if self.bomb_value == 0 {
                            " ".to_string()
                        } else {
                            self.bomb_value.to_string()
                        }
                    }
                }
            }
        }
    }
    fn set_bomb_value(&mut self, bomb_value:u8) {
        self.bomb_value = bomb_value;
    }
    fn is_blank(&self) -> bool {
        true
    }
    fn get_bomb_value(&self) -> Option<u8> {
        Some(self.bomb_value)
    }
}

pub fn new_blank_panel() -> BlankPanel{
    BlankPanel {
        open_state: OpenState::Close,
        flag_state: FlagState::NoFlag,
        bomb_value: 0,
    }
}

pub struct BoarderPanel {
}

impl Panel for BoarderPanel {
    // Common Accessor
    fn get_open_state(&self) -> &OpenState {
        &OpenState::Open
    }
    fn set_open_state(&mut self, _:OpenState) {
    }
    fn get_flag_state(&self) -> &FlagState {
        &FlagState::NoFlag
    }
    fn set_flag_state(&mut self, _:FlagState) {
    }
    fn open(&mut self) -> OpenResult{
        OpenResult::Safe
    }
    fn to_str(&self) -> String {
        "=".to_string()
    }
    fn flag(&mut self) {
    }
}

pub fn new_boarder_panel() -> BoarderPanel{
    BoarderPanel {}
}