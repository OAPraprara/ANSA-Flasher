#ifndef ANSA_TIME_H
#define ANSA_TIME_H

#include <stdint.h>

/**
 * @brief Initialize the ANSA system time base.
 *
 * Configures the Cortex-M4 SysTick timer to generate
 * one interrupt every 1 millisecond.
 *
 * @return 0 on success.
 */
int ANSA_TIME_Init(void);

/**
 * @brief Get the current system time in milliseconds.
 *
 * @return Number of milliseconds elapsed since initialization.
 */
uint32_t ANSA_TIME_GetMilliseconds(void);

/**
 * @brief Increment the system millisecond counter.
 *
 * This function is called from the SysTick interrupt handler.
 */
void ANSA_TIME_Tick(void);

#endif /* ANSA_TIME_H */
