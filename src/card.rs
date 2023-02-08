pub trait CardTraits
{   
    fn get_title(&self) -> String;
    fn get_desc(&self) -> String;
    fn print_card(&self);
}
pub struct GreenCard
{
    pub title: String,
    pub desc : String
}

#[allow(dead_code)]
pub struct RedCard
{
    pub title: String,
    pub desc : String
}

#[allow(dead_code)]
impl CardTraits for GreenCard
{
    fn print_card(&self)
    {
        println!("Green:\n{}\n{}\n", &self.title, &self.desc);
    }
    fn get_title(&self) -> String 
    {
        todo!()
    }
    fn get_desc(&self) -> String 
    {
        todo!()
    }
}

impl CardTraits for RedCard
{
    fn print_card(&self)
    {
        println!("RED:\n{}\n{}\n", &self.title, &self.desc);
    }
    fn get_title(&self) -> String 
    {
        todo!()
    }
    fn get_desc(&self) -> String 
    {
        todo!()
    }
}