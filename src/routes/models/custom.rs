const FORMAT: &str = "%Y-%m-%d";

pub fn serialize_dt<S>(
    dt: &Option<DateTime<Utc>>, 
    serializer: S
) -> Result<S::Ok, S::Error> 
where
    S: Serializer {
    match dt {
        Some(dt) => {
            let s = format!("{}", date.as_ref().unwrap().format(FORMAT));
            serializer.serialize_str(&s)},
        None => serializer.serialize_none(),
    }
}