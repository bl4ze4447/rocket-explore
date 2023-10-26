#include "harddisk.h"

BOOL C_WRAPPER_get_disk_space(LPCSTR root, DiskSpace *ds) {
    return GetDiskFreeSpaceA(root, &ds->sectorsPerCluster, &ds->bytesPerSector, &ds->numberOfFreeClusters, &ds->totalNumberOfClusters);
}

const char *C_WRAPPER_get_disks() {
    char *buf = malloc(256 * sizeof(char));
    if (!buf) {
        return FALSE;
    }

    DWORD ok = GetLogicalDriveStringsA(256, buf);
    if (ok < 1) {
        return FALSE;
    }

    const char *nbuf = malloc(ok * sizeof(char));
    if (!nbuf) {
        return FALSE;
    }

    return nbuf;
}