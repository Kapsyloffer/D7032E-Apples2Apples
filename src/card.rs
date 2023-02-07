#[derive(Clone)]
pub struct GreenCard
{
    content: String,
}

pub struct RedCard
{
    content: String,
}

impl GreenCard
{
    fn print_card(&self)
    {
        println!("{}", &self.content);
    }
}