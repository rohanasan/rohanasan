#include <ctype.h>
#include <stdlib.h>
#include <string.h>


// Help for this function taken from:
// https://stackoverflow.com/a/14530993

extern inline void __internal_url_decoding_system__(char *dst, const char *src) {
  char a, b;
  while (*src) {
    if ((*src == '%') && ((a = src[1]) && (b = src[2])) && (isxdigit(a) && isxdigit(b))) {
      if (a >= 'a')
        a -= 'a' - 'A';
      if (a >= 'A')
        a -= ('A' - 10);
      else
        a -= '0';
      if (b >= 'a')
        b -= 'a' - 'A';
      if (b >= 'A')
        b -= ('A' - 10);
      else
        b -= '0';
      *dst++ = 16 * a + b;
      src += 3;
    } else if (*src == '+') {
      *dst++ = ' ';
      src++;
    } else {
      *dst++ = *src++;
    }
  }
  *dst++ = '\0';
}

extern inline const char *uri_decode(const char *input) {
  char *output = (char *)malloc(strlen(input) + 1);
  __internal_url_decoding_system__(output, input);
  return (const char*)output;
}