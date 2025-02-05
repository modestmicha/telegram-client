use std::sync::{RwLock};
use std::collections::HashMap;
use futures::channel::mpsc;
use rtdlib::types::{RObject, TdType};

lazy_static! {
  static ref OBSERVER: Observer = {
    Observer::new()
  };
}

struct Observer {
  channels: RwLock<HashMap<String, mpsc::Sender<TdType>>>,
}

impl Observer {
  fn new() -> Self {
    Self {
      channels: RwLock::new(HashMap::new())
    }
  }

  fn notify(&self, payload: TdType) {
    let extra = match &payload {


      TdType::TestUseUpdate(value) => value.extra(),

      TdType::UpdateActiveNotifications(value) => value.extra(),

      TdType::UpdateAnimatedEmojiMessageClicked(value) => value.extra(),

      TdType::UpdateAnimationSearchParameters(value) => value.extra(),

      TdType::UpdateAuthorizationState(value) => value.extra(),

      TdType::UpdateBasicGroup(value) => value.extra(),

      TdType::UpdateBasicGroupFullInfo(value) => value.extra(),

      TdType::UpdateCall(value) => value.extra(),

      TdType::UpdateChatAction(value) => value.extra(),

      TdType::UpdateChatActionBar(value) => value.extra(),

      TdType::UpdateChatDefaultDisableNotification(value) => value.extra(),

      TdType::UpdateChatDefaultMessageSenderId(value) => value.extra(),

      TdType::UpdateChatDraftMessage(value) => value.extra(),

      TdType::UpdateChatFilters(value) => value.extra(),

      TdType::UpdateChatHasProtectedContent(value) => value.extra(),

      TdType::UpdateChatHasScheduledMessages(value) => value.extra(),

      TdType::UpdateChatIsBlocked(value) => value.extra(),

      TdType::UpdateChatIsMarkedAsUnread(value) => value.extra(),

      TdType::UpdateChatLastMessage(value) => value.extra(),

      TdType::UpdateChatMember(value) => value.extra(),

      TdType::UpdateChatMessageTtlSetting(value) => value.extra(),

      TdType::UpdateChatNotificationSettings(value) => value.extra(),

      TdType::UpdateChatOnlineMemberCount(value) => value.extra(),

      TdType::UpdateChatPendingJoinRequests(value) => value.extra(),

      TdType::UpdateChatPermissions(value) => value.extra(),

      TdType::UpdateChatPhoto(value) => value.extra(),

      TdType::UpdateChatPosition(value) => value.extra(),

      TdType::UpdateChatReadInbox(value) => value.extra(),

      TdType::UpdateChatReadOutbox(value) => value.extra(),

      TdType::UpdateChatReplyMarkup(value) => value.extra(),

      TdType::UpdateChatTheme(value) => value.extra(),

      TdType::UpdateChatThemes(value) => value.extra(),

      TdType::UpdateChatTitle(value) => value.extra(),

      TdType::UpdateChatUnreadMentionCount(value) => value.extra(),

      TdType::UpdateChatVideoChat(value) => value.extra(),

      TdType::UpdateConnectionState(value) => value.extra(),

      TdType::UpdateDeleteMessages(value) => value.extra(),

      TdType::UpdateDiceEmojis(value) => value.extra(),

      TdType::UpdateFavoriteStickers(value) => value.extra(),

      TdType::UpdateFile(value) => value.extra(),

      TdType::UpdateFileGenerationStart(value) => value.extra(),

      TdType::UpdateFileGenerationStop(value) => value.extra(),

      TdType::UpdateGroupCall(value) => value.extra(),

      TdType::UpdateGroupCallParticipant(value) => value.extra(),

      TdType::UpdateHavePendingNotifications(value) => value.extra(),

      TdType::UpdateInstalledStickerSets(value) => value.extra(),

      TdType::UpdateLanguagePackStrings(value) => value.extra(),

      TdType::UpdateMessageContent(value) => value.extra(),

      TdType::UpdateMessageContentOpened(value) => value.extra(),

      TdType::UpdateMessageEdited(value) => value.extra(),

      TdType::UpdateMessageInteractionInfo(value) => value.extra(),

      TdType::UpdateMessageIsPinned(value) => value.extra(),

      TdType::UpdateMessageLiveLocationViewed(value) => value.extra(),

      TdType::UpdateMessageMentionRead(value) => value.extra(),

      TdType::UpdateMessageSendAcknowledged(value) => value.extra(),

      TdType::UpdateMessageSendFailed(value) => value.extra(),

      TdType::UpdateMessageSendSucceeded(value) => value.extra(),

      TdType::UpdateNewCallSignalingData(value) => value.extra(),

      TdType::UpdateNewCallbackQuery(value) => value.extra(),

      TdType::UpdateNewChat(value) => value.extra(),

      TdType::UpdateNewChatJoinRequest(value) => value.extra(),

      TdType::UpdateNewChosenInlineResult(value) => value.extra(),

      TdType::UpdateNewCustomEvent(value) => value.extra(),

      TdType::UpdateNewCustomQuery(value) => value.extra(),

      TdType::UpdateNewInlineCallbackQuery(value) => value.extra(),

      TdType::UpdateNewInlineQuery(value) => value.extra(),

      TdType::UpdateNewMessage(value) => value.extra(),

      TdType::UpdateNewPreCheckoutQuery(value) => value.extra(),

      TdType::UpdateNewShippingQuery(value) => value.extra(),

      TdType::UpdateNotification(value) => value.extra(),

      TdType::UpdateNotificationGroup(value) => value.extra(),

      TdType::UpdateOption(value) => value.extra(),

      TdType::UpdatePoll(value) => value.extra(),

      TdType::UpdatePollAnswer(value) => value.extra(),

      TdType::UpdateRecentStickers(value) => value.extra(),

      TdType::UpdateSavedAnimations(value) => value.extra(),

      TdType::UpdateScopeNotificationSettings(value) => value.extra(),

      TdType::UpdateSecretChat(value) => value.extra(),

      TdType::UpdateSelectedBackground(value) => value.extra(),

      TdType::UpdateServiceNotification(value) => value.extra(),

      TdType::UpdateStickerSet(value) => value.extra(),

      TdType::UpdateSuggestedActions(value) => value.extra(),

      TdType::UpdateSupergroup(value) => value.extra(),

      TdType::UpdateSupergroupFullInfo(value) => value.extra(),

      TdType::UpdateTermsOfService(value) => value.extra(),

      TdType::UpdateTrendingStickerSets(value) => value.extra(),

      TdType::UpdateUnreadChatCount(value) => value.extra(),

      TdType::UpdateUnreadMessageCount(value) => value.extra(),

      TdType::UpdateUser(value) => value.extra(),

      TdType::UpdateUserFullInfo(value) => value.extra(),

      TdType::UpdateUserPrivacySettingRules(value) => value.extra(),

      TdType::UpdateUserStatus(value) => value.extra(),

      TdType::UpdateUsersNearby(value) => value.extra(),


      TdType::AuthorizationState(value) => value.extra(),

      TdType::CanTransferOwnershipResult(value) => value.extra(),

      TdType::ChatStatistics(value) => value.extra(),

      TdType::CheckChatUsernameResult(value) => value.extra(),

      TdType::CheckStickerSetNameResult(value) => value.extra(),

      TdType::InternalLinkType(value) => value.extra(),

      TdType::JsonValue(value) => value.extra(),

      TdType::LanguagePackStringValue(value) => value.extra(),

      TdType::LogStream(value) => value.extra(),

      TdType::LoginUrlInfo(value) => value.extra(),

      TdType::MessageFileType(value) => value.extra(),

      TdType::OptionValue(value) => value.extra(),

      TdType::PassportElement(value) => value.extra(),

      TdType::ResetPasswordResult(value) => value.extra(),

      TdType::StatisticalGraph(value) => value.extra(),

      TdType::Update(value) => value.extra(),

      TdType::AccountTtl(value) => value.extra(),

      TdType::AnimatedEmoji(value) => value.extra(),

      TdType::Animations(value) => value.extra(),

      TdType::AuthenticationCodeInfo(value) => value.extra(),

      TdType::AutoDownloadSettingsPresets(value) => value.extra(),

      TdType::Background(value) => value.extra(),

      TdType::Backgrounds(value) => value.extra(),

      TdType::BankCardInfo(value) => value.extra(),

      TdType::BasicGroup(value) => value.extra(),

      TdType::BasicGroupFullInfo(value) => value.extra(),

      TdType::BotCommands(value) => value.extra(),

      TdType::CallId(value) => value.extra(),

      TdType::CallbackQueryAnswer(value) => value.extra(),

      TdType::Chat(value) => value.extra(),

      TdType::ChatAdministrators(value) => value.extra(),

      TdType::ChatEvents(value) => value.extra(),

      TdType::ChatFilter(value) => value.extra(),

      TdType::ChatFilterInfo(value) => value.extra(),

      TdType::ChatInviteLink(value) => value.extra(),

      TdType::ChatInviteLinkCounts(value) => value.extra(),

      TdType::ChatInviteLinkInfo(value) => value.extra(),

      TdType::ChatInviteLinkMembers(value) => value.extra(),

      TdType::ChatInviteLinks(value) => value.extra(),

      TdType::ChatJoinRequests(value) => value.extra(),

      TdType::ChatLists(value) => value.extra(),

      TdType::ChatMember(value) => value.extra(),

      TdType::ChatMembers(value) => value.extra(),

      TdType::ChatPhotos(value) => value.extra(),

      TdType::Chats(value) => value.extra(),

      TdType::ChatsNearby(value) => value.extra(),

      TdType::ConnectedWebsites(value) => value.extra(),

      TdType::Count(value) => value.extra(),

      TdType::Countries(value) => value.extra(),

      TdType::CustomRequestResult(value) => value.extra(),

      TdType::DatabaseStatistics(value) => value.extra(),

      TdType::DeepLinkInfo(value) => value.extra(),

      TdType::EmailAddressAuthenticationCodeInfo(value) => value.extra(),

      TdType::Emojis(value) => value.extra(),

      TdType::Error(value) => value.extra(),

      TdType::File(value) => value.extra(),

      TdType::FilePart(value) => value.extra(),

      TdType::FormattedText(value) => value.extra(),

      TdType::FoundMessages(value) => value.extra(),

      TdType::GameHighScores(value) => value.extra(),

      TdType::GroupCall(value) => value.extra(),

      TdType::GroupCallId(value) => value.extra(),

      TdType::Hashtags(value) => value.extra(),

      TdType::HttpUrl(value) => value.extra(),

      TdType::ImportedContacts(value) => value.extra(),

      TdType::InlineQueryResults(value) => value.extra(),

      TdType::LanguagePackInfo(value) => value.extra(),

      TdType::LanguagePackStrings(value) => value.extra(),

      TdType::LocalizationTargetInfo(value) => value.extra(),

      TdType::LogTags(value) => value.extra(),

      TdType::LogVerbosityLevel(value) => value.extra(),

      TdType::Message(value) => value.extra(),

      TdType::MessageCalendar(value) => value.extra(),

      TdType::MessageLink(value) => value.extra(),

      TdType::MessageLinkInfo(value) => value.extra(),

      TdType::MessagePositions(value) => value.extra(),

      TdType::MessageSenders(value) => value.extra(),

      TdType::MessageStatistics(value) => value.extra(),

      TdType::MessageThreadInfo(value) => value.extra(),

      TdType::Messages(value) => value.extra(),

      TdType::NetworkStatistics(value) => value.extra(),

      TdType::Ok(value) => value.extra(),

      TdType::OrderInfo(value) => value.extra(),

      TdType::PassportAuthorizationForm(value) => value.extra(),

      TdType::PassportElements(value) => value.extra(),

      TdType::PassportElementsWithErrors(value) => value.extra(),

      TdType::PasswordState(value) => value.extra(),

      TdType::PaymentForm(value) => value.extra(),

      TdType::PaymentReceipt(value) => value.extra(),

      TdType::PaymentResult(value) => value.extra(),

      TdType::PhoneNumberInfo(value) => value.extra(),

      TdType::Proxies(value) => value.extra(),

      TdType::Proxy(value) => value.extra(),

      TdType::PushReceiverId(value) => value.extra(),

      TdType::RecommendedChatFilters(value) => value.extra(),

      TdType::RecoveryEmailAddress(value) => value.extra(),

      TdType::ScopeNotificationSettings(value) => value.extra(),

      TdType::Seconds(value) => value.extra(),

      TdType::SecretChat(value) => value.extra(),

      TdType::Session(value) => value.extra(),

      TdType::Sessions(value) => value.extra(),

      TdType::SponsoredMessages(value) => value.extra(),

      TdType::Sticker(value) => value.extra(),

      TdType::StickerSet(value) => value.extra(),

      TdType::StickerSets(value) => value.extra(),

      TdType::Stickers(value) => value.extra(),

      TdType::StorageStatistics(value) => value.extra(),

      TdType::StorageStatisticsFast(value) => value.extra(),

      TdType::Supergroup(value) => value.extra(),

      TdType::SupergroupFullInfo(value) => value.extra(),

      TdType::TMeUrls(value) => value.extra(),

      TdType::TemporaryPasswordState(value) => value.extra(),

      TdType::TestBytes(value) => value.extra(),

      TdType::TestInt(value) => value.extra(),

      TdType::TestString(value) => value.extra(),

      TdType::TestVectorInt(value) => value.extra(),

      TdType::TestVectorIntObject(value) => value.extra(),

      TdType::TestVectorString(value) => value.extra(),

      TdType::TestVectorStringObject(value) => value.extra(),

      TdType::Text(value) => value.extra(),

      TdType::TextEntities(value) => value.extra(),

      TdType::Updates(value) => value.extra(),

      TdType::User(value) => value.extra(),

      TdType::UserFullInfo(value) => value.extra(),

      TdType::UserPrivacySettingRules(value) => value.extra(),

      TdType::Users(value) => value.extra(),

      TdType::ValidatedOrderInfo(value) => value.extra(),

      TdType::WebPage(value) => value.extra(),

      TdType::WebPageInstantView(value) => value.extra(),


    };
    match extra {
      Some(extra) => {
        let mut map = self.channels.write().unwrap();
        if let Some(sender) = map.get_mut(&extra) {
          sender.try_send(payload).unwrap();
        }
      },
      None => {}
    }
  }

  fn subscribe(&self, extra: String) -> mpsc::Receiver<TdType> {
    let (sender, receiver) = mpsc::channel::<TdType>(1);
    match self.channels.write() {
      Ok(mut map) => {
        map.insert(extra, sender);
      }
      _ => {}
    };
    receiver
  }

  fn unsubscribe(&self, extra: &str) {
    match self.channels.write() {
      Ok(mut map) => {
        map.remove(extra);
      }
      _ => {}
    };
  }
}


pub fn notify(payload: TdType) {
  OBSERVER.notify(payload)
}

pub fn subscribe<T: AsRef<str>>(extra: T) -> mpsc::Receiver<TdType>{
  OBSERVER.subscribe(extra.as_ref().to_string())
}

pub fn unsubscribe<T: AsRef<str>>(extra: T) {
  OBSERVER.unsubscribe(extra.as_ref())
}

