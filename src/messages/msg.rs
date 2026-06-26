use derive_more::{Deref, Eq, PartialEq};

#[cfg(not(feature = "thread-share"))]
use implicit_clone::sync::IString;
#[cfg(feature = "thread-share")]
use implicit_clone::unsync::IString;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum MessageKind {
    Info,
    Warning,
    Error,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct SString(IString);

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for SString {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        String::schema_name()
    }
    fn json_schema(x: &mut schemars::SchemaGenerator) -> schemars::Schema {
        String::json_schema(x)
    }
}

/// Represents a general purpose message that the application can
/// add to the report at any level for user information that has
/// no specific format and doesn't fit into the normal structure.
///
/// - Each message has a `Level` (generic parameter) that limits
///   the context in which it is applicable
/// - The [`kind`](MessageKind) specifies the severity of the
///   described event
/// - The message can be any string for maximum flexibility when
///   reporting problems or just hints to the user
///
/// Next to [`Self::new`] and [`Self::cnew`] (for const) there
/// are also utility constructors for every kind.
#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Message<Level> {
    kind: MessageKind,
    #[cfg_attr(feature = "utoipa", schema(value_type=String))]
    msg: SString,
    #[cfg_attr(feature = "serde", serde(skip))]
    phantom: std::marker::PhantomData<Level>,
}

impl<Level> Message<Level> {
    pub fn new(kind: MessageKind, msg: impl Into<IString>) -> Self {
        Self {
            kind,
            msg: SString(msg.into()),
            phantom: std::marker::PhantomData {},
        }
    }
    pub const fn cnew(kind: MessageKind, msg: &'static str) -> Self {
        Self {
            kind,
            msg: SString(IString::Static(msg)),
            phantom: std::marker::PhantomData {},
        }
    }
    pub const fn cinfo(msg: &'static str) -> Self {
        Self::cnew(MessageKind::Info, msg)
    }
    pub const fn cwarning(msg: &'static str) -> Self {
        Self::cnew(MessageKind::Warning, msg)
    }
    pub const fn cerror(msg: &'static str) -> Self {
        Self::cnew(MessageKind::Error, msg)
    }

    pub fn info(msg: impl Into<IString>) -> Self {
        Self::new(MessageKind::Info, msg)
    }
    pub fn warning(msg: impl Into<IString>) -> Self {
        Self::new(MessageKind::Warning, msg)
    }
    pub fn error(msg: impl Into<IString>) -> Self {
        Self::new(MessageKind::Error, msg)
    }

    pub const fn kind(&self) -> &MessageKind {
        &self.kind
    }
    pub fn msg(&self) -> &str {
        self.msg.as_ref()
    }
    pub fn change_level<L>(self) -> Message<L> {
        Message {
            kind: self.kind,
            msg: self.msg,
            phantom: std::marker::PhantomData {},
        }
    }
}

impl<Level> PartialOrd for Message<Level> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<Level> Ord for Message<Level> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind()
            .cmp(other.kind())
            .then(self.msg().cmp(other.msg()))
    }
}

impl<Level> Clone for Message<Level> {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind,
            msg: self.msg.clone(),
            phantom: self.phantom,
        }
    }
}

impl<Level> implicit_clone::ImplicitClone for Message<Level> {}
