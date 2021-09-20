use crate::hello::non_world::r#abstract::attributes::{Color, Size};

struct Cloud {
    size: Size,
    pub color: Color,
}

impl Cloud {
    fn size_up(&mut self) {
        match self.size {
            Size::Small => self.size = Size::Medium,
            Size::Medium => self.size = Size::Big,
            Size::Big => {}
        }
    }
}

fn create_cloud() -> Cloud {
    Cloud {
        size: Size::Small,
        color: Color::Black,
    }
}
