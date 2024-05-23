use crate::rule::IntoRuleList;

pub fn string_validate<R: IntoRuleList<String, M>, M>(
    value: String,
    rules: R,
) -> Result<String, Vec<M>> {
    let list = rules.into_list();
    let mut string = value;
    let results = list.call(&mut string);

    if results.is_empty() {
        Ok(string)
    } else {
        Err(results)
    }
}

pub fn string_validate_ref<R: IntoRuleList<String, M>, M>(
    value: &mut String,
    rules: R,
) -> Result<(), Vec<M>> {
    let results = rules.into_list().call(value);
    if results.is_empty() {
        Ok(())
    } else {
        Err(results)
    }
}
