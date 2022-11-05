# scratchstack-wrapper-aws-crt
Rust wrappers for [AWS Common Runtime (CRT) libraries](https://docs.aws.amazon.com/sdkref/latest/guide/common-runtime.html).

Currently, this only provided the minimum set of bindings necessary for Scratchstack to verify interoperability
with AWS CRT authentication libraries.  This is not intended to be a complete set of bindings for the CRT. You
probably *do not want to use these bindings* in your own projects. If you need to communicate with AWS services,
use the [official AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust) instead.

Documentation: https://docs.rs/scratchstack-wrapper-aws-crt
