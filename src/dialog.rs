use crate::neuro_sdk::DialogLine;


// this is going to be one hell of a function
impl DialogLine {
    pub fn get_speaker(&self) -> &'static str {
        match self.offset_id {
            1654803 | 1816037 | 1826151 |
            1708355 => "Sam's Mom",
            1373104 | 1418785 | 1508873 |
            1571290 => "Darkness",
            1480974 => "Pajama Man and then Darkness",
            1525330 | 1562051 | 1601925 => "Pajama Man",

            1327670 | 1690926 | 1778104 |
            1840243 | 1850879 | 1884576 |
            1914070 | 1926314 | 1977686 |
            2014154 | 1997101 | 2045653 |
            1681100 => "Sam",
            _ => "Pajama Sam"
        }
    }
}