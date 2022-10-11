pub enum Method{
    GET(String),
    HEAD(u32),
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}