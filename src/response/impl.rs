use crate::*;

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

impl Response {
    /// Creates a new instance of `Response`.
    ///
    /// # Returns
    /// - An initialized `Response` with default values.
    pub fn new() -> Self {
        Response {
            version: HttpVersion::HTTP1_1,
            status_code: 200,
            reason_phrase: EMPTY_STR.to_owned(),
            headers: hash_map_xx_hash3_64(),
            body: Vec::new(),
        }
    }

    /// Retrieves the value of a response header by its key.
    ///
    /// # Parameters
    /// - `key`: The header's key, which can be of any type that implements `Into<ResponseHeadersKey>`.
    ///
    /// # Returns
    /// - `OptionResponseHeadersValue`: Returns `Some(value)` if the key exists in the response headers,
    ///   or `None` if the key does not exist.
    pub fn get_header<K>(&self, key: K) -> OptionResponseHeadersValue
    where
        K: Into<ResponseHeadersKey>,
    {
        self.headers
            .get(&key.into())
            .and_then(|data| Some(data.clone()))
    }

    /// Retrieves the body content of the object as a UTF-8 encoded string.
    ///
    /// This method uses `String::from_utf8_lossy` to convert the byte slice returned by `self.get_body()` into a string.
    /// If the byte slice contains invalid UTF-8 sequences, they will be replaced with the Unicode replacement character (�).
    ///
    /// # Returns
    /// A `String` containing the body content.
    pub fn get_body_string(&self) -> String {
        String::from_utf8_lossy(self.get_body()).into_owned()
    }

    /// Deserializes the body content of the object into a specified type `T`.
    ///
    /// This method first retrieves the body content as a UTF-8 encoded string using `self.get_body()`.
    /// It then attempts to deserialize the string into the specified type `T` using `json_from_slice`.
    ///
    /// # Type Parameters
    /// - `T`: The target type to deserialize into. It must implement the `DeserializeOwned` trait.
    ///
    /// # Returns
    /// - `Ok(T)`: The deserialized object of type `T` if the deserialization is successful.
    /// - `Err(ResultJsonError)`: An error if the deserialization fails (e.g., invalid JSON format or type mismatch).
    pub fn get_body_json<T>(&self) -> ResultJsonError<T>
    where
        T: DeserializeOwned,
    {
        json_from_slice(self.get_body())
    }

    /// Adds a header to the response.
    ///
    /// This function inserts a key-value pair into the response headers.
    /// The key and value are converted into `ResponseHeadersKey`, allowing for efficient handling of both owned and borrowed string data.
    ///
    /// # Parameters
    /// - `key`: The header key, which will be converted into a `ResponseHeadersKey`.
    /// - `value`: The value of the header, which will be converted into a `ResponseHeadersValue`.
    ///
    /// # Returns
    /// - Returns a mutable reference to the current instance (`&mut Self`), allowing for method chaining.
    pub fn set_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<ResponseHeadersKey>,
        V: Into<ResponseHeadersValue>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set the body of the response.
    ///
    /// This method allows you to set the body of the response by converting the provided
    /// value into a `ResponseBody` type. The `body` is updated with the converted value,
    /// and the method returns a mutable reference to the current instance for method chaining.
    ///
    /// # Parameters
    /// - `body`: The body of the response to be set. It can be any type that can be converted
    ///   into a `ResponseBody` using the `Into` trait.
    ///
    /// # Return Value
    /// - Returns a mutable reference to the current instance of the struct, enabling method chaining.
    /// Set the body of the response.
    ///
    /// This method allows you to set the body of the response by converting the provided
    /// value into a `ResponseBody` type. The `body` is updated with the converted value,
    /// and the method returns a mutable reference to the current instance for method chaining.
    ///
    /// # Parameters
    /// - `body`: The body of the response to be set. It can be any type that can be converted
    ///   into a `ResponseBody` using the `Into` trait.
    ///
    /// # Return Value
    /// - Returns a mutable reference to the current instance of the struct, enabling method chaining.
    pub fn set_body<T: Into<ResponseBody>>(&mut self, body: T) -> &mut Self {
        self.body = body.into();
        self
    }

    /// Set the reason phrase of the response.
    ///
    /// This method allows you to set the reason phrase of the response by converting the
    /// provided value into a `ResponseReasonPhrase` type. The `reason_phrase` is updated
    /// with the converted value, and the method returns a mutable reference to the current
    /// instance for method chaining.
    ///
    /// # Parameters
    /// - `reason_phrase`: The reason phrase to be set for the response. It can be any type
    ///   that can be converted into a `ResponseReasonPhrase` using the `Into` trait.
    ///
    /// # Return Value
    /// - Returns a mutable reference to the current instance of the struct, enabling method chaining.
    pub fn set_reason_phrase<T: Into<ResponseReasonPhrase>>(
        &mut self,
        reason_phrase: T,
    ) -> &mut Self {
        self.reason_phrase = reason_phrase.into();
        self
    }

    /// Pushes a header with a key and value into the response string.
    ///
    /// # Parameters
    /// - `response_string`: A mutable reference to the string where the header will be added.
    /// - `key`: The header key as a string slice (`&str`).
    /// - `value`: The header value as a string slice (`&str`).
    pub(super) fn push_header(response_string: &mut String, key: &str, value: &str) {
        response_string.push_str(key);
        response_string.push_str(COLON_SPACE);
        response_string.push_str(value);
        response_string.push_str(HTTP_BR);
    }

    /// Pushes the first line of an HTTP response (version, status code, and reason phrase) into the response string.
    /// This corresponds to the status line of the HTTP response.
    ///
    /// # Parameters
    /// - `response_string`: A mutable reference to the string where the first line will be added.
    pub(super) fn push_http_response_first_line(&self, response_string: &mut String) {
        response_string.push_str(&self.get_version().to_string());
        response_string.push_str(SPACE);
        response_string.push_str(&self.get_status_code().to_string());
        response_string.push_str(SPACE);
        response_string.push_str(self.get_reason_phrase());
        response_string.push_str(HTTP_BR);
    }

    /// Builds the full HTTP response as a byte vector.
    /// # Returns
    /// - `ResponseData`: response data
    pub fn build(&mut self) -> ResponseData {
        if self.reason_phrase.is_empty() {
            self.set_reason_phrase(HttpStatus::phrase(*self.get_status_code()));
        }
        let mut response_string: String = String::new();
        self.push_http_response_first_line(&mut response_string);
        let mut compress_type_opt: OptionCompress = None;
        let mut connection_opt: OptionString = None;
        let mut content_type_opt: OptionString = None;
        let headers: ResponseHeaders = self
            .get_mut_headers()
            .drain()
            .map(|(key, value)| (key.to_lowercase(), value))
            .collect();
        let mut unset_content_length: bool = false;
        for (key, value) in headers.iter() {
            if key == CONTENT_LENGTH {
                continue;
            } else if key == CONTENT_ENCODING {
                compress_type_opt = Some(value.parse::<Compress>().unwrap_or_default());
            } else if key == CONNECTION {
                connection_opt = Some(value.to_owned());
            } else if key == CONTENT_TYPE {
                content_type_opt = Some(value.to_owned());
                if value.eq_ignore_ascii_case(TEXT_EVENT_STREAM) {
                    unset_content_length = true;
                }
            }
            Self::push_header(&mut response_string, key, value);
        }
        if connection_opt.is_none() {
            Self::push_header(&mut response_string, CONNECTION, KEEP_ALIVE);
        }
        if content_type_opt.is_none() {
            let mut content_type: String = String::with_capacity(
                TEXT_HTML.len() + SEMICOLON_SPACE.len() + CHARSET_UTF_8.len(),
            );
            content_type.push_str(TEXT_HTML);
            content_type.push_str(SEMICOLON_SPACE);
            content_type.push_str(CHARSET_UTF_8);
            Self::push_header(&mut response_string, CONTENT_TYPE, &content_type);
        }
        let mut body: Cow<Vec<u8>> = Cow::Borrowed(self.get_body());
        if !unset_content_length {
            if let Some(compress_type) = compress_type_opt {
                if !compress_type.is_unknown() {
                    let tmp_body: Cow<'_, Vec<u8>> =
                        compress_type.encode(&body, DEFAULT_BUFFER_SIZE);
                    body = Cow::Owned(tmp_body.into_owned());
                }
            }
            let len_string: String = body.len().to_string();
            Self::push_header(&mut response_string, CONTENT_LENGTH, &len_string);
        }
        response_string.push_str(HTTP_BR);
        let mut response_bytes: Vec<u8> = response_string.into_bytes();
        response_bytes.extend_from_slice(&body);
        response_bytes
    }

    /// Converts the response to a formatted string representation.
    ///
    /// - Returns: A `String` containing formatted response details.
    pub fn get_string(&self) -> String {
        let body: &Vec<u8> = self.get_body();
        let body_type: &'static str = if std::str::from_utf8(body).is_ok() {
            PLAIN
        } else {
            BINARY
        };
        format!(
            "[Response] => [version]: {}; [status code]: {}; [reason]: {}; [headers]: {:?}; [body]: {} bytes {};",
            self.get_version(),
            self.get_status_code(),
            self.get_reason_phrase(),
            self.get_headers(),
            body.len(),
            body_type
        )
    }
}
