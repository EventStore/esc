use serde::ser::SerializeSeq;

pub trait ToV1 {
    type V1Type: std::fmt::Debug + serde::Serialize;
    fn to_v1(self) -> Self::V1Type;
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct StringNoQuotes(pub String);

impl std::fmt::Display for StringNoQuotes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Debug for StringNoQuotes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

pub struct List<A>(pub Vec<A>);

impl<A> std::fmt::Debug for List<A>
where
    A: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for value in self.0.iter() {
            writeln!(f, "{:?}", value)?;
        }

        Ok(())
    }
}

impl<A> serde::ser::Serialize for List<A>
where
    A: serde::ser::Serialize,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for elem in self.0.iter() {
            seq.serialize_element(elem)?;
        }

        seq.end()
    }
}
