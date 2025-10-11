#ifndef _ZGC_OS_H_
#define _ZGC_OS_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <assert.h>
#include <ctype.h>

#include <regex.h>

#if !defined(WINDOWS)
#include <dirent.h>

#if !defined(_ALPINE) && !defined(TD_ASTRA)
#include <execinfo.h>
#endif

#if !defined(TD_ASTRA)
#include <libgen.h>
#include <wordexp.h>
#include <sys/param.h>
#include <sys/shm.h>
#include <sys/statvfs.h>
#include <termios.h>
#else
#include <astra.h>
#endif

#include <sched.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/utsname.h>
#include <sys/wait.h>

#if defined(DARWIN)
#include <pwd.h>
#else
#if !defined(TD_ASTRA)
#include <argp.h>
#include <sys/prctl.h>
#include <sys/sysinfo.h>
#if defined(_TD_X86_)
#include <cpuid.h>
#endif
#endif
#endif
#else

#ifndef __func__
#define __func__ __FUNCTION__
#endif
#include <malloc.h>
#include <time.h>
#ifndef TD_USE_WINSOCK
#include <winsock2.h>
#else
#include <winsock.h>
#endif
#endif

#include <errno.h>
#include <fcntl.h>
#include <float.h>
#include <inttypes.h>
#include <limits.h>
#include <locale.h>
#include <math.h>
#include <setjmp.h>
#include <signal.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <wchar.h>
#include <wctype.h>

#if __AVX__
#include <immintrin.h>
#elif __SSE4_2__
#include <nmmintrin.h>
#endif


#include "osThread.h"

#include "osAtomic.h"
#include "osDef.h"
#include "osDir.h"
#include "osEndian.h"
#include "osEnv.h"
#include "osFile.h"
#include "osLocale.h"
#include "osLz4.h"
#include "osMath.h"
#include "osMemory.h"
#include "osMemPool.h"
#include "osRand.h"
#include "osSemaphore.h"
#include "osSignal.h"
#include "osSleep.h"
#include "osSocket.h"
#include "osString.h"
#include "osSysinfo.h"
#include "osSystem.h"
#include "osTime.h"
#include "osTimer.h"
#include "osTimezone.h"
#include "taoserror.h"
#include "tlog.h"

extern int32_t          tsRandErrChance;
extern int64_t          tsRandErrDivisor;
extern int64_t          tsRandErrScope;
extern threadlocal bool tsEnableRandErr;

#define TAOS_UNUSED(expr) (void)(expr)
#define TAOS_SKIP_ERROR(expr) \
  {                           \
    int32_t _code = terrno;   \
    (void)(expr);             \
    terrno = _code;           \
  }

#define OS_PARAM_CHECK(_o)             \
  do {                                 \
    if ((_o) == NULL) {                \
      terrno = TSDB_CODE_INVALID_PARA; \
      return terrno;                   \
    }                                  \
  } while (0)

// NOTE: use TD_ALWAYS_ASSERT to enforce assertion even in release build
//       this is for test cases to use!!!
#define TD_ALWAYS_ASSERT(pred)                                  \
  if (!(pred)) {                                                \
    fprintf(stderr, "Assertion `%s` failed.\n", #pred);         \
    abort();                                                    \
  }

#ifdef __cplusplus
}
#endif

#endif