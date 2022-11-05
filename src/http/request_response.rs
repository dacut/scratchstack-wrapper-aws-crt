use {
    crate::{
        common::{AwsCAllocator, AwsCByteCursor, AwsCAtomicVar, AwsCArrayList, AwsCString},
        http::{AwsCHttpConnection, AwsCHttpVersion, AwsCHttpMethod},
        io::AwsCInputStream,
    },
    std::{
        ffi::c_void,
        mem::ManuallyDrop,
    },
};

#[repr(C)]
struct AwsCHttpStreamVtable {
    destroy: *const extern "C" fn(stream: *mut AwsCHttpStream),
    update_window: *const extern "C" fn(stream: *mut AwsCHttpStream, increment_size: usize),
    activate: *const extern "C" fn(stream: *mut AwsCHttpStream) -> i32,
    http1_write_chunk: *const extern "C" fn(http1_stream: *mut AwsCHttpStream, options: *const AwsCHttp1ChunkOptions) -> i32,
    http1_add_trailer: *const extern "C" fn(http1_stream: *mut AwsCHttpStream, trailing_headers: *const AwsCHttpHeaders) -> i32,
    http2_reset_stream: *const extern "C" fn(http2_stream: *mut AwsCHttpStream, http2_error: u32) -> i32,
    http2_get_received_error_code: *const extern "C" fn(http2_stream: *const AwsCHttpStream, http2_error: *mut u32) -> i32,
    http2_get_sent_error_code: *const extern "C" fn(http2_stream: *const AwsCHttpStream, http2_error: *mut u32) -> i32,
    http2_write_data: *const extern "C" fn(http2_stream: *mut AwsCHttpStream, options: *const AwsCHttp2StreamWriteDataOptions) -> i32,
}

#[repr(C)]
pub struct AwsCHttpStream {
    // Private implementation
    vtable: *const AwsCHttpStreamVtable,
    alloc: *const AwsCAllocator,
    owning_connection: *mut AwsCHttpConnection,
    id: u32,
    user_data: *mut c_void,
    on_incoming_headers: *const AwsCHttpOnIncomingHeadersFn,
    on_incoming_header_block_done: *const AwsCHttpOnIncomingHeaderBlockDoneFn,
    on_incoming_body: *const AwsCHttpOnIncomingBodyFn,
    on_complete: *const AwsCHttpOnStreamCompleteFn,
    on_destroy: *const AwsCHttpOnStreamDestroyFn,
    refcount: AwsCAtomicVar,
    request_method: AwsCHttpMethod,
    client_or_server_data: CClientOrServerData,
    client_data: *mut AwsCHttpStreamClientData,
    server_data: *mut AwsCHttpStreamServerData,
}

#[repr(C)]
union CClientOrServerData {
    client: ManuallyDrop<AwsCHttpStreamClientData>,
    server: ManuallyDrop<AwsCHttpStreamServerData>,
}

#[repr(C)]
struct AwsCHttpStreamClientData {
    response_status: i32
}

#[repr(C)]
struct AwsCHttpStreamServerData {
    request_method_str: AwsCByteCursor,
    request_path: AwsCByteCursor,
    on_request_done: *const AwsCHttpOnIncomingRequestDoneFn,
}


#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCHttpHeaderCompression {
    AWS_HTTP_HEADER_COMPRESSION_USE_CACHE,
    AWS_HTTP_HEADER_COMPRESSION_NO_CACHE,
    AWS_HTTP_HEADER_COMPRESSION_NO_FORWARD_CACHE,
}

#[repr(C)]
pub struct AwsCHttpHeader {
    name: AwsCByteCursor,
    value: AwsCByteCursor,
    compression: AwsCHttpHeaderCompression,
}

#[repr(C)]
pub struct AwsCHttpHeaders {
    // Private implementation
    alloc: *const AwsCAllocator,
    array_list: AwsCArrayList,
    refcount: AwsCAtomicVar,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCHttpHeaderBlock {
    AWS_HTTP_HEADER_BLOCK_MAIN,
    AWS_HTTP_HEADER_BLOCK_INFORMATIONAL,
    AWS_HTTP_HEADER_BLOCK_TRAILING,
}

#[repr(C)]
pub struct AwsCHttpMessage {
    // Private implementation
    allocator: *const AwsCAllocator,
    headers: *mut AwsCHttpHeaders,
    body_stream: *mut AwsCInputStream,
    refcount: AwsCAtomicVar,
    http_version: AwsCHttpVersion,
    subclass_data: AwsCHttpMessageSubclassData,
    request_data: *mut AwsCHttpMessageRequestData,
    response_data: *mut AwsCHttpMessageResponseData,
}

#[repr(C)]
union AwsCHttpMessageSubclassData {
    request: ManuallyDrop<AwsCHttpMessageRequestData>,
    response: ManuallyDrop<AwsCHttpMessageResponseData>,
}

#[repr(C)]
struct AwsCHttpMessageRequestData {
    method: *mut AwsCString,
    path: *mut AwsCString,
}

#[repr(C)]
struct AwsCHttpMessageResponseData {
    status: i32,
}

pub type AwsCHttpMessageTransformCompleteFn =
    extern "C" fn(message: *mut AwsCHttpMessage, error_code: i32, complete_ctx: *mut c_void);

pub type AwsCHttpMessageTransformFn = extern "C" fn(
    message: *mut AwsCHttpMessage,
    user_data: *mut c_void,
    complete_fn: *const AwsCHttpMessageTransformCompleteFn,
    complete_ctx: *mut c_void,
);

pub type AwsCHttpOnIncomingHeadersFn = extern "C" fn(
    stream: *mut AwsCHttpStream,
    header_block: AwsCHttpHeaderBlock,
    header_array: *const AwsCHttpHeader,
    num_headers: usize,
    user_data: *mut c_void,
) -> i32;

pub type AwsCHttpOnIncomingHeaderBlockDoneFn =
    extern "C" fn(stream: *mut AwsCHttpStream, header_block: AwsCHttpHeaderBlock, user_data: *mut c_void) -> i32;

pub type AwsCHttpOnIncomingBodyFn =
    extern "C" fn(stream: *mut AwsCHttpStream, data: *const AwsCByteCursor, user_data: *mut c_void) -> i32;

pub type AwsCHttpOnIncomingRequestDoneFn = extern "C" fn(stream: *mut AwsCHttpStream, user_data: *mut c_void) -> i32;

pub type AwsCHttpOnStreamCompleteFn =
    extern "C" fn(stream: *mut AwsCHttpStream, error_code: i32, user_data: *mut c_void);

pub type AwsCHttpOnStreamDestroyFn = extern "C" fn(user_data: *mut c_void);

#[repr(C)]
pub struct AwsCHttpMakeRequestOptions {
    pub self_size: usize,
    pub request: *mut AwsCHttpMessage,
    pub user_data: *mut c_void,
    pub on_reponse_headers: *const AwsCHttpOnIncomingHeadersFn,
    pub on_response_header_block_done: *const AwsCHttpOnIncomingHeaderBlockDoneFn,
    pub on_response_body: *const AwsCHttpOnIncomingBodyFn,
    pub on_complete: *const AwsCHttpOnStreamCompleteFn,
    pub on_destroy: *const AwsCHttpOnStreamDestroyFn,
    pub http2_use_manual_data_writes: bool,
}

#[repr(C)]
pub struct AwsCHttpRequestHandlerOptions {
    pub self_size: usize,
    pub server_connection: *mut AwsCHttpConnection,
    pub user_data: *mut c_void,
    pub on_request_headers: *const AwsCHttpOnIncomingHeadersFn,
    pub on_request_header_block_done: *const AwsCHttpOnIncomingHeaderBlockDoneFn,
    pub on_request_body: *const AwsCHttpOnIncomingBodyFn,
    pub on_request_done: *const AwsCHttpOnIncomingRequestDoneFn,
    pub on_complete: *const AwsCHttpOnStreamCompleteFn,
    pub on_destroy: *const AwsCHttpOnStreamDestroyFn,
}

pub type AwsCHttpStreamWriteCompleteFn =
    extern "C" fn(stream: *mut AwsCHttpStream, error_code: i32, user_data: *mut c_void);

pub type AwsCHttp1StreamWriteChunkCompleteFn = AwsCHttpStreamWriteCompleteFn;

#[repr(C)]
pub struct AwsCHttp1ChunkExtension {
    pub key: AwsCByteCursor,
    pub value: AwsCByteCursor,
}

#[repr(C)]
pub struct AwsCHttp1ChunkOptions {
    pub chunk_data: *mut AwsCInputStream,
    pub chunk_data_size: u64,
    pub extensions: *mut AwsCHttp1ChunkExtension,
    pub num_extensions: usize,
    pub on_complete: *const AwsCHttp1StreamWriteChunkCompleteFn,
    pub user_data: *mut c_void,
}

pub type AwsCHttp2StreamWriteDataCompleteFn = AwsCHttpStreamWriteCompleteFn;

#[repr(C)]
pub struct AwsCHttp2StreamWriteDataOptions {
    pub data: *mut AwsCInputStream,
    pub end_stream: bool,
    pub on_complete: *const AwsCHttp2StreamWriteDataCompleteFn,
    pub user_data: *mut c_void,
}

#[link(name = "aws-c-http")]
extern "C" {
    pub fn aws_http_header_name_eq(name_a: AwsCByteCursor, name_b: AwsCByteCursor) -> bool;
    pub fn aws_http_headers_new(allocator: *mut AwsCAllocator) -> *mut AwsCHttpHeaders;
    pub fn aws_http_headers_acquire(headers: *mut AwsCHttpHeaders);
    pub fn aws_http_headers_release(headers: *mut AwsCHttpHeaders);

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_headers_add_header(headers: *mut AwsCHttpHeaders, header: *const AwsCHttpHeader) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_headers_set(headers: *mut AwsCHttpHeaders, name: AwsCByteCursor, value: AwsCByteCursor) -> i32;

    pub fn aws_http_headers_count(headers: *const AwsCHttpHeaders) -> usize;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_headers_get_index(
        headers: *const AwsCHttpHeaders,
        index: usize,
        out_header: *mut AwsCHttpHeader,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_headers_get(
        headers: *const AwsCHttpHeaders,
        name: AwsCByteCursor,
        out_value: *mut AwsCByteCursor,
    ) -> i32;

    pub fn aws_http_headers_has(headers: *const AwsCHttpHeaders, name: AwsCByteCursor) -> bool;

    pub fn aws_http_headers_erase(headers: *mut AwsCHttpHeaders, name: AwsCByteCursor) -> i32;

    pub fn aws_http_headers_erase_value(
        headers: *mut AwsCHttpHeaders,
        name: AwsCByteCursor,
        value: AwsCByteCursor,
    ) -> i32;

    pub fn aws_http_headers_erase_index(headers: *mut AwsCHttpHeaders, index: usize) -> i32;

    pub fn aws_http_headers_clear(headers: *mut AwsCHttpHeaders);

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http2_headers_get_request_method(
        h2_headers: *const AwsCHttpHeaders,
        out_method: *mut AwsCByteCursor,
    ) -> i32;

    pub fn aws_http2_headers_set_request_method(h2_headers: *const AwsCHttpHeaders, method: AwsCByteCursor) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http2_headers_get_request_scheme(
        h2_headers: *const AwsCHttpHeaders,
        out_scheme: *mut AwsCByteCursor,
    ) -> i32;

    pub fn aws_http2_headers_set_request_scheme(h2_headers: *const AwsCHttpHeaders, scheme: AwsCByteCursor) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http2_headers_get_request_authority(
        h2_headers: *const AwsCHttpHeaders,
        out_authority: *mut AwsCByteCursor,
    ) -> i32;

    pub fn aws_http2_headers_set_request_authority(
        h2_headers: *const AwsCHttpHeaders,
        authority: AwsCByteCursor,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http2_headers_get_request_path(h2_headers: *const AwsCHttpHeaders, out_path: *mut AwsCByteCursor)
        -> i32;

    pub fn aws_http2_headers_set_request_path(h2_headers: *const AwsCHttpHeaders, path: AwsCByteCursor) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http2_headers_get_response_status(h2_headers: *const AwsCHttpHeaders, out_status: *mut i32) -> i32;

    pub fn aws_http2_headers_set_response_status(h2_headers: *const AwsCHttpHeaders, status: i32) -> i32;

    pub fn aws_http_message_new_request(allocator: *const AwsCAllocator) -> *mut AwsCHttpMessage;

    pub fn aws_http_message_new_request_with_headers(
        allocator: *const AwsCAllocator,
        existing_headers: *mut AwsCHttpHeaders,
    ) -> *mut AwsCHttpMessage;

    pub fn aws_http_message_new_response(allocator: *const AwsCAllocator) -> *mut AwsCHttpMessage;

    pub fn aws_http2_message_new_request(allocator: *const AwsCAllocator) -> *mut AwsCHttpMessage;

    pub fn aws_http2_message_new_response(allocator: *const AwsCAllocator) -> *mut AwsCHttpMessage;

    pub fn aws_http2_message_new_from_http1(
        allocator: *const AwsCAllocator,
        http1_msg: *mut AwsCHttpMessage,
    ) -> *mut AwsCHttpMessage;

    pub fn aws_http_message_acquire(message: *mut AwsCHttpMessage) -> *mut AwsCHttpMessage;

    pub fn aws_http_message_release(message: *mut AwsCHttpMessage) -> *mut AwsCHttpMessage;

    #[deprecated(since = "0.1.0", note = "use aws_http_message_release instead")]
    pub fn aws_http_message_destroy(message: *mut AwsCHttpMessage);

    pub fn aws_http_message_is_request(message: *const AwsCHttpMessage) -> bool;

    pub fn aws_http_message_is_response(message: *const AwsCHttpMessage) -> bool;

    pub fn aws_http_message_get_protocol_version(message: *const AwsCHttpMessage) -> AwsCHttpVersion;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_message_get_request_method(
        request_message: *const AwsCHttpMessage,
        out_method: *mut AwsCByteCursor,
    ) -> i32;

    pub fn aws_http_message_set_request_method(request_message: *mut AwsCHttpMessage, method: AwsCByteCursor) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_message_get_request_path(
        request_message: *const AwsCHttpMessage,
        out_path: *mut AwsCByteCursor,
    ) -> i32;

    pub fn aws_http_message_set_request_path(request_message: *mut AwsCHttpMessage, path: AwsCByteCursor) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_message_get_response_status(response_message: *const AwsCHttpMessage, out_status: *mut i32) -> i32;

    pub fn aws_http_message_set_response_status(response_message: *mut AwsCHttpMessage, status: i32) -> i32;

    pub fn aws_http_message_get_body_stream(message: *const AwsCHttpMessage) -> *mut AwsCInputStream;

    pub fn aws_http_message_set_body_stream(message: *mut AwsCHttpMessage, body: *mut AwsCInputStream);

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http1_stream_write_chunk(
        http1_stream: *mut AwsCHttpStream,
        options: *const AwsCHttp1ChunkOptions,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http2_stream_write_data(
        http2_stream: *mut AwsCHttpStream,
        options: *const AwsCHttp2StreamWriteDataOptions,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http1_stream_add_chunked_trailer(
        http1_stream: *mut AwsCHttpStream,
        trailing_headers: *const AwsCHttpHeaders,
    ) -> i32;

    pub fn aws_http_message_get_headers(message: *const AwsCHttpMessage) -> *mut AwsCHttpHeaders;

    pub fn aws_http_message_get_const_headers(message: *const AwsCHttpMessage) -> *const AwsCHttpHeaders;

    pub fn aws_http_message_get_header_count(message: *const AwsCHttpMessage) -> usize;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_message_get_header(
        message: *const AwsCHttpMessage,
        out_header: *mut AwsCHttpHeader,
        index: usize,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_message_add_header(message: *mut AwsCHttpMessage, header: AwsCHttpHeader) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_message_add_header_array(
        message: *mut AwsCHttpMessage,
        headers: *const AwsCHttpHeader,
        num_headers: usize,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_message_erase_header(message: *mut AwsCHttpMessage, index: usize) -> i32;

    pub fn aws_http_connection_make_request(
        client_connection: *mut AwsCHttpConnection,
        options: AwsCHttpMakeRequestOptions,
    ) -> *mut AwsCHttpStream;

    pub fn aws_http_stream_new_server_request_handler(
        options: *const AwsCHttpRequestHandlerOptions,
    ) -> *mut AwsCHttpStream;

    pub fn aws_http_stream_release(stream: *mut AwsCHttpStream);

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_stream_activate(stream: *mut AwsCHttpStream) -> i32;

    pub fn aws_http_stream_get_connection(stream: *const AwsCHttpStream) -> *mut AwsCHttpConnection;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_stream_get_incoming_response_status(stream: *const AwsCHttpStream, out_status: *mut i32) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_stream_get_incoming_request_method(
        stream: *const AwsCHttpStream,
        out_method: *mut AwsCByteCursor,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_stream_get_incoming_request_uri(stream: *const AwsCHttpStream, out_uri: *mut AwsCByteCursor)
        -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http_stream_send_response(stream: *mut AwsCHttpStream, response: *mut AwsCHttpMessage) -> i32;

    pub fn aws_http_stream_update_window(stream: *mut AwsCHttpStream, increment_size: usize);

    pub fn aws_http_stream_get_id(stream: *const AwsCHttpStream) -> u32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http2_stream_reset(http2_stream: *mut AwsCHttpStream, http2_error: u32) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http2_stream_get_received_reset_error_code(
        http2_stream: *const AwsCHttpStream,
        out_http2_error: *mut u32,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_http2_stream_get_sent_reset_error_code(
        http2_stream: *const AwsCHttpStream,
        out_http2_error: *mut u32,
    ) -> i32;
}
