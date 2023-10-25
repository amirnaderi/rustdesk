#include <dlfcn.h>
#include "my_application.h"

#define RUSTDESK_LIB_PATH "libdshelpdesk.so"
// #define RUSTDESK_LIB_PATH "/usr/lib/dshelpdesk/libdshelpdesk.so"
typedef bool (*DsHelpDeskCoreMain)();
bool gIsConnectionManager = false;

bool flutter_dshelpdesk_core_main() {
   void* libdshelpdesk = dlopen(RUSTDESK_LIB_PATH, RTLD_LAZY);
   if (!libdshelpdesk) {
     fprintf(stderr,"load libdshelpdesk.so failed\n");
     return true;
   }
   auto core_main = (DsHelpDeskCoreMain) dlsym(libdshelpdesk,"dshelpdesk_core_main");
   char* error;
   if ((error = dlerror()) != nullptr) {
       fprintf(stderr, "error finding dshelpdesk_core_main: %s", error);
       return true;
   }
   return core_main();
}

int main(int argc, char** argv) {
  if (!flutter_dshelpdesk_core_main()) {
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
