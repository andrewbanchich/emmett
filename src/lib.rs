mod filters;
mod inputs;
mod outputs;
use filters::*;
use inputs::*;
use outputs::*;

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct Pipeline(pub InputBlock, pub FilterBlock, pub OutputBlock);

impl Pipeline {
    pub fn run(self) {
        let filter_receiver = self.0.run();
        let output_receiver = self.1.run(filter_receiver);
        self.2.run(output_receiver);
    }

    pub fn from_toml(path: &Path) -> Result<Pipeline, toml::de::Error> {
        // outputs
        let stdout = Output::Stdout(outputs::Stdout {
            ..Default::default()
        });

        // blocks
        let mut inputs = InputBlock(vec![]);
        let mut filters = FilterBlock(vec![]);
        let outputs = OutputBlock(vec![stdout]);

        // read pipeline config
        let mut config_file = File::open(path).expect("Couldn't open config file.");

        let mut config = String::new();
        config_file
            .read_to_string(&mut config)
            .expect("Couldn't read config file.");

        let toml: toml::Value = config.parse()?;

        if let Some(input_block) = toml.get("inputs") {
            if let Some(input_block) = input_block.as_array() {
                input_block.iter().for_each(|input| {
                    if let Some(input_block) = input.as_table() {
                        input_block.iter().for_each(|input| {
                            inputs.0.push(input.to_input());
                        })
                    }
                });
            }
        }

        if let Some(filter_block) = toml.get("filters") {
            if let Some(filter_block) = filter_block.as_array() {
                filter_block.iter().for_each(|filter| {
                    if let Some(filter_block) = filter.as_table() {
                        filter_block.iter().for_each(|filter| {
                            filters.0.push(filter.to_filter());
                        })
                    }
                });
            }
        }

        Ok(Pipeline(inputs, filters, outputs))
    }
}

trait InputConfig {
    fn to_input(&self) -> Input;
}

impl InputConfig for (&String, &toml::Value) {
    fn to_input(&self) -> Input {
        match self.0.as_str() {
            "generator" => {
                let plugin = Generator::try_from(self.1).unwrap();
                Input::Generator(plugin, None)
            }
            "http_poller" => {
                let plugin = HttpPoller::try_from(self.1).unwrap();
                Input::HttpPoller(Box::new(plugin), None)
            }
            _ => panic!("Bad configuration for {} input block.", self.0)
        }
    }
}

trait FilterConfig {
    fn to_filter(&self) -> Filter;
}

impl FilterConfig for (&String, &toml::Value) {
    fn to_filter(&self) -> Filter {
        match self.0.as_str() {
            "mutate" => {
                let plugin = Mutate::try_from(self.1).expect("Incorrect Mutate filter config.");
                Filter::Mutate(Box::new(plugin))
            }
            "json" => {
                let plugin = Json::try_from(self.1).expect("Incorrect Json filter config.");
                Filter::Json(plugin)
            }
            _ => panic!("Bad configuration for {} filter block.", self.0)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;
    use std::path::Path;

    #[test]
    fn parse_toml() {
        let pipeline = Pipeline::from_toml(Path::new("./example_configs/full.toml"));
        assert!(pipeline.is_ok());
    }
}
