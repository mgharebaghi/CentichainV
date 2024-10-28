use tauri::Emitter;

//Validator turn's structure model
#[derive(Debug)]
pub struct Turn {
    pub shift: bool,
    pub waiting: u16,
}

impl Turn {
    //it means validator turn
    pub fn on(&mut self, window: &tauri::Window) {
        self.shift = true;
        self.waiting = 0;
        window.emit("patience", 0).unwrap();
        window.emit("turn", self.shift.to_string()).unwrap();
    }

    //Set validator turn off and number of waiting
    pub fn off(&mut self, waiting: u16, window: &tauri::Window) {
        self.shift = false;
        self.waiting = waiting;
        window.emit("patience", self.waiting).unwrap();
        window.emit("turn", self.shift.to_string()).unwrap();
    }

    //make a new Turn model
    pub fn new() -> Self {
        Self {
            shift: false,
            waiting: 0,
        }
    }

    pub fn waiting_update(&mut self, window: &tauri::Window) {
        self.waiting -= 1;
        window.emit("patience", self.waiting).unwrap();
        window.emit("turn", self.shift.to_string()).unwrap();
    }
}

//validator sync enumeration
pub enum Sync {
    Synced,
    NotSynced,
}

impl Sync {
    pub fn new() -> Self {
        Self::NotSynced
    }

    pub fn synced(&mut self) {
        *self = Self::Synced;
    }
}
