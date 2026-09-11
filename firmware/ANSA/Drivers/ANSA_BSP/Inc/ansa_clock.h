#ifndef ANSA_CLOCK_H
#define ANSA_CLOCK_H

#include "stm32f411xe.h"
#include <stdint.h>

/**
 * @brief Initialize the system clock.
 *
 * Configures the STM32F411 system clock to use
 * the internal 16 MHz HSI oscillator.
 *
 * Clock configuration:
 * SYSCLK = 16 MHz
 * AHB    = 16 MHz
 * APB1   = 16 MHz
 * APB2   = 16 MHz
 *
 * @return 0 on success.
 */
int ANSA_CLOCK_Init(void);

/**
 * @brief Get the current system clock frequency.
 *
 * @return System clock frequency in Hz.
 */
uint32_t ANSA_CLOCK_GetFrequency(void);

#endif /* ANSA_CLOCK_H */
