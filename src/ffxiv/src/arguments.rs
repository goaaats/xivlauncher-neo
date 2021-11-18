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
