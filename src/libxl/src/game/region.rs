use strum_macros::EnumString;

#[derive(Copy, Clone, Debug, EnumString)]
pub enum AccountRegion {
  Japan = 1,
  NorthAmerica = 2,
  Europe = 3,
}

// TODO(goat): Is it possible to macro this?
impl From<u8> for AccountRegion {
    fn from(val: u8) -> Self {
        match val {
            1 => AccountRegion::Japan,
            2 => AccountRegion::NorthAmerica,
            3 => AccountRegion::Europe,
            _ => panic!("Unimplemented region")
        }
    }
}

impl From<AccountRegion> for u8 {
    fn from(val: AccountRegion) -> Self {
        val as u8
    }
}