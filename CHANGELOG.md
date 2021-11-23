# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2021-11-23

[0.1.2]: ../../../compare/1cba7adf28d4fbe1cceba5f9724bd75377283b92...cde5937fd245c908a7993711933c3d45001fcd77

### Bug Fixes

- *opencl-api:* Add error code description for every api function [#7] ([4febe34](4febe345a3b2923412d797d08ebb9a6d1570b937))
- *opencl-api:* Impl `From` trait to convert `Status` and `StatusCode` ([cde5937](cde5937fd245c908a7993711933c3d45001fcd77))

### Improvements

- *opencl-api:* Improve status code errors to be more descriptive ([9ebe1f6](9ebe1f6a692e4bf89c82d23924d4ddb545444e82))
- *opencl-api:* Update error names and description to be more specific ([5b5239f](5b5239f3d35c730a3e3845ae76742b36fd4d85a0))

<!-- CHANGELOG SPLIT MARKER -->
## [0.1.1] - 2021-11-21

[0.1.1]: ../../../compare/aaba02614cd435ab668e6961a3b53fa3ee83ab3a...1cba7adf28d4fbe1cceba5f9724bd75377283b92

### Bug Fixes

- *opencl-api:* Fix `CL_INVALID_VALUE` error at get_count [#6] ([35ed6dc](35ed6dcde43873ef146572aadaf7425a4c0a02d0))
- *opencl-api:* Fix access to multiple bitfield combinations [#10] ([590bd6d](590bd6d8326a4f0173cc645da8524ced5015d2fe))

### Documentation

- *opencl-rs:* Update license.html ([2d7f2b0](2d7f2b0dd487359f9d784f9c95914af7a0b94a1c))

### Improvements

- *opencl-api:* Improve error handling with thiserror derivations ([bdf3a66](bdf3a66a2f66f0b450236ae80c5b6ce0f407da42))
- *opencl-api:* Generalize property list gneration for every api [#8] ([fb5fed5](fb5fed5d8d783793ee5f1af7db76cd3b3806349a))

### Refactoring Updates

- *opencl-api:* Classify data objects to separate files for brevity ([8bb0e27](8bb0e27bab0c96f234251e62a5d8f947dfbb3c24))

### Test Checks

- *opencl-api:* Fix test case `CommandQueueProperties` null pointer ([bf947b6](bf947b6e142ba731cfc04370d2f32b04465db02c))
- *opencl-rs:* Fix test cases for compatiblity with ci builds ([b1f309f](b1f309f9b7f6f48c547bc37df836e7051a0332be))

<!-- CHANGELOG SPLIT MARKER -->
## [0.1.0] - 2021-11-18

[0.1.0]: ../../../releases/tag/v0.1.0

### Bug Fixes

- *opencl-api:* Fix api bindings from ffis and start project clinfo ([4cd80f3](4cd80f3b88be604c52670147256785943c6bbb7a))
- *opencl-api:* Fix optional in pfn_notify with few more changes ([9f1d353](9f1d353ddceba8144882a70a121678f2ee93b998))
- *opencl-heads:* Support mem_object for now, add buffer later ([9b899d9](9b899d9b2b0d112e9222b6b05b8bc35352d44050))
- *opencl-rs:* Update repo links and n3nx project meta tags ([f699d68](f699d685c15485440ad1c2f47f2fb002f454abe8))

### Build Tools

- *helpers:* Add build tools and progress api development further ([30cdcd2](30cdcd27cc3fc41315a949aa8eb5ae188ca07155))

### Documentation

- *opencl-rs:* Add readme.md and update search keywords in cargo.toml ([d03bb68](d03bb68cbac0deb11ce1f92884edb4c9deb8e51e))
- *opencl-rs:* Add discord links, badge color in readme.md ([c3e2948](c3e2948ecd0ea878fe059a40b0bf7e24b66391d3))

### Extra Tasks

- *opencl-heads:* Add opencl header dependencies ([e35e2b4](e35e2b4498f0b822beca9bcc10dd4b2912612172))

### Features

- *cliff:* Add N3N cliff config file for changelog generation ([28abcbe](28abcbefd5b40fa262c829762ab938447e129182))
- *memory:* Create memory module with its apis ([f8dcc26](f8dcc26af540e7971f2d73c68ebea06312f1bc79))
- *opencl-api:* Add enums to match constants and respective datatypes ([e3a8561](e3a8561a9380fe6e85bd8d7ae85b9f9a807a637b))
- *opencl-api:* Add status enums with necessary traits ([a43206d](a43206dd715d691e35e0d66677fcc3509abcb1dd))
- *opencl-api:* Add platform info api with reorder of few helpers ([843ae23](843ae237a6392ec0134220d195f4379b47a8792e))
- *opencl-api:* Create APIs for command queues ([64d1439](64d14396e6c6d7f16f2a6a8840dc5079b087db96))
- *opencl-api:* Add device.rs file with including its APIs ([9ae600a](9ae600aa516d8547c009e522a58fcbdce794ad5e))
- *opencl-api:* Add `clGetDeviceInfo` interface with more datatypes ([3dde3f1](3dde3f114a21540439bee5192ffb117f7557af95))
- *opencl-api:* Add more apis in device, update struct for device types ([38cf143](38cf1434ed432102bc02c5aed139cbf8bf57e049))
- *opencl-api:* Add apis in context, update queue api, few minor fixes ([d242d4b](d242d4bc7df49af6dee3db119e6c1726bf9209f0))
- *opencl-api:* Add buffer apis and add wrapped pointers support ([586e37c](586e37cb634579488cf733dfb61e1585d46c5111))
- *opencl-api:* Add image module with its apis ([13f1e6e](13f1e6e92eaa798bf148a44e5a15de9c9a86388d))
- *opencl-api:* Add image apis and fix few bugs along the way ([f706446](f706446835ca0238b02ce7d4c2af10daaa653b50))
- *opencl-api:* Add pipe module with apis ([b418a35](b418a354f290cb36630722b11f9b28b6abb51c0d))
- *opencl-api:* Add sampler apis with descriptive comments ([ee67483](ee67483f12f9294d1a4d3b8e42e78b042b692458))
- *opencl-heads:* Map core typedefs and constants from headers ([8f9b71f](8f9b71fa7aa81d1702ef59fcb708be5c25145533))
- *opencl-heads:* Map api functions from headers ([f2986a9](f2986a978aca78f901ee2014db3f287c9c12b85b))

### Improvements

- *opencl-api:* Optimize struct types to represent bitfields ([8a42643](8a42643e69205de2a45aaa4b7f1c37d84c7c0753))
- *opencl-api:* Improve error handling and reordering of bitfields ([5a28b32](5a28b32924fa282e04c2b1326307baeb9aba845a))
- *opencl-api:* Ffi type independency from libc ([eab263c](eab263cf517ce445d9be77282cd768a253cebb70))
- *opencl-api:* Add support for macronized objects function generators ([633dda3](633dda3a56fef402baa644999cac08181c67b2b5))
- *opencl-api:* Apply adding operation for bitfields with more changes ([9e3137b](9e3137b8f22146275c2bff2add639403d437544a))
- *opencl-api:* Macronize counting objects and object list generator ([673c259](673c2592892e69625717b05ddc81bc78e4f218a5))
- *opencl-api:* Create structures to wrap pointers for security ([138632e](138632eab27fee095bfc82c97dba76315062e9a7))
- *opencl-heads:* Update cl_image_desc structure to separate versions ([715c71c](715c71ce553b9076589c7c69dc1eb913e723234c))

### Refactoring Updates

- *helpers:* Update helper functions to map directory ([47250a5](47250a596b5ca34fc652c9dabeb0fe1720344af5))
- *opencl-api:* Update error handling to support function names ([dbfd9bf](dbfd9bf56172c18e7bdd1214117ce83dfe73b10d))
- *opencl-heads:* Refactor constants to match their respective types ([6c53eb8](6c53eb882266b52326610ba1fd8591c259c62cdb))
- *opencl-heads:* Add and refactor constants to their types ([40f76d8](40f76d840e7e0ff703ae8b395b7efcb48b461940))
- *opencl-heads:* Move ffi to separate module, add platforms support ([f555d7d](f555d7d7dceca81a75c143222eb9d2c814b6f091))
- *opencl-heads:* Create section for cl ffi bindings to remain compatible ([178fc8d](178fc8d99673eef8cdf1d2a4a043050e6981934a))

### Styling

- *api:* Format api image, memory and pipe files and add docker tests ([8d57ea5](8d57ea561677e27f032152ac56318425da3a8121))

<!-- CHANGELOG SPLIT MARKER -->
