pub trait CardTraits
{   
    fn get_title(&self) -> String;
    fn get_desc(&self) -> String;
}
#[derive(Clone)]
pub struct GreenCard
{
    pub title: String,
    pub desc : String
}

#[derive(Clone)]
pub struct RedCard
{
    pub title: String,
    pub desc : String
}

#[allow(dead_code)]
impl CardTraits for GreenCard
{
    fn get_title(&self) -> String 
    {
        self.title.clone()
    }
    fn get_desc(&self) -> String 
    {
        self.desc.clone()
    }
}

impl CardTraits for RedCard
{
    fn get_title(&self) -> String 
    {
        self.title.clone()
    }
    fn get_desc(&self) -> String 
    {
        self.desc.clone()
    }
}