use ggez::graphics::{MeshBuilder, Image, DrawParam, Color, Text, Mesh};


pub struct Drawing {
    pub params: DrawParam,
    pub image: Option<Image>,
    pub width: Option<u32>,
    pub height: Option<u32>,    
    
    pub color: Option<Color>,
    pub mesh: Option<Mesh>,
    pub meshbuilder: Option<MeshBuilder>,
    pub text: Option<Text>
}

impl Drawing {
    pub fn new_mesh(mesh: MeshBuilder) -> Self {
        Drawing {
            meshbuilder: Some(mesh),
            ..Default::default()
        }
    }

    pub fn new_image(image: Option<Image>) -> Self {
        Drawing {
            image,
            ..Default::default()
        }
    }
}

impl Default for Drawing {
    fn default() -> Self {
        Self {
            params: DrawParam::new(),
            image: Default::default(),
            color: Default::default(),
            meshbuilder: Default::default(),
            text: Default::default(),
            width: Default::default(),
            height: Default::default(),
            mesh: Default::default(),
        }
    }
}

pub struct DrawResult<'a> {
    pub params: DrawParam,
    pub drawing: &'a Drawing,
}
