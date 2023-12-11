#include <dlfcn.h>
#include "my_application.h"

#define DSHLDESK_LIB_PATH "libdshldesk.so"
// #define DSHLDESK_LIB_PATH "/usr/lib/dshldesk/libdshldesk.so"
typedef bool (*DshlDeskCoreMain)();
bool gIsConnectionManager = false;

bool flutter_dshldesk_core_main() {
   void* libdshldesk = dlopen(DSHLDESK_LIB_PATH, RTLD_LAZY);
   if (!libdshldesk) {
     fprintf(stderr,"load libdshldesk.so failed\n");
     return true;
   }
   auto core_main = (DshlDeskCoreMain) dlsym(libdshldesk,"dshldesk_core_main");
   char* error;
   if ((error = dlerror()) != nullptr) {
       fprintf(stderr, "error finding dshldesk_core_main: %s", error);
       return true;
   }
   return core_main();
}

int main(int argc, char** argv) {
  if (!flutter_dshldesk_core_main()) {
      return 0;
  }
  for (int i = 0; i < argc; i++) {
    if (strcmp(argv[i], "--cm") == 0) {
      gIsConnectionManager = true;
    }
  }
  g_autoptr(MyApplication) app = my_application_new();
  return g_application_run(G_APPLICATION(app), argc, argv);
}
