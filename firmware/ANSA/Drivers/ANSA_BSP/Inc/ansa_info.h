#ifndef ANSA_INFO_H
#define ANSA_INFO_H

/**
 * @brief Get ANSA firmware version string.
 *
 * @return Null-terminated firmware version string.
 */
const char *ANSA_INFO_GetFirmwareVersion(void);

/**
 * @brief Get ANSA target board string.
 *
 * @return Null-terminated board identification string.
 */
const char *ANSA_INFO_GetBoardName(void);

/**
 * @brief Get ANSA build type string.
 *
 * @return Null-terminated build type string.
 */
const char *ANSA_INFO_GetBuildType(void);

#endif /* ANSA_INFO_H */
