#[derive(Clone)]
#[allow(dead_code)]
pub struct GreenCard
{
    content: String,
}

#[allow(dead_code)]
pub struct RedCard
{
    content: String,
}

#[allow(dead_code)]
impl GreenCard
{
    fn print_card(&self)
    {
        println!("{}", &self.content);
    }
}