// use shard::Context;
// use shard::plug::{Command, Plug, PlugSet, PlugStatus, PlugResult};
// use shard::util;
// use discord::GetMessages;
//
// plug! { Clear => {
//     fn matches_name(&self, name: &str) -> bool { name == "clear" }
//     fn handle_command(&self, command: &Command, context: &mut Context) -> PlugResult {
//         if let Some(count) = command.arguments.get(0) {
//             let mut limit = count.parse::<u64>()? + 1;
//             while limit > 0 {
//                 let count = if limit > 50 { 50 } else { limit };
//                 let messages = context.discord.get_messages(command.message.channel_id,
//                     GetMessages::MostRecent, Some(count))?.iter().map(|m| m.id).collect::<Vec<_>>();
//                 let found = messages.len();
//                 match found {
//                     0 => limit = 0,
//                     1 => context.discord.delete_message(command.message.channel_id, messages[0])?,
//                     2 => {
//                         context.discord.delete_message(command.message.channel_id, messages[0])?;
//                         context.discord.delete_message(command.message.channel_id, messages[1])?;
//                     },
//                     _ => context.discord.delete_messages(command.message.channel_id, &messages[..])?
//                 }
//                 if found as u64 > limit {
//                     limit = 0;
//                 } else {
//                     limit = limit - found as u64;
//                 }
//             }
//
//             Ok(PlugStatus::Stop)
//         } else {
//             util::send_incorrect_argument(0, command.message.channel_id, context)?;
//             Ok(PlugStatus::Stop)
//         }
//     }
// } }
//
// pub(super) fn init(set: &mut PlugSet) {
//     set.push(Clear);
// }
