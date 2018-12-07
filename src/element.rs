use vertex::{TextureVertex, Square};

pub enum ElementType
{
    AIR,
    WATER,
    EARTH,
    FIRE,
    SALT,
    SULFUR,
    MERCURY,
    ASH,
}
impl ElementType
{
    pub fn data(&self) -> ElementTypeData
    {
        match *self
        {
            ElementType::AIR => ElementTypeData
                                {
                                    x_offset: 0.00,
                                    y_offset: 0.75,
                                    value: 1,
                                    interacts: vec![ElementType::AIR],
                                    produces: Option::Some(ElementType::SALT),
                                },
            ElementType::FIRE => ElementTypeData
                                {
                                    x_offset: 0.25,
                                    y_offset: 0.75,
                                    value: 1,
                                    interacts: vec![ElementType::FIRE],
                                    produces: Option::Some(ElementType::SALT),
                                },
            ElementType::WATER => ElementTypeData
                                {
                                    x_offset: 0.50,
                                    y_offset: 0.75,
                                    value: 1,
                                    interacts: vec![ElementType::WATER],
                                    produces: Option::Some(ElementType::SALT),
                                },
            ElementType::EARTH => ElementTypeData
                                {
                                    x_offset: 0.75,
                                    y_offset: 0.75,
                                    value: 1,
                                    interacts: vec![ElementType::FIRE],
                                    produces: Option::Some(ElementType::SALT),
                                },
            ElementType::SALT => ElementTypeData
                                {
                                    x_offset: 0.00,
                                    y_offset: 0.50,
                                    value: 3,
                                    interacts: vec![ElementType::SALT],
                                    produces: Option::Some(ElementType::SULFUR),
                                },
            ElementType::SULFUR => ElementTypeData
                                {
                                    x_offset: 0.25,
                                    y_offset: 0.50,
                                    value: 3,
                                    interacts: vec![ElementType::SULFUR],
                                    produces: Option::Some(ElementType::MERCURY),
                                },
            ElementType::MERCURY => ElementTypeData
                                {
                                    x_offset: 0.50,
                                    y_offset: 0.50,
                                    value: 3,
                                    interacts: vec![ElementType::MERCURY],
                                    produces: Option::None,
                                },
            ElementType::ASH => ElementTypeData
                                {
                                    x_offset: 0.75,
                                    y_offset: 0.50,
                                    value: 3,
                                    interacts: vec![ElementType::ASH],
                                    produces: Option::Some(ElementType::ASH),
                                },
        }
    }
}

pub struct ElementTypeData
{
    pub x_offset : f32,
    pub y_offset : f32,
    pub value : u32,
    interacts : Vec<ElementType>,
    produces : Option<ElementType>,
}

pub struct Element
{
    x : f32,
    y : f32,
    t : ElementType,
}
impl Element
{
    pub fn new(x:f32, y:f32, t:ElementType) -> Element
    {
        Element
        {
            x : x,
            y : y,
            t : t,
        }
    }

    pub fn get_vertices(&self) -> Vec<TextureVertex>
    {
        let s = 2.0;
        let depth = 1.0;
        let dt = 0.25;

        let tx = self.t.data().x_offset;
        let ty = self.t.data().y_offset;

        let sq = Square {
            top_left:     TextureVertex { position: [ self.x,  self.y+s, depth], tex_coords: [ tx, ty+dt ] },
            top_right:    TextureVertex { position: [self.x+s, self.y+s, depth], tex_coords: [ tx+dt, ty+dt ] },
            bottom_left:  TextureVertex { position: [ self.x,   self.y,  depth], tex_coords: [ tx, ty ] },
            bottom_right: TextureVertex { position: [self.x+s,  self.y,  depth], tex_coords: [ tx+dt, ty ] },
        };
        sq.get_vec()
    }
}