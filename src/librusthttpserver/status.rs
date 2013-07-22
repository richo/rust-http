// DO NOT MODIFY THIS FILE DIRECTLY. It is generated by generate_status.py.

use std::num::IntConvertible;

/// HTTP status code
pub enum Status {

    // 1xx Informational
    Continue,
    SwitchingProtocols,
    Processing,  // WebDAV; RFC 2518

    // 2xx Success
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,  // since HTTP/1.1
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,  // WebDAV; RFC 4918
    AlreadyReported,  // WebDAV; RFC 5842
    ImUsed,  // RFC 3229

    // 3xx Redirection
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,  // since HTTP/1.1
    NotModified,
    UseProxy,  // since HTTP/1.1
    SwitchProxy,
    TemporaryRedirect,  // since HTTP/1.1
    PermanentRedirect,  // approved as experimental RFC: http://tools.ietf.org/html/draft-reschke-http-status-308

    // 4xx Client Error
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    RequestEntityTooLarge,
    RequestUriTooLong,
    UnsupportedMediaType,
    RequestedRangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,  // RFC 2324
    AuthenticationTimeout,
    UnprocessableEntity,  // WebDAV; RFC 4918
    Locked,  // WebDAV; RFC 4918
    FailedDependency,  // WebDAV; RFC 4918
    MethodFailure,  // WebDAV
    UnorderedCollection,  // Internet draft
    UpgradeRequired,  // RFC 2817
    PreconditionRequired,  // RFC 6585
    TooManyRequests,  // RFC 6585
    RequestHeaderFieldsTooLarge,  // RFC 6585
    UnavailableForLegalReasons,  // Internet draft

    // 5xx Server Error
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,  // RFC 2295
    InsufficientStorage,  // WebDAV; RFC 4918
    LoopDetected,  // WebDAV; RFC 5842
    NotExtended,  // RFC 2774
    NetworkAuthenticationRequired,  // RFC 6585

    UnregisteredStatus(u16, ~str),
}

impl Status {

    /// Get the status code
    pub fn code(&self) -> u16 {
        match *self {

            // 1xx Informational
            Continue                      => 100,
            SwitchingProtocols            => 101,
            Processing                    => 102,

            // 2xx Success
            Ok                            => 200,
            Created                       => 201,
            Accepted                      => 202,
            NonAuthoritativeInformation   => 203,
            NoContent                     => 204,
            ResetContent                  => 205,
            PartialContent                => 206,
            MultiStatus                   => 207,
            AlreadyReported               => 208,
            ImUsed                        => 226,

            // 3xx Redirection
            MultipleChoices               => 300,
            MovedPermanently              => 301,
            Found                         => 302,
            SeeOther                      => 303,
            NotModified                   => 304,
            UseProxy                      => 305,
            SwitchProxy                   => 306,
            TemporaryRedirect             => 307,
            PermanentRedirect             => 308,

            // 4xx Client Error
            BadRequest                    => 400,
            Unauthorized                  => 401,
            PaymentRequired               => 402,
            Forbidden                     => 403,
            NotFound                      => 404,
            MethodNotAllowed              => 405,
            NotAcceptable                 => 406,
            ProxyAuthenticationRequired   => 407,
            RequestTimeout                => 408,
            Conflict                      => 409,
            Gone                          => 410,
            LengthRequired                => 411,
            PreconditionFailed            => 412,
            RequestEntityTooLarge         => 413,
            RequestUriTooLong             => 414,
            UnsupportedMediaType          => 415,
            RequestedRangeNotSatisfiable  => 416,
            ExpectationFailed             => 417,
            ImATeapot                     => 418,
            AuthenticationTimeout         => 419,
            UnprocessableEntity           => 422,
            Locked                        => 423,
            FailedDependency              => 424,
            MethodFailure                 => 424,
            UnorderedCollection           => 425,
            UpgradeRequired               => 426,
            PreconditionRequired          => 428,
            TooManyRequests               => 429,
            RequestHeaderFieldsTooLarge   => 431,
            UnavailableForLegalReasons    => 451,

            // 5xx Server Error
            InternalServerError           => 500,
            NotImplemented                => 501,
            BadGateway                    => 502,
            ServiceUnavailable            => 503,
            GatewayTimeout                => 504,
            HttpVersionNotSupported       => 505,
            VariantAlsoNegotiates         => 506,
            InsufficientStorage           => 507,
            LoopDetected                  => 508,
            NotExtended                   => 510,
            NetworkAuthenticationRequired => 511,

            UnregisteredStatus(code, _)   => code,
        }
    }

    /// Get the reason phrase
    pub fn reason(&self) -> ~str {
        match *self {

            // 1xx Informational
            Continue                      => ~"Continue",
            SwitchingProtocols            => ~"Switching Protocols",
            Processing                    => ~"Processing",

            // 2xx Success
            Ok                            => ~"OK",
            Created                       => ~"Created",
            Accepted                      => ~"Accepted",
            NonAuthoritativeInformation   => ~"Non-Authoritative Information",
            NoContent                     => ~"No Content",
            ResetContent                  => ~"Reset Content",
            PartialContent                => ~"Partial Content",
            MultiStatus                   => ~"Multi-Status",
            AlreadyReported               => ~"Already Reported",
            ImUsed                        => ~"IM Used",

            // 3xx Redirection
            MultipleChoices               => ~"Multiple Choices",
            MovedPermanently              => ~"Moved Permanently",
            Found                         => ~"Found",
            SeeOther                      => ~"See Other",
            NotModified                   => ~"Not Modified",
            UseProxy                      => ~"Use Proxy",
            SwitchProxy                   => ~"Switch Proxy",
            TemporaryRedirect             => ~"Temporary Redirect",
            PermanentRedirect             => ~"Permanent Redirect",

            // 4xx Client Error
            BadRequest                    => ~"Bad Request",
            Unauthorized                  => ~"Unauthorized",
            PaymentRequired               => ~"Payment Required",
            Forbidden                     => ~"Forbidden",
            NotFound                      => ~"Not Found",
            MethodNotAllowed              => ~"Method Not Allowed",
            NotAcceptable                 => ~"Not Acceptable",
            ProxyAuthenticationRequired   => ~"Proxy Authentication Required",
            RequestTimeout                => ~"Request Timeout",
            Conflict                      => ~"Conflict",
            Gone                          => ~"Gone",
            LengthRequired                => ~"Length Required",
            PreconditionFailed            => ~"Precondition Failed",
            RequestEntityTooLarge         => ~"Request Entity Too Large",
            RequestUriTooLong             => ~"Request-URI Too Long",
            UnsupportedMediaType          => ~"Unsupported Media Type",
            RequestedRangeNotSatisfiable  => ~"Requested Range Not Satisfiable",
            ExpectationFailed             => ~"Expectation Failed",
            ImATeapot                     => ~"I'm a teapot",
            AuthenticationTimeout         => ~"Authentication Timeout",
            UnprocessableEntity           => ~"Unprocessable Entity",
            Locked                        => ~"Locked",
            FailedDependency              => ~"Failed Dependency",
            MethodFailure                 => ~"Method Failure",
            UnorderedCollection           => ~"Unordered Collection",
            UpgradeRequired               => ~"Upgrade Required",
            PreconditionRequired          => ~"Precondition Required",
            TooManyRequests               => ~"Too Many Requests",
            RequestHeaderFieldsTooLarge   => ~"Request Header Fields Too Large",
            UnavailableForLegalReasons    => ~"Unavailable For Legal Reasons",

            // 5xx Server Error
            InternalServerError           => ~"Internal Server Error",
            NotImplemented                => ~"Not Implemented",
            BadGateway                    => ~"Bad Gateway",
            ServiceUnavailable            => ~"Service Unavailable",
            GatewayTimeout                => ~"Gateway Timeout",
            HttpVersionNotSupported       => ~"HTTP Version Not Supported",
            VariantAlsoNegotiates         => ~"Variant Also Negotiates",
            InsufficientStorage           => ~"Insufficient Storage",
            LoopDetected                  => ~"Loop Detected",
            NotExtended                   => ~"Not Extended",
            NetworkAuthenticationRequired => ~"Network Authentication Required",

            UnregisteredStatus(_, ref reason) => (*reason).clone(),
        }
    }
}

impl ToStr for Status {
    /// Produce the HTTP status message incorporating both code and message,
    /// e.g. `ImATeapot.to_str() == "418 I'm a teapot"`
	pub fn to_str(&self) -> ~str {
		fmt!("%? %s", self.code(), self.reason())
	}
}

impl IntConvertible for Status {

    /// Equivalent to `self.code()`
    pub fn to_int(&self) -> int {
        self.code() as int
    }

    /// Get a *registered* status code from the number of its status code.
    ///
    /// This will fail if an unknown code is passed.
    ///
    /// For example, `from_int(200)` will return `OK`.
    pub fn from_int(n: int) -> Status {
        match n {

            // 1xx Informational
            100 => Continue,
            101 => SwitchingProtocols,
            102 => Processing,

            // 2xx Success
            200 => Ok,
            201 => Created,
            202 => Accepted,
            203 => NonAuthoritativeInformation,
            204 => NoContent,
            205 => ResetContent,
            206 => PartialContent,
            207 => MultiStatus,
            208 => AlreadyReported,
            226 => ImUsed,

            // 3xx Redirection
            300 => MultipleChoices,
            301 => MovedPermanently,
            302 => Found,
            303 => SeeOther,
            304 => NotModified,
            305 => UseProxy,
            306 => SwitchProxy,
            307 => TemporaryRedirect,
            308 => PermanentRedirect,

            // 4xx Client Error
            400 => BadRequest,
            401 => Unauthorized,
            402 => PaymentRequired,
            403 => Forbidden,
            404 => NotFound,
            405 => MethodNotAllowed,
            406 => NotAcceptable,
            407 => ProxyAuthenticationRequired,
            408 => RequestTimeout,
            409 => Conflict,
            410 => Gone,
            411 => LengthRequired,
            412 => PreconditionFailed,
            413 => RequestEntityTooLarge,
            414 => RequestUriTooLong,
            415 => UnsupportedMediaType,
            416 => RequestedRangeNotSatisfiable,
            417 => ExpectationFailed,
            418 => ImATeapot,
            419 => AuthenticationTimeout,
            422 => UnprocessableEntity,
            423 => Locked,
            424 => FailedDependency,
            425 => UnorderedCollection,
            426 => UpgradeRequired,
            428 => PreconditionRequired,
            429 => TooManyRequests,
            431 => RequestHeaderFieldsTooLarge,
            451 => UnavailableForLegalReasons,

            // 5xx Server Error
            500 => InternalServerError,
            501 => NotImplemented,
            502 => BadGateway,
            503 => ServiceUnavailable,
            504 => GatewayTimeout,
            505 => HttpVersionNotSupported,
            506 => VariantAlsoNegotiates,
            507 => InsufficientStorage,
            508 => LoopDetected,
            510 => NotExtended,
            511 => NetworkAuthenticationRequired,

            _   => { fail!(fmt!("No registered HTTP status code %d", n)); }
        }
    }
}