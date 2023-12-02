use crate::models::{agents::agent_traits::{FactSheet, SpecialFunctions}, agent_basic::basic_agent::BasicAgent};

pub struct ManagingAgent {
  attributes: BasicAgent,
  factsheet: FactSheet,
  agents: Vec<Box<dyn SpecialFunctions>>
}