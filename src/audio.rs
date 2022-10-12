
pub trait AudioOutput {

}

pub trait AudioInput {

}

pub trait AudioConfiguration {
    fn get_outputs(&mut self) -> Vec<String>;
    fn get_inputs(&mut self) -> Vec<String>;
}

pub fn get_configuration() -> Box<dyn AudioConfiguration> {
    Box::new(crate::cpal::get_configuration())
}