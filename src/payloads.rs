//! Request data sent to Telegram.

/// This module re-exports all the setters traits as `_`.
///
/// When used with a glob import:
///
/// ```
/// use teloxide_core::payloads::setters::*;
/// ```
///
/// It allows you to use all the payloads setters, without polluting your
/// namespace.
pub mod setters;

// This block is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS BLOCK**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
mod add_sticker_to_set;
mod answer_callback_query;
mod answer_inline_query;
mod answer_pre_checkout_query;
mod answer_shipping_query;
mod approve_chat_join_request;
mod ban_chat_member;
mod ban_chat_sender_chat;
mod close;
mod copy_message;
mod create_chat_invite_link;
mod create_new_sticker_set;
mod decline_chat_join_request;
mod delete_chat_photo;
mod delete_chat_sticker_set;
mod delete_message;
mod delete_my_commands;
mod delete_sticker_from_set;
mod delete_webhook;
mod edit_chat_invite_link;
mod edit_message_caption;
mod edit_message_caption_inline;
mod edit_message_live_location;
mod edit_message_live_location_inline;
mod edit_message_media;
mod edit_message_media_inline;
mod edit_message_reply_markup;
mod edit_message_reply_markup_inline;
mod edit_message_text;
mod edit_message_text_inline;
mod export_chat_invite_link;
mod forward_message;
mod get_chat;
mod get_chat_administrators;
mod get_chat_member;
mod get_chat_member_count;
mod get_chat_members_count;
mod get_file;
mod get_game_high_scores;
mod get_me;
mod get_my_commands;
mod get_sticker_set;
mod get_updates;
mod get_user_profile_photos;
mod get_webhook_info;
mod kick_chat_member;
mod leave_chat;
mod log_out;
mod pin_chat_message;
mod promote_chat_member;
mod restrict_chat_member;
mod revoke_chat_invite_link;
mod send_animation;
mod send_audio;
mod send_chat_action;
mod send_contact;
mod send_dice;
mod send_document;
mod send_game;
mod send_invoice;
mod send_location;
mod send_media_group;
mod send_message;
mod send_photo;
mod send_poll;
mod send_sticker;
mod send_venue;
mod send_video;
mod send_video_note;
mod send_voice;
mod set_chat_administrator_custom_title;
mod set_chat_description;
mod set_chat_permissions;
mod set_chat_photo;
mod set_chat_sticker_set;
mod set_chat_title;
mod set_game_score;
mod set_game_score_inline;
mod set_my_commands;
mod set_passport_data_errors;
mod set_sticker_position_in_set;
mod set_sticker_set_thumb;
mod set_webhook;
mod stop_message_live_location;
mod stop_message_live_location_inline;
mod stop_poll;
mod unban_chat_member;
mod unban_chat_sender_chat;
mod unpin_all_chat_messages;
mod unpin_chat_message;
mod upload_sticker_file;

pub use add_sticker_to_set::{AddStickerToSet, AddStickerToSetSetters};
pub use answer_callback_query::{AnswerCallbackQuery, AnswerCallbackQuerySetters};
pub use answer_inline_query::{AnswerInlineQuery, AnswerInlineQuerySetters};
pub use answer_pre_checkout_query::{AnswerPreCheckoutQuery, AnswerPreCheckoutQuerySetters};
pub use answer_shipping_query::{AnswerShippingQuery, AnswerShippingQuerySetters};
pub use approve_chat_join_request::{ApproveChatJoinRequest, ApproveChatJoinRequestSetters};
pub use ban_chat_member::{BanChatMember, BanChatMemberSetters};
pub use ban_chat_sender_chat::{BanChatSenderChat, BanChatSenderChatSetters};
pub use close::{Close, CloseSetters};
pub use copy_message::{CopyMessage, CopyMessageSetters};
pub use create_chat_invite_link::{CreateChatInviteLink, CreateChatInviteLinkSetters};
pub use create_new_sticker_set::{CreateNewStickerSet, CreateNewStickerSetSetters};
pub use decline_chat_join_request::{DeclineChatJoinRequest, DeclineChatJoinRequestSetters};
pub use delete_chat_photo::{DeleteChatPhoto, DeleteChatPhotoSetters};
pub use delete_chat_sticker_set::{DeleteChatStickerSet, DeleteChatStickerSetSetters};
pub use delete_message::{DeleteMessage, DeleteMessageSetters};
pub use delete_my_commands::{DeleteMyCommands, DeleteMyCommandsSetters};
pub use delete_sticker_from_set::{DeleteStickerFromSet, DeleteStickerFromSetSetters};
pub use delete_webhook::{DeleteWebhook, DeleteWebhookSetters};
pub use edit_chat_invite_link::{EditChatInviteLink, EditChatInviteLinkSetters};
pub use edit_message_caption::{EditMessageCaption, EditMessageCaptionSetters};
pub use edit_message_caption_inline::{EditMessageCaptionInline, EditMessageCaptionInlineSetters};
pub use edit_message_live_location::{EditMessageLiveLocation, EditMessageLiveLocationSetters};
pub use edit_message_live_location_inline::{
    EditMessageLiveLocationInline, EditMessageLiveLocationInlineSetters,
};
pub use edit_message_media::{EditMessageMedia, EditMessageMediaSetters};
pub use edit_message_media_inline::{EditMessageMediaInline, EditMessageMediaInlineSetters};
pub use edit_message_reply_markup::{EditMessageReplyMarkup, EditMessageReplyMarkupSetters};
pub use edit_message_reply_markup_inline::{
    EditMessageReplyMarkupInline, EditMessageReplyMarkupInlineSetters,
};
pub use edit_message_text::{EditMessageText, EditMessageTextSetters};
pub use edit_message_text_inline::{EditMessageTextInline, EditMessageTextInlineSetters};
pub use export_chat_invite_link::{ExportChatInviteLink, ExportChatInviteLinkSetters};
pub use forward_message::{ForwardMessage, ForwardMessageSetters};
pub use get_chat::{GetChat, GetChatSetters};
pub use get_chat_administrators::{GetChatAdministrators, GetChatAdministratorsSetters};
pub use get_chat_member::{GetChatMember, GetChatMemberSetters};
pub use get_chat_member_count::{GetChatMemberCount, GetChatMemberCountSetters};
pub use get_chat_members_count::{GetChatMembersCount, GetChatMembersCountSetters};
pub use get_file::{GetFile, GetFileSetters};
pub use get_game_high_scores::{GetGameHighScores, GetGameHighScoresSetters};
pub use get_me::{GetMe, GetMeSetters};
pub use get_my_commands::{GetMyCommands, GetMyCommandsSetters};
pub use get_sticker_set::{GetStickerSet, GetStickerSetSetters};
pub use get_updates::{GetUpdates, GetUpdatesSetters};
pub use get_user_profile_photos::{GetUserProfilePhotos, GetUserProfilePhotosSetters};
pub use get_webhook_info::{GetWebhookInfo, GetWebhookInfoSetters};
pub use kick_chat_member::{KickChatMember, KickChatMemberSetters};
pub use leave_chat::{LeaveChat, LeaveChatSetters};
pub use log_out::{LogOut, LogOutSetters};
pub use pin_chat_message::{PinChatMessage, PinChatMessageSetters};
pub use promote_chat_member::{PromoteChatMember, PromoteChatMemberSetters};
pub use restrict_chat_member::{RestrictChatMember, RestrictChatMemberSetters};
pub use revoke_chat_invite_link::{RevokeChatInviteLink, RevokeChatInviteLinkSetters};
pub use send_animation::{SendAnimation, SendAnimationSetters};
pub use send_audio::{SendAudio, SendAudioSetters};
pub use send_chat_action::{SendChatAction, SendChatActionSetters};
pub use send_contact::{SendContact, SendContactSetters};
pub use send_dice::{SendDice, SendDiceSetters};
pub use send_document::{SendDocument, SendDocumentSetters};
pub use send_game::{SendGame, SendGameSetters};
pub use send_invoice::{SendInvoice, SendInvoiceSetters};
pub use send_location::{SendLocation, SendLocationSetters};
pub use send_media_group::{SendMediaGroup, SendMediaGroupSetters};
pub use send_message::{SendMessage, SendMessageSetters};
pub use send_photo::{SendPhoto, SendPhotoSetters};
pub use send_poll::{SendPoll, SendPollSetters};
pub use send_sticker::{SendSticker, SendStickerSetters};
pub use send_venue::{SendVenue, SendVenueSetters};
pub use send_video::{SendVideo, SendVideoSetters};
pub use send_video_note::{SendVideoNote, SendVideoNoteSetters};
pub use send_voice::{SendVoice, SendVoiceSetters};
pub use set_chat_administrator_custom_title::{
    SetChatAdministratorCustomTitle, SetChatAdministratorCustomTitleSetters,
};
pub use set_chat_description::{SetChatDescription, SetChatDescriptionSetters};
pub use set_chat_permissions::{SetChatPermissions, SetChatPermissionsSetters};
pub use set_chat_photo::{SetChatPhoto, SetChatPhotoSetters};
pub use set_chat_sticker_set::{SetChatStickerSet, SetChatStickerSetSetters};
pub use set_chat_title::{SetChatTitle, SetChatTitleSetters};
pub use set_game_score::{SetGameScore, SetGameScoreSetters};
pub use set_game_score_inline::{SetGameScoreInline, SetGameScoreInlineSetters};
pub use set_my_commands::{SetMyCommands, SetMyCommandsSetters};
pub use set_passport_data_errors::{SetPassportDataErrors, SetPassportDataErrorsSetters};
pub use set_sticker_position_in_set::{SetStickerPositionInSet, SetStickerPositionInSetSetters};
pub use set_sticker_set_thumb::{SetStickerSetThumb, SetStickerSetThumbSetters};
pub use set_webhook::{SetWebhook, SetWebhookSetters};
pub use stop_message_live_location::{StopMessageLiveLocation, StopMessageLiveLocationSetters};
pub use stop_message_live_location_inline::{
    StopMessageLiveLocationInline, StopMessageLiveLocationInlineSetters,
};
pub use stop_poll::{StopPoll, StopPollSetters};
pub use unban_chat_member::{UnbanChatMember, UnbanChatMemberSetters};
pub use unban_chat_sender_chat::{UnbanChatSenderChat, UnbanChatSenderChatSetters};
pub use unpin_all_chat_messages::{UnpinAllChatMessages, UnpinAllChatMessagesSetters};
pub use unpin_chat_message::{UnpinChatMessage, UnpinChatMessageSetters};
pub use upload_sticker_file::{UploadStickerFile, UploadStickerFileSetters};

// end of auto generated block

mod get_updates_fault_tolerant;

pub use get_updates_fault_tolerant::GetUpdatesFaultTolerant;
