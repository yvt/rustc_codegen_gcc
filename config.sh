set -e

export CARGO_INCREMENTAL=0

if [ -f ./gcc_path ]; then 
    export GCC_PATH=$(cat gcc_path)
else
    echo 'Please put the path to your custom build of libgccjit in the file `gcc_path`, see Readme.md for details'
    exit 1
fi


unamestr=`uname`
if [[ "$unamestr" == 'Linux' ]]; then
   dylib_ext='so'
elif [[ "$unamestr" == 'Darwin' ]]; then
   dylib_ext='dylib'
else
   echo "Unsupported os"
   exit 1
fi

HOST_TRIPLE=$(rustc -vV | grep host | cut -d: -f2 | tr -d " ")
TARGET_TRIPLE=$HOST_TRIPLE
#TARGET_TRIPLE="m68k-unknown-linux-gnu"
TARGET_TRIPLE="rx-elf"

linker=''
RUN_WRAPPER=''
if [[ "$HOST_TRIPLE" != "$TARGET_TRIPLE" ]]; then
   if [[ "$TARGET_TRIPLE" == "m68k-unknown-linux-gnu" ]]; then
       TARGET_TRIPLE="mips-unknown-linux-gnu"
       linker='-Clinker=m68k-linux-gcc'
   elif [[ "$TARGET_TRIPLE" == "rx-elf" ]]; then
       TARGET_TRIPLE="rx-none-elf"
       TARGET_SPEC="$(pwd)/rx-none-elf.json"
       linker='-Clinker=rx-elf-gcc'
       export CC_${TARGET_TRIPLE//-/_}="rx-elf-gcc"
   elif [[ "$TARGET_TRIPLE" == "aarch64-unknown-linux-gnu" ]]; then
      # We are cross-compiling for aarch64. Use the correct linker and run tests in qemu.
      linker='-Clinker=aarch64-linux-gnu-gcc'
      RUN_WRAPPER='qemu-aarch64 -L /usr/aarch64-linux-gnu'
   else
      echo "Unknown non-native platform"
   fi
fi

if [ -z "$TARGET_SPEC" ]; then
   TARGET_SPEC="$TARGET_TRIPLE"
fi
export TARGET_SPEC

target_triple_upper="$(echo "$TARGET_TRIPLE" | tr '[:lower:]-' '[:upper:]_')"

codegen_dylib="$(pwd)/target/${CHANNEL:-debug}/librustc_codegen_gcc.$dylib_ext"
rustflags="$linker -Cpanic=abort -Csymbol-mangling-version=v0 -Cdebuginfo=2 -Clto=off -Zpanic-abort-tests -Zcodegen-backend=$codegen_dylib --sysroot $(pwd)/build_sysroot/sysroot"
export LD_PRELOAD="$codegen_dylib${LD_PRELOAD:+:}$LD_PRELOAD"

# FIXME(antoyo): remove once the atomic shim is gone
if [[ `uname` == 'Darwin' ]]; then
   rustflags="$rustflags -Clink-arg=-undefined -Clink-arg=dynamic_lookup"
fi

RUSTC="rustc $rustflags -L crate=target/out --out-dir target/out"
export RUSTC_LOG=warn # display metadata load errors

export CARGO_TARGET_${target_triple_upper}_RUSTFLAGS="$rustflags"

export LD_LIBRARY_PATH="$(pwd)/target/out:$(pwd)/build_sysroot/sysroot/lib/rustlib/$TARGET_TRIPLE/lib:$GCC_PATH"
export DYLD_LIBRARY_PATH=$LD_LIBRARY_PATH
