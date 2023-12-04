# We document platform support for minimum glibc 2.17 and kernel 3.2.
# CentOS 7 has headers for kernel 3.10, but that's fine as long as we don't
# actually use newer APIs in rustc or std without a fallback. It's more
# important that we match glibc for ELF symbol versioning.
FROM centos:7

WORKDIR /build

RUN yum upgrade -y && \
    yum install -y epel-release && \
    yum install -y \
      bzip2 \
      file \
      git \
      python3 && \
    yum clean all

# Foo 2

RUN mkdir -p /rustroot/bin

ENV PATH=/rustroot/bin:$PATH
ENV LD_LIBRARY_PATH=/rustroot/lib64:/rustroot/lib32:/rustroot/lib
ENV PKG_CONFIG_PATH=/rustroot/lib/pkgconfig
WORKDIR /tmp
RUN mkdir /home/user
COPY host-x86_64/dist-x86_64-linux/shared.sh /tmp/

# rustc-perf version from 2023-05-30
# Should also be changed in the opt-dist tool for other environments.
ENV PERF_COMMIT 8b2ac3042e1ff2c0074455a0a3618adef97156b1

ENV PGO_HOST=x86_64-unknown-linux-gnu

ENV HOSTS=x86_64-unknown-linux-gnu

ENV RUST_CONFIGURE_ARGS \
      --enable-full-tools \
      --enable-sanitizers \
      --enable-profiler \
      --enable-compiler-docs \
      --set llvm.download-ci-llvm=if-available

ENV SCRIPT ls
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang

# This is the only builder which will create source tarballs
ENV DIST_SRC 1

# When we build cargo in this container, we don't want it to use the system
# libcurl, instead it should compile its own.
ENV LIBCURL_NO_PKG_CONFIG 1

ENV DIST_REQUIRE_ALL_TOOLS 1
