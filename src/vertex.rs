    #[derive(Copy, Clone)]
    pub struct Vertex 
    {
        pub position: [f32; 3]
    }
    pub fn macrocall()
    {
        implement_vertex!(Vertex, position);
    }