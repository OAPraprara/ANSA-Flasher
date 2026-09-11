#include "ansa_uart.h"
#include "ansa_info.h"

#define ANSA_UART_COMMAND_BUFFER_SIZE    64U

static char ansa_uart_command_buffer[ANSA_UART_COMMAND_BUFFER_SIZE];
static uint32_t ansa_uart_command_length = 0U;

/*
 * USART2 configuration
 *
 * System clock : 16 MHz
 * APB1 clock   : 16 MHz
 * Baud rate    : 115200
 * Oversampling : 16
 */

#define ANSA_UART_PCLK1_HZ       16000000U
#define ANSA_UART_BAUDRATE       115200U

/*
 * USARTDIV = PCLK / (16 * Baud)
 *
 * BRR is represented in Q4 format when OVER8 = 0.
 *
 * BRR = PCLK / Baud
 */
#define ANSA_UART_BRR_VALUE      \
    ((ANSA_UART_PCLK1_HZ + (ANSA_UART_BAUDRATE / 2U)) / \
     ANSA_UART_BAUDRATE)


int ANSA_UART_Init(void)
{
    /*
     * Enable GPIOA clock.
     */
    RCC->AHB1ENR |= RCC_AHB1ENR_GPIOAEN;

    /*
     * Enable USART2 clock on APB1.
     */
    RCC->APB1ENR |= RCC_APB1ENR_USART2EN;

    /*
     * PA2 = USART2_TX
     * PA3 = USART2_RX
     *
     * Configure both pins as Alternate Function.
     */
    GPIOA->MODER &= ~(
        GPIO_MODER_MODER2_Msk |
        GPIO_MODER_MODER3_Msk
    );

    GPIOA->MODER |= (
        (2U << GPIO_MODER_MODER2_Pos) |
        (2U << GPIO_MODER_MODER3_Pos)
    );

    /*
     * Configure GPIO speed to very high speed.
     */
    GPIOA->OSPEEDR &= ~(
        GPIO_OSPEEDR_OSPEED2_Msk |
        GPIO_OSPEEDR_OSPEED3_Msk
    );

    GPIOA->OSPEEDR |= (
        (3U << GPIO_OSPEEDR_OSPEED2_Pos) |
        (3U << GPIO_OSPEEDR_OSPEED3_Pos)
    );

    /*
     * Configure push-pull output type.
     */
    GPIOA->OTYPER &= ~(
        GPIO_OTYPER_OT2 |
        GPIO_OTYPER_OT3
    );

    /*
     * No pull-up / pull-down.
     */
    GPIOA->PUPDR &= ~(
        GPIO_PUPDR_PUPD2_Msk |
        GPIO_PUPDR_PUPD3_Msk
    );

    /*
     * Select Alternate Function 7 for USART2.
     */
    GPIOA->AFR[0] &= ~(
        GPIO_AFRL_AFSEL2_Msk |
        GPIO_AFRL_AFSEL3_Msk
    );

    GPIOA->AFR[0] |= (
        (7U << GPIO_AFRL_AFSEL2_Pos) |
        (7U << GPIO_AFRL_AFSEL3_Pos)
    );

    /*
     * Disable USART2 before configuration.
     */
    USART2->CR1 &= ~USART_CR1_UE;

    /*
     * Configure baud rate.
     *
     * BRR = 16,000,000 / 115,200
     *     ≈ 138.89
     *     = 0x8B
     */
    USART2->BRR = ANSA_UART_BRR_VALUE;

    /*
     * Reset USART configuration registers.
     */
    USART2->CR1 = 0U;
    USART2->CR2 = 0U;
    USART2->CR3 = 0U;

    /*
     * Enable transmitter and receiver.
     */
    USART2->CR1 |= (
        USART_CR1_TE |
        USART_CR1_RE
    );

    /*
     * Enable USART2.
     */
    USART2->CR1 |= USART_CR1_UE;

    return 0;
}


void ANSA_UART_WriteChar(uint8_t data)
{
    /*
     * Wait until transmit data register is empty.
     */
    while ((USART2->SR & USART_SR_TXE) == 0U)
    {
        /* Wait */
    }

    /*
     * Write data to USART data register.
     */
    USART2->DR = data;

    /*
     * Wait until transmission is complete.
     */
    while ((USART2->SR & USART_SR_TC) == 0U)
    {
        /* Wait */
    }
}


void ANSA_UART_WriteString(const char *str)
{
    if (str == 0)
    {
        return;
    }

    while (*str != '\0')
    {
        ANSA_UART_WriteChar((uint8_t)*str);
        str++;
    }
}


/**
 * @brief Process one received UART character.
 *
 * Accumulates characters into a command buffer and
 * processes a command when a line terminator is received.
 *
 * @param data Received UART character.
 */
void ANSA_UART_ProcessChar(uint8_t data)
{
    /*
     * Process command when Enter is received.
     */
    if ((data == '\r') || (data == '\n'))
    {
        if (ansa_uart_command_length > 0U)
        {
            /*
             * Null-terminate the command.
             */
            ansa_uart_command_buffer[ansa_uart_command_length] = '\0';

            /*
             * Execute the "about" command.
             */
            if ((ansa_uart_command_buffer[0] == 'a') &&
                (ansa_uart_command_buffer[1] == 'b') &&
                (ansa_uart_command_buffer[2] == 'o') &&
                (ansa_uart_command_buffer[3] == 'u') &&
                (ansa_uart_command_buffer[4] == 't') &&
                (ansa_uart_command_buffer[5] == '\0'))
            {
                ANSA_UART_WriteString("\r\nANSA\r\n");

                ANSA_UART_WriteString("BOARD: ");
                ANSA_UART_WriteString(ANSA_INFO_GetBoardName());
                ANSA_UART_WriteString("\r\n");

                ANSA_UART_WriteString("FW: ");
                ANSA_UART_WriteString(ANSA_INFO_GetFirmwareVersion());
                ANSA_UART_WriteString("\r\n");

                ANSA_UART_WriteString("BUILD: ");
                ANSA_UART_WriteString(ANSA_INFO_GetBuildType());
                ANSA_UART_WriteString("\r\n");
            }
            else
            {
                /*
                 * Unknown command.
                 */
                ANSA_UART_WriteString(
                    "\r\nERROR: UNKNOWN COMMAND\r\n"
                );
            }
        }

        /*
         * Reset command buffer for the next command.
         */
        ansa_uart_command_length = 0U;
        ansa_uart_command_buffer[0] = '\0';

        return;
    }

    /*
     * Handle backspace.
     */
    if ((data == '\b') || (data == 127U))
    {
        if (ansa_uart_command_length > 0U)
        {
            ansa_uart_command_length--;

            ANSA_UART_WriteString("\b \b");
        }

        return;
    }

    /*
     * Ignore non-printable characters.
     */
    if ((data < 32U) || (data > 126U))
    {
        return;
    }

    /*
     * Store character if there is room for the
     * terminating null character.
     */
    if (ansa_uart_command_length <
        (ANSA_UART_COMMAND_BUFFER_SIZE - 1U))
    {
        ansa_uart_command_buffer[ansa_uart_command_length] =
            (char)data;

        ansa_uart_command_length++;

        /*
         * Echo received character.
         */
        ANSA_UART_WriteChar(data);
    }
}


/**
 * @brief Process received UART data.
 *
 * Checks USART2 for received characters and passes
 * them to the ANSA UART command parser.
 */
void ANSA_UART_Process(void)
{
    uint8_t data;

    /*
     * Check whether USART2 has received data.
     */
    if ((USART2->SR & USART_SR_RXNE) != 0U)
    {
        /*
         * Read received character.
         */
        data = (uint8_t)USART2->DR;

        /*
         * Pass character to command parser.
         */
        ANSA_UART_ProcessChar(data);
    }
}
