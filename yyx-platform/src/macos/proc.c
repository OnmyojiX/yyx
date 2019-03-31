#include <libproc.h>
#include <unistd.h>

static char Path[PROC_PIDPATHINFO_MAXSIZE] = {0};

char *get_self_path()
{
  pid_t pid = getpid();
  int ret = proc_pidpath(pid, Path, sizeof(Path));
  if (ret <= 0)
  {
    return 0;
  }
  else
  {
    return Path;
  }
}