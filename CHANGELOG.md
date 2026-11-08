# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2026-11-08

### Added

- [`9636fdd`](https://github.com/shapeshed/wenmoon/commit/9636fdd85ef933fe4f484f3c287dbc349d96f102) Add CODEOWNERS
- [`300ef87`](https://github.com/shapeshed/wenmoon/commit/300ef8773a2b8e220c59476321198a80f1bb2b3b) Add contributor and security documentation

### Changed

- [`bed2f5c`](https://github.com/shapeshed/wenmoon/commit/bed2f5caf5438649b4702f492819b5c32837e740) Update CI (#144)
- [`4aa9690`](https://github.com/shapeshed/wenmoon/commit/4aa9690b475b2e0392b01f3f651c736a98e38dc0) Update renovate.json5
- [`3217a4f`](https://github.com/shapeshed/wenmoon/commit/3217a4f81af55bfa12e11d342e4c2cad6c6537d2) Enable automerge for patch updates (#225)
- [`df1e14a`](https://github.com/shapeshed/wenmoon/commit/df1e14a2a1149cb0a0ef6a454452d019e2f09c9d) Harden runtime, config, and provider handling
- [`65f718c`](https://github.com/shapeshed/wenmoon/commit/65f718cf0b15e97bdf56a3da129098fbd680855a) Update vulnerable transitive dependencies
- [`ccbd048`](https://github.com/shapeshed/wenmoon/commit/ccbd048f3e91167e69439a6b89626cbf0b94dc2e) Harden CI and release workflows
- [`56f066e`](https://github.com/shapeshed/wenmoon/commit/56f066e1a87a6e572e565c3f37a671f4ffbe2989) Enable Renovate vulnerability alerts and action pinning
- [`069b907`](https://github.com/shapeshed/wenmoon/commit/069b907088632bc6298fad5c08d9b63b3db3ee1e) Improve changelog grouping and release checks
- [`ec138c9`](https://github.com/shapeshed/wenmoon/commit/ec138c9a8a9d29b1ebc5eb3bccaf71b596bca8af) Automate crates.io publishing from the release workflow

### Dependencies

- [`14cfda8`](https://github.com/shapeshed/wenmoon/commit/14cfda80ee128c6aefe992b40b7840c59338be0c) Bump clap from 4.5.30 to 4.5.31 (#112)
- [`87eb601`](https://github.com/shapeshed/wenmoon/commit/87eb601aedb94385df1cad841ca385c3496ab9fb) Bump serde from 1.0.218 to 1.0.219 (#114)
- [`e3135ab`](https://github.com/shapeshed/wenmoon/commit/e3135ab349fa776c55a5496f65bc4649fcfa9c18) Bump async-trait from 0.1.86 to 0.1.87 (#113)
- [`aaad670`](https://github.com/shapeshed/wenmoon/commit/aaad6709f37aab9516f6f070c20f38e647f0a2cd) Bump serde_json from 1.0.139 to 1.0.140 (#115)
- [`0ff2a1c`](https://github.com/shapeshed/wenmoon/commit/0ff2a1c256a17e8a6cd142793190be641a046017) Bump tokio from 1.43.0 to 1.44.0 (#116)
- [`3aa17c2`](https://github.com/shapeshed/wenmoon/commit/3aa17c2a95343369c227bffa4851dccb26004487) Bump async-trait from 0.1.87 to 0.1.88 (#120)
- [`d6375dc`](https://github.com/shapeshed/wenmoon/commit/d6375dc0f3bfd2d6c525fd6411a936bd40b41130) Bump reqwest from 0.12.12 to 0.12.15 (#121)
- [`73e98e9`](https://github.com/shapeshed/wenmoon/commit/73e98e984e1d365e3f141c73b0c27e922668c78e) Bump openssl from 0.10.71 to 0.10.72 (#124)
- [`ff62842`](https://github.com/shapeshed/wenmoon/commit/ff628426be41f02255a3b4170df9d58e991c3191) Bump tokio from 1.44.0 to 1.45.0 (#128)
- [`36327d4`](https://github.com/shapeshed/wenmoon/commit/36327d41119e3e893d6f89215b9916ad476f47a3) Bump clap from 4.5.31 to 4.5.38 (#129)
- [`fa2b02e`](https://github.com/shapeshed/wenmoon/commit/fa2b02e0fdbba46cb89326256c82c618a8130478) Bump nix from 0.29.0 to 0.30.1 (#132)
- [`3580d3d`](https://github.com/shapeshed/wenmoon/commit/3580d3debc95f4d3b886541a22f25c9efd942e28) Bump tabled from 0.18.0 to 0.20.0 (#137)
- [`8f2b971`](https://github.com/shapeshed/wenmoon/commit/8f2b97190e6785f86df0a3decf2a9d96711c1d77) Bump which from 7.0.2 to 8.0.0 (#136)
- [`6dda3db`](https://github.com/shapeshed/wenmoon/commit/6dda3dbf4d254f3d6bd400daccd12cc650e59fba) Bump Rust to 1.88.0 (#143)
- [`3de71d1`](https://github.com/shapeshed/wenmoon/commit/3de71d1def8e5e5c978cbad32e83577efb51a8f9) Bump tokio from 1.45.0 to 1.47.1 (#142)
- [`061caf0`](https://github.com/shapeshed/wenmoon/commit/061caf096ce9fbc5869f2c1f757ac1c869205e46) Bump toml from 0.8.20 to 0.9.4 (#141)
- [`439320f`](https://github.com/shapeshed/wenmoon/commit/439320f1c4d6cfbaceac06699fd540e175a70bb5) Bump clap from 4.5.38 to 4.5.43 (#148)
- [`8d97eee`](https://github.com/shapeshed/wenmoon/commit/8d97eeed67bb98f478fd2d766aa30b4463ce0152) Bump openssl from 0.10.72 to 0.10.73 (#147)
- [`c4270c3`](https://github.com/shapeshed/wenmoon/commit/c4270c3abb5d4e948658fd9174e854c97ff3465a) Bump serde_json from 1.0.140 to 1.0.142 (#146)
- [`dd0eb02`](https://github.com/shapeshed/wenmoon/commit/dd0eb02b523f10d62a9a5c29f25641c64f3bbc4f) Bump reqwest from 0.12.15 to 0.12.22 (#145)
- [`b6d78c2`](https://github.com/shapeshed/wenmoon/commit/b6d78c2063e79176ed952fc8b04294078e86b869) Bump serde from 1.0.219 to 1.0.223 (#156)
- [`b44fac5`](https://github.com/shapeshed/wenmoon/commit/b44fac5f06533169410203be928c83c9522219a5) Bump serde_json from 1.0.142 to 1.0.145 (#155)
- [`71d4b23`](https://github.com/shapeshed/wenmoon/commit/71d4b23ed215ba2854ffd9fbd796988b92119fa7) Bump clap from 4.5.43 to 4.5.47 (#154)
- [`d4eea79`](https://github.com/shapeshed/wenmoon/commit/d4eea7956149df3a512c2fa5ec5bde276d3a2ec4) Bump reqwest from 0.12.22 to 0.12.23 (#150)
- [`eb00615`](https://github.com/shapeshed/wenmoon/commit/eb00615731b2dd8f1fcfdd5fa4344af81d83b8b1) Bump async-trait from 0.1.88 to 0.1.89 (#149)
- [`6b73750`](https://github.com/shapeshed/wenmoon/commit/6b737504433a9f831aa1aeef9b2633f9d4ffd8e2) Bump toml from 0.9.5 to 0.9.8 (#161)
- [`d55f863`](https://github.com/shapeshed/wenmoon/commit/d55f863a9cca009f393f2820ccdb9eb80e11255c) Bump serde from 1.0.223 to 1.0.228 (#160)
- [`a1da56e`](https://github.com/shapeshed/wenmoon/commit/a1da56e08f0184f2978b691095b22a396ac90152) Bump clap from 4.5.47 to 4.5.48 (#158)
- [`5fa6135`](https://github.com/shapeshed/wenmoon/commit/5fa6135fb5bc75adcf73f146ac822cfadb6ed9cd) Bump openssl from 0.10.73 to 0.10.74 (#165)
- [`31ba392`](https://github.com/shapeshed/wenmoon/commit/31ba392be1beba3a605350990c9c2d7958d7f37c) Bump tokio from 1.47.1 to 1.48.0 (#164)
- [`368b0cb`](https://github.com/shapeshed/wenmoon/commit/368b0cbfd870453181f44d6af45c03aeb68bc6c6) Bump clap from 4.5.48 to 4.5.49 (#163)
- [`5ef32a5`](https://github.com/shapeshed/wenmoon/commit/5ef32a5326b2914eea61ada275cf50f18f1fd564) Bump reqwest from 0.12.23 to 0.12.24 (#162)

### Fixed

- [`a36f948`](https://github.com/shapeshed/wenmoon/commit/a36f948157270e38aa98b59a70d8d67bca543bff) Fix README formatting
- [`2663e63`](https://github.com/shapeshed/wenmoon/commit/2663e63f788649fa200777de4a79d99e4be4360e) Fix handling of nullable provider API fields
- [`d12957e`](https://github.com/shapeshed/wenmoon/commit/d12957e645076e94351b5f61c5427c73a1bf58c8) Fix release script

### Others

- [`bf93fd7`](https://github.com/shapeshed/wenmoon/commit/bf93fd746317061c9bbe4d98a2c94932f8bdd2d4) Update Swatinem/rust-cache action to v2.8.1 (#167)
- [`bff30f8`](https://github.com/shapeshed/wenmoon/commit/bff30f8bbf72575058d3580c1a1f16ecca287822) Update softprops/action-gh-release action to v2 (#170)
- [`6cb237b`](https://github.com/shapeshed/wenmoon/commit/6cb237b7a531001808e129b73176ac727647c364) Update GitHub Artifact Actions (#169)
- [`82c57f5`](https://github.com/shapeshed/wenmoon/commit/82c57f512f756191e4cd7aa8aa7a0dc06235c869) Update actions/checkout action to v5 (#168)
- [`9b28e7d`](https://github.com/shapeshed/wenmoon/commit/9b28e7d2396589f1f3c9ed5c39fc78c7dc38ba7e) Update Rust crate clap to v4.5.50 (#166)
- [`151b13c`](https://github.com/shapeshed/wenmoon/commit/151b13c7fccb5b4736f0ce03b5d33746cdaa6674) Move to renovate
- [`c61022b`](https://github.com/shapeshed/wenmoon/commit/c61022b7f5665be123bce3bb7c8179d3f74e7579) Migrate to json5
- [`fda6eaf`](https://github.com/shapeshed/wenmoon/commit/fda6eafdd3cd37e372c348e51c35a04216c7069c) Edit renovate.json5
- [`11d7b34`](https://github.com/shapeshed/wenmoon/commit/11d7b34acfd33c9190eafdbaa191a136222ab809) Migrate config .github/renovate.json5 (#172)
- [`67e54e6`](https://github.com/shapeshed/wenmoon/commit/67e54e61ca3f9747b635db98eee870eed21985af) Run renovate prs any time
- [`a40ef26`](https://github.com/shapeshed/wenmoon/commit/a40ef26f15d3b807cfd43b472b13f0d7bd7f3892) Update Rust crate clap to v4.5.51 (#173)
- [`f1f341d`](https://github.com/shapeshed/wenmoon/commit/f1f341dc1aa85b7147685aeb75d3eed231ef423b) Use default prs
- [`eeef8b1`](https://github.com/shapeshed/wenmoon/commit/eeef8b1ad6c559222b0c39036c143838df4f6c23) Update Rust crate clap to v4.5.52 (#175)
- [`8fea749`](https://github.com/shapeshed/wenmoon/commit/8fea74930a4b0dcbc79efb8c20b60e6e3128c2b0) Update Rust crate reqwest to v0.12.25 (#179)
- [`b0eedf7`](https://github.com/shapeshed/wenmoon/commit/b0eedf71382e6fc85538339625071dcae96801a1) Update Swatinem/rust-cache action to v2.8.2 (#178)
- [`66630cc`](https://github.com/shapeshed/wenmoon/commit/66630cc911789dbf55bdb240c13255066b85200f) Update actions/checkout action to v6 (#177)
- [`8dec2fd`](https://github.com/shapeshed/wenmoon/commit/8dec2fd6cf3b81e7f2631df5808444457dabb5d7) Update Rust crate clap to v4.5.53 (#176)
- [`7fc37ed`](https://github.com/shapeshed/wenmoon/commit/7fc37ed17913f823be88e78b3b4e303020c44ca2) Update Rust crate openssl to v0.10.75 (#174)
- [`d4026a2`](https://github.com/shapeshed/wenmoon/commit/d4026a206ae9fa611dee4c6cb423abb784812f4c) Update GitHub Artifact Actions (#180)
- [`2517eae`](https://github.com/shapeshed/wenmoon/commit/2517eaefd351daa4f12319dda520f6738cd1bd73) Update Rust crate reqwest to v0.12.26 (#181)
- [`773234f`](https://github.com/shapeshed/wenmoon/commit/773234f1ab7490549c953a69fd76cfc658d6e020) Update Rust crate toml to v0.9.11 (#182)
- [`5c20973`](https://github.com/shapeshed/wenmoon/commit/5c20973abd9b8a46f077db50c60d7d637ab8e649) Update Rust crate serde_json to v1.0.149 (#183)
- [`fd5d597`](https://github.com/shapeshed/wenmoon/commit/fd5d597ec1e5342217f824d4b3b90f374f870601) Update Rust crate nix to 0.31.0 (#189)
- [`b315bb0`](https://github.com/shapeshed/wenmoon/commit/b315bb040383846136be61ad748f27ec66a26efd) Update Rust crate colored to v3.1.1 (#188)
- [`7c83b26`](https://github.com/shapeshed/wenmoon/commit/7c83b26752ccb8d888119203e7f32446d6b6ed53) Update Rust crate tokio to v1.49.0 (#187)
- [`80a50dc`](https://github.com/shapeshed/wenmoon/commit/80a50dcf78c660438d10b2467095f1face284e94) Update Rust crate clap to v4.5.56 (#186)
- [`c055eb2`](https://github.com/shapeshed/wenmoon/commit/c055eb2de31025e251dbc570347c2058611eaab8) Update Rust crate reqwest to 0.13 (#185)
- [`8515129`](https://github.com/shapeshed/wenmoon/commit/85151294acc432feabae94cacb3e4cd235be7c13) Update Rust crate reqwest to v0.13.4 (#191)
- [`a00eeef`](https://github.com/shapeshed/wenmoon/commit/a00eeef1022055d3c8efbd59ddfaf98714611275) Update Rust crate clap to v4.6.1 (#190)
- [`0f24cff`](https://github.com/shapeshed/wenmoon/commit/0f24cff98862bd0c99ce6a637078f9aab9ed7638) Update Rust crate toml to v0.9.12 (#192)
- [`b1996e9`](https://github.com/shapeshed/wenmoon/commit/b1996e9966faaf0dad768f8de508affdad1088c5) Update Rust crate tabled to 0.21.0 (#201)
- [`613f9fb`](https://github.com/shapeshed/wenmoon/commit/613f9fb6a1900d529b6abf6ff4954392cfb33e7d) Update Rust crate serde_json to v1.0.150 (#200)
- [`ba2c40a`](https://github.com/shapeshed/wenmoon/commit/ba2c40aca7d3481bbe08b40820b04d2c65a65aa7) Update Swatinem/rust-cache action to v2.9.1 (#199)
- [`6b0cc5d`](https://github.com/shapeshed/wenmoon/commit/6b0cc5daf04835de30c9a924014685a65ee7ec99) Update Rust crate openssl to v0.10.80 (#198)
- [`564e736`](https://github.com/shapeshed/wenmoon/commit/564e7360185a25a969159e7133b654d3ba2a4756) Update Rust crate toml to v1 (#193)
- [`8e95d0d`](https://github.com/shapeshed/wenmoon/commit/8e95d0d39374f2ea7e403aa7eb7cdfd5e0ba1e2d) Update Rust crate nix to v0.31.3 (#195)
- [`c304013`](https://github.com/shapeshed/wenmoon/commit/c30401374f7a895b63df2dd698ffc2157307b628) Update Rust crate which to v8.0.2 (#197)
- [`9e2825b`](https://github.com/shapeshed/wenmoon/commit/9e2825b70063e78eaa197745bcdc4140f8038231) Update GitHub Artifact Actions (#194)
- [`6efda3f`](https://github.com/shapeshed/wenmoon/commit/6efda3ffec7d66bf6c29fb27ca40d08f281c434b) Update Rust crate tokio to v1.52.3 (#196)
- [`fdb0761`](https://github.com/shapeshed/wenmoon/commit/fdb0761292fe687bda283717b97c20a061a34fec) Update softprops/action-gh-release action to v3 (#202)
- [`6455cd2`](https://github.com/shapeshed/wenmoon/commit/6455cd260f73cee94953f5630ca01e784107d299) Update Rust crate which to v8.0.3 (#203)
- [`f62eb2c`](https://github.com/shapeshed/wenmoon/commit/f62eb2c3528b14d9c7667176a0cf2d57cde49835) Update Rust crate toml to v1.1.3 (#207)
- [`3f85da9`](https://github.com/shapeshed/wenmoon/commit/3f85da99904e9eab45f7ea5f80d51ec3798585ac) Update Rust crate openssl to v0.10.81 (#204)
- [`642e08c`](https://github.com/shapeshed/wenmoon/commit/642e08c637d91a60606e3d0dbb6fc9818e75af2d) Update actions/checkout action to v7 (#206)
- [`c1249f1`](https://github.com/shapeshed/wenmoon/commit/c1249f1e2312521a9d567b51aec41687e8d70bc1) Update Rust crate which to v8.0.4 (#205)
- [`2aa8d09`](https://github.com/shapeshed/wenmoon/commit/2aa8d091b4e5a22625b6736b263d8e8582991ce9) Update Rust crate which to v8.0.5 (#208)
- [`b31f579`](https://github.com/shapeshed/wenmoon/commit/b31f579e4003a3761f704a7b145f51c6aebcb7de) Update Swatinem/rust-cache action to v2.9.2 (#215)
- [`1c3dda9`](https://github.com/shapeshed/wenmoon/commit/1c3dda98745bcec2c914feb5482d17f0dd3dc320) Update Rust crate toml to v1.1.4 (#214)
- [`2131746`](https://github.com/shapeshed/wenmoon/commit/2131746a5b9d80053b737a80f1a4bfabe421e054) Update Rust crate serde_json to v1.0.151 (#213)
- [`240b4ef`](https://github.com/shapeshed/wenmoon/commit/240b4ef2f28ad25a26f363b86b868f21c731bb62) Update serde monorepo to v1.0.229 (#212)
- [`3a18527`](https://github.com/shapeshed/wenmoon/commit/3a1852718122a637d3aa045bd1586097d38bb1ec) Update Rust crate tokio to v1.53.1 (#210)
- [`2c5bb16`](https://github.com/shapeshed/wenmoon/commit/2c5bb16bbe2c7c20fe2f054c8a2beb3270aa3702) Update Rust crate openssl to v0.10.82 (#223)
- [`9ac4703`](https://github.com/shapeshed/wenmoon/commit/9ac4703fd1afbbb66e7474e27ebbdcf22cd86859) Update Rust crate async-trait to v0.1.92 (#211)
- [`69f95fe`](https://github.com/shapeshed/wenmoon/commit/69f95fe5e8b29ad20f7fec41eb3a73d74d3d0ef2) Update Rust crate clap to v4.6.7 (#209)
- [`ae0e1af`](https://github.com/shapeshed/wenmoon/commit/ae0e1af5e491f161e16da9be698d13b9ca20561a) Only rebase patch updates on conflict (#226)
- [`dbd789c`](https://github.com/shapeshed/wenmoon/commit/dbd789cd0a90930ffa7ae8d6f73ed816c135aa08) Update Rust crate reqwest to v0.13.5 (#219)
- [`fd16f91`](https://github.com/shapeshed/wenmoon/commit/fd16f9145690b9c6c9f352ebf76ec54cce9c9488) Update Rust crate serde_json to v1.0.154 (#221)
- [`971ecf0`](https://github.com/shapeshed/wenmoon/commit/971ecf0accad66082ec85df8bcddb2f1ec89e2a2) Update Rust crate which to v8.0.6 (#216)
- [`fcb3313`](https://github.com/shapeshed/wenmoon/commit/fcb331314cb168171bd674959685276417841255) Update Swatinem/rust-cache action to v2.9.3 (#222)
- [`6bf3ab6`](https://github.com/shapeshed/wenmoon/commit/6bf3ab629134328342fecd2224543db20733c0ee) Update Rust crate toml to v1.2.1 (#217)
- [`9cd5528`](https://github.com/shapeshed/wenmoon/commit/9cd5528a8fafb097b7824f30f1098e29aabd9a0e) Update Rust crate tokio to v1.54.1 (#220)
- [`6e75e70`](https://github.com/shapeshed/wenmoon/commit/6e75e70b7b48ee43a2179734e3c2928e8ff7ead2) Update Rust crate tabled to 0.22.0 (#218)

### Removed

- [`5ed6ea5`](https://github.com/shapeshed/wenmoon/commit/5ed6ea5b8c9ad13d54cd499d663354952663832c) Remove onboarding
- [`c9ec48f`](https://github.com/shapeshed/wenmoon/commit/c9ec48ffd617bce275c37d36b86325c30def3e27) Remove unused dependencies and add lint metadata

## [0.4.0] - 2025-03-02

### Changed

- [`3c386c7`](https://github.com/shapeshed/wenmoon/commit/3c386c7b34e7b49d0bcdf5048a25f65587d7fdc7) Refactor config to use toml
- [`dae421d`](https://github.com/shapeshed/wenmoon/commit/dae421dc64dd552a9f2ae4c34ff116aa029d4833) Update README for toml

### Dependencies

- [`a4e665c`](https://github.com/shapeshed/wenmoon/commit/a4e665c6052707a44fd4a280850b5b9e7e922bfd) Bump tabled from 0.16.0 to 0.18.0 (#103)
- [`fa80953`](https://github.com/shapeshed/wenmoon/commit/fa8095370a07acb9fc86f5fae7fba651f067f6a2) Bump clap from 4.5.23 to 4.5.29 (#108)
- [`65c0f16`](https://github.com/shapeshed/wenmoon/commit/65c0f16a9cf00ce9ebf2298e96af172efc5ee448) Bump tokio from 1.42.0 to 1.43.0 (#107)
- [`0308162`](https://github.com/shapeshed/wenmoon/commit/0308162e5df7cfd10edcdf04fedcd74558209351) Bump reqwest from 0.12.9 to 0.12.12 (#106)
- [`060bfbd`](https://github.com/shapeshed/wenmoon/commit/060bfbd0b14badd23d66e66e8d0e4284e9dc199c) Bump openssl from 0.10.68 to 0.10.71 (#104)
- [`60384f5`](https://github.com/shapeshed/wenmoon/commit/60384f55082fd77ae620133024b22cfbe6690e8e) Bump async-trait from 0.1.83 to 0.1.86 (#105)
- [`0814145`](https://github.com/shapeshed/wenmoon/commit/081414594bb1f510aa855cbbc8c920f3a23672ec) Bump serde from 1.0.217 to 1.0.218 (#111)
- [`e044c34`](https://github.com/shapeshed/wenmoon/commit/e044c343e3d3f36709f24a17dfbf97f2193458ec) Bump clap from 4.5.29 to 4.5.30 (#110)
- [`f27eb56`](https://github.com/shapeshed/wenmoon/commit/f27eb5654db50e794193dfcf3e042ac548d87ff1) Bump serde_json from 1.0.138 to 1.0.139 (#109)

## [0.3.3] - 2025-02-10

### Fixed

- [`aabb03a`](https://github.com/shapeshed/wenmoon/commit/aabb03af992dca1d16739fc53995119ef66dbd9b) Fix macos build

### Others

- [`feba42b`](https://github.com/shapeshed/wenmoon/commit/feba42b9871189a3003a73a49961c01c42ea495d) Release 0.3.3

## [0.3.2] - 2025-02-10

### Dependencies

- [`584e1ec`](https://github.com/shapeshed/wenmoon/commit/584e1ec6a1a361c042b27931215ddaee1cbc851b) Bump reqwest from 0.12.2 to 0.12.3 (#30)
- [`3a8e9e8`](https://github.com/shapeshed/wenmoon/commit/3a8e9e89ec8edaf6050ca7331c32e348d0124934) Bump async-trait from 0.1.79 to 0.1.80 (#31)
- [`90f31ec`](https://github.com/shapeshed/wenmoon/commit/90f31ec2dd0f63f36a776bccd17949ac6cbce6ad) Bump serde from 1.0.197 to 1.0.198 (#32)
- [`e1365fb`](https://github.com/shapeshed/wenmoon/commit/e1365fb11d9c7cf43464ca53b7d50e0774359efe) Bump reqwest from 0.12.3 to 0.12.4 (#34)
- [`1302205`](https://github.com/shapeshed/wenmoon/commit/13022051d03eec85c48c38295c29aa204f2ab90c) Bump serde_json from 1.0.115 to 1.0.116 (#33)
- [`e1d71bd`](https://github.com/shapeshed/wenmoon/commit/e1d71bdbaea632fd637bdc8e8a7b00716934d375) Bump serde from 1.0.198 to 1.0.199 (#35)
- [`45f99a9`](https://github.com/shapeshed/wenmoon/commit/45f99a99756b17e544731fdc55ae2b4b4266f1b0) Bump serde from 1.0.199 to 1.0.200 (#36)
- [`e3351fc`](https://github.com/shapeshed/wenmoon/commit/e3351fcb306f1d6f0f1b8b9d29ca1fde20551e77) Bump serde from 1.0.200 to 1.0.201 (#37)
- [`e493d18`](https://github.com/shapeshed/wenmoon/commit/e493d18f00a9e61a21119a22bab3394afdc4d364) Bump serde_json from 1.0.116 to 1.0.117 (#38)
- [`7608fb5`](https://github.com/shapeshed/wenmoon/commit/7608fb581f0d16811072413a092da540ecabbdca) Bump nix from 0.28.0 to 0.29.0 (#41)
- [`bba9ab5`](https://github.com/shapeshed/wenmoon/commit/bba9ab5d920731c5336ac01921e5a5313bf62a49) Bump serde from 1.0.201 to 1.0.203 (#40)
- [`d1c47bf`](https://github.com/shapeshed/wenmoon/commit/d1c47bf605c25d7befff79792afe0a70c98b326c) Bump async-trait from 0.1.80 to 0.1.81 (#49)
- [`198ea5d`](https://github.com/shapeshed/wenmoon/commit/198ea5d86ef2214b57150e95ec66e9c6dec267b9) Bump tokio from 1.37.0 to 1.38.0 (#42)
- [`6c213ae`](https://github.com/shapeshed/wenmoon/commit/6c213ae641aaf1098f1e6da2f57b8c8d44cf1a27) Bump reqwest from 0.12.4 to 0.12.5 (#45)
- [`ae3c1a1`](https://github.com/shapeshed/wenmoon/commit/ae3c1a1b0c44b5b212e39e0a8b9f71d08ae73f0a) Bump clap from 4.5.4 to 4.5.8 (#46)
- [`665852e`](https://github.com/shapeshed/wenmoon/commit/665852e3a8ebba69dacb289ff1e466e9e7f86b40) Bump serde_json from 1.0.117 to 1.0.120 (#48)
- [`c5c19a9`](https://github.com/shapeshed/wenmoon/commit/c5c19a9b4f2c80862a9507624bd8eb4aa9145268) Bump serde from 1.0.203 to 1.0.204 (#51)
- [`08f1671`](https://github.com/shapeshed/wenmoon/commit/08f1671cf8ecbaf380502caaf8539cea2a6a2355) Bump clap from 4.5.8 to 4.5.9 (#50)
- [`b02ffda`](https://github.com/shapeshed/wenmoon/commit/b02ffda0f2ed040fbeecfea8069e12e070d72291) Bump tokio from 1.38.0 to 1.38.1 (#52)
- [`401f0bf`](https://github.com/shapeshed/wenmoon/commit/401f0bffbab24333ea9ea068027c928ea694c9c3) Bump openssl from 0.10.64 to 0.10.66 (#53)
- [`82b8ad0`](https://github.com/shapeshed/wenmoon/commit/82b8ad0327b9d0cd0d1ab5bd25572cc1d31b6a70) Bump clap from 4.5.9 to 4.5.11 (#56)
- [`3460a0d`](https://github.com/shapeshed/wenmoon/commit/3460a0dc76550465641a8a09f99f5a4bcb6e7af4) Bump serde_json from 1.0.120 to 1.0.121 (#55)
- [`2cb6c6a`](https://github.com/shapeshed/wenmoon/commit/2cb6c6a7c321a63ca16d57572caa6310c8b68b36) Bump tokio from 1.38.1 to 1.39.2 (#54)
- [`ecd161f`](https://github.com/shapeshed/wenmoon/commit/ecd161f2e9053105efbe9bd5f3f5ae6613b2946e) Bump clap from 4.5.11 to 4.5.13 (#59)
- [`a86b321`](https://github.com/shapeshed/wenmoon/commit/a86b321621e1cb07ab6df13438922526bd96b6a0) Bump serde_json from 1.0.121 to 1.0.122 (#58)
- [`b4c6669`](https://github.com/shapeshed/wenmoon/commit/b4c6669d31e81ac12d0ac1146cf34b378c43ec3f) Bump which from 6.0.1 to 6.0.2 (#57)
- [`ae77fb7`](https://github.com/shapeshed/wenmoon/commit/ae77fb7f188b88db7e6d08cb1a40c9641aa899e5) Bump tabled from 0.15.0 to 0.16.0 (#61)
- [`3ec4f4b`](https://github.com/shapeshed/wenmoon/commit/3ec4f4bf4f447eedb98ab7a035b5c8a305f7cdef) Bump serde_json from 1.0.122 to 1.0.125 (#67)
- [`b6fb404`](https://github.com/shapeshed/wenmoon/commit/b6fb404ef7e486ee282494d77f17e595b492dc19) Bump serde from 1.0.204 to 1.0.208 (#66)
- [`e2184a3`](https://github.com/shapeshed/wenmoon/commit/e2184a3d47c07742425c255b955e3032c0d6c379) Bump which from 6.0.2 to 6.0.3 (#64)
- [`0ba5df8`](https://github.com/shapeshed/wenmoon/commit/0ba5df8d2621f5398c26500abd4fd0024f00a64d) Bump clap from 4.5.13 to 4.5.16 (#65)
- [`35688f0`](https://github.com/shapeshed/wenmoon/commit/35688f0ded2f1545336360aee5e71f9b8afb02bf) Bump tokio from 1.39.2 to 1.40.0 (#72)
- [`d16c604`](https://github.com/shapeshed/wenmoon/commit/d16c6042560905a62ee25b11ff3514a85ca0a1bb) Bump serde_json from 1.0.125 to 1.0.128 (#74)
- [`a7bb698`](https://github.com/shapeshed/wenmoon/commit/a7bb698c6a89c2d9f916627f7b7eb8b100429e2c) Bump serde from 1.0.208 to 1.0.210 (#75)
- [`1f73ef7`](https://github.com/shapeshed/wenmoon/commit/1f73ef76f9e353c8e49602b14266a177bd135134) Bump async-trait from 0.1.81 to 0.1.83 (#76)
- [`4c916b7`](https://github.com/shapeshed/wenmoon/commit/4c916b73a5fa05eadf345d1a1d7c3f2d73e85d12) Bump reqwest from 0.12.5 to 0.12.8 (#77)
- [`da56421`](https://github.com/shapeshed/wenmoon/commit/da564212455f2f9ef095a80726400e164fbecbe2) Bump serde_json from 1.0.128 to 1.0.132 (#78)
- [`69211cb`](https://github.com/shapeshed/wenmoon/commit/69211cbe28278421bd2e7afdd8206801eec9e2af) Bump clap from 4.5.16 to 4.5.20 (#79)
- [`2f949e0`](https://github.com/shapeshed/wenmoon/commit/2f949e068c9bd4a6815ddadec6709d40c741222f) Bump openssl from 0.10.66 to 0.10.68 (#80)
- [`50c673f`](https://github.com/shapeshed/wenmoon/commit/50c673f6a00226e39853383275194e40ecaa962f) Bump serde from 1.0.210 to 1.0.213 (#81)
- [`1d34cbc`](https://github.com/shapeshed/wenmoon/commit/1d34cbcabf9f9760a7b4444be736783b109395a4) Bump tokio from 1.40.0 to 1.41.0 (#82)
- [`42a68a3`](https://github.com/shapeshed/wenmoon/commit/42a68a3566252e9761a6b89be0f29e287ac29c90) Bump reqwest from 0.12.8 to 0.12.9 (#85)
- [`5d809cb`](https://github.com/shapeshed/wenmoon/commit/5d809cb73d71286e7b9adb1f4256d51a447fb5ff) Bump which from 6.0.3 to 7.0.0 (#84)
- [`881948b`](https://github.com/shapeshed/wenmoon/commit/881948b5556ef6258fc4c4ea3236941fdfe1b4ba) Bump serde from 1.0.213 to 1.0.214 (#83)
- [`15db939`](https://github.com/shapeshed/wenmoon/commit/15db9398f79bdcacdb8eac1b5f2a359de1165193) Bump tokio from 1.41.0 to 1.41.1 (#86)
- [`eb9f03f`](https://github.com/shapeshed/wenmoon/commit/eb9f03fc65187c0a445c35c9c6ba2fe21276a0a5) Bump serde_json from 1.0.132 to 1.0.133 (#87)
- [`7ee4645`](https://github.com/shapeshed/wenmoon/commit/7ee4645eb5d5e70a77e159ee43bb5c1b3b3e3be9) Bump clap from 4.5.20 to 4.5.21 (#89)
- [`93ac07a`](https://github.com/shapeshed/wenmoon/commit/93ac07a76eb5da8ded3c58e0dddac855f9c1241d) Bump serde from 1.0.214 to 1.0.215 (#88)
- [`0d6ecdd`](https://github.com/shapeshed/wenmoon/commit/0d6ecdda5d68cef251ca8361c6dd57f80e964ac2) Bump clap from 4.5.21 to 4.5.23 (#91)
- [`411b224`](https://github.com/shapeshed/wenmoon/commit/411b224d3a6b094cee526c9a41b762d947e8211c) Bump tokio from 1.41.1 to 1.42.0 (#92)
- [`f4a4fea`](https://github.com/shapeshed/wenmoon/commit/f4a4fea3c6b1440e8d60d2ee8720cff3291bc61f) Bump serde from 1.0.215 to 1.0.217 (#97)
- [`0b15a59`](https://github.com/shapeshed/wenmoon/commit/0b15a59e9a1db1298577e835261d7e9b0317d6ba) Bump colored from 2.1.0 to 3.0.0 (#98)
- [`7977650`](https://github.com/shapeshed/wenmoon/commit/7977650163fcd11b69b3c2872e23beaff951f297) Bump serde_json from 1.0.133 to 1.0.138 (#101)
- [`9dd62b7`](https://github.com/shapeshed/wenmoon/commit/9dd62b70daaaecd4bedcd8e307b910c6ecceeb38) Bump which from 7.0.0 to 7.0.2 (#102)
- [`d178b1d`](https://github.com/shapeshed/wenmoon/commit/d178b1db0f5e30bc5afffd517fce160ab5fb1bfd) Bump rust version

### Fixed

- [`eb3c71a`](https://github.com/shapeshed/wenmoon/commit/eb3c71ad4fba40bdb59b2672abb72dcd925ff04c) Fix CHANGELOG
- [`3127b90`](https://github.com/shapeshed/wenmoon/commit/3127b903f9b3e68919458a1bd55f1eeff9dc1d10) Fix CHANGELOG

### Others

- [`cf5075e`](https://github.com/shapeshed/wenmoon/commit/cf5075ee65cc8dbd2b1ab04c2a88390f1367e917) Release 0.3.2

## [0.3.0] - 2024-04-01

### Changed

- [`0f3d2d1`](https://github.com/shapeshed/wenmoon/commit/0f3d2d1e736184f45b15920b30815c43ab68b403) Update Cargo.lock

### Dependencies

- [`e977fe9`](https://github.com/shapeshed/wenmoon/commit/e977fe98ed4ae3883e1bb4bd398407131939fa04) Bump separator from 0.3.1 to 0.4.1 (#12)
- [`2aa9d93`](https://github.com/shapeshed/wenmoon/commit/2aa9d933f7697059bf8c7fee4f33e65346891d89) Bump reqwest from 0.11.24 to 0.11.25 (#16)
- [`d3f537b`](https://github.com/shapeshed/wenmoon/commit/d3f537be78d257f595f31927aaa8d5c235f47929) Bump async-trait from 0.1.77 to 0.1.78 (#19)
- [`bd68df4`](https://github.com/shapeshed/wenmoon/commit/bd68df4df1376dc52b2911806b7ac8f446a92755) Bump serde_yaml from 0.9.32 to 0.9.33 (#18)
- [`95fb72d`](https://github.com/shapeshed/wenmoon/commit/95fb72d8985ccde53a4f1d6d6e2151b0821e03ef) Bump reqwest from 0.11.25 to 0.11.26 (#17)
- [`4479f34`](https://github.com/shapeshed/wenmoon/commit/4479f34d5fcc49e00d88692af8baf99fe7eecf5a) Bump which from 6.0.0 to 6.0.1 (#21)
- [`d4c889f`](https://github.com/shapeshed/wenmoon/commit/d4c889f83f08eb950810bfc36a6461d3b15c6518) Bump reqwest from 0.11.26 to 0.12.1 (#24)
- [`87c3f80`](https://github.com/shapeshed/wenmoon/commit/87c3f801e9c1ecad78dd7c6622838f2e0e041cf8) Bump async-trait from 0.1.78 to 0.1.79 (#22)
- [`7bb31ba`](https://github.com/shapeshed/wenmoon/commit/7bb31ba2d696825b77223d690c7f96efb111a228) Bump serde_yaml from 0.9.33 to 0.9.34+deprecated (#23)
- [`885991b`](https://github.com/shapeshed/wenmoon/commit/885991b5d80103bc4031fbefc443b64201267958) Bump serde_json from 1.0.114 to 1.0.115 (#28)
- [`7180b2c`](https://github.com/shapeshed/wenmoon/commit/7180b2c12efc204b1a41de386de6dc6c1fdeca75) Bump tokio from 1.36.0 to 1.37.0 (#27)
- [`07f625e`](https://github.com/shapeshed/wenmoon/commit/07f625e269fba3d094fc4aad7d648985751e5155) Bump reqwest from 0.12.1 to 0.12.2 (#25)
- [`bda313d`](https://github.com/shapeshed/wenmoon/commit/bda313d38325cd180752c685326e9f15e4d59631) Upgrade to clap 0.4.x (#29)

## [0.2.0] - 2024-03-04

### Changed

- [`6dd0584`](https://github.com/shapeshed/wenmoon/commit/6dd05843c9ada3a8fe60942bf7372418ac45eff2) Change api client to use traits (#10)

### Others

- [`20b9995`](https://github.com/shapeshed/wenmoon/commit/20b99957114484dadfbd2241b6beeb4042506ed8) Document sort (#8)
- [`c1130cc`](https://github.com/shapeshed/wenmoon/commit/c1130ccb278457d49da7f45822e634970f8c6db0) Add CoinGecko (#13)
- [`dc72303`](https://github.com/shapeshed/wenmoon/commit/dc72303b601c47a760c9ccfe51faa347c6d6968b) Commit Cargo.lock (#14)

### Removed

- [`780f311`](https://github.com/shapeshed/wenmoon/commit/780f31151c5736a37887cc002fceae366f26a3f9) Remove display file (#9)

## [0.1.0] - 2024-03-02

### Added

- [`703be14`](https://github.com/shapeshed/wenmoon/commit/703be14c3387ee598648f477ba6777ca9c0560b0) Add crates.io badge (#3)

### Changed

- [`abcd0e1`](https://github.com/shapeshed/wenmoon/commit/abcd0e1a00f33032cf52865cef4d60643a294c0c) Change price to use three decimals

### Dependencies

- [`a842b5f`](https://github.com/shapeshed/wenmoon/commit/a842b5f05eb68d549ebd74bb56936699492746cd) Bump version (#5)

### Fixed

- [`1404c07`](https://github.com/shapeshed/wenmoon/commit/1404c077d7939baaf261f4dde8b8954df90d2251) Fix bottom row display
- [`2e852ac`](https://github.com/shapeshed/wenmoon/commit/2e852acf44be07cd44e66fd454ba1fb2976e2768) Fix release task
- [`3a4cbbf`](https://github.com/shapeshed/wenmoon/commit/3a4cbbf8e4b8eeca516f52f41506546b4e36f754) Fix release

### Others

- [`7e0dfe5`](https://github.com/shapeshed/wenmoon/commit/7e0dfe51b9e79f85e673c8d9a7c0f651950c519e) Add hourly weekly (#6)
- [`0e5eaa6`](https://github.com/shapeshed/wenmoon/commit/0e5eaa6bb579f6909b82ea4d669d45f3c27d362d) Add monthly (#7)

## [0.0.2] - 2024-02-27

### Fixed

- [`994f41f`](https://github.com/shapeshed/wenmoon/commit/994f41f997f5a5c10dabcd80e41bfb7dfdf5fc9d) Fix license name
- [`c1826aa`](https://github.com/shapeshed/wenmoon/commit/c1826aa0cb56a7ae4150e4b50c783ac7c608f841) Fix layout issue with BTC price

### Others

- [`7567e1b`](https://github.com/shapeshed/wenmoon/commit/7567e1b2f3db300038268504a91c4537be50d6d0) Set version
- [`f2ac0fb`](https://github.com/shapeshed/wenmoon/commit/f2ac0fb54d990fce3484281d94bedbc3cfd72c4e) Fix license name (#1)
- [`33009d1`](https://github.com/shapeshed/wenmoon/commit/33009d140baf5ddd4389b666b07b3105da25e9dd) Merge pull request #2 from shapeshed/fix-price-colum

## [0.0.1] - 2024-02-27

### Others

- [`b39e90c`](https://github.com/shapeshed/wenmoon/commit/b39e90cf5162e6925bfb1e823bbbb5d03941b703) Initial commit

[0.5.0]: https://github.com/shapeshed/wenmoon/compare/0.4.0..0.5.0
[0.4.0]: https://github.com/shapeshed/wenmoon/compare/0.3.3..0.4.0
[0.3.3]: https://github.com/shapeshed/wenmoon/compare/0.3.2..0.3.3
[0.3.2]: https://github.com/shapeshed/wenmoon/compare/0.3.0..0.3.2
[0.3.0]: https://github.com/shapeshed/wenmoon/compare/0.2.0..0.3.0
[0.2.0]: https://github.com/shapeshed/wenmoon/compare/0.1.0..0.2.0
[0.1.0]: https://github.com/shapeshed/wenmoon/compare/0.0.2..0.1.0
[0.0.2]: https://github.com/shapeshed/wenmoon/compare/0.0.1..0.0.2
[0.0.1]: https://github.com/shapeshed/wenmoon/releases/tag/0.0.1

