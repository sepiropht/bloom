use crate::{
    config::Config,
    consts::NamespaceType,
    consts::{self, BillingPlan, TwoFaMethod},
    db::DB,
    domain::{files, inbox},
    drivers,
    entities::{Session, User},
    notifications::PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE_ID,
    notifications::PAYMENT_FAILED_EMAIL_TEMPLATE,
    notifications::PAYMENT_FAILED_EMAIL_TEMPLATE_ID,
    notifications::REGISTRATION_EMAIL_TEMPLATE,
    notifications::REGISTRATION_EMAIL_TEMPLATE_ID,
    notifications::SIGN_IN_EMAIL_TEMPLATE,
    notifications::{
        EMAIL_CHANGED_EMAIL_TEMPLATE, EMAIL_CHANGED_EMAIL_TEMPLATE_ID, GROUP_INVITATION_EMAIL_TEMPLATE,
        GROUP_INVITATION_EMAIL_TEMPLATE_ID, PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE, SIGN_IN_EMAIL_TEMPLATE_ID,
        VERIFY_EMAIL_EMAIL_TEMPLATE, VERIFY_EMAIL_EMAIL_TEMPLATE_ID,
    },
    repository::Repository,
    Actor, Error,
};
use std::{collections::HashSet, fmt::Debug, sync::Arc};
use stdx::uuid::Uuid;

mod accept_group_invitation;
mod cancel_group_invitation;
mod check_namespace_exists;
mod check_namespace_membership;
mod complete_registration;
mod complete_sign_in;
mod complete_two_fa_challenge;
mod complete_two_fa_setup;
mod config;
mod create_group;
mod create_namespace;
mod decline_group_invitation;
mod delete_group;
mod delete_my_account;
mod disable_two_fa;
mod find_group_and_membership;
mod find_namespace_and_membership;
mod find_upload;
mod invite_people_in_group;
mod quit_group;
mod register;
mod remove_member_from_group;
mod revoke_session;
mod send_register_email;
mod setup_two_fa;
mod sign_in;
mod update_group_profile;
mod update_my_profile;
mod update_namespace;
mod utils;
mod validators;
mod verify_email;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
    config: Arc<Config>,
    queue: Arc<dyn drivers::Queue>,
    mailer: Arc<dyn drivers::Mailer>,
    storage: Arc<dyn drivers::Storage>,
    templates: tera::Tera,
    invalid_namespaces: HashSet<String>,
    valid_namespace_alphabet: HashSet<char>,
    files_service: Option<Arc<dyn files::Service>>,
    inbox_service: Option<Arc<dyn inbox::Service>>,
    xss: Arc<dyn drivers::XssSanitizer>,
}

impl Service {
    pub fn new(
        config: Config,
        db: DB,
        queue: Arc<dyn drivers::Queue>,
        mailer: Arc<dyn drivers::Mailer>,
        storage: Arc<dyn drivers::Storage>,
        xss: Arc<dyn drivers::XssSanitizer>,
    ) -> Service {
        let mut templates = tera::Tera::default();
        // don't escape input as it's provided by us
        templates.autoescape_on(Vec::new());
        templates
            .add_raw_template(REGISTRATION_EMAIL_TEMPLATE_ID, REGISTRATION_EMAIL_TEMPLATE)
            .expect("kernel: parsing REGISTRATION_EMAIL_TEMPLATE");
        templates
            .add_raw_template(SIGN_IN_EMAIL_TEMPLATE_ID, SIGN_IN_EMAIL_TEMPLATE)
            .expect("kernel: parsing SIGN_IN_EMAIL_TEMPLATE");
        templates
            .add_raw_template(PAYMENT_FAILED_EMAIL_TEMPLATE_ID, PAYMENT_FAILED_EMAIL_TEMPLATE)
            .expect("kernel: parsing PAYMENT_FAILED_EMAIL_TEMPLATE");
        templates
            .add_raw_template(
                PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE_ID,
                PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE,
            )
            .expect("kernel: parsing PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE");
        templates
            .add_raw_template(VERIFY_EMAIL_EMAIL_TEMPLATE_ID, VERIFY_EMAIL_EMAIL_TEMPLATE)
            .expect("kernel: parsing VERIFY_EMAIL_EMAIL_TEMPLATE");
        templates
            .add_raw_template(EMAIL_CHANGED_EMAIL_TEMPLATE_ID, EMAIL_CHANGED_EMAIL_TEMPLATE)
            .expect("kernel: parsing EMAIL_CHANGED_EMAIL_TEMPLATE");
        templates
            .add_raw_template(GROUP_INVITATION_EMAIL_TEMPLATE_ID, GROUP_INVITATION_EMAIL_TEMPLATE)
            .expect("kernel: parsing GROUP_INVITATION_EMAIL_TEMPLATE");

        let repo = Repository::new();

        let invalid_namespaces = consts::INVALID_NAMESPACES
            .iter()
            .map(|namespace| namespace.to_string())
            .collect();

        let valid_namespace_alphabet = consts::NAMESPACE_ALPHABET.chars().collect();

        let config = Arc::new(config);

        Service {
            db,
            repo,
            config,
            queue,
            mailer,
            storage,
            templates,
            invalid_namespaces,
            valid_namespace_alphabet,
            files_service: None,
            inbox_service: None,
            xss,
        }
    }

    pub async fn send_sign_in_email(&self, input: SendSignInEmailInput) -> Result<(), Error> {
        todo!(); // TODO
    }

    pub async fn send_email_changed_email(&self, input: SendEmailChangedEmailInput) -> Result<(), Error> {
        todo!(); // TODO
    }

    pub async fn send_verify_email_email(&self, input: SendVerifyEmailEmailInput) -> Result<(), Error> {
        todo!(); // TODO
    }

    pub async fn send_group_invitation_email(&self, input: SendGroupInvitationEmailInput) -> Result<(), Error> {
        todo!(); // TODO
    }

    pub async fn decode_and_validate_session_token(&self, _token: String) -> Result<User, Error> {
        todo!(); // TODO
    }

    pub async fn decode_and_validate_anonymous_token(&self, _token: String) -> Result<Uuid, Error> {
        todo!(); // TODO
    }

    pub async fn dispatch_delete_old_data(&self) -> Result<(), Error> {
        todo!(); // TODO
    }

    // create an entity that can be retrieved later with the size, the user and the tmp_key
    pub async fn get_signed_storage_url(
        &self,
        _actor: Actor,
        _input: GetSignedStorageUploadUrlInput,
    ) -> Result<SignedStorageUrl, Error> {
        todo!(); // TODO
    }
}

#[derive(Debug, Clone)]
pub enum SignedIn {
    Success { session: Session, user: User },
    TwoFa(TwoFaMethod),
}

#[derive(Debug, Clone)]
pub struct Registered {
    pub session: Session,
    pub user: User,
}

/// RegisterInput are the data required to start to register to bloom
#[derive(Debug, Clone)]
pub struct RegisterInput {
    pub email: String,
    pub username: String,
}

/// CompleteRegistrationInput are the data required to complete a bloom registration
#[derive(Debug, Clone)]
pub struct CompleteRegistrationInput {
    pub pending_user_id: Uuid,
    pub code: String,
}

/// CompleteSignInInput are the data required to complete a sign in
#[derive(Debug, Clone)]
pub struct CompleteSignInInput {
    pub pending_session_id: Uuid,
    pub code: String,
}

/// SignInInput are the data required to start a sign in
#[derive(Debug, Clone)]
pub struct SignInInput {
    pub email_or_username: String,
}

#[derive(Debug, Clone)]
pub struct CreateGroupInput {
    pub name: String,
    pub path: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct DeleteGroupInput {
    pub group_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct CreateNamespaceInput {
    pub path: String,
    pub namespace_type: NamespaceType,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct UpdatePaymentMethodInput {
    pub stripe_id: String,
    pub namespace_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct ChangeSubscriptionInput {
    pub namespace_id: Uuid,
    pub plan: BillingPlan,
}

#[derive(Debug, Clone)]
pub struct GetCheckoutSessionInput {
    pub namespace: String,
    pub plan: BillingPlan,
}

#[derive(Debug, Clone)]
pub struct UpdateBillingInformationInput {
    pub namespace: String,
    pub name: String,
    pub email: String,
    pub country_code: String,
    pub city: String,
    pub postal_code: String,
    pub address_line1: String,
    pub address_line2: String,
    pub state: String,
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SyncBillingWithProviderInput {
    pub namespace: String,
}

#[derive(Debug, Clone)]
pub struct UpdateMyProfileInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub description: Option<String>,
    // pub avatar: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct VerifyPendingEmailInput {
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct UpdateGroupProfileInput {
    pub group_id: Uuid,
    pub name: Option<String>,
    pub path: Option<String>,
    pub description: Option<String>,
    // pub avatar: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct InvitePeopleInGroupInput {
    pub group_id: Uuid,
    pub usernames: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AcceptGroupInvitationInput {
    pub invitation_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct CancelGroupInvitationInput {
    pub invitation_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct DeclineGroupInvitationInput {
    pub invitation_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct RemoveMemberFromGroupInput {
    pub group_id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct QuitGroupInput {
    pub group_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct GetSignedStorageUploadUrlInput {
    pub filesize: u64,
}

#[derive(Debug, Clone)]
pub struct EnableTwoFaInput {
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct DisableTwoFaInput {
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct CompleteTwoFaChallengeInput {
    pub pending_session_id: Uuid,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct DeleteMyAccountInput {
    pub two_fa_totp_code: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RevokeSessionInput {
    pub session_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct VerifyEmailInput {
    pub pending_email_id: Uuid,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct DecodedSessionToken {
    pub session_id: Uuid,
    pub secret: Vec<u8>,
}

// type GroupMember struct {
// 	User
// 	Role GroupRole `db:"role"`
// }

#[derive(Debug, Clone)]
pub struct SignedStorageUrl {
    pub url: String,
    pub upload_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct SendGroupInvitationEmailInput {
    pub invitation_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct SendRegisterEmailInput {
    pub email: String,
    pub username: String,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct SendSignInEmailInput {
    pub email: String,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct SendEmailChangedEmailInput {
    pub email: String,
    pub name: String,
    pub new_email: String,
}

#[derive(Debug, Clone)]
pub struct SendVerifyEmailEmailInput {
    pub email: String,
    pub name: String,
    pub code: String,
}

// type NamespaceAndCustomer struct {
// 	Customer
// 	Namespace
// }
