use crate::*;

impl Default for Methods {
    #[inline]
    fn default() -> Self {
        Self::UNKNOWN(String::new())
    }
}

impl Display for Methods {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            Self::GET => GET,
            Self::POST => POST,
            Self::CONNECT => CONNECT,
            Self::DELETE => DELETE,
            Self::HEAD => HEAD,
            Self::PATCH => PATCH,
            Self::TRACE => TRACE,
            Self::PUT => PUT,
            Self::OPTIONS => OPTIONS,
            Self::UNKNOWN(methods) => methods,
        };
        write!(f, "{}", res)
    }
}

impl FromStr for Methods {
    type Err = ();

    #[inline]
    fn from_str(methods_str: &str) -> Result<Self, Self::Err> {
        match methods_str {
            methods if methods == GET => Ok(Self::GET),
            methods if methods == POST => Ok(Self::POST),
            methods if methods == PUT => Ok(Self::PUT),
            methods if methods == DELETE => Ok(Self::DELETE),
            methods if methods == PATCH => Ok(Self::PATCH),
            methods if methods == HEAD => Ok(Self::HEAD),
            methods if methods == OPTIONS => Ok(Self::OPTIONS),
            methods if methods == CONNECT => Ok(Self::CONNECT),
            methods if methods == TRACE => Ok(Self::TRACE),
            _ => Ok(Self::UNKNOWN(methods_str.to_string())),
        }
    }
}

impl Methods {
    /// Creates a new instance of `Methods` with the default value of `Self::GET`.
    ///
    /// This is a shorthand for using the `default` method.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if the current method is `GET`.
    ///
    /// Returns `true` if the method is `GET`, otherwise returns `false`.
    #[inline]
    pub fn is_get(&self) -> bool {
        matches!(self, Self::GET)
    }

    /// Checks if the current method is `POST`.
    ///
    /// Returns `true` if the method is `POST`, otherwise returns `false`.
    #[inline]
    pub fn is_post(&self) -> bool {
        matches!(self, Self::POST)
    }

    /// Checks if the current method is `PUT`.
    ///
    /// Returns `true` if the method is `PUT`, otherwise returns `false`.
    #[inline]
    pub fn is_put(&self) -> bool {
        matches!(self, Self::PUT)
    }

    /// Checks if the current method is `DELETE`.
    ///
    /// Returns `true` if the method is `DELETE`, otherwise returns `false`.
    #[inline]
    pub fn is_delete(&self) -> bool {
        matches!(self, Self::DELETE)
    }

    /// Checks if the current method is `PATCH`.
    ///
    /// Returns `true` if the method is `PATCH`, otherwise returns `false`.
    #[inline]
    pub fn is_patch(&self) -> bool {
        matches!(self, Self::PATCH)
    }

    /// Checks if the current method is `HEAD`.
    ///
    /// Returns `true` if the method is `HEAD`, otherwise returns `false`.
    #[inline]
    pub fn is_head(&self) -> bool {
        matches!(self, Self::HEAD)
    }

    /// Checks if the current method is `OPTIONS`.
    ///
    /// Returns `true` if the method is `OPTIONS`, otherwise returns `false`.
    #[inline]
    pub fn is_options(&self) -> bool {
        matches!(self, Self::OPTIONS)
    }

    /// Checks if the current method is `CONNECT`.
    ///
    /// Returns `true` if the method is `CONNECT`, otherwise returns `false`.
    #[inline]
    pub fn is_connect(&self) -> bool {
        matches!(self, Self::CONNECT)
    }

    /// Checks if the current method is `TRACE`.
    ///
    /// Returns `true` if the method is `TRACE`, otherwise returns `false`.
    #[inline]
    pub fn is_trace(&self) -> bool {
        matches!(self, Self::TRACE)
    }

    /// Checks if the current method is `UNKNOWN`.
    ///
    /// Returns `true` if the method is `UNKNOWN`, otherwise returns `false`.
    #[inline]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::UNKNOWN(_))
    }
}
