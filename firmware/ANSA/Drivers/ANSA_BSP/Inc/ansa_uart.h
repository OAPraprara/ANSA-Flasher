#ifndef ANSA_UART_H
#define ANSA_UART_H

#include "stm32f411xe.h"
#include <stdint.h>

/**
 * @brief Initialize ANSA UART using USART2.
 *
 * Configures USART2 for:
 * 115200 baud, 8 data bits, no parity, 1 stop bit.
 *
 * @return 0 on success.
 */
int ANSA_UART_Init(void);

/**
 * @brief Transmit one character through ANSA UART.
 *
 * @param data Character to transmit.
 */
void ANSA_UART_WriteChar(uint8_t data);

/**
 * @brief Transmit a string through ANSA UART.
 *
 * @param str Null-terminated string.
 */
void ANSA_UART_WriteString(const char *str);

/**
 * @brief Process one received UART character.
 *
 * Accumulates characters into a command buffer and
 * processes a command when a line terminator is received.
 *
 * @param data Received UART character.
 */
void ANSA_UART_ProcessChar(uint8_t data);
void ANSA_UART_Process(void);

#endif /* ANSA_UART_H */
