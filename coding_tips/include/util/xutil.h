#ifndef _ZGC_XUTIL_H_
#define _ZGC_XUTIL_H_

#define TAOS_RETURN(CODE)     \
  do {                        \
    return (terrno = (CODE)); \
  } while(0)


#endif