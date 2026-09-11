#include "ansa_info.h"

/*
 * ANSA firmware information.
 *
 * These values are development values for now.
 * Final board/version/build identification should be
 * defined by the ANSA project configuration.
 */

#define ANSA_FW_VERSION    "0.1.0"
#define ANSA_BOARD_NAME    "STM32F411CEU6"
#define ANSA_BUILD_TYPE    "DEV"

const char *ANSA_INFO_GetFirmwareVersion(void)
{
    return ANSA_FW_VERSION;
}

const char *ANSA_INFO_GetBoardName(void)
{
    return ANSA_BOARD_NAME;
}

const char *ANSA_INFO_GetBuildType(void)
{
    return ANSA_BUILD_TYPE;
}
