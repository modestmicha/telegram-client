use futures::future::LocalBoxFuture;
use rtdlib::types::*;
use std::sync::Arc;

use crate::api::Api;
use crate::errors::*;

/// Telegram client event listener
#[derive(Clone, Default)]
pub struct RasyncListener {
  exception: Option<Arc<dyn Send + Sync + Fn((Api, TGError)) -> LocalBoxFuture<'static, ()>>>,
  receive: Option<Arc<dyn Send + Sync + Fn((Api, String)) -> LocalBoxFuture<'static, TGResult<()>>>>,



  test_use_update: Option<Arc<dyn Send + Sync + Fn((Api, TestUseUpdate)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_authorization_state: Option<Arc<dyn Send + Sync + Fn((Api, UpdateAuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_basic_group: Option<Arc<dyn Send + Sync + Fn((Api, UpdateBasicGroup)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_basic_group_full_info: Option<Arc<dyn Send + Sync + Fn((Api, UpdateBasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_call: Option<Arc<dyn Send + Sync + Fn((Api, UpdateCall)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_default_disable_notification: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatDefaultDisableNotification)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_draft_message: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatDraftMessage)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_is_marked_as_unread: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatIsMarkedAsUnread)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_is_pinned: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatIsPinned)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_is_sponsored: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatIsSponsored)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_last_message: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatLastMessage)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_notification_settings: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_order: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatOrder)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_photo: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatPhoto)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_read_inbox: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatReadInbox)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_read_outbox: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatReadOutbox)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_reply_markup: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatReplyMarkup)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_title: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatTitle)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_unread_mention_count: Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatUnreadMentionCount)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_connection_state: Option<Arc<dyn Send + Sync + Fn((Api, UpdateConnectionState)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_delete_messages: Option<Arc<dyn Send + Sync + Fn((Api, UpdateDeleteMessages)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_favorite_stickers: Option<Arc<dyn Send + Sync + Fn((Api, UpdateFavoriteStickers)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_file: Option<Arc<dyn Send + Sync + Fn((Api, UpdateFile)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_file_generation_start: Option<Arc<dyn Send + Sync + Fn((Api, UpdateFileGenerationStart)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_file_generation_stop: Option<Arc<dyn Send + Sync + Fn((Api, UpdateFileGenerationStop)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_installed_sticker_sets: Option<Arc<dyn Send + Sync + Fn((Api, UpdateInstalledStickerSets)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_language_pack_strings: Option<Arc<dyn Send + Sync + Fn((Api, UpdateLanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_message_content: Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageContent)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_message_content_opened: Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageContentOpened)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_message_edited: Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageEdited)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_message_mention_read: Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageMentionRead)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_message_send_acknowledged: Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageSendAcknowledged)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_message_send_failed: Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageSendFailed)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_message_send_succeeded: Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageSendSucceeded)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_message_views: Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageViews)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_new_callback_query: Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_new_chat: Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewChat)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_new_chosen_inline_result: Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewChosenInlineResult)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_new_custom_event: Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewCustomEvent)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_new_custom_query: Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewCustomQuery)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_new_inline_callback_query: Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewInlineCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_new_inline_query: Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewInlineQuery)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_new_message: Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewMessage)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_new_pre_checkout_query: Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewPreCheckoutQuery)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_new_shipping_query: Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewShippingQuery)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_option: Option<Arc<dyn Send + Sync + Fn((Api, UpdateOption)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_recent_stickers: Option<Arc<dyn Send + Sync + Fn((Api, UpdateRecentStickers)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_saved_animations: Option<Arc<dyn Send + Sync + Fn((Api, UpdateSavedAnimations)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_scope_notification_settings: Option<Arc<dyn Send + Sync + Fn((Api, UpdateScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_secret_chat: Option<Arc<dyn Send + Sync + Fn((Api, UpdateSecretChat)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_service_notification: Option<Arc<dyn Send + Sync + Fn((Api, UpdateServiceNotification)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_supergroup: Option<Arc<dyn Send + Sync + Fn((Api, UpdateSupergroup)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_supergroup_full_info: Option<Arc<dyn Send + Sync + Fn((Api, UpdateSupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_terms_of_service: Option<Arc<dyn Send + Sync + Fn((Api, UpdateTermsOfService)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_trending_sticker_sets: Option<Arc<dyn Send + Sync + Fn((Api, UpdateTrendingStickerSets)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_unread_chat_count: Option<Arc<dyn Send + Sync + Fn((Api, UpdateUnreadChatCount)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_unread_message_count: Option<Arc<dyn Send + Sync + Fn((Api, UpdateUnreadMessageCount)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_user: Option<Arc<dyn Send + Sync + Fn((Api, UpdateUser)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_user_chat_action: Option<Arc<dyn Send + Sync + Fn((Api, UpdateUserChatAction)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_user_full_info: Option<Arc<dyn Send + Sync + Fn((Api, UpdateUserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_user_privacy_setting_rules: Option<Arc<dyn Send + Sync + Fn((Api, UpdateUserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_user_status: Option<Arc<dyn Send + Sync + Fn((Api, UpdateUserStatus)) -> LocalBoxFuture<'static, TGResult<()>>>>,


  authorization_state: Option<Arc<dyn Send + Sync + Fn((Api, AuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  check_chat_username_result: Option<Arc<dyn Send + Sync + Fn((Api, CheckChatUsernameResult)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  language_pack_string_value: Option<Arc<dyn Send + Sync + Fn((Api, LanguagePackStringValue)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  option_value: Option<Arc<dyn Send + Sync + Fn((Api, OptionValue)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  passport_element: Option<Arc<dyn Send + Sync + Fn((Api, PassportElement)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update: Option<Arc<dyn Send + Sync + Fn((Api, Update)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  account_ttl: Option<Arc<dyn Send + Sync + Fn((Api, AccountTtl)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  animations: Option<Arc<dyn Send + Sync + Fn((Api, Animations)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  authentication_code_info: Option<Arc<dyn Send + Sync + Fn((Api, AuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  basic_group: Option<Arc<dyn Send + Sync + Fn((Api, BasicGroup)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  basic_group_full_info: Option<Arc<dyn Send + Sync + Fn((Api, BasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  call_id: Option<Arc<dyn Send + Sync + Fn((Api, CallId)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  callback_query_answer: Option<Arc<dyn Send + Sync + Fn((Api, CallbackQueryAnswer)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat: Option<Arc<dyn Send + Sync + Fn((Api, Chat)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_events: Option<Arc<dyn Send + Sync + Fn((Api, ChatEvents)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_invite_link: Option<Arc<dyn Send + Sync + Fn((Api, ChatInviteLink)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_invite_link_info: Option<Arc<dyn Send + Sync + Fn((Api, ChatInviteLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_member: Option<Arc<dyn Send + Sync + Fn((Api, ChatMember)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_members: Option<Arc<dyn Send + Sync + Fn((Api, ChatMembers)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_report_spam_state: Option<Arc<dyn Send + Sync + Fn((Api, ChatReportSpamState)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chats: Option<Arc<dyn Send + Sync + Fn((Api, Chats)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  connected_websites: Option<Arc<dyn Send + Sync + Fn((Api, ConnectedWebsites)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  count: Option<Arc<dyn Send + Sync + Fn((Api, Count)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  custom_request_result: Option<Arc<dyn Send + Sync + Fn((Api, CustomRequestResult)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  deep_link_info: Option<Arc<dyn Send + Sync + Fn((Api, DeepLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  email_address_authentication_code_info: Option<Arc<dyn Send + Sync + Fn((Api, EmailAddressAuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  error: Option<Arc<dyn Send + Sync + Fn((Api, Error)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  file: Option<Arc<dyn Send + Sync + Fn((Api, File)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  formatted_text: Option<Arc<dyn Send + Sync + Fn((Api, FormattedText)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  found_messages: Option<Arc<dyn Send + Sync + Fn((Api, FoundMessages)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  game_high_scores: Option<Arc<dyn Send + Sync + Fn((Api, GameHighScores)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  hashtags: Option<Arc<dyn Send + Sync + Fn((Api, Hashtags)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  imported_contacts: Option<Arc<dyn Send + Sync + Fn((Api, ImportedContacts)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  inline_query_results: Option<Arc<dyn Send + Sync + Fn((Api, InlineQueryResults)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  language_pack_strings: Option<Arc<dyn Send + Sync + Fn((Api, LanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  localization_target_info: Option<Arc<dyn Send + Sync + Fn((Api, LocalizationTargetInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  message: Option<Arc<dyn Send + Sync + Fn((Api, Message)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  messages: Option<Arc<dyn Send + Sync + Fn((Api, Messages)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  network_statistics: Option<Arc<dyn Send + Sync + Fn((Api, NetworkStatistics)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  ok: Option<Arc<dyn Send + Sync + Fn((Api, Ok)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  order_info: Option<Arc<dyn Send + Sync + Fn((Api, OrderInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  passport_authorization_form: Option<Arc<dyn Send + Sync + Fn((Api, PassportAuthorizationForm)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  passport_elements: Option<Arc<dyn Send + Sync + Fn((Api, PassportElements)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  password_state: Option<Arc<dyn Send + Sync + Fn((Api, PasswordState)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  payment_form: Option<Arc<dyn Send + Sync + Fn((Api, PaymentForm)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  payment_receipt: Option<Arc<dyn Send + Sync + Fn((Api, PaymentReceipt)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  payment_result: Option<Arc<dyn Send + Sync + Fn((Api, PaymentResult)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  proxies: Option<Arc<dyn Send + Sync + Fn((Api, Proxies)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  proxy: Option<Arc<dyn Send + Sync + Fn((Api, Proxy)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  public_message_link: Option<Arc<dyn Send + Sync + Fn((Api, PublicMessageLink)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  recovery_email_address: Option<Arc<dyn Send + Sync + Fn((Api, RecoveryEmailAddress)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  scope_notification_settings: Option<Arc<dyn Send + Sync + Fn((Api, ScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  seconds: Option<Arc<dyn Send + Sync + Fn((Api, Seconds)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  secret_chat: Option<Arc<dyn Send + Sync + Fn((Api, SecretChat)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  sessions: Option<Arc<dyn Send + Sync + Fn((Api, Sessions)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  sticker_emojis: Option<Arc<dyn Send + Sync + Fn((Api, StickerEmojis)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  sticker_set: Option<Arc<dyn Send + Sync + Fn((Api, StickerSet)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  sticker_sets: Option<Arc<dyn Send + Sync + Fn((Api, StickerSets)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  stickers: Option<Arc<dyn Send + Sync + Fn((Api, Stickers)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  storage_statistics: Option<Arc<dyn Send + Sync + Fn((Api, StorageStatistics)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  storage_statistics_fast: Option<Arc<dyn Send + Sync + Fn((Api, StorageStatisticsFast)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  supergroup: Option<Arc<dyn Send + Sync + Fn((Api, Supergroup)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  supergroup_full_info: Option<Arc<dyn Send + Sync + Fn((Api, SupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  t_me_urls: Option<Arc<dyn Send + Sync + Fn((Api, TMeUrls)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  temporary_password_state: Option<Arc<dyn Send + Sync + Fn((Api, TemporaryPasswordState)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  test_bytes: Option<Arc<dyn Send + Sync + Fn((Api, TestBytes)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  test_int: Option<Arc<dyn Send + Sync + Fn((Api, TestInt)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  test_string: Option<Arc<dyn Send + Sync + Fn((Api, TestString)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  test_vector_int: Option<Arc<dyn Send + Sync + Fn((Api, TestVectorInt)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  test_vector_int_object: Option<Arc<dyn Send + Sync + Fn((Api, TestVectorIntObject)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  test_vector_string: Option<Arc<dyn Send + Sync + Fn((Api, TestVectorString)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  test_vector_string_object: Option<Arc<dyn Send + Sync + Fn((Api, TestVectorStringObject)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  text: Option<Arc<dyn Send + Sync + Fn((Api, Text)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  text_entities: Option<Arc<dyn Send + Sync + Fn((Api, TextEntities)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  user: Option<Arc<dyn Send + Sync + Fn((Api, User)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  user_full_info: Option<Arc<dyn Send + Sync + Fn((Api, UserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  user_privacy_setting_rules: Option<Arc<dyn Send + Sync + Fn((Api, UserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  user_profile_photos: Option<Arc<dyn Send + Sync + Fn((Api, UserProfilePhotos)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  users: Option<Arc<dyn Send + Sync + Fn((Api, Users)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  validated_order_info: Option<Arc<dyn Send + Sync + Fn((Api, ValidatedOrderInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  wallpapers: Option<Arc<dyn Send + Sync + Fn((Api, Wallpapers)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  web_page: Option<Arc<dyn Send + Sync + Fn((Api, WebPage)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  web_page_instant_view: Option<Arc<dyn Send + Sync + Fn((Api, WebPageInstantView)) -> LocalBoxFuture<'static, TGResult<()>>>>,

}


impl RasyncListener {
  pub fn new() -> Self { RasyncListener::default() }

  #[allow(dead_code)]
  pub(crate) fn has_receive_listen(&self) -> bool { self.receive.is_some() }

  pub(crate) fn lout(&self) -> RasyncLout { RasyncLout::new(self.clone()) }


  /// when receive data from tdlib
  pub fn on_receive<F>(&mut self, fnc: F) -> &mut Self where F: Send + Sync + Fn((Api, String)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.receive = Some(Arc::new(fnc));
    self
  }

  /// when telegram client throw exception
  pub fn on_exception<F>(&mut self, fnc: F) -> &mut Self where F: Send + Sync + Fn((Api, TGError)) -> LocalBoxFuture<'static, ()> + 'static {
    self.exception = Some(Arc::new(fnc));
    self
  }






  /// Does nothing and ensures that the Update object is used; for testing only
  pub fn on_test_use_update<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, TestUseUpdate)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.test_use_update = Some(Arc::new(fnc));
    self
  }

  /// The user authorization state has changed
  pub fn on_update_authorization_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateAuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_authorization_state = Some(Arc::new(fnc));
    self
  }

  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the client
  pub fn on_update_basic_group<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateBasicGroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_basic_group = Some(Arc::new(fnc));
    self
  }

  /// Some data from basicGroupFullInfo has been changed
  pub fn on_update_basic_group_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateBasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_basic_group_full_info = Some(Arc::new(fnc));
    self
  }

  /// New call was created or information about a call was updated
  pub fn on_update_call<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateCall)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_call = Some(Arc::new(fnc));
    self
  }

  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  pub fn on_update_chat_default_disable_notification<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatDefaultDisableNotification)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_default_disable_notification = Some(Arc::new(fnc));
    self
  }

  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update shouldn't be applied
  pub fn on_update_chat_draft_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatDraftMessage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_draft_message = Some(Arc::new(fnc));
    self
  }

  /// A chat was marked as unread or was read
  pub fn on_update_chat_is_marked_as_unread<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatIsMarkedAsUnread)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_is_marked_as_unread = Some(Arc::new(fnc));
    self
  }

  /// A chat was pinned or unpinned
  pub fn on_update_chat_is_pinned<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatIsPinned)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_is_pinned = Some(Arc::new(fnc));
    self
  }

  /// A chat's is_sponsored field has changed
  pub fn on_update_chat_is_sponsored<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatIsSponsored)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_is_sponsored = Some(Arc::new(fnc));
    self
  }

  /// The last message of a chat was changed. If last_message is null then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
  pub fn on_update_chat_last_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatLastMessage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_last_message = Some(Arc::new(fnc));
    self
  }

  /// Notification settings for a chat were changed
  pub fn on_update_chat_notification_settings<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// The order of the chat in the chats list has changed. Instead of this update updateChatLastMessage, updateChatIsPinned or updateChatDraftMessage might be sent
  pub fn on_update_chat_order<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatOrder)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_order = Some(Arc::new(fnc));
    self
  }

  /// A chat photo was changed
  pub fn on_update_chat_photo<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatPhoto)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_photo = Some(Arc::new(fnc));
    self
  }

  /// Incoming messages were read or number of unread messages has been changed
  pub fn on_update_chat_read_inbox<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatReadInbox)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_read_inbox = Some(Arc::new(fnc));
    self
  }

  /// Outgoing messages were read
  pub fn on_update_chat_read_outbox<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatReadOutbox)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_read_outbox = Some(Arc::new(fnc));
    self
  }

  /// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user
  pub fn on_update_chat_reply_markup<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatReplyMarkup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_reply_markup = Some(Arc::new(fnc));
    self
  }

  /// The title of a chat was changed
  pub fn on_update_chat_title<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatTitle)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_title = Some(Arc::new(fnc));
    self
  }

  /// The chat unread_mention_count has changed
  pub fn on_update_chat_unread_mention_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateChatUnreadMentionCount)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_chat_unread_mention_count = Some(Arc::new(fnc));
    self
  }

  /// The connection state has changed
  pub fn on_update_connection_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateConnectionState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_connection_state = Some(Arc::new(fnc));
    self
  }

  /// Some messages were deleted
  pub fn on_update_delete_messages<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateDeleteMessages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_delete_messages = Some(Arc::new(fnc));
    self
  }

  /// The list of favorite stickers was updated
  pub fn on_update_favorite_stickers<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateFavoriteStickers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_favorite_stickers = Some(Arc::new(fnc));
    self
  }

  /// Information about a file was updated
  pub fn on_update_file<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateFile)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_file = Some(Arc::new(fnc));
    self
  }

  /// The file generation process needs to be started by the client
  pub fn on_update_file_generation_start<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateFileGenerationStart)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_file_generation_start = Some(Arc::new(fnc));
    self
  }

  /// File generation is no longer needed
  pub fn on_update_file_generation_stop<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateFileGenerationStop)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_file_generation_stop = Some(Arc::new(fnc));
    self
  }

  /// The list of installed sticker sets was updated
  pub fn on_update_installed_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateInstalledStickerSets)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_installed_sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// Some language pack strings have been updated
  pub fn on_update_language_pack_strings<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateLanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_language_pack_strings = Some(Arc::new(fnc));
    self
  }

  /// The message content has changed
  pub fn on_update_message_content<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateMessageContent)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_message_content = Some(Arc::new(fnc));
    self
  }

  /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages
  pub fn on_update_message_content_opened<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateMessageContentOpened)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_message_content_opened = Some(Arc::new(fnc));
    self
  }

  /// A message was edited. Changes in the message content will come in a separate updateMessageContent
  pub fn on_update_message_edited<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateMessageEdited)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_message_edited = Some(Arc::new(fnc));
    self
  }

  /// A message with an unread mention was read
  pub fn on_update_message_mention_read<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateMessageMentionRead)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_message_mention_read = Some(Arc::new(fnc));
    self
  }

  /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
  pub fn on_update_message_send_acknowledged<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateMessageSendAcknowledged)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_message_send_acknowledged = Some(Arc::new(fnc));
    self
  }

  /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
  pub fn on_update_message_send_failed<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateMessageSendFailed)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_message_send_failed = Some(Arc::new(fnc));
    self
  }

  /// A message has been successfully sent
  pub fn on_update_message_send_succeeded<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateMessageSendSucceeded)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_message_send_succeeded = Some(Arc::new(fnc));
    self
  }

  /// The view count of the message has changed
  pub fn on_update_message_views<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateMessageViews)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_message_views = Some(Arc::new(fnc));
    self
  }

  /// A new incoming callback query; for bots only
  pub fn on_update_new_callback_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateNewCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_new_callback_query = Some(Arc::new(fnc));
    self
  }

  /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the client. The chat field changes will be reported through separate updates
  pub fn on_update_new_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateNewChat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_new_chat = Some(Arc::new(fnc));
    self
  }

  /// The user has chosen a result of an inline query; for bots only
  pub fn on_update_new_chosen_inline_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateNewChosenInlineResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_new_chosen_inline_result = Some(Arc::new(fnc));
    self
  }

  /// A new incoming event; for bots only
  pub fn on_update_new_custom_event<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateNewCustomEvent)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_new_custom_event = Some(Arc::new(fnc));
    self
  }

  /// A new incoming query; for bots only
  pub fn on_update_new_custom_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateNewCustomQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_new_custom_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming callback query from a message sent via a bot; for bots only
  pub fn on_update_new_inline_callback_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateNewInlineCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_new_inline_callback_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming inline query; for bots only
  pub fn on_update_new_inline_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateNewInlineQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_new_inline_query = Some(Arc::new(fnc));
    self
  }

  /// A new message was received; can also be an outgoing message
  pub fn on_update_new_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateNewMessage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_new_message = Some(Arc::new(fnc));
    self
  }

  /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
  pub fn on_update_new_pre_checkout_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateNewPreCheckoutQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_new_pre_checkout_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming shipping query; for bots only. Only for invoices with flexible price
  pub fn on_update_new_shipping_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateNewShippingQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_new_shipping_query = Some(Arc::new(fnc));
    self
  }

  /// An option changed its value
  pub fn on_update_option<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateOption)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_option = Some(Arc::new(fnc));
    self
  }

  /// The list of recently used stickers was updated
  pub fn on_update_recent_stickers<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateRecentStickers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_recent_stickers = Some(Arc::new(fnc));
    self
  }

  /// The list of saved animations was updated
  pub fn on_update_saved_animations<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateSavedAnimations)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_saved_animations = Some(Arc::new(fnc));
    self
  }

  /// Notification settings for some type of chats were updated
  pub fn on_update_scope_notification_settings<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_scope_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the client
  pub fn on_update_secret_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateSecretChat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_secret_chat = Some(Arc::new(fnc));
    self
  }

  /// Service notification from the server. Upon receiving this the client must show a popup with the content of the notification
  pub fn on_update_service_notification<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateServiceNotification)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_service_notification = Some(Arc::new(fnc));
    self
  }

  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the client
  pub fn on_update_supergroup<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateSupergroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_supergroup = Some(Arc::new(fnc));
    self
  }

  /// Some data from supergroupFullInfo has been changed
  pub fn on_update_supergroup_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateSupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_supergroup_full_info = Some(Arc::new(fnc));
    self
  }

  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method should be called with the reason "Decline ToS update"
  pub fn on_update_terms_of_service<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateTermsOfService)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_terms_of_service = Some(Arc::new(fnc));
    self
  }

  /// The list of trending sticker sets was updated or some of them were viewed
  pub fn on_update_trending_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateTrendingStickerSets)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_trending_sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if a message database is used
  pub fn on_update_unread_chat_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateUnreadChatCount)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_unread_chat_count = Some(Arc::new(fnc));
    self
  }

  /// Number of unread messages has changed. This update is sent only if a message database is used
  pub fn on_update_unread_message_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateUnreadMessageCount)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_unread_message_count = Some(Arc::new(fnc));
    self
  }

  /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the client
  pub fn on_update_user<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateUser)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_user = Some(Arc::new(fnc));
    self
  }

  /// User activity in the chat has changed
  pub fn on_update_user_chat_action<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateUserChatAction)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_user_chat_action = Some(Arc::new(fnc));
    self
  }

  /// Some data from userFullInfo has been changed
  pub fn on_update_user_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateUserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_user_full_info = Some(Arc::new(fnc));
    self
  }

  /// Some privacy setting rules have been changed
  pub fn on_update_user_privacy_setting_rules<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateUserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_user_privacy_setting_rules = Some(Arc::new(fnc));
    self
  }

  /// The user went online or offline
  pub fn on_update_user_status<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UpdateUserStatus)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update_user_status = Some(Arc::new(fnc));
    self
  }



  /// Represents the current authorization state of the client
  pub fn on_authorization_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, AuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.authorization_state = Some(Arc::new(fnc));
    self
  }

  /// Represents result of checking whether a username can be set for a chat
  pub fn on_check_chat_username_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, CheckChatUsernameResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.check_chat_username_result = Some(Arc::new(fnc));
    self
  }

  /// Represents the value of a string in a language pack
  pub fn on_language_pack_string_value<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, LanguagePackStringValue)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.language_pack_string_value = Some(Arc::new(fnc));
    self
  }

  /// Represents the value of an option
  pub fn on_option_value<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, OptionValue)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.option_value = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a Telegram Passport element
  pub fn on_passport_element<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, PassportElement)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.passport_element = Some(Arc::new(fnc));
    self
  }

  /// Contains notifications about data changes
  pub fn on_update<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Update)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.update = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the period of inactivity after which the current user's account will automatically be deleted
  pub fn on_account_ttl<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, AccountTtl)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.account_ttl = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of animations
  pub fn on_animations<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Animations)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.animations = Some(Arc::new(fnc));
    self
  }

  /// Information about the authentication code that was sent
  pub fn on_authentication_code_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, AuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.authentication_code_info = Some(Arc::new(fnc));
    self
  }

  /// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
  pub fn on_basic_group<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, BasicGroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.basic_group = Some(Arc::new(fnc));
    self
  }

  /// Contains full information about a basic group
  pub fn on_basic_group_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, BasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.basic_group_full_info = Some(Arc::new(fnc));
    self
  }

  /// Contains the call identifier
  pub fn on_call_id<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, CallId)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.call_id = Some(Arc::new(fnc));
    self
  }

  /// Contains a bot's answer to a callback query
  pub fn on_callback_query_answer<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, CallbackQueryAnswer)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.callback_query_answer = Some(Arc::new(fnc));
    self
  }

  /// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
  pub fn on_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Chat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.chat = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat events
  pub fn on_chat_events<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, ChatEvents)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.chat_events = Some(Arc::new(fnc));
    self
  }

  /// Contains a chat invite link
  pub fn on_chat_invite_link<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, ChatInviteLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.chat_invite_link = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a chat invite link
  pub fn on_chat_invite_link_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, ChatInviteLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.chat_invite_link_info = Some(Arc::new(fnc));
    self
  }

  /// A user with information about joining/leaving a chat
  pub fn on_chat_member<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, ChatMember)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.chat_member = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat members
  pub fn on_chat_members<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, ChatMembers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.chat_members = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the availability of the "Report spam" action for a chat
  pub fn on_chat_report_spam_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, ChatReportSpamState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.chat_report_spam_state = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of chats
  pub fn on_chats<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Chats)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.chats = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of websites the current user is logged in with Telegram
  pub fn on_connected_websites<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, ConnectedWebsites)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.connected_websites = Some(Arc::new(fnc));
    self
  }

  /// Contains a counter
  pub fn on_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Count)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.count = Some(Arc::new(fnc));
    self
  }

  /// Contains the result of a custom request
  pub fn on_custom_request_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, CustomRequestResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.custom_request_result = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a tg:// deep link
  pub fn on_deep_link_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, DeepLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.deep_link_info = Some(Arc::new(fnc));
    self
  }

  /// Information about the email address authentication code that was sent
  pub fn on_email_address_authentication_code_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, EmailAddressAuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.email_address_authentication_code_info = Some(Arc::new(fnc));
    self
  }

  /// An object of this type can be returned on every function call, in case of an error
  pub fn on_error<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Error)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.error = Some(Arc::new(fnc));
    self
  }

  /// Represents a file
  pub fn on_file<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, File)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.file = Some(Arc::new(fnc));
    self
  }

  /// A text with some entities
  pub fn on_formatted_text<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, FormattedText)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.formatted_text = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of messages found by a search
  pub fn on_found_messages<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, FoundMessages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.found_messages = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of game high scores
  pub fn on_game_high_scores<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, GameHighScores)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.game_high_scores = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of hashtags
  pub fn on_hashtags<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Hashtags)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.hashtags = Some(Arc::new(fnc));
    self
  }

  /// Represents the result of an ImportContacts request
  pub fn on_imported_contacts<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, ImportedContacts)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.imported_contacts = Some(Arc::new(fnc));
    self
  }

  /// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
  pub fn on_inline_query_results<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, InlineQueryResults)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.inline_query_results = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of language pack strings
  pub fn on_language_pack_strings<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, LanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.language_pack_strings = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the current localization target
  pub fn on_localization_target_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, LocalizationTargetInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.localization_target_info = Some(Arc::new(fnc));
    self
  }

  /// Describes a message
  pub fn on_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Message)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.message = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of messages
  pub fn on_messages<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Messages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.messages = Some(Arc::new(fnc));
    self
  }

  /// A full list of available network statistic entries
  pub fn on_network_statistics<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, NetworkStatistics)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.network_statistics = Some(Arc::new(fnc));
    self
  }

  /// An object of this type is returned on a successful function call for certain functions
  pub fn on_ok<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Ok)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.ok = Some(Arc::new(fnc));
    self
  }

  /// Order information
  pub fn on_order_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, OrderInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.order_info = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a Telegram Passport authorization form that was requested
  pub fn on_passport_authorization_form<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, PassportAuthorizationForm)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.passport_authorization_form = Some(Arc::new(fnc));
    self
  }

  /// Contains information about saved Telegram Passport elements
  pub fn on_passport_elements<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, PassportElements)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.passport_elements = Some(Arc::new(fnc));
    self
  }

  /// Represents the current state of 2-step verification
  pub fn on_password_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, PasswordState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.password_state = Some(Arc::new(fnc));
    self
  }

  /// Contains information about an invoice payment form
  pub fn on_payment_form<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, PaymentForm)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.payment_form = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a successful payment
  pub fn on_payment_receipt<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, PaymentReceipt)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.payment_receipt = Some(Arc::new(fnc));
    self
  }

  /// Contains the result of a payment request
  pub fn on_payment_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, PaymentResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.payment_result = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of proxy servers
  pub fn on_proxies<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Proxies)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.proxies = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a proxy server
  pub fn on_proxy<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Proxy)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.proxy = Some(Arc::new(fnc));
    self
  }

  /// Contains a public HTTPS link to a message in a public supergroup or channel
  pub fn on_public_message_link<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, PublicMessageLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.public_message_link = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the current recovery email address
  pub fn on_recovery_email_address<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, RecoveryEmailAddress)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.recovery_email_address = Some(Arc::new(fnc));
    self
  }

  /// Contains information about notification settings for several chats
  pub fn on_scope_notification_settings<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, ScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.scope_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// Contains a value representing a number of seconds
  pub fn on_seconds<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Seconds)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.seconds = Some(Arc::new(fnc));
    self
  }

  /// Represents a secret chat
  pub fn on_secret_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, SecretChat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.secret_chat = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of sessions
  pub fn on_sessions<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Sessions)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.sessions = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of all emoji corresponding to a sticker in a sticker set. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
  pub fn on_sticker_emojis<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, StickerEmojis)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.sticker_emojis = Some(Arc::new(fnc));
    self
  }

  /// Represents a sticker set
  pub fn on_sticker_set<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, StickerSet)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.sticker_set = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of sticker sets
  pub fn on_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, StickerSets)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of stickers
  pub fn on_stickers<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Stickers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.stickers = Some(Arc::new(fnc));
    self
  }

  /// Contains the exact storage usage statistics split by chats and file type
  pub fn on_storage_statistics<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, StorageStatistics)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.storage_statistics = Some(Arc::new(fnc));
    self
  }

  /// Contains approximate storage usage statistics, excluding files of unknown file type
  pub fn on_storage_statistics_fast<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, StorageStatisticsFast)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.storage_statistics_fast = Some(Arc::new(fnc));
    self
  }

  /// Represents a supergroup or channel with zero or more members (subscribers in the case of channels). From the point of view of the system, a channel is a special kind of a supergroup: only administrators can post and see the list of members, and posts from all administrators use the name and photo of the channel instead of individual names and profile photos. Unlike supergroups, channels can have an unlimited number of subscribers
  pub fn on_supergroup<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Supergroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.supergroup = Some(Arc::new(fnc));
    self
  }

  /// Contains full information about a supergroup or channel
  pub fn on_supergroup_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, SupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.supergroup_full_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of t.me URLs
  pub fn on_t_me_urls<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, TMeUrls)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.t_me_urls = Some(Arc::new(fnc));
    self
  }

  /// Returns information about the availability of a temporary password, which can be used for payments
  pub fn on_temporary_password_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, TemporaryPasswordState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.temporary_password_state = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a sequence of bytes; for testing only
  pub fn on_test_bytes<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, TestBytes)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.test_bytes = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a number; for testing only
  pub fn on_test_int<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, TestInt)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.test_int = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a string; for testing only
  pub fn on_test_string<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, TestString)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.test_string = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of numbers; for testing only
  pub fn on_test_vector_int<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, TestVectorInt)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.test_vector_int = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of objects that hold a number; for testing only
  pub fn on_test_vector_int_object<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, TestVectorIntObject)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.test_vector_int_object = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of strings; for testing only
  pub fn on_test_vector_string<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, TestVectorString)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.test_vector_string = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of objects that hold a string; for testing only
  pub fn on_test_vector_string_object<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, TestVectorStringObject)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.test_vector_string_object = Some(Arc::new(fnc));
    self
  }

  /// Contains some text
  pub fn on_text<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Text)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.text = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of text entities
  pub fn on_text_entities<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, TextEntities)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.text_entities = Some(Arc::new(fnc));
    self
  }

  /// Represents a user
  pub fn on_user<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, User)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.user = Some(Arc::new(fnc));
    self
  }

  /// Contains full information about a user (except the full list of profile photos)
  pub fn on_user_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.user_full_info = Some(Arc::new(fnc));
    self
  }

  /// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
  pub fn on_user_privacy_setting_rules<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.user_privacy_setting_rules = Some(Arc::new(fnc));
    self
  }

  /// Contains part of the list of user photos
  pub fn on_user_profile_photos<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, UserProfilePhotos)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.user_profile_photos = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of users
  pub fn on_users<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Users)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.users = Some(Arc::new(fnc));
    self
  }

  /// Contains a temporary identifier of validated order information, which is stored for one hour. Also contains the available shipping options
  pub fn on_validated_order_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, ValidatedOrderInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.validated_order_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of wallpapers
  pub fn on_wallpapers<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, Wallpapers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.wallpapers = Some(Arc::new(fnc));
    self
  }

  /// Describes a web page preview
  pub fn on_web_page<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, WebPage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.web_page = Some(Arc::new(fnc));
    self
  }

  /// Describes an instant view page for a web page
  pub fn on_web_page_instant_view<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, WebPageInstantView)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.web_page_instant_view = Some(Arc::new(fnc));
    self
  }

}


/// Get listener
#[derive(Clone)]
pub struct RasyncLout {
  listener: RasyncListener,
  supports: Vec<&'static str>
}

impl RasyncLout {
  fn new(listener: RasyncListener) -> Self {
    let supports = vec![


      "testUseUpdate",
      "updateAuthorizationState",
      "updateBasicGroup",
      "updateBasicGroupFullInfo",
      "updateCall",
      "updateChatDefaultDisableNotification",
      "updateChatDraftMessage",
      "updateChatIsMarkedAsUnread",
      "updateChatIsPinned",
      "updateChatIsSponsored",
      "updateChatLastMessage",
      "updateChatNotificationSettings",
      "updateChatOrder",
      "updateChatPhoto",
      "updateChatReadInbox",
      "updateChatReadOutbox",
      "updateChatReplyMarkup",
      "updateChatTitle",
      "updateChatUnreadMentionCount",
      "updateConnectionState",
      "updateDeleteMessages",
      "updateFavoriteStickers",
      "updateFile",
      "updateFileGenerationStart",
      "updateFileGenerationStop",
      "updateInstalledStickerSets",
      "updateLanguagePackStrings",
      "updateMessageContent",
      "updateMessageContentOpened",
      "updateMessageEdited",
      "updateMessageMentionRead",
      "updateMessageSendAcknowledged",
      "updateMessageSendFailed",
      "updateMessageSendSucceeded",
      "updateMessageViews",
      "updateNewCallbackQuery",
      "updateNewChat",
      "updateNewChosenInlineResult",
      "updateNewCustomEvent",
      "updateNewCustomQuery",
      "updateNewInlineCallbackQuery",
      "updateNewInlineQuery",
      "updateNewMessage",
      "updateNewPreCheckoutQuery",
      "updateNewShippingQuery",
      "updateOption",
      "updateRecentStickers",
      "updateSavedAnimations",
      "updateScopeNotificationSettings",
      "updateSecretChat",
      "updateServiceNotification",
      "updateSupergroup",
      "updateSupergroupFullInfo",
      "updateTermsOfService",
      "updateTrendingStickerSets",
      "updateUnreadChatCount",
      "updateUnreadMessageCount",
      "updateUser",
      "updateUserChatAction",
      "updateUserFullInfo",
      "updateUserPrivacySettingRules",
      "updateUserStatus",


      "AuthorizationState",
      "CheckChatUsernameResult",
      "LanguagePackStringValue",
      "OptionValue",
      "PassportElement",
      "Update",
      "accountTtl",
      "animations",
      "authenticationCodeInfo",
      "basicGroup",
      "basicGroupFullInfo",
      "callId",
      "callbackQueryAnswer",
      "chat",
      "chatEvents",
      "chatInviteLink",
      "chatInviteLinkInfo",
      "chatMember",
      "chatMembers",
      "chatReportSpamState",
      "chats",
      "connectedWebsites",
      "count",
      "customRequestResult",
      "deepLinkInfo",
      "emailAddressAuthenticationCodeInfo",
      "error",
      "file",
      "formattedText",
      "foundMessages",
      "gameHighScores",
      "hashtags",
      "importedContacts",
      "inlineQueryResults",
      "languagePackStrings",
      "localizationTargetInfo",
      "message",
      "messages",
      "networkStatistics",
      "ok",
      "orderInfo",
      "passportAuthorizationForm",
      "passportElements",
      "passwordState",
      "paymentForm",
      "paymentReceipt",
      "paymentResult",
      "proxies",
      "proxy",
      "publicMessageLink",
      "recoveryEmailAddress",
      "scopeNotificationSettings",
      "seconds",
      "secretChat",
      "sessions",
      "stickerEmojis",
      "stickerSet",
      "stickerSets",
      "stickers",
      "storageStatistics",
      "storageStatisticsFast",
      "supergroup",
      "supergroupFullInfo",
      "tMeUrls",
      "temporaryPasswordState",
      "testBytes",
      "testInt",
      "testString",
      "testVectorInt",
      "testVectorIntObject",
      "testVectorString",
      "testVectorStringObject",
      "text",
      "textEntities",
      "user",
      "userFullInfo",
      "userPrivacySettingRules",
      "userProfilePhotos",
      "users",
      "validatedOrderInfo",
      "wallpapers",
      "webPage",
      "webPageInstantView",


    ];
    Self { listener, supports }
  }

  pub fn is_support<S: AsRef<str>>(&self, name: S) -> bool {
    self.supports.iter()
      .find(|&&item| item == name.as_ref())
      .is_some()
  }

  pub async fn handle_type(&self, api: &Api, td_type: &TdType) -> TGResult<bool>  {
    match td_type {


      TdType::TestUseUpdate(value) => match &self.listener.test_use_update {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateAuthorizationState(value) => match &self.listener.update_authorization_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateBasicGroup(value) => match &self.listener.update_basic_group {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateBasicGroupFullInfo(value) => match &self.listener.update_basic_group_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateCall(value) => match &self.listener.update_call {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatDefaultDisableNotification(value) => match &self.listener.update_chat_default_disable_notification {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatDraftMessage(value) => match &self.listener.update_chat_draft_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatIsMarkedAsUnread(value) => match &self.listener.update_chat_is_marked_as_unread {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatIsPinned(value) => match &self.listener.update_chat_is_pinned {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatIsSponsored(value) => match &self.listener.update_chat_is_sponsored {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatLastMessage(value) => match &self.listener.update_chat_last_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatNotificationSettings(value) => match &self.listener.update_chat_notification_settings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatOrder(value) => match &self.listener.update_chat_order {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatPhoto(value) => match &self.listener.update_chat_photo {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatReadInbox(value) => match &self.listener.update_chat_read_inbox {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatReadOutbox(value) => match &self.listener.update_chat_read_outbox {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatReplyMarkup(value) => match &self.listener.update_chat_reply_markup {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatTitle(value) => match &self.listener.update_chat_title {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatUnreadMentionCount(value) => match &self.listener.update_chat_unread_mention_count {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateConnectionState(value) => match &self.listener.update_connection_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateDeleteMessages(value) => match &self.listener.update_delete_messages {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFavoriteStickers(value) => match &self.listener.update_favorite_stickers {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFile(value) => match &self.listener.update_file {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFileGenerationStart(value) => match &self.listener.update_file_generation_start {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFileGenerationStop(value) => match &self.listener.update_file_generation_stop {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateInstalledStickerSets(value) => match &self.listener.update_installed_sticker_sets {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateLanguagePackStrings(value) => match &self.listener.update_language_pack_strings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageContent(value) => match &self.listener.update_message_content {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageContentOpened(value) => match &self.listener.update_message_content_opened {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageEdited(value) => match &self.listener.update_message_edited {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageMentionRead(value) => match &self.listener.update_message_mention_read {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageSendAcknowledged(value) => match &self.listener.update_message_send_acknowledged {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageSendFailed(value) => match &self.listener.update_message_send_failed {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageSendSucceeded(value) => match &self.listener.update_message_send_succeeded {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageViews(value) => match &self.listener.update_message_views {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewCallbackQuery(value) => match &self.listener.update_new_callback_query {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewChat(value) => match &self.listener.update_new_chat {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewChosenInlineResult(value) => match &self.listener.update_new_chosen_inline_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewCustomEvent(value) => match &self.listener.update_new_custom_event {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewCustomQuery(value) => match &self.listener.update_new_custom_query {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewInlineCallbackQuery(value) => match &self.listener.update_new_inline_callback_query {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewInlineQuery(value) => match &self.listener.update_new_inline_query {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewMessage(value) => match &self.listener.update_new_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewPreCheckoutQuery(value) => match &self.listener.update_new_pre_checkout_query {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewShippingQuery(value) => match &self.listener.update_new_shipping_query {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateOption(value) => match &self.listener.update_option {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateRecentStickers(value) => match &self.listener.update_recent_stickers {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateSavedAnimations(value) => match &self.listener.update_saved_animations {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateScopeNotificationSettings(value) => match &self.listener.update_scope_notification_settings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateSecretChat(value) => match &self.listener.update_secret_chat {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateServiceNotification(value) => match &self.listener.update_service_notification {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateSupergroup(value) => match &self.listener.update_supergroup {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateSupergroupFullInfo(value) => match &self.listener.update_supergroup_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateTermsOfService(value) => match &self.listener.update_terms_of_service {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateTrendingStickerSets(value) => match &self.listener.update_trending_sticker_sets {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUnreadChatCount(value) => match &self.listener.update_unread_chat_count {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUnreadMessageCount(value) => match &self.listener.update_unread_message_count {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUser(value) => match &self.listener.update_user {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUserChatAction(value) => match &self.listener.update_user_chat_action {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUserFullInfo(value) => match &self.listener.update_user_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUserPrivacySettingRules(value) => match &self.listener.update_user_privacy_setting_rules {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUserStatus(value) => match &self.listener.update_user_status {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },


      TdType::AuthorizationState(value) => match &self.listener.authorization_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CheckChatUsernameResult(value) => match &self.listener.check_chat_username_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LanguagePackStringValue(value) => match &self.listener.language_pack_string_value {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::OptionValue(value) => match &self.listener.option_value {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PassportElement(value) => match &self.listener.passport_element {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Update(value) => match &self.listener.update {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AccountTtl(value) => match &self.listener.account_ttl {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Animations(value) => match &self.listener.animations {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AuthenticationCodeInfo(value) => match &self.listener.authentication_code_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BasicGroup(value) => match &self.listener.basic_group {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BasicGroupFullInfo(value) => match &self.listener.basic_group_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CallId(value) => match &self.listener.call_id {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CallbackQueryAnswer(value) => match &self.listener.callback_query_answer {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Chat(value) => match &self.listener.chat {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatEvents(value) => match &self.listener.chat_events {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatInviteLink(value) => match &self.listener.chat_invite_link {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatInviteLinkInfo(value) => match &self.listener.chat_invite_link_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatMember(value) => match &self.listener.chat_member {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatMembers(value) => match &self.listener.chat_members {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatReportSpamState(value) => match &self.listener.chat_report_spam_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Chats(value) => match &self.listener.chats {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ConnectedWebsites(value) => match &self.listener.connected_websites {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Count(value) => match &self.listener.count {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CustomRequestResult(value) => match &self.listener.custom_request_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::DeepLinkInfo(value) => match &self.listener.deep_link_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::EmailAddressAuthenticationCodeInfo(value) => match &self.listener.email_address_authentication_code_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Error(value) => match &self.listener.error {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::File(value) => match &self.listener.file {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FormattedText(value) => match &self.listener.formatted_text {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundMessages(value) => match &self.listener.found_messages {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GameHighScores(value) => match &self.listener.game_high_scores {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Hashtags(value) => match &self.listener.hashtags {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ImportedContacts(value) => match &self.listener.imported_contacts {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::InlineQueryResults(value) => match &self.listener.inline_query_results {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LanguagePackStrings(value) => match &self.listener.language_pack_strings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LocalizationTargetInfo(value) => match &self.listener.localization_target_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Message(value) => match &self.listener.message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Messages(value) => match &self.listener.messages {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::NetworkStatistics(value) => match &self.listener.network_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Ok(value) => match &self.listener.ok {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::OrderInfo(value) => match &self.listener.order_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PassportAuthorizationForm(value) => match &self.listener.passport_authorization_form {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PassportElements(value) => match &self.listener.passport_elements {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PasswordState(value) => match &self.listener.password_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PaymentForm(value) => match &self.listener.payment_form {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PaymentReceipt(value) => match &self.listener.payment_receipt {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PaymentResult(value) => match &self.listener.payment_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Proxies(value) => match &self.listener.proxies {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Proxy(value) => match &self.listener.proxy {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PublicMessageLink(value) => match &self.listener.public_message_link {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::RecoveryEmailAddress(value) => match &self.listener.recovery_email_address {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ScopeNotificationSettings(value) => match &self.listener.scope_notification_settings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Seconds(value) => match &self.listener.seconds {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::SecretChat(value) => match &self.listener.secret_chat {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Sessions(value) => match &self.listener.sessions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StickerEmojis(value) => match &self.listener.sticker_emojis {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StickerSet(value) => match &self.listener.sticker_set {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StickerSets(value) => match &self.listener.sticker_sets {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Stickers(value) => match &self.listener.stickers {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StorageStatistics(value) => match &self.listener.storage_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StorageStatisticsFast(value) => match &self.listener.storage_statistics_fast {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Supergroup(value) => match &self.listener.supergroup {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::SupergroupFullInfo(value) => match &self.listener.supergroup_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TMeUrls(value) => match &self.listener.t_me_urls {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TemporaryPasswordState(value) => match &self.listener.temporary_password_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestBytes(value) => match &self.listener.test_bytes {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestInt(value) => match &self.listener.test_int {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestString(value) => match &self.listener.test_string {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestVectorInt(value) => match &self.listener.test_vector_int {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestVectorIntObject(value) => match &self.listener.test_vector_int_object {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestVectorString(value) => match &self.listener.test_vector_string {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestVectorStringObject(value) => match &self.listener.test_vector_string_object {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Text(value) => match &self.listener.text {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TextEntities(value) => match &self.listener.text_entities {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::User(value) => match &self.listener.user {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UserFullInfo(value) => match &self.listener.user_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UserPrivacySettingRules(value) => match &self.listener.user_privacy_setting_rules {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UserProfilePhotos(value) => match &self.listener.user_profile_photos {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Users(value) => match &self.listener.users {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ValidatedOrderInfo(value) => match &self.listener.validated_order_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Wallpapers(value) => match &self.listener.wallpapers {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::WebPage(value) => match &self.listener.web_page {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::WebPageInstantView(value) => match &self.listener.web_page_instant_view {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },


  }
  }

  /// when telegram client throw exception
  pub fn exception(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TGError)) -> LocalBoxFuture<'static, ()> + 'static>> {
    &self.listener.exception
  }

  /// when receive data from tdlib
  pub fn receive(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, String)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.receive
  }





  /// Does nothing and ensures that the Update object is used; for testing only
  pub fn test_use_update(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TestUseUpdate)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.test_use_update
  }

  /// The user authorization state has changed
  pub fn update_authorization_state(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateAuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_authorization_state
  }

  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the client
  pub fn update_basic_group(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateBasicGroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_basic_group
  }

  /// Some data from basicGroupFullInfo has been changed
  pub fn update_basic_group_full_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateBasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_basic_group_full_info
  }

  /// New call was created or information about a call was updated
  pub fn update_call(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateCall)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_call
  }

  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  pub fn update_chat_default_disable_notification(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatDefaultDisableNotification)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_default_disable_notification
  }

  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update shouldn't be applied
  pub fn update_chat_draft_message(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatDraftMessage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_draft_message
  }

  /// A chat was marked as unread or was read
  pub fn update_chat_is_marked_as_unread(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatIsMarkedAsUnread)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_is_marked_as_unread
  }

  /// A chat was pinned or unpinned
  pub fn update_chat_is_pinned(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatIsPinned)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_is_pinned
  }

  /// A chat's is_sponsored field has changed
  pub fn update_chat_is_sponsored(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatIsSponsored)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_is_sponsored
  }

  /// The last message of a chat was changed. If last_message is null then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
  pub fn update_chat_last_message(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatLastMessage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_last_message
  }

  /// Notification settings for a chat were changed
  pub fn update_chat_notification_settings(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_notification_settings
  }

  /// The order of the chat in the chats list has changed. Instead of this update updateChatLastMessage, updateChatIsPinned or updateChatDraftMessage might be sent
  pub fn update_chat_order(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatOrder)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_order
  }

  /// A chat photo was changed
  pub fn update_chat_photo(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatPhoto)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_photo
  }

  /// Incoming messages were read or number of unread messages has been changed
  pub fn update_chat_read_inbox(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatReadInbox)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_read_inbox
  }

  /// Outgoing messages were read
  pub fn update_chat_read_outbox(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatReadOutbox)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_read_outbox
  }

  /// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user
  pub fn update_chat_reply_markup(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatReplyMarkup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_reply_markup
  }

  /// The title of a chat was changed
  pub fn update_chat_title(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatTitle)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_title
  }

  /// The chat unread_mention_count has changed
  pub fn update_chat_unread_mention_count(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateChatUnreadMentionCount)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_chat_unread_mention_count
  }

  /// The connection state has changed
  pub fn update_connection_state(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateConnectionState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_connection_state
  }

  /// Some messages were deleted
  pub fn update_delete_messages(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateDeleteMessages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_delete_messages
  }

  /// The list of favorite stickers was updated
  pub fn update_favorite_stickers(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateFavoriteStickers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_favorite_stickers
  }

  /// Information about a file was updated
  pub fn update_file(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateFile)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_file
  }

  /// The file generation process needs to be started by the client
  pub fn update_file_generation_start(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateFileGenerationStart)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_file_generation_start
  }

  /// File generation is no longer needed
  pub fn update_file_generation_stop(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateFileGenerationStop)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_file_generation_stop
  }

  /// The list of installed sticker sets was updated
  pub fn update_installed_sticker_sets(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateInstalledStickerSets)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_installed_sticker_sets
  }

  /// Some language pack strings have been updated
  pub fn update_language_pack_strings(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateLanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_language_pack_strings
  }

  /// The message content has changed
  pub fn update_message_content(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageContent)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_message_content
  }

  /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages
  pub fn update_message_content_opened(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageContentOpened)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_message_content_opened
  }

  /// A message was edited. Changes in the message content will come in a separate updateMessageContent
  pub fn update_message_edited(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageEdited)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_message_edited
  }

  /// A message with an unread mention was read
  pub fn update_message_mention_read(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageMentionRead)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_message_mention_read
  }

  /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
  pub fn update_message_send_acknowledged(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageSendAcknowledged)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_message_send_acknowledged
  }

  /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
  pub fn update_message_send_failed(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageSendFailed)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_message_send_failed
  }

  /// A message has been successfully sent
  pub fn update_message_send_succeeded(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageSendSucceeded)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_message_send_succeeded
  }

  /// The view count of the message has changed
  pub fn update_message_views(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateMessageViews)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_message_views
  }

  /// A new incoming callback query; for bots only
  pub fn update_new_callback_query(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_new_callback_query
  }

  /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the client. The chat field changes will be reported through separate updates
  pub fn update_new_chat(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewChat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_new_chat
  }

  /// The user has chosen a result of an inline query; for bots only
  pub fn update_new_chosen_inline_result(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewChosenInlineResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_new_chosen_inline_result
  }

  /// A new incoming event; for bots only
  pub fn update_new_custom_event(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewCustomEvent)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_new_custom_event
  }

  /// A new incoming query; for bots only
  pub fn update_new_custom_query(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewCustomQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_new_custom_query
  }

  /// A new incoming callback query from a message sent via a bot; for bots only
  pub fn update_new_inline_callback_query(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewInlineCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_new_inline_callback_query
  }

  /// A new incoming inline query; for bots only
  pub fn update_new_inline_query(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewInlineQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_new_inline_query
  }

  /// A new message was received; can also be an outgoing message
  pub fn update_new_message(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewMessage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_new_message
  }

  /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
  pub fn update_new_pre_checkout_query(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewPreCheckoutQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_new_pre_checkout_query
  }

  /// A new incoming shipping query; for bots only. Only for invoices with flexible price
  pub fn update_new_shipping_query(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateNewShippingQuery)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_new_shipping_query
  }

  /// An option changed its value
  pub fn update_option(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateOption)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_option
  }

  /// The list of recently used stickers was updated
  pub fn update_recent_stickers(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateRecentStickers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_recent_stickers
  }

  /// The list of saved animations was updated
  pub fn update_saved_animations(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateSavedAnimations)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_saved_animations
  }

  /// Notification settings for some type of chats were updated
  pub fn update_scope_notification_settings(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_scope_notification_settings
  }

  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the client
  pub fn update_secret_chat(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateSecretChat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_secret_chat
  }

  /// Service notification from the server. Upon receiving this the client must show a popup with the content of the notification
  pub fn update_service_notification(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateServiceNotification)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_service_notification
  }

  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the client
  pub fn update_supergroup(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateSupergroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_supergroup
  }

  /// Some data from supergroupFullInfo has been changed
  pub fn update_supergroup_full_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateSupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_supergroup_full_info
  }

  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method should be called with the reason "Decline ToS update"
  pub fn update_terms_of_service(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateTermsOfService)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_terms_of_service
  }

  /// The list of trending sticker sets was updated or some of them were viewed
  pub fn update_trending_sticker_sets(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateTrendingStickerSets)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_trending_sticker_sets
  }

  /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if a message database is used
  pub fn update_unread_chat_count(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateUnreadChatCount)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_unread_chat_count
  }

  /// Number of unread messages has changed. This update is sent only if a message database is used
  pub fn update_unread_message_count(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateUnreadMessageCount)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_unread_message_count
  }

  /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the client
  pub fn update_user(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateUser)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_user
  }

  /// User activity in the chat has changed
  pub fn update_user_chat_action(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateUserChatAction)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_user_chat_action
  }

  /// Some data from userFullInfo has been changed
  pub fn update_user_full_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateUserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_user_full_info
  }

  /// Some privacy setting rules have been changed
  pub fn update_user_privacy_setting_rules(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateUserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_user_privacy_setting_rules
  }

  /// The user went online or offline
  pub fn update_user_status(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UpdateUserStatus)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update_user_status
  }



  /// Represents the current authorization state of the client
  pub fn authorization_state(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, AuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.authorization_state
  }

  /// Represents result of checking whether a username can be set for a chat
  pub fn check_chat_username_result(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, CheckChatUsernameResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.check_chat_username_result
  }

  /// Represents the value of a string in a language pack
  pub fn language_pack_string_value(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, LanguagePackStringValue)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.language_pack_string_value
  }

  /// Represents the value of an option
  pub fn option_value(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, OptionValue)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.option_value
  }

  /// Contains information about a Telegram Passport element
  pub fn passport_element(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, PassportElement)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.passport_element
  }

  /// Contains notifications about data changes
  pub fn update(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Update)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.update
  }

  /// Contains information about the period of inactivity after which the current user's account will automatically be deleted
  pub fn account_ttl(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, AccountTtl)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.account_ttl
  }

  /// Represents a list of animations
  pub fn animations(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Animations)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.animations
  }

  /// Information about the authentication code that was sent
  pub fn authentication_code_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, AuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.authentication_code_info
  }

  /// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
  pub fn basic_group(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, BasicGroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.basic_group
  }

  /// Contains full information about a basic group
  pub fn basic_group_full_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, BasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.basic_group_full_info
  }

  /// Contains the call identifier
  pub fn call_id(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, CallId)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.call_id
  }

  /// Contains a bot's answer to a callback query
  pub fn callback_query_answer(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, CallbackQueryAnswer)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.callback_query_answer
  }

  /// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
  pub fn chat(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Chat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.chat
  }

  /// Contains a list of chat events
  pub fn chat_events(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, ChatEvents)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.chat_events
  }

  /// Contains a chat invite link
  pub fn chat_invite_link(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, ChatInviteLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.chat_invite_link
  }

  /// Contains information about a chat invite link
  pub fn chat_invite_link_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, ChatInviteLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.chat_invite_link_info
  }

  /// A user with information about joining/leaving a chat
  pub fn chat_member(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, ChatMember)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.chat_member
  }

  /// Contains a list of chat members
  pub fn chat_members(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, ChatMembers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.chat_members
  }

  /// Contains information about the availability of the "Report spam" action for a chat
  pub fn chat_report_spam_state(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, ChatReportSpamState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.chat_report_spam_state
  }

  /// Represents a list of chats
  pub fn chats(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Chats)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.chats
  }

  /// Contains a list of websites the current user is logged in with Telegram
  pub fn connected_websites(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, ConnectedWebsites)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.connected_websites
  }

  /// Contains a counter
  pub fn count(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Count)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.count
  }

  /// Contains the result of a custom request
  pub fn custom_request_result(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, CustomRequestResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.custom_request_result
  }

  /// Contains information about a tg:// deep link
  pub fn deep_link_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, DeepLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.deep_link_info
  }

  /// Information about the email address authentication code that was sent
  pub fn email_address_authentication_code_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, EmailAddressAuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.email_address_authentication_code_info
  }

  /// An object of this type can be returned on every function call, in case of an error
  pub fn error(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Error)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.error
  }

  /// Represents a file
  pub fn file(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, File)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.file
  }

  /// A text with some entities
  pub fn formatted_text(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, FormattedText)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.formatted_text
  }

  /// Contains a list of messages found by a search
  pub fn found_messages(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, FoundMessages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.found_messages
  }

  /// Contains a list of game high scores
  pub fn game_high_scores(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, GameHighScores)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.game_high_scores
  }

  /// Contains a list of hashtags
  pub fn hashtags(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Hashtags)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.hashtags
  }

  /// Represents the result of an ImportContacts request
  pub fn imported_contacts(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, ImportedContacts)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.imported_contacts
  }

  /// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
  pub fn inline_query_results(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, InlineQueryResults)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.inline_query_results
  }

  /// Contains a list of language pack strings
  pub fn language_pack_strings(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, LanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.language_pack_strings
  }

  /// Contains information about the current localization target
  pub fn localization_target_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, LocalizationTargetInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.localization_target_info
  }

  /// Describes a message
  pub fn message(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Message)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.message
  }

  /// Contains a list of messages
  pub fn messages(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Messages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.messages
  }

  /// A full list of available network statistic entries
  pub fn network_statistics(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, NetworkStatistics)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.network_statistics
  }

  /// An object of this type is returned on a successful function call for certain functions
  pub fn ok(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Ok)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.ok
  }

  /// Order information
  pub fn order_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, OrderInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.order_info
  }

  /// Contains information about a Telegram Passport authorization form that was requested
  pub fn passport_authorization_form(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, PassportAuthorizationForm)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.passport_authorization_form
  }

  /// Contains information about saved Telegram Passport elements
  pub fn passport_elements(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, PassportElements)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.passport_elements
  }

  /// Represents the current state of 2-step verification
  pub fn password_state(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, PasswordState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.password_state
  }

  /// Contains information about an invoice payment form
  pub fn payment_form(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, PaymentForm)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.payment_form
  }

  /// Contains information about a successful payment
  pub fn payment_receipt(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, PaymentReceipt)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.payment_receipt
  }

  /// Contains the result of a payment request
  pub fn payment_result(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, PaymentResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.payment_result
  }

  /// Represents a list of proxy servers
  pub fn proxies(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Proxies)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.proxies
  }

  /// Contains information about a proxy server
  pub fn proxy(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Proxy)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.proxy
  }

  /// Contains a public HTTPS link to a message in a public supergroup or channel
  pub fn public_message_link(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, PublicMessageLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.public_message_link
  }

  /// Contains information about the current recovery email address
  pub fn recovery_email_address(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, RecoveryEmailAddress)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.recovery_email_address
  }

  /// Contains information about notification settings for several chats
  pub fn scope_notification_settings(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, ScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.scope_notification_settings
  }

  /// Contains a value representing a number of seconds
  pub fn seconds(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Seconds)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.seconds
  }

  /// Represents a secret chat
  pub fn secret_chat(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, SecretChat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.secret_chat
  }

  /// Contains a list of sessions
  pub fn sessions(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Sessions)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.sessions
  }

  /// Represents a list of all emoji corresponding to a sticker in a sticker set. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
  pub fn sticker_emojis(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, StickerEmojis)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.sticker_emojis
  }

  /// Represents a sticker set
  pub fn sticker_set(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, StickerSet)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.sticker_set
  }

  /// Represents a list of sticker sets
  pub fn sticker_sets(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, StickerSets)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.sticker_sets
  }

  /// Represents a list of stickers
  pub fn stickers(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Stickers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.stickers
  }

  /// Contains the exact storage usage statistics split by chats and file type
  pub fn storage_statistics(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, StorageStatistics)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.storage_statistics
  }

  /// Contains approximate storage usage statistics, excluding files of unknown file type
  pub fn storage_statistics_fast(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, StorageStatisticsFast)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.storage_statistics_fast
  }

  /// Represents a supergroup or channel with zero or more members (subscribers in the case of channels). From the point of view of the system, a channel is a special kind of a supergroup: only administrators can post and see the list of members, and posts from all administrators use the name and photo of the channel instead of individual names and profile photos. Unlike supergroups, channels can have an unlimited number of subscribers
  pub fn supergroup(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Supergroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.supergroup
  }

  /// Contains full information about a supergroup or channel
  pub fn supergroup_full_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, SupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.supergroup_full_info
  }

  /// Contains a list of t.me URLs
  pub fn t_me_urls(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TMeUrls)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.t_me_urls
  }

  /// Returns information about the availability of a temporary password, which can be used for payments
  pub fn temporary_password_state(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TemporaryPasswordState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.temporary_password_state
  }

  /// A simple object containing a sequence of bytes; for testing only
  pub fn test_bytes(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TestBytes)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.test_bytes
  }

  /// A simple object containing a number; for testing only
  pub fn test_int(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TestInt)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.test_int
  }

  /// A simple object containing a string; for testing only
  pub fn test_string(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TestString)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.test_string
  }

  /// A simple object containing a vector of numbers; for testing only
  pub fn test_vector_int(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TestVectorInt)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.test_vector_int
  }

  /// A simple object containing a vector of objects that hold a number; for testing only
  pub fn test_vector_int_object(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TestVectorIntObject)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.test_vector_int_object
  }

  /// A simple object containing a vector of strings; for testing only
  pub fn test_vector_string(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TestVectorString)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.test_vector_string
  }

  /// A simple object containing a vector of objects that hold a string; for testing only
  pub fn test_vector_string_object(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TestVectorStringObject)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.test_vector_string_object
  }

  /// Contains some text
  pub fn text(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Text)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.text
  }

  /// Contains a list of text entities
  pub fn text_entities(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TextEntities)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.text_entities
  }

  /// Represents a user
  pub fn user(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, User)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.user
  }

  /// Contains full information about a user (except the full list of profile photos)
  pub fn user_full_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.user_full_info
  }

  /// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
  pub fn user_privacy_setting_rules(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.user_privacy_setting_rules
  }

  /// Contains part of the list of user photos
  pub fn user_profile_photos(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, UserProfilePhotos)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.user_profile_photos
  }

  /// Represents a list of users
  pub fn users(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Users)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.users
  }

  /// Contains a temporary identifier of validated order information, which is stored for one hour. Also contains the available shipping options
  pub fn validated_order_info(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, ValidatedOrderInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.validated_order_info
  }

  /// Contains a list of wallpapers
  pub fn wallpapers(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, Wallpapers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.wallpapers
  }

  /// Describes a web page preview
  pub fn web_page(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, WebPage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.web_page
  }

  /// Describes an instant view page for a web page
  pub fn web_page_instant_view(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, WebPageInstantView)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.web_page_instant_view
  }


}



