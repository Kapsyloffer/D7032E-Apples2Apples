pub trait CardTraits
{   
    fn get_title(&self) -> String;
    fn get_desc(&self) -> String;
}

pub trait WildApple
{
    fn set_title(&mut self, new_title : String);
}
#[derive(Clone)]
#[derive(Hash)]
pub struct GreenCard
{
    title: String,
    desc : String
}

#[derive(Clone)]
#[derive(Hash)]
pub struct RedCard
{
    title: String,
    desc : String,
    wild: bool,
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

impl RedCard
{
    pub fn is_wild(&self) -> bool
    {
        return self.wild;
    }
}

impl WildApple for RedCard
{
    fn set_title(&mut self, new_title : String)
    {
        self.title = new_title;
        self.desc = String::new();
    }
}

pub fn wild_red_factory() -> RedCard
{
    return RedCard 
    { 
        title: "WILD RED APPLE".to_string(), 
        desc: "type whatever you want".to_string(), 
        wild: true
    }
}

pub fn redcard_factory(title : String, desc: String) -> RedCard
{
    return RedCard
    {
        title: title,
        desc: desc,
        wild: false,
    }
}

pub fn greencard_factory(title: String, desc: String) -> GreenCard
{
    return GreenCard 
    { 
        title: title, 
        desc: desc, 
    }
}