use pest_derive::Parser;
use futures::{future::lazy, sync::mpsc};

pub mod plugins;
use plugins::{input, filter, output};

#[derive(Parser)]
#[grammar = "logstash.pest"]
pub struct ConfigParser;

fn main() {

    let http_poller = input::HttpPollerInput::new(
        1000,
        vec!["http://httpbin.org/ip".to_string()]
    );

    let poller = plugins::Plugin(http_poller);

    tokio::run(lazy(move || {
        
        // let (tx, rx) = mpsc::channel(1_024);

        tokio::spawn(poller);

        println!("hello");

        Ok(())

    }));
    
}

// fn test(input: &str) -> Box<dyn Run> {
//     if input == "one" {
//         Box::new(HttpPollerInput::new(
//             "test".to_string(),
//             vec!["http://httpbin.org/ip".to_string(), "http://google.com".to_string()]
//         ))
//     } else {
//         Box::new(plugins::StdinInput::new())
//     }
// }


// #[derive(Debug)]
// pub struct Pipeline {
//     pub input: Vec<Box<dyn plugins::Plugin>>,
//     pub filter: Vec<Box<dyn plugins::Plugin>>,
//     pub output: Vec<Box<dyn plugins::Plugin>>
// }

// impl Default for Pipeline {
//     fn default() -> Self {
//         Self {
//             input: Vec::new(),
//             filter: Vec::new(),
//             output: Vec::new()
//         }
//     }
// }

// impl Pipeline {
    
//     pub fn new() -> Self {
//         Self {
//             ..Default::default()
//         }
//     }

//     pub fn from_file(path: &Path) -> Self {

//         let conf = read_to_string(path)
//             .expect("Cannot read Logstash configuration file.");
        
//         let conf = ConfigParser::parse(Rule::config, &conf)
//             .expect("Could not parse Logstash configuration file.")
//             .next()
//             .unwrap();

//         Pipeline::from_pair(conf)

//     }
    
//     fn from_pair(conf: Pair<Rule>) -> Self {

//         conf.into_inner()
//             .fold(Pipeline::new(), |mut acc, config_block| {

//                 let mut config_type = "";
                
//                 let mut config_block = config_block.into_inner()
//                     .fold(Pipeline::new(), |mut acc, config_type_plugin| {

//                         // Pest Pair is either "config_type" or "plugin"
//                         match config_type_plugin.as_rule() {
//                             Rule::config_type => {
//                                 config_type = config_type_plugin.as_str();
//                             },
//                             Rule::plugin => {

//                                 let from_pair = plugins::from_pair;
                                
//                                 match config_type {
//                                     "input" => {
//                                         acc.input.push(
//                                             from_pair(config_type, config_type_plugin));
//                                     },
//                                     "filter" => {
//                                         acc.filter.push(
//                                             from_pair(config_type, config_type_plugin));
//                                     },
//                                     "output" => {
//                                         acc.output.push(
//                                             from_pair(config_type, config_type_plugin));
//                                     },
//                                     _ => ()
//                                 }
                                
//                             },
//                             _ => ()
//                         };

//                         acc
                            
//                     });

//                 acc.input.append(&mut config_block.input);
//                 acc.filter.append(&mut config_block.filter);
//                 acc.output.append(&mut config_block.output);
//                 acc
                    
//             })
        
//     }
    
// }


// #[cfg(test)]
// mod tests {

//     #[test]
//     fn input_stdin() {
//         crate::Pipeline::from_file(std::path::Path::new("./example_configs/input/stdin.conf"));
//     }

//     #[test]
//     fn full_one() {
//         crate::Pipeline::from_file(std::path::Path::new("./example_configs/full.conf"));
//     }
    
// }
