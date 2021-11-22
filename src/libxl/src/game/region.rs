use strum_macros::EnumString;

#[derive(Copy, Clone, EnumString)]
pub enum AccountRegion {
  Japan = 1,
  NorthAmerica = 2,
  Europe = 3,
}

impl From<u8> for AccountRegion {
    fn from(val: u8) -> Self {
        todo!()
    }
}

impl From<AccountRegion> for u8 {
    fn from(val: AccountRegion) -> Self {
        val as u8
    }
}