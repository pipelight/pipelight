use crate::traits::Getters;
impl Getters<Trigger> for Trigger {
    fn get() -> Result<Vec<Trigger>> {
        let pipelines = Pipeline::get()?;
        let mut triggers = pipelines
            .iter()
            .map(|p| p.triggers.clone().unwrap_or_default())
            .collect::<Vec<Vec<Trigger>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<Trigger>>();
        triggers.sort();
        triggers.dedup();
        Ok(triggers)
    }
    fn get_by_name(name: &str) -> Result<Trigger> {
        let triggers = Trigger::get();
        let binding = triggers?;
        let trigger = binding.first().unwrap();
        Ok(trigger.to_owned())
    }
}
