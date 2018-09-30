# __A Pure Rust Implementation of an Elliptic Curve Keypair Generator in an Intel SGX Enclave__

## __:black_nib: Notes:__

More specifically, an Secp256k1 key pair generator & message signer where both the enclave _and_ the app are written in pure Rust. Made possible by the fantastic Rust SGX Software Developer Kit by Baidux Labs:
https://github.com/baidu/rust-sgx-sdk

&nbsp;

## __:page_with_curl: Instructions:__

__❍ Pull requisite files:__

_Pull the Rust SGX SDK Docker image_

_**`❍ sgx-nuc@~$ docker pull baiduxlab/sgx-rust`**_

_Clone this Repo_

_**`❍ sgx-nuc@~$ git clone https://gitlab.com/gskapka/secp256k1-enclave-rust.git`**_

&nbsp;

_**❍ Prepare the Docker Container:**_

If using __SIMULATION__ mode:

_**`❍ sgx-nuc@~$ sudo docker run -v /path/to/secp256k1-enclave-rust:/root/keygen -ti baiduxlab/sgx-rust`**_

Else if using __HARDWARE__ mode:

_**`❍ sgx-nuc@~$ sudo docker run -v/path/to/secp256k1-enclave-rust:/root/keygen -ti --device /dev/isgx baiduxlab/sgx-rust`**_

Rebuild the tool chain:

_**`❍ sgx-nuc-docker@~# rustup default nightly-2018-08-25-x86_64-unknown-linux-gnu`**_

Add required components:

_**`❍ sgx-nuc-docker@~# rustup component add rust-src`**_

Finally, if using __HARDWARE__ mode, import the service:_

_**`❍ sgx-nuc-docker@~# /opt/intel/sgxpsw/aesm/aesm_service &`**_

&nbsp;

__❍ Prepare the keygen:__

In the `❍ sgx-nuc-docker@~/keygen` directory inside the docker, first ensure the desired mode (__SW__ or __HW__) is set correctly inside the `Makefile`:

```javascript
    // ... Beginning of file ...

    ######## SGX SDK Settings ########

    SGX_SDK ?= /opt/intel/sgxsdk
    SGX_MODE ?= HW                // <-- This option. HW for Hardware or SW for software.
    SGX_ARCH ?= x64

    // ... Remainder of file ...
```

Next, set the environment variable inside the docker to the desired mode:

_**`❍ sgx-nuc-docker@~/keygen# export SGX_MODE=HW`**_

Then build the project:

_**`❍ sgx-nuc-docker@~/keygen# make`**_

And finally run it to see the usage notes:

_**`❍ sgx-nuc-docker@~/keygen# cd bin && ./ethkeysgx`**_

&nbsp;

## __:clipboard: To Do List:__

:white_check_mark: Refactor to lib crate.

:white_check_mark: Test on real nuc in HW mode.

:white_check_mark: Make CLI with Docopt.

:black_square_button: Remotely attest!

:black_square_button: Make ECRecoverable sigs.

:black_square_button: Stream to the enc. to allow file encryption.

:black_square_button: Add rudimentary password protection.

:black_square_button: Use mister enclave instead of mister signer!

:black_square_button: Make threaded to have vanity addresses (can limit tried?)

:black_square_button: Abstract out generic enclave funcs (mem. allocing etc)

:white_check_mark: Separate the app from the SDK repo enclave to make it lean and mean.

:black_square_button: Make a stand alone binary for D/L.

&nbsp;

## __:books: Resources:__

[__❍__ Here's some stuff about the __EDL__ file](https://software.intel.com/en-us/documentation/intel-sgx-web-based-training/the-enclave-definition-language)

[__❍__ Here's some stuff about the Makefile syntax:](https://www3.nd.edu/~zxu2/acms60212-40212/Makefile.pdf)
