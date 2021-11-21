use crate::blowfish::Blowfish;

const CHECKSUM_TBL: &[char; 16] = &[
    'f', 'X', '1', 'p', 'G', 't', 'd', 'S',
    '5', 'C', 'A', 'P', '4', '_', 'V', 'L',
];

const ENC_VERSION: u32 = 0x3;

#[derive(Default)]
pub(crate) struct Builder<'a> {
    //TODO (Chiv) Second tuple type cow?
    arguments: Vec<(&'a str, std::borrow::Cow<'a, str>)>,
}

impl<'a> Builder<'a> {
    pub(crate) fn append(mut self, key: &'a str, value: std::borrow::Cow<'a, str>) -> Self {
        self.arguments.push((key, value));
        self
    }

    pub(crate) fn build(self) -> String {
        // let capacity = self
        //     .arguments
        //     .iter()
        //     .map(|x| x.0.len() + x.0.len())
        //     .sum::<usize>()
        //     + self.arguments.len() * 2;
        self.arguments
            .into_iter()
            // .fold(
            // String::with_capacity(capacity),
            // |mut aggr, (key, value)| {
            //     aggr.push_str(&format!(" {}={}", key, value));
            //     aggr
            // },)
            .map(|(key, value)| format!(" {}={}", key, value))
            .collect()
    }

    pub(crate) fn build_encrypted(self) -> String {
        let key = Builder::derive_key();
        self.build_with_key(key)
    } 

    pub(crate) fn build_with_key(mut self, key: u32) -> String {
        let checksum = Builder::derive_checksum(key);

        self.arguments.insert(0, ("T", key.to_string().into()));

        // The format for encrypted arguments differs
        let arg_string: String = self.arguments.into_iter()
            .map(|(key, value)| format!(" /{} ={}", key, value))
            .collect();

        let fish = Blowfish::new(format!("{:08x}", key).as_bytes());
        let data = fish.encrypt(arg_string.as_bytes());
        let b64 = base64::encode(data)
            .replace("+", "-")
            .replace("/", "_");

        format!("//**sqex{:04}{}{}**//", ENC_VERSION, b64, checksum)
    }

    fn derive_checksum(key: u32) -> char {
        let key_index = (key & 0x000F_0000) >> 16;
        if key_index > CHECKSUM_TBL.len().try_into().unwrap() {
            panic!("key_index out of range");
        }
    
        CHECKSUM_TBL[key_index as usize]
    }
}

impl<'a, T> From<T> for Builder<'a>
where
    T: Into<Vec<(&'a str, std::borrow::Cow<'a, str>)>>,
{
    fn from(f: T) -> Self {
        Self {
            arguments: f.into(),
        }
    }
}

#[cfg(test)]
#[test]
fn build_correct_arg_string() {
    let a = Builder::default()
        .append("DEV.DataPathType", "1".into())
        .append("DEV.MaxEntitledExpansionID", "3".into())
        .append("DEV.TestSID", "a".into())
        .append("DEV.UseSqPack", "1".into())
        .append("SYS.Region", "0".into())
        .append("language", "1".into())
        .append("ver", "2021.08.15.00004".into());

    assert_eq!(a.build(), " DEV.DataPathType=1 DEV.MaxEntitledExpansionID=3 DEV.TestSID=a DEV.UseSqPack=1 SYS.Region=0 language=1 ver=2021.08.15.00004");
}

#[cfg(test)]
#[test]
fn build_encrypted_same_as_csharp() {
    let a = Builder::default()
        .append("DEV.DataPathType", "1".into())
        .append("DEV.MaxEntitledExpansionID", "3".into())
        .append("DEV.TestSID", "a".into())
        .append("DEV.UseSqPack", "1".into())
        .append("SYS.Region", "0".into())
        .append("language", "1".into())
        .append("ver", "2021.08.15.00004".into());

    assert_eq!(a.build_with_key(0), "//**sqex0003dDvxpwp6Q4Cu1Mf6pIYrV_IN8zx1PxJfur658Tc90lHJJo6rx_wuyU8AGCr0spP-VLNHUog9SIR83lWdtCRTJBru9B6cFiAr26cf1Rx1c1bnEA6-LZz2O3bAlm0xLPQdONA58sub7a6Ue0qxQHLDQNdDsHI47BAA5UWs7Zl7uAEhdueyckXusSaagpyuT0lYf**//");
}

