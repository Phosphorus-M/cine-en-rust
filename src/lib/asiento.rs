#[derive(Debug, Clone)]
pub struct Asiento {
    pub ocupado: bool,
}

impl Asiento {
    pub fn new() -> Asiento {
        Asiento {
            ocupado: false,
        }
    }
}