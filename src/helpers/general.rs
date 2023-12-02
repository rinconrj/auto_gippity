use std::path;

use reqwest::Client;
use serde::de::DeserializeOwned;

const CODE_TEMPLATE_PATH: &str =
    "/home/ricardo/workspace/personal/rust/web_template/src/code_template.rs";
const EXEC_MAIN_PATH: &str = "/home/ricardo/workspace/personal/rust/web_template/src/main.rs";
const API_SCHEMA_PATH: &str =
    "/home/ricardo/workspace/personal/rust/auto_gippity/src/schemas/api_schema.json";

use crate::{
    apis::call_request::call_gpt,
    models::general::llm::{self, Message},
};

use super::command_line::PrintCommand;

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_string = ai_func(func_input);

    let msg: String = format!(
        "FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the results of functions.
    Nothing else. No commentary. Here is the input to the function: {}.
    Print out what the function will return.",
        ai_function_string, func_input
    );

    Message {
        role: "system".to_string(),
        content: msg,
    }
}

pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    let func_msg: Message = extend_ai_function(function_pass, &msg_context);

    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    let llm_response_res = call_gpt(vec![func_msg.clone()]).await;

    match llm_response_res {
        Ok(llm_response) => llm_response,
        Err(e) => call_gpt(vec![func_msg.clone()]).await.expect("Failed"),
    }
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decode_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Faled to Decode ai response from serde_json");

    decode_response
}

pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

pub fn read_code_template_contents() -> String {
    let path = String::from(CODE_TEMPLATE_PATH);
    std::fs::read_to_string(path).expect("Failed to read code template file")
}

pub fn write_code_template_contents(content: String) {
    let path = String::from(EXEC_MAIN_PATH);
    std::fs::write(path, content).expect("Failed to write code template file");
}

pub fn save_api_schema(content: String) {
    let path = String::from(API_SCHEMA_PATH);
    std::fs::write(path, content).expect("Failed to write code template file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
    #[test]
    fn test_extend_ai_function() {
        let msg: Message = extend_ai_function(convert_user_input_to_goal, "dummy");
        assert_eq!(msg.role, "system");
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_func_param = "Build a login page made with mui".to_string();
        let res = ai_task_request(
            ai_func_param,
            "Managin Agent",
            "Definig user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}
