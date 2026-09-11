#ifndef ANSA_LED_H
#define ANSA_LED_H

#include "stm32f411xe.h"
#include <stdint.h>

/**
 * @brief Initialize the ANSA status LED.
 *
 * @return 0 on success.
 */
int ANSA_LED_Init(void);

/**
 * @brief Turn the ANSA status LED on.
 */
void ANSA_LED_On(void);

/**
 * @brief Turn the ANSA status LED off.
 */
void ANSA_LED_Off(void);

/**
 * @brief Toggle the ANSA status LED.
 */
void ANSA_LED_Toggle(void);
void ANSA_LED_Fault(void);

#endif /* ANSA_LED_H */
