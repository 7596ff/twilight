use bitflags::bitflags;

bitflags! {
    /// A set of bitflags which can be used to specify what resource to process
    /// into the cache.
    ///
    /// For example, specifying [`CHANNEL`] but not [`MESSAGE`] will cache
    /// created channels, channel updates, and channel deletes, but not their
    /// messages.
    pub struct ResourceType: u64 {
        const CHANNEL = 1;
        const EMOJI = 1 << 1;
        const GUILD = 1 << 2;
        const MEMBER = 1 << 3;
        const MESSAGE = 1 << 4;
        const PRESENCE = 1 << 5;
        const REACTION = 1 << 6;
        const ROLE = 1 << 7;
        const USER_CURRENT = 1 << 8;
        const USER = 1 << 9;
        const VOICE_STATE = 1 << 10;
        const STAGE_INSTANCE = 1 << 11;
        const INTEGRATION = 1 << 12;
    }
}

/// Configuration for an [`InMemoryCache`].
///
/// [`InMemoryCache`]: crate::InMemoryCache
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub(super) resource_types: ResourceType,
    pub(super) message_cache_size: usize,
}

impl Config {
    /// Create a new default configuration.
    ///
    /// Refer to individual getters for their defaults.
    pub const fn new() -> Self {
        Self {
            resource_types: ResourceType::all(),
            message_cache_size: 100,
        }
    }

    /// Returns an immutable reference to the message cache size.
    ///
    /// Defaults to 100.
    pub const fn message_cache_size(&self) -> usize {
        self.message_cache_size
    }

    /// Returns a mutable reference to the message cache size.
    pub fn message_cache_size_mut(&mut self) -> &mut usize {
        &mut self.message_cache_size
    }
    /// Returns an immutable reference to the resource types enabled.
    ///
    /// Defaults to all resource types.
    pub const fn resource_types(&self) -> ResourceType {
        self.resource_types
    }

    /// Returns a mutable reference to the resource types enabled.
    pub fn resource_types_mut(&mut self) -> &mut ResourceType {
        &mut self.resource_types
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, ResourceType};
    use static_assertions::assert_fields;

    assert_fields!(Config: resource_types, message_cache_size);

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_resource_type_const_values() {
        assert_eq!(1, ResourceType::CHANNEL.bits());
        assert_eq!(1 << 1, ResourceType::EMOJI.bits());
        assert_eq!(1 << 2, ResourceType::GUILD.bits());
        assert_eq!(1 << 3, ResourceType::MEMBER.bits());
        assert_eq!(1 << 4, ResourceType::MESSAGE.bits());
        assert_eq!(1 << 5, ResourceType::PRESENCE.bits());
        assert_eq!(1 << 6, ResourceType::REACTION.bits());
        assert_eq!(1 << 7, ResourceType::ROLE.bits());
        assert_eq!(1 << 8, ResourceType::USER_CURRENT.bits());
        assert_eq!(1 << 9, ResourceType::USER.bits());
        assert_eq!(1 << 10, ResourceType::VOICE_STATE.bits());
        assert_eq!(1 << 11, ResourceType::STAGE_INSTANCE.bits());
    }

    #[test]
    fn test_defaults() {
        let conf = Config {
            resource_types: ResourceType::all(),
            message_cache_size: 100,
        };
        let default = Config::default();
        assert_eq!(conf.resource_types, default.resource_types);
        assert_eq!(conf.message_cache_size, default.message_cache_size);
    }
}
