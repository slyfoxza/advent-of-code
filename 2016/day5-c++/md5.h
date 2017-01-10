#pragma once
#if defined(USE_OPENSSL)
#include "md5-openssl.h"
#elif defined(USE_CNG)
#include "md5-cng.h"
#endif
