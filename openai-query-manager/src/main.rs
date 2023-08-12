use std::error::Error;

use async_openai::Client;
use async_openai::types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let client = Client::new();
    //
    // let request = CreateChatCompletionRequestArgs::default()
    //     .max_tokens(1024u16)
    //     .model("gpt-3.5-turbo")
    //     .messages([
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::System)
    //             .content("You are a story teller from the future.")
    //             .build()?,
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::User)
    //             .content("Tell me a story that will make me feel positive about humanity's future")
    //             .build()?
    //     ])
    //     .build()?;
    //
    // let response = client.chat().create(request).await?;
    //
    //
    // println!("{:?}", response);
    //
    // for choice in response.choices{
    //     println!(
    //         "{}: Role: {}  Content: {:?}",
    //         choice.index, choice.message.role, choice.message.content
    //     )
    // }

    Ok(())
}
