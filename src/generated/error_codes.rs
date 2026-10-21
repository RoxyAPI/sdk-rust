string_enum! {
    /// A stable API error code. Codes this SDK version does not list arrive as `Other`.
    ErrorCode {
        ///
        /// HTTP 400.
        ///
        /// A field is missing or invalid. The body lists every failing field at once in issues, each with its path and the expected shape.
        ValidationError = "validation_error",
        ///
        /// HTTP 400.
        ///
        /// The request was rejected as a whole, for example a body that is not valid JSON or a date range that ends before it starts.
        BadRequest = "bad_request",
        ///
        /// HTTP 400.
        ///
        /// A date lies outside the span the planetary positions are served for. The error names the supported dates.
        DateOutOfRange = "date_out_of_range",
        ///
        /// HTTP 401.
        ///
        /// No API key was sent.
        ApiKeyRequired = "api_key_required",
        ///
        /// HTTP 401.
        ///
        /// The key sent is malformed or cannot be verified.
        InvalidApiKey = "invalid_api_key",
        ///
        /// HTTP 401.
        ///
        /// The key was valid once and has since been revoked from the account page.
        ApiKeyRevoked = "api_key_revoked",
        ///
        /// HTTP 401.
        ///
        /// The key verifies, but the subscription behind it no longer exists.
        SubscriptionNotFound = "subscription_not_found",
        ///
        /// HTTP 401.
        ///
        /// The subscription behind the key is cancelled, expired or suspended, or a trial is past its end date.
        SubscriptionInactive = "subscription_inactive",
        ///
        /// HTTP 401.
        ///
        /// A publishable key was used on a Remote MCP server, which takes a secret key.
        PublishableKeyNotAllowedOnMcp = "publishable_key_not_allowed_on_mcp",
        ///
        /// HTTP 401.
        ///
        /// A keyless request could not be attributed to a caller.
        InvalidClientIp = "invalid_client_ip",
        ///
        /// HTTP 401.
        ///
        /// Authentication failed with no more specific reason to give.
        Unauthorized = "unauthorized",
        ///
        /// HTTP 403.
        ///
        /// The key is valid, but this request is not permitted.
        Forbidden = "forbidden",
        ///
        /// HTTP 403.
        ///
        /// A publishable key with an origin allowlist was used with no Origin header, as from a server or a native app.
        OriginRequired = "origin_required",
        ///
        /// HTTP 403.
        ///
        /// The calling site is not on the origin allowlist of this publishable key.
        OriginNotAllowed = "origin_not_allowed",
        ///
        /// HTTP 404.
        ///
        /// Nothing is served at this path, or the identifier in it does not exist. A path miss adds suggestions, the closest real endpoints.
        NotFound = "not_found",
        ///
        /// HTTP 405.
        ///
        /// The path exists for a different HTTP method. The body lists the accepted methods in allow, as does the Allow header.
        MethodNotAllowed = "method_not_allowed",
        ///
        /// HTTP 409.
        ///
        /// The request cannot be applied in the current state.
        Conflict = "conflict",
        ///
        /// HTTP 410.
        ///
        /// The path was removed permanently.
        Gone = "gone",
        ///
        /// HTTP 413.
        ///
        /// The request body is larger than the API accepts. A body far over the cap is refused before it reaches the API and returns a bare 413 with no JSON body.
        PayloadTooLarge = "payload_too_large",
        ///
        /// HTTP 415.
        ///
        /// A request with a body was sent without Content-Type: application/json.
        UnsupportedMediaType = "unsupported_media_type",
        ///
        /// HTTP 422.
        ///
        /// Every field is valid, but the combination cannot be computed.
        UnprocessableEntity = "unprocessable_entity",
        ///
        /// HTTP 429.
        ///
        /// The monthly request allowance is used up. Retry-After gives the seconds to the reset.
        RateLimitExceeded = "rate_limit_exceeded",
        ///
        /// HTTP 429.
        ///
        /// The per-minute limit of the plan was reached. Retry-After gives the seconds to the next minute, and the monthly allowance is not charged.
        RateLimitPerMinute = "rate_limit_per_minute",
        ///
        /// HTTP 429.
        ///
        /// A rate limit was reached, reported without the plan detail.
        RateLimited = "rate_limited",
        ///
        /// HTTP 429.
        ///
        /// A keyless request has used up its trial allowance.
        FreeTierExhausted = "free_tier_exhausted",
        ///
        /// HTTP 500.
        ///
        /// The request failed inside our service. Retry with exponential backoff.
        InternalError = "internal_error",
        ///
        /// HTTP 503.
        ///
        /// Heavy calculations are briefly at capacity. Send the same request again after Retry-After seconds.
        ComputeSaturated = "compute_saturated",
        ///
        /// Any HTTP status.
        ///
        /// A fallback for a status with no more specific code. The status and the error field carry the signal.
        Error = "error",
    }
}
impl ErrorCode {
    /// The HTTP status the API documents for this code, when listed.
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::ValidationError => Some(400),
            Self::BadRequest => Some(400),
            Self::DateOutOfRange => Some(400),
            Self::ApiKeyRequired => Some(401),
            Self::InvalidApiKey => Some(401),
            Self::ApiKeyRevoked => Some(401),
            Self::SubscriptionNotFound => Some(401),
            Self::SubscriptionInactive => Some(401),
            Self::PublishableKeyNotAllowedOnMcp => Some(401),
            Self::InvalidClientIp => Some(401),
            Self::Unauthorized => Some(401),
            Self::Forbidden => Some(403),
            Self::OriginRequired => Some(403),
            Self::OriginNotAllowed => Some(403),
            Self::NotFound => Some(404),
            Self::MethodNotAllowed => Some(405),
            Self::Conflict => Some(409),
            Self::Gone => Some(410),
            Self::PayloadTooLarge => Some(413),
            Self::UnsupportedMediaType => Some(415),
            Self::UnprocessableEntity => Some(422),
            Self::RateLimitExceeded => Some(429),
            Self::RateLimitPerMinute => Some(429),
            Self::RateLimited => Some(429),
            Self::FreeTierExhausted => Some(429),
            Self::InternalError => Some(500),
            Self::ComputeSaturated => Some(503),
            Self::Error => None,
            Self::Other(_) => None,
        }
    }
}
