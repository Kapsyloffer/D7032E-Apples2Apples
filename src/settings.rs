#![allow(unused)]
pub struct Settings
{
    pub (self) judge : bool,
    pub (self) discard: bool,
    pub (self) wild_apples: i32,
    pub (self) winreq: Vec<(i32, i32)>
}

impl Settings
{
    pub fn use_judge (&self) -> bool
    {
        return self.judge;
    }

    pub fn use_discard (&self) -> bool
    {
        return self.discard;
    }

    pub fn wild_red_apples (&self) -> i32
    {
        return self.wild_apples;
    }
}

pub fn custom_settings(j: bool, d: bool, w: i32) -> Settings
{
    return Settings
    {
        judge: j,
        discard: d,
        wild_apples: w,
        winreq: Vec::new(),
    };
}

pub fn default_settings() -> Settings
{
    return Settings
    {
        judge: true,
        discard: false,
        wild_apples: 0,
        winreq: Vec::new(),
    };
}