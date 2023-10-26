#include <windows.h>
#include <inttypes.h>

typedef struct DiskSpace {
    DWORD sectorsPerCluster;
    DWORD bytesPerSector;
    DWORD numberOfFreeClusters;
    DWORD totalNumberOfClusters;
}DiskSpace;

BOOL C_WRAPPER_get_disk_space(LPCSTR root, DiskSpace *ds);

const char *C_WRAPPER_get_disks();