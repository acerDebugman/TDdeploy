



支持 openssl: 

在 TDengine/cmake/external.cmake 里，有 ssl 部分涉及 平台组件的编译，修改这部分，来支持 windows 上编译，比如 libcurl 就已经支持：

```
# ssl
if(NOT ${TD_WINDOWS})       # {
    # TODO: why at this moment???
    # file(MAKE_DIRECTORY $ENV{HOME}/.cos-local.2/)
    if(${TD_LINUX})
        set(ext_ssl_static libssl.a)
        set(ext_crypto_static libcrypto.a)
    elseif(${TD_DARWIN})
        set(ext_ssl_static libssl.a)
        set(ext_crypto_static libcrypto.a)
    endif()
    INIT_EXT(ext_ssl
        INC_DIR          include
        LIB              lib/${ext_ssl_static}
                         lib/${ext_crypto_static}
        # debugging github working flow
        # CHK_NAME         SSL
    )
    list(SUBLIST ext_ssl_libs 0 1 ext_ssl_lib_ssl)
    list(SUBLIST ext_ssl_libs 1 1 ext_ssl_lib_crypto)
    # URL https://github.com/openssl/openssl/releases/download/openssl-3.1.3/openssl-3.1.3.tar.gz
    # URL_HASH SHA256=f0316a2ebd89e7f2352976445458689f80302093788c466692fb2a188b2eacf6
    get_from_local_if_exists("https://github.com/openssl/openssl/releases/download/openssl-3.1.3/openssl-3.1.3.tar.gz")
    ExternalProject_Add(ext_ssl
        URL ${_url}
        URL_HASH SHA256=f0316a2ebd89e7f2352976445458689f80302093788c466692fb2a188b2eacf6
        # GIT_SHALLOW TRUE
        PREFIX "${_base}"
        BUILD_IN_SOURCE TRUE
        CMAKE_ARGS -DCMAKE_BUILD_TYPE:STRING=${TD_CONFIG_NAME}
        CMAKE_ARGS -DCMAKE_INSTALL_PREFIX:STRING=${_ins}
        CONFIGURE_COMMAND
            # COMMAND ./Configure --prefix=$ENV{HOME}/.cos-local.2 no-shared
            COMMAND ./Configure --prefix=${_ins} no-shared --libdir=lib
        BUILD_COMMAND
            COMMAND make -j4
        INSTALL_COMMAND
            COMMAND make install_sw -j4
        EXCLUDE_FROM_ALL TRUE
        VERBATIM
    )
    add_dependencies(build_externals ext_ssl)     # this is for github workflow in cache-miss step.
endif(NOT ${TD_WINDOWS})    # }

# libcurl
if(${TD_LINUX})
    set(ext_curl_static libcurl.a)
    set(_c_flags_list -fPIC)
elseif(${TD_DARWIN})
    set(ext_curl_static libcurl.a)
    set(_c_flags_list)
elseif(${TD_WINDOWS})
    set(ext_curl_static libcurl$<$<STREQUAL:${TD_CONFIG_NAME},Debug>:-d>.lib)
    set(_c_flags_list)
endif()

INIT_EXT(ext_curl
    INC_DIR          include
    LIB              lib/${ext_curl_static}
    # currently: tqStreamNotify.c uses curl_ws_send, but CURL4_OPENSSL exports curl_easy_send
    #            libcurl4-openssl-dev on ubuntu 22.04 is too old
    # CHK_NAME         CURL4_OPENSSL
)

if(${TD_WINDOWS})
    # URL https://github.com/curl/curl/releases/download/curl-8_2_1/curl-8.2.1.tar.gz
    # URL_HASH MD5=b25588a43556068be05e1624e0e74d41
    get_from_local_if_exists("https://github.com/curl/curl/releases/download/curl-8_2_1/curl-8.2.1.tar.gz")
    ExternalProject_Add(ext_curl
        URL ${_url}
        URL_HASH MD5=b25588a43556068be05e1624e0e74d41
        PREFIX "${_base}"
        CMAKE_ARGS -DCMAKE_BUILD_TYPE:STRING=${TD_CONFIG_NAME}
        CMAKE_ARGS -DCMAKE_INSTALL_PREFIX:STRING=${_ins}
        CMAKE_ARGS -DCMAKE_INSTALL_LIBDIR:PATH=lib
        CMAKE_ARGS -DBUILD_SHARED_LIBS:BOOL=OFF
        CMAKE_ARGS -DBUILD_TESTING:BOOL=OFF
        CMAKE_ARGS -DBUILD_CURL_EXE:BOOL=OFF
        CMAKE_ARGS -DENABLE_WEBSOCKETS:BOOL=ON
        CMAKE_ARGS -DCURL_USE_SCHANNEL:BOOL=ON
        CMAKE_ARGS -DCURL_USE_OPENSSL:BOOL=OFF
        CMAKE_ARGS -DCURL_ZLIB:BOOL=OFF
        CMAKE_ARGS -DCURL_DISABLE_LDAP:BOOL=ON
        CMAKE_ARGS -DCURL_DISABLE_LDAPS:BOOL=ON
        BUILD_COMMAND
            COMMAND "${CMAKE_COMMAND}" --build . --config "${TD_CONFIG_NAME}"
        INSTALL_COMMAND
            COMMAND "${CMAKE_COMMAND}" --install . --config "${TD_CONFIG_NAME}" --prefix "${_ins}"
        EXCLUDE_FROM_ALL TRUE
        VERBATIM
    )
else()
    string(JOIN " " _c_flags ${_c_flags_list})
    # URL https://github.com/curl/curl/releases/download/curl-8_2_1/curl-8.2.1.tar.gz
    # URL_HASH MD5=b25588a43556068be05e1624e0e74d41
    get_from_local_if_exists("https://github.com/curl/curl/releases/download/curl-8_2_1/curl-8.2.1.tar.gz")
    ExternalProject_Add(ext_curl
        URL ${_url}
        URL_HASH MD5=b25588a43556068be05e1624e0e74d41
        # GIT_SHALLOW TRUE
        DEPENDS ext_ssl
        PREFIX "${_base}"
        BUILD_IN_SOURCE TRUE
        CMAKE_ARGS -DCMAKE_BUILD_TYPE:STRING=${TD_CONFIG_NAME}
        CMAKE_ARGS -DCMAKE_INSTALL_PREFIX:STRING=${_ins}
        CONFIGURE_COMMAND
            # COMMAND ./Configure --prefix=$ENV{HOME}/.cos-local.2 no-shared
            COMMAND ./configure --prefix=${_ins} --with-ssl=${ext_ssl_install}
                    --enable-websockets --enable-shared=no --disable-ldap
                    --disable-ldaps --without-brotli --without-zstd
                    --without-libidn2 --without-nghttp2 --without-libpsl
                    --without-librtmp #--enable-debug
                    CFLAGS=${_c_flags}
                    CXXFLAGS=${_c_flags}
        BUILD_COMMAND
            COMMAND make -j4
        INSTALL_COMMAND
            COMMAND make install
        EXCLUDE_FROM_ALL TRUE
        VERBATIM
    )
endif()
add_dependencies(build_externals ext_curl)     # this is for github workflow in cache-miss step.

```





1. 安装 strawberryperl , 到 strawberryperl.com 安装即可

2. 安装 nasm

```
Invoke-WebRequest -Uri "https://www.nasm.us/pub/nasm/releasebuilds/2.16.03/win64/nasm-2.16.03-installer-x64.exe" -OutFile "C:\workspace\0\nasm-installer.exe"

Start-Process -FilePath "C:\workspace\0\nasm-installer.exe" -ArgumentList "/S" -Wait

$env:PATH = "C:\Program Files\NASM;" + $env:PATH

# 验证
nasm -v

```

3. 









