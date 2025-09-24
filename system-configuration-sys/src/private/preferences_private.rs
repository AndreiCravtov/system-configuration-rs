use crate::preferences::AuthorizationRef;

extern "C" {
    pub static kSCPreferencesUseEntitlementAuthorization: AuthorizationRef;
}