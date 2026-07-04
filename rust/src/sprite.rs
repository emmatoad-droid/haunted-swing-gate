pub enum SpriteMood {
    Irritated,
    Chaotic,
    Unhelpful,
}

pub struct Jenkins {
    pub mood: SpriteMood,
}

impl Jenkins {
    pub fn heckle(&self) {
        match self.mood {
            SpriteMood::Irritated =>
                println!("⚡ OFF YA FUCK YA DICKS!!"),
            SpriteMood::Chaotic =>
                println!("⚡ WHO SHIPPED THIS?"),
            SpriteMood::Unhelpful =>
                println!("⚡ FIGURE IT OUT THEN."),
        }
    }
}
